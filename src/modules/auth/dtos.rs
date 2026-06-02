use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// 1. Request สำหรับการสมัครสมาชิก
#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "รูปแบบอีเมลไม่ถูกต้อง"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "รหัสผ่านต้องมีอย่างน้อย 6 ตัวอักษร"))]
    pub password: String,
    
    // รับชื่อมาพร้อมกันเลยเพื่อไปสร้างตาราง user_profiles ด้วย
    #[validate(length(min = 1, message = "กรุณากรอกชื่อจริง"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "กรุณากรอกคำลับเพื่อการกู้คืนรหัสผ่าน"))]
    pub secret_word: String,
    
    #[validate(length(min = 1, message = "กรุณากรอกนามสกุล"))]
    pub last_name: String,
}

// 2. Request สำหรับการเข้าสู่ระบบ
#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "รูปแบบอีเมลไม่ถูกต้อง"))]
    pub email: String,
    pub password: String,
}

// 3. Response หลังจาก Login/Register สำเร็จ
#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String, // ปกติจะเป็นคำว่า "Bearer"
    pub expires_in: u64,    // อายุของ Access Token (วินาที)
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1, message = "กรุณาส่ง Refresh Token"))]
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email(message = "รูปแบบอีเมลไม่ถูกต้อง"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "กรุณากรอกคำลับเพื่อการกู้คืนรหัสผ่าน"))]
    pub secret_word: String,
    
    #[validate(length(min = 6, message = "รหัสผ่านใหม่ต้องมีอย่างน้อย 6 ตัวอักษร"))]
    pub new_password: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "กรุณากรอกรหัสผ่านเดิม"))]
    pub old_password: String,
    
    #[validate(length(min = 6, message = "รหัสผ่านใหม่ต้องมีอย่างน้อย 6 ตัวอักษร"))]
    pub new_password: String,
}