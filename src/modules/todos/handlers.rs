use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::todos::{
        dtos::{
            CreateTodoItemRequest, CreateTodoListRequest, ReorderTodoItemsRequest,
            TodoItemResponse, TodoListResponse, UpdateTodoListRequest,
        },
        services::TodoService,
    },
};

#[utoipa::path(
    get,
    path = "/todos",
    tag = "Todos",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo lists found successfully", body = ApiResponse<Vec<TodoListResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_todo_lists(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<TodoListResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = TodoService::get_all_todo_lists(&mut conn, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Data retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/todos",
    tag = "Todos",
    request_body = CreateTodoListRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Todo list created successfully", body = ApiResponse<TodoListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_todo_list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateTodoListRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TodoListResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::create_todo_list(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Todo list created successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/todos/{list_id}",
    tag = "Todos",
    request_body = UpdateTodoListRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo list updated successfully", body = ApiResponse<TodoListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_todo_list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(list_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTodoListRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TodoListResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::update_todo_list(&mut conn, &payload, claims.sub, list_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "Todo list updated successfully", result))))
}

#[utoipa::path(
    delete,
    path = "/todos/{list_id}",
    tag = "Todos",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo list deleted successfully"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_todo_list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(list_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state.get_conn()?;
    TodoService::delete_todo_list(&mut conn, list_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "Todo list deleted successfully"))))
}

#[utoipa::path(
    patch,
    path = "/todos/{list_id}/move-to-top",
    tag = "Todos",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo list moved to top successfully", body = ApiResponse<Vec<TodoListResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn move_todo_list_to_top(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(list_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<TodoListResponse>>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::move_todo_list_to_top(&mut conn, list_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "Todo list moved to top successfully", result))))
}

#[utoipa::path(
    post,
    path = "/todos/{list_id}/items",
    tag = "Todos",
    request_body = CreateTodoItemRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Todo item created successfully", body = ApiResponse<TodoItemResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_todo_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(list_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateTodoItemRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TodoItemResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::create_todo_item(&mut conn, &payload, claims.sub, list_id)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Todo item created successfully", result)),
    ))
}

#[utoipa::path(
    patch,
    path = "/todos/{list_id}/items/reorder",
    tag = "Todos",
    request_body = ReorderTodoItemsRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo items reordered successfully", body = ApiResponse<Vec<TodoItemResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn reorder_todo_items(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(list_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<ReorderTodoItemsRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<TodoItemResponse>>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::reorder_todo_items(&mut conn, &payload, claims.sub, list_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "Todo items reordered successfully", result))))
}

#[utoipa::path(
    patch,
    path = "/todos/items/{item_id}/toggle",
    tag = "Todos",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo item status toggled successfully", body = ApiResponse<TodoItemResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn toggle_todo_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(item_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TodoItemResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TodoService::toggle_todo_item(&mut conn, item_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "Todo item status toggled successfully", result))))
}

#[utoipa::path(
    delete,
    path = "/todos/items/{item_id}",
    tag = "Todos",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Todo item deleted successfully"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_todo_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(item_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state.get_conn()?;
    TodoService::delete_todo_item(&mut conn, item_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "Todo item deleted successfully"))))
}
