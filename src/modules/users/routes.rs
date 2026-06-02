// ไฟล์ src/modules/users/routes.rs (สร้างใหม่)
use axum::{routing::{get, post}, middleware, Router};
use crate::{AppState, core::middleware::auth_guard};
use super::handlers;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_users))
        .route("/me", get(handlers::get_me).put(handlers::update_me))
        .route("/assign-role", post(handlers::assign_role))
        .route_layer(middleware::from_fn(auth_guard)) 
}