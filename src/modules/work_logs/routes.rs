use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{Router, middleware, routing::get};

pub fn work_logs_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_work_logs).post(handlers::create_work_log),
        )
        .route(
            "/{work_log_id}",
            get(handlers::get_work_log)
                .put(handlers::update_work_log)
                .delete(handlers::delete_work_log),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
