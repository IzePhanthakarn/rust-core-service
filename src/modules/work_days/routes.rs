use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{Router, middleware, routing::post};

pub fn work_days_routes() -> Router<AppState> {
    Router::new()
        .route("/holiday-fetch", post(handlers::fetch_holidays))
        .route_layer(middleware::from_fn(auth_guard))
}
