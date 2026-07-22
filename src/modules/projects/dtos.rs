use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::modules::projects::models::{NoteType, ProjectStatus, TaskPriority, TaskType};

/// Distinguishes "field not sent" (outer None = leave untouched) from "null sent" (Some(None) = clear to NULL)
fn double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let len = title.trim().chars().count();
    if len == 0 || len > 255 {
        return Err(ValidationError::new("invalid_title"));
    }
    Ok(())
}

// ===== Project requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateProjectRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(length(min = 1, message = "description must not be empty"))]
    pub description: String,
    pub status: Option<ProjectStatus>,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateProjectRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(length(min = 1, message = "description must not be empty"))]
    pub description: String,
    pub status: ProjectStatus,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
}

// ===== Member requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct AddProjectMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, IntoParams)]
pub struct ProjectMemberFilterQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

// ===== Note requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateNoteRequest {
    pub parent_id: Option<Uuid>,
    #[serde(rename = "type")]
    pub type_: NoteType,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    pub content: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateNoteRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    pub content: Option<String>,
}

// ===== Sprint requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateSprintRequest {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters long"))]
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateSprintRequest {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters long"))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<String>)]
    pub goal: Option<Option<String>>,
    pub start_date: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<String>)]
    pub end_date: Option<Option<DateTime<Utc>>>,
    pub is_active: Option<bool>,
}

// ===== Task requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateTaskRequest {
    pub project_id: Uuid,
    /// If not specified, it will be placed into Backlogs (sprint_id = NULL)
    pub sprint_id: Option<Uuid>,
    /// If not specified, it will be placed in the first column (lowest order_index) of the project board
    pub column_id: Option<Uuid>,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(length(min = 1, message = "description must not be empty"))]
    pub description: String,
    #[serde(rename = "type")]
    pub type_: TaskType,
    pub priority: TaskPriority,
    pub story_points: Option<i32>,
    pub assignee_id: Option<Uuid>,
    #[validate(length(max = 20, message = "tag must not exceed 20 characters"))]
    pub tag: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateTaskRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "description must not be empty"))]
    pub description: Option<String>,
    pub column_id: Option<Uuid>,
    #[serde(rename = "type")]
    pub type_: Option<TaskType>,
    pub priority: Option<TaskPriority>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<i32>)]
    pub story_points: Option<Option<i32>>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<String>)]
    pub sprint_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<String>)]
    pub assignee_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "double_option")]
    #[schema(value_type = Option<String>)]
    pub tag: Option<Option<String>>,
}

// ===== Comment requests =====

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, message = "content must not be empty"))]
    pub content: String,
}

// ===== Responses =====

#[derive(Serialize, ToSchema)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_id: Uuid,
    pub status: ProjectStatus,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub member_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct MemberResponse {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct ProjectDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_id: Uuid,
    pub status: ProjectStatus,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub members: Vec<MemberResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct NoteResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[serde(rename = "type")]
    pub type_: NoteType,
    pub title: String,
    pub content: Option<String>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Child notes/folders nested under this item (supports folders within folders)
    #[schema(no_recursion)]
    pub children: Vec<NoteResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct SprintResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Serialize, ToSchema)]
pub struct TaskResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub board_id: Uuid,
    pub sprint_id: Option<Uuid>,
    pub column_id: Uuid,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: TaskType,
    pub priority: TaskPriority,
    pub story_points: Option<i32>,
    pub assignee_id: Option<Uuid>,
    pub assignee: Option<TaskAssigneeResponse>,
    pub reporter_id: Uuid,
    pub tag: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct TaskAssigneeResponse {
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BoardColumnResponse {
    pub id: Uuid,
    pub name: String,
    pub order_index: i32,
}

#[derive(Serialize, ToSchema)]
pub struct BoardResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub columns: Vec<BoardColumnResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct KanbanColumnResponse {
    pub id: Uuid,
    pub name: String,
    pub order_index: i32,
    pub tasks: Vec<TaskResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct KanbanResponse {
    pub board_id: Uuid,
    pub board_name: String,
    pub active_sprint: Option<SprintResponse>,
    pub columns: Vec<KanbanColumnResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct CommentResponse {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
