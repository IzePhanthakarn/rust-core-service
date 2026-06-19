use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post, put},
};

pub fn todos_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_all_todo_lists).post(handlers::create_todo_list),
        )
        .route(
            "/{list_id}",
            put(handlers::update_todo_list).delete(handlers::delete_todo_list),
        )
        .route(
            "/{list_id}/move-to-top",
            patch(handlers::move_todo_list_to_top),
        )
        .route("/{list_id}/items", post(handlers::create_todo_item))
        .route(
            "/{list_id}/items/reorder",
            patch(handlers::reorder_todo_items),
        )
        .route("/items/{item_id}", delete(handlers::delete_todo_item))
        .route("/items/{item_id}/toggle", patch(handlers::toggle_todo_item))
        .route_layer(middleware::from_fn(auth_guard))
}
