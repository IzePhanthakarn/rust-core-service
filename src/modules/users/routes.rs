use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{get, patch},
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_users))
        .route("/me", get(handlers::get_me).put(handlers::update_me))
        .route("/{id}/status", patch(handlers::update_user_status))
        .route(
            "/{id}",
            get(handlers::get_user_by_id).delete(handlers::delete_user_by_id),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
