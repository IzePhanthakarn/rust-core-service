use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::time::Instant;
use tokio::net::TcpListener;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_scalar::{Scalar, Servable};

mod config;
mod core;
mod modules;
mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: config::database::DbPool,
    pub start_time: Instant,
}

// อัปเดต OpenAPI ให้กวาดอ่าน Route และ Schema ที่เราสร้างไว้
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health Check Route
        modules::health::handlers::health_check,

        // Auth Routes
        modules::auth::handlers::register,
        modules::auth::handlers::login,
        modules::auth::handlers::refresh_token,
        modules::auth::handlers::reset_password,
        modules::auth::handlers::logout,

        // User Routes
        modules::users::handlers::get_users,
        modules::users::handlers::assign_role,
        modules::users::handlers::get_me,
        modules::users::handlers::update_me,
    ),
    components(schemas(
        // ==== Common Response Schemas ===
        core::response::ApiResponse<modules::health::handlers::HealthData>,
        core::response::ApiResponse<core::response::EmptyData>,
        core::response::ApiResponse<modules::auth::dtos::AuthResponse>,
        core::response::EmptyData,
        // ================================

        // ==== Health ====
        modules::health::handlers::HealthData,
        modules::health::handlers::DbHealth,
        // ================================
        
        // ==== Auth ====
        modules::auth::dtos::RegisterRequest,
        modules::auth::dtos::LoginRequest,
        modules::auth::dtos::AuthResponse,
        modules::auth::dtos::RefreshRequest,
        modules::auth::dtos::ResetPasswordRequest,
        // ================================

        // ==== Users ====
        modules::users::models::User,
        modules::users::models::UserStatus,
        core::response::PaginatedData<modules::users::models::User>,
        core::response::ApiResponse<core::response::PaginatedData<modules::users::models::User>>,
        modules::users::models::AssignRoleRequest,
        modules::users::models::UserProfile,
        modules::users::models::MeResponse,
        modules::users::models::UpdateProfileRequest,
        core::response::ApiResponse<modules::users::models::MeResponse>,
        // ================================
    )),
    tags(
        (name = "System Health", description = "Endpoints for monitoring server status"),
        (name = "Auth", description = "Authentication & User Management") ,
        (name = "Users", description = "User Management & Roles")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;
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

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_pool = config::database::establish_connection_pool();
    let state = AppState {
        db_pool,
        start_time: Instant::now(),
    };

    let app = Router::new()
        .route("/health", get(modules::health::handlers::health_check))
        // เรียกใช้ routes ผ่าน module ย่อยที่เราเพิ่งแยก
        .nest("/auth", modules::auth::routes::auth_routes())
        .nest("/users", modules::users::routes::user_routes())
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📚 API Documentation available at http://{}/docs", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
