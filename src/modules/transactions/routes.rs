use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{get, patch, put},
};

pub fn transaction_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_all_transactions).post(handlers::create_transaction),
        )
        .route(
            "/{transaction_id}",
            get(handlers::get_one_transaction)
                .put(handlers::update_transaction)
                .delete(handlers::delete_transaction),
        )
        .route_layer(middleware::from_fn(auth_guard))
}

pub fn subscription_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_all_subscriptions).post(handlers::create_subscription),
        )
        .route(
            "/{subscription_id}",
            put(handlers::update_subscription).delete(handlers::delete_subscription),
        )
        .route(
            "/{subscription_id}/toggle",
            patch(handlers::toggle_subscription),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
