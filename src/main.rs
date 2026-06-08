use axum::http::{HeaderValue, Method};
use axum::{Router, routing::get};
use docs::ApiDoc;
use dotenvy::dotenv;
use std::time::Instant;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod config;
mod core;
mod docs;
mod modules;
mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: config::database::DbPool,
    pub start_time: Instant,
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

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/health", get(modules::health::handlers::health_check))
        .nest("/auth", modules::auth::routes::auth_routes())
        .nest("/users", modules::users::routes::user_routes())
        .nest(
            "/properties",
            modules::properties::routes::property_routes(),
        )
        // ปรับให้ไม่มี /api เหมือนเส้นอื่นๆ จะได้เข้าผ่าน /roles/... ได้เลย
        .nest("/roles", modules::roles::routes::role_routes())
        .nest("/work-logs", modules::work_logs::routes::work_logs_routes())
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .with_state(state)
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📚 API Documentation available at http://{}/docs", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
