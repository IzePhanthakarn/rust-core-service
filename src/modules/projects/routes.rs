use super::handlers;
use crate::{AppState, core::middleware::auth_guard};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn project_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::list_projects).post(handlers::create_project),
        )
        .route(
            "/{id}",
            get(handlers::get_project_detail)
                .put(handlers::update_project)
                .delete(handlers::delete_project),
        )
        .route(
            "/{id}/members",
            get(handlers::list_members).post(handlers::add_member),
        )
        .route(
            "/{id}/members/{member_id}",
            delete(handlers::remove_member),
        )
        .route(
            "/{id}/notes",
            get(handlers::get_notes).post(handlers::create_note),
        )
        .route(
            "/{id}/notes/{note_id}",
            put(handlers::update_note).delete(handlers::delete_note),
        )
        .route("/{id}/boards", get(handlers::get_boards))
        .route("/{id}/kanban", get(handlers::get_kanban))
        .route("/{id}/backlogs", get(handlers::get_backlogs))
        .route(
            "/{id}/sprints",
            get(handlers::list_sprints).post(handlers::create_sprint),
        )
        .route(
            "/{id}/sprints/{sprint_id}/tasks",
            get(handlers::get_sprint_tasks),
        )
        .route("/{id}/sprints/{sprint_id}", put(handlers::update_sprint))
        .route_layer(middleware::from_fn(auth_guard))
}

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_task))
        .route(
            "/{id}",
            put(handlers::update_task).delete(handlers::delete_task),
        )
        .route(
            "/{id}/comments",
            get(handlers::list_comments).post(handlers::create_comment),
        )
        .route_layer(middleware::from_fn(auth_guard))
}
