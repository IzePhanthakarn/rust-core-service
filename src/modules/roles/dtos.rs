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