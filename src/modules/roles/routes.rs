use axum::{Router, middleware, routing::{get, post}};
use crate::{AppState, core::middleware::auth_guard};
use super::handlers;

pub fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_roles).post(handlers::create_role))          // จะกลายเป็น GET /roles
        .route("/assign", post(handlers::assign_role)) // จะกลายเป็น POST /roles/assign
        .route("/revoke", post(handlers::revoke_role)) // จะกลายเป็น POST /roles/revoke
        .route_layer(middleware::from_fn(auth_guard))
}