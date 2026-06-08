use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

pub fn property_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(handlers::create_property_type).put(handlers::update_property_type),
        )
        .route(
            "/{property_type_id}",
            get(handlers::get_one_property_type).delete(handlers::delete_property_type),
        )
        .route("/options", post(handlers::create_property_option))
        .route(
            "/options/{property_option_id}",
            delete(handlers::delete_property_option),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
