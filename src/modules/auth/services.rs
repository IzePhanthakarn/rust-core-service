use crate::core::{errors::AppError, security::hash_password};
use crate::modules::{
    auth::dtos::RegisterRequest,
    users::{models::NewUser, repositories::UserRepository},
};
use diesel::PgConnection;

pub struct AuthService;

impl AuthService {
    pub fn register(conn: &mut PgConnection, req: RegisterRequest) -> Result<(), AppError> {
        // 1. เช็คอีเมลซ้ำ
        let existing_user = UserRepository::find_by_email(conn, &req.email)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("อีเมลนี้ถูกใช้งานแล้ว".to_string()));
        }

        // 2. แฮชรหัสผ่าน
        let hashed_password = hash_password(&req.password)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเข้ารหัสผ่านได้".to_string()))?;

        // 3. แฮชคำลับ (secret_word) ที่ผู้ใช้ส่งมา
        let hashed_secret_word = hash_password(&req.secret_word)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเข้ารหัสคำลับได้".to_string()))?;

        // 4. ผูกข้อมูลเพื่อเตรียม Insert
        let new_user = NewUser {
            email: req.email,
            password_hash: hashed_password,
            secret_word: Some(hashed_secret_word), // ใส่คำลับที่ผ่านการ Hash แล้วลงไป
        };

        // 5. บันทึกลง 3 ตารางรวดเดียวผ่าน Repository
        UserRepository::create_user_with_profile(
            conn, 
            new_user, 
            req.first_name, 
            req.last_name
        )
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 6. คืนค่าข้อความสำเร็จ
        Ok(())
    }
}
