use axum::{routing::post, Router};
use crate::AppState;
use super::handlers; // ชี้ไปที่ไฟล์ handlers.rs ในโฟลเดอร์เดียวกัน

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
}