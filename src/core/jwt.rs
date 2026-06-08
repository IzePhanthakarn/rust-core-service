use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

// โครงสร้างของข้อมูลที่จะฝังไปใน JWT (Payload)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,          // User ID
    pub exp: usize,         // เวลาหมดอายุ (Timestamp)
    pub iat: usize,         // เวลาที่สร้าง Token (Timestamp)
    pub token_version: i32, // สำหรับทำ Force Logout
    pub roles: Vec<String>, // Role ของ User
    pub token_type: String, // แยกประเภท Access / Refresh
}

// ฟังก์ชันสร้างทั้ง Access และ Refresh Token คืนค่าเป็น Tuple (access, refresh)
pub fn generate_tokens(
    user_id: Uuid,
    token_version: i32,
    roles: Vec<String>,
) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let now = Utc::now();

    // 1. สร้าง Access Token (อายุ 1 วัน)
    let access_exp = now + Duration::days(1);
    let access_claims = Claims {
        sub: user_id,
        exp: access_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_version,
        roles: roles.clone(),
        token_type: "access".to_string(),
    };

    let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(access_secret.as_bytes()),
    )?;

    // 2. สร้าง Refresh Token (อายุ 7 วัน)
    let refresh_exp = now + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id,
        exp: refresh_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_version,
        roles, // แนบ Role ไปด้วยเพื่อใช้ออก Access Token ใหม่ได้เลย
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

// === 2. เพิ่มฟังก์ชันสำหรับตรวจสอบ Refresh Token โดยเฉพาะ ===
pub fn verify_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
