use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

// Structure of the data embedded in the JWT (payload)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,          // User ID
    pub exp: usize,         // Expiration time (timestamp)
    pub iat: usize,         // Token creation time (timestamp)
    pub token_version: i32, // For forced logout
    pub role: String,       // User role
    pub token_type: String, // Distinguishes access / refresh
}

impl Claims {
    pub fn is_super_admin(&self) -> bool {
        self.role == "super_admin"
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role.as_str(), "super_admin" | "admin")
    }
}

// Function that generates both the access and refresh tokens, returned as a tuple (access, refresh)
pub fn generate_tokens(
    user_id: Uuid,
    token_version: i32,
    role: String,
) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let now = Utc::now();

    // 1. Create the access token (valid for 1 day)
    let access_exp = now + Duration::days(1);
    let access_claims = Claims {
        sub: user_id,
        exp: access_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_version,
        role: role.clone(),
        token_type: "access".to_string(),
    };

    let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(access_secret.as_bytes()),
    )?;

    // 2. Create the refresh token (valid for 7 days)
    let refresh_exp = now + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id,
        exp: refresh_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_version,
        role,
        token_type: "refresh".to_string(),
    };

    let refresh_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(refresh_secret.as_bytes()),
    )?;

    Ok((access_token, refresh_token))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

// === 2. Add a function specifically for verifying the refresh token ===
pub fn verify_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
