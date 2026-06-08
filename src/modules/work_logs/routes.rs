use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{Router, middleware, routing::post};

pub fn work_logs_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_work_log))
        .route_layer(middleware::from_fn(auth_guard))
}
