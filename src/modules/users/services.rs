use diesel::PgConnection;
use uuid::Uuid;
use crate::core::errors::AppError;
use crate::modules::users::dtos::UpdateProfileRequest;
use crate::modules::users::models::{UserProfile, UserStatus};
use crate::modules::users::repositories::UserRepository;

pub struct UserService;

impl UserService {
    pub fn update_profile(conn: &mut PgConnection, target_user_id: Uuid, req: &UpdateProfileRequest) -> Result<UserProfile, AppError> {
        let profile = UserRepository::update_profile(conn, target_user_id, &req.first_name, &req.last_name)
            .map_err(|_| AppError::InternalServerError("เกิดข้อผิดพลาดที่ระบบฐานข้อมูล ไม่สามารถอัปเดตได้".to_string()))?;

        match profile {
            Some(p) => Ok(p),
            None => Err(AppError::BadRequest("ไม่พบข้อมูลโปรไฟล์ของคุณในระบบ (อาจถูกลบไปแล้ว)".to_string())),
        }
    }

    pub fn update_user_status(conn: &mut PgConnection, target_user_id: Uuid, new_status: &UserStatus) -> Result<(), AppError> {
        let updated_rows = UserRepository::update_user_status(conn, target_user_id, new_status)
            .map_err(|_| AppError::InternalServerError("ระบบฐานข้อมูลขัดข้อง ไม่สามารถเปลี่ยนสถานะได้".to_string()))?;

        if updated_rows == 0 {
            return Err(AppError::BadRequest("ไม่พบบัญชีผู้ใช้งานที่ต้องการเปลี่ยนสถานะ (อาจถูกลบไปแล้ว)".to_string()));
        }

        Ok(())
    }

    pub fn delete_user(conn: &mut PgConnection, target_user_id: Uuid) -> Result<(), AppError> {
        let updated_rows = UserRepository::delete_user(conn, target_user_id)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถลบบัญชีได้".to_string()))?;

        if updated_rows == 0 {
            return Err(AppError::BadRequest("ไม่พบบัญชีผู้ใช้งานที่ต้องการลบ หรือบัญชีนี้ถูกลบไปแล้ว".to_string()));
        }

        Ok(())
    }

    // ห่อหุ้มฟังก์ชันจาก Repository เพื่อให้ Auth Service/Handler เรียกใช้ง่ายๆ
    pub fn update_password(conn: &mut PgConnection, user_id: Uuid, new_password_hash: &str) -> Result<(), AppError> {
        UserRepository::update_password(conn, user_id, new_password_hash)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเปลี่ยนรหัสผ่านได้".to_string()))?;
        Ok(())
    }

    pub fn increment_token_version(conn: &mut PgConnection, user_id: Uuid) -> Result<(), AppError> {
        UserRepository::increment_token_version(conn, user_id)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถออกจากระบบได้".to_string()))?;
        Ok(())
    }
}