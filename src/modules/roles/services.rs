use diesel::PgConnection;
use uuid::Uuid;
use crate::core::errors::AppError;
use crate::modules::roles::repositories::RoleRepository;
use crate::modules::users::repositories::UserRepository;

pub struct RoleService;

impl RoleService {
    pub fn assign_role(conn: &mut PgConnection, target_user_id: Uuid, role_id: Uuid, assigned_by: Uuid) -> Result<(), AppError> {
        RoleRepository::assign_role(conn, target_user_id, role_id, assigned_by)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถกำหนดสิทธิ์ได้".to_string()))?;
        Ok(())
    }

    pub fn revoke_role(conn: &mut PgConnection, target_user_id: Uuid, role_id_to_revoke: Uuid) -> Result<(), AppError> {
        let deleted_rows = RoleRepository::revoke_role(conn, target_user_id, role_id_to_revoke)
            .map_err(|_| AppError::InternalServerError("ระบบฐานข้อมูลขัดข้อง ไม่สามารถถอดสิทธิ์ได้".to_string()))?;

        if deleted_rows == 0 {
            return Err(AppError::BadRequest("ผู้ใช้งานคนนี้ไม่ได้มีสิทธิ์ดังกล่าวอยู่แล้ว หรือไม่มี Role นี้ในระบบ".to_string()));
        }

        // เพิ่ม token_version เพื่อบังคับให้ออกจากระบบ (เรียกข้ามไปที่ UserRepository เพราะมันแตะตาราง Users)
        let _ = UserRepository::increment_token_version(conn, target_user_id);

        Ok(())
    }
}