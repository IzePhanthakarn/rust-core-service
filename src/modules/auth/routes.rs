use axum::{middleware, routing::post, Router};
use crate::{core::middleware::auth_guard, AppState};
use super::handlers;

pub fn auth_routes() -> Router<AppState> {
    // 1. กลุ่มที่ "ต้องใช้ Token" ในการเข้าถึง
    let protected_routes = Router::new()
        .route("/logout", post(handlers::logout))
        .route_layer(middleware::from_fn(auth_guard)); // สวมยามให้กลุ่มนี้

    // 2. กลุ่มที่ "ไม่ต้องใช้ Token"
    let public_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/refresh", post(handlers::refresh_token))
        .route("/reset-password", post(handlers::reset_password));

    // 3. จับรวมกัน
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}