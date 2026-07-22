use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{get, put},
};

pub fn transportation_expense_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_all_transportation_expenses)
                .post(handlers::create_transportation_expense),
        )
        .route(
            "/{expense_id}",
            put(handlers::update_transportation_expense)
                .delete(handlers::delete_transportation_expense),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
