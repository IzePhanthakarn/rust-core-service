use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// 1. Request for registering a new account
#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    // Names are taken together to also create the user_profiles record
    #[validate(length(min = 1, message = "Please enter your first name"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "Please enter a secret word for password recovery"))]
    pub secret_word: String,

    #[validate(length(min = 1, message = "Please enter your last name"))]
    pub last_name: String,
}

// 2. Request for logging in
#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

// 3. Response after a successful login/register
#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String, // Usually the word "Bearer"
    pub expires_in: u64,    // Lifetime of the access token (seconds)
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1, message = "Please provide a refresh token"))]
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Please enter the secret word for password recovery"))]
    pub secret_word: String,

    #[validate(length(min = 6, message = "New password must be at least 6 characters"))]
    pub new_password: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Please enter your old password"))]
    pub old_password: String,

    #[validate(length(min = 6, message = "New password must be at least 6 characters"))]
    pub new_password: String,
}
