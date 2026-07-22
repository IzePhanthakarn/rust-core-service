use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let title_len = title.trim().chars().count();

    if title_len == 0 || title_len > 100 {
        return Err(ValidationError::new("invalid_todo_title"));
    }

    Ok(())
}

fn validate_description(description: &str) -> Result<(), ValidationError> {
    if description.chars().count() > 3000 {
        return Err(ValidationError::new("invalid_todo_description"));
    }

    Ok(())
}

// ===== List requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateTodoListRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    #[validate(length(max = 20, message = "Color must not exceed 20 characters"))]
    pub color: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateTodoListRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    #[validate(length(max = 20, message = "Color must not exceed 20 characters"))]
    pub color: Option<String>,
}

// ===== Item requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateTodoItemRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ReorderTodoItemsRequest {
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub item_ids: Vec<Uuid>,
}

// ===== Responses =====

#[derive(Serialize, ToSchema)]
pub struct TodoItemResponse {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct TodoListResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub position: i32,
    pub items: Vec<TodoItemResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
