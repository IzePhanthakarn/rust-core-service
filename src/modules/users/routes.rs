// ไฟล์ src/modules/users/routes.rs (สร้างใหม่)
use axum::{Router, middleware, routing::{get, patch, post}};
use crate::{AppState, core::middleware::auth_guard};
use super::handlers;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_users))
        .route("/me", get(handlers::get_me).put(handlers::update_me))
        .route("/:id/status", patch(handlers::update_user_status))
        .route("/roles", get(handlers::get_roles))
        .route("/assign-role", post(handlers::assign_role))
        .route("/revoke-role", post(handlers::revoke_role))
        .route("/:id", get(handlers::get_user_by_id).delete(handlers::delete_user_by_id))
        .route_layer(middleware::from_fn(auth_guard)) 
}