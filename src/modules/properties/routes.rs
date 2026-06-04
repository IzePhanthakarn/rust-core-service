use axum::{Router, middleware, routing::{post}};
use crate::{AppState, core::middleware::auth_guard};
use super::handlers;

pub fn property_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_property_type))
        .route_layer(middleware::from_fn(auth_guard))
}