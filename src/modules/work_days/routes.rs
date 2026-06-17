use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{Router, middleware, routing::{get, post}};

pub fn work_days_routes() -> Router<AppState> {
    Router::new()
        .route("/holidays", get(handlers::get_holidays))
        .route("/holiday-fetch", post(handlers::fetch_holidays))
        .route_layer(middleware::from_fn(auth_guard))
}
