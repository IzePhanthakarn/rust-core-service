// src/docs.rs
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use crate::{modules, core};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health Check Route
        modules::health::handlers::health_check,

        // Auth Routes
        modules::auth::handlers::register,
        modules::auth::handlers::login,
        modules::auth::handlers::refresh_token,
        modules::auth::handlers::change_password,
        modules::auth::handlers::reset_password,
        modules::auth::handlers::logout,

        // User Routes
        modules::users::handlers::get_users,
        modules::users::handlers::update_me,
        modules::users::handlers::get_me,
        modules::users::handlers::update_user_status,
        modules::users::handlers::delete_user_by_id,
        modules::users::handlers::get_user_by_id,

        // Roles Routes (ย้ายมาเรียกผ่าน module roles)
        modules::roles::handlers::get_roles,
        modules::roles::handlers::create_role,
        modules::roles::handlers::assign_role,
        modules::roles::handlers::revoke_role,
    ),
    components(schemas(
        // ==== Common Response Schemas ===
        core::response::ApiResponse<modules::health::dtos::HealthData>,
        core::response::ApiResponse<core::response::EmptyData>,
        core::response::ApiResponse<modules::auth::dtos::AuthResponse>,
        core::response::EmptyData,
        // ================================

        // ==== Health ====
        modules::health::dtos::HealthData,
        modules::health::dtos::DbHealth,
        // ================================
        
        // ==== Auth ====
        modules::auth::dtos::RegisterRequest,
        modules::auth::dtos::LoginRequest,
        modules::auth::dtos::AuthResponse,
        modules::auth::dtos::RefreshRequest,
        modules::auth::dtos::ChangePasswordRequest,
        modules::auth::dtos::ResetPasswordRequest,
        // ================================

        // ==== Users ====
        modules::users::models::User,
        modules::users::models::UserStatus,
        modules::users::models::UserProfile,
        
        modules::users::dtos::MeResponse,
        modules::users::dtos::UpdateProfileRequest,
        modules::users::dtos::UpdateUserStatusRequest,
        modules::users::dtos::UserDetailResponse,
        
        core::response::PaginatedData<modules::users::models::User>,
        core::response::ApiResponse<core::response::PaginatedData<modules::users::models::User>>,
        core::response::ApiResponse<modules::users::dtos::MeResponse>,
        core::response::ApiResponse<modules::users::dtos::UserDetailResponse>,
        // ================================
        
        // ==== Roles ====
        modules::roles::dtos::CreateRoleRequest,
        modules::roles::dtos::AssignRoleRequest,
        modules::roles::dtos::RevokeRoleRequest,
        modules::roles::models::Role,
        core::response::ApiResponse<Vec<modules::roles::models::Role>>,
        // ================================
    )),
    tags(
        (name = "System Health", description = "Endpoints for monitoring server status"),
        (name = "Auth", description = "Authentication & User Management") ,
        (name = "Users", description = "User Management"),
        (name = "Roles", description = "Role Management") // เพิ่ม Tag Roles เข้าไปใน Swagger
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}