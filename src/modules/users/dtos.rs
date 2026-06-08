use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::modules::users::models::UserStatus; // ดึง Enum มาจาก models

#[derive(Serialize, ToSchema)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "กรุณากรอกชื่อจริง"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "กรุณากรอกนามสกุล"))]
    pub last_name: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateUserStatusRequest {
    pub status: UserStatus,
}

#[derive(Serialize, ToSchema)]
pub struct UserDetailResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: UserStatus,
    pub roles: Vec<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct UserFilterQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub email: Option<String>,
    pub status: Option<UserStatus>,
}
