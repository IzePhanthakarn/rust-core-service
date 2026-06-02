use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct AssignRoleRequest {
    pub target_user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RevokeRoleRequest {
    pub target_user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, message = "กรุณาระบุชื่อ Role ที่ต้องการสร้าง"))]
    pub name: String,
    pub description: Option<String>,
}