use axum::{Router, middleware, routing::{post, get}};
use crate::{AppState, core::middleware::auth_guard};
use super::handlers;

pub fn property_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_property_type))
        .route("/{property_type_id}", get(handlers::get_one_property_type))
        .route("/options", post(handlers::create_property_option))
        .route_layer(middleware::from_fn(auth_guard))
}
