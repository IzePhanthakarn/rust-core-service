use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{post, put},
};

pub fn auth_routes() -> Router<AppState> {
    // 1. The group that "requires a token" to access
    let protected_routes = Router::new()
        .route("/change-password", put(handlers::change_password))
        .route("/logout", post(handlers::logout))
        .route_layer(middleware::from_fn(auth_guard)); // Guard this group

    // 2. The group that "does not require a token"
    let public_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/refresh", post(handlers::refresh_token))
        .route("/reset-password", post(handlers::reset_password));

    // 3. Combine them
    Router::new().merge(public_routes).merge(protected_routes)
}
