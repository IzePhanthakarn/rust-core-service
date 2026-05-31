use axum::{routing::get, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use std::time::Instant;

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
        modules::health::handlers::health_check,
        modules::auth::handlers::register 
    ),
    components(schemas(
        core::response::ApiResponse<modules::health::handlers::HealthData>,
        core::response::ApiResponse<core::response::EmptyData>, // << เปลี่ยนจาก () เป็น EmptyData
        core::response::EmptyData,                              // << นำเข้า Schema EmptyData ให้ Scalar รู้จัก
        modules::health::handlers::HealthData,
        modules::health::handlers::DbHealth,
        modules::auth::dtos::RegisterRequest 
    )),
    tags(
        (name = "System Health", description = "Endpoints for monitoring server status"),
        (name = "Auth", description = "Authentication & User Management") 
    )
)]
struct ApiDoc;

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
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📚 API Documentation available at http://{}/docs", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}