use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{
    board_columns, boards, project_members, project_notes, projects, sprints, task_comments, tasks,
};

// ===== Enums =====

#[derive(Debug, Clone, Copy, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::ProjectStatus"]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    Active,
    Completed,
    OnHold,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::NoteType"]
#[serde(rename_all = "snake_case")]
pub enum NoteType {
    Folder,
    File,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::TaskType"]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Epic,
    Story,
    Task,
    Bug,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::TaskPriority"]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

// ===== Queryable models =====

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_id: Uuid,
    pub status: ProjectStatus,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = project_notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectNote {
    pub id: Uuid,
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[serde(rename = "type")]
    #[diesel(column_name = type_)]
    pub type_: NoteType,
    pub title: String,
    pub content: Option<String>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = boards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Board {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = board_columns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BoardColumn {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub order_index: i32,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = sprints)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sprint {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub board_id: Uuid,
    pub sprint_id: Option<Uuid>,
    pub column_id: Uuid,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    #[diesel(column_name = type_)]
    pub type_: TaskType,
    pub priority: TaskPriority,
    pub story_points: Option<i32>,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub tag: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = task_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaskComment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ===== Insertable models =====

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub owner_id: Uuid,
    pub status: ProjectStatus,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = project_members)]
pub struct NewProjectMember {
    pub project_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = boards)]
pub struct NewBoard<'a> {
    pub project_id: Uuid,
    pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = board_columns)]
pub struct NewBoardColumn<'a> {
    pub board_id: Uuid,
    pub name: &'a str,
    pub order_index: i32,
}

#[derive(Insertable)]
#[diesel(table_name = sprints)]
pub struct NewSprint<'a> {
    pub project_id: Uuid,
    pub name: &'a str,
    pub goal: Option<&'a str>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = project_notes)]
pub struct NewProjectNote<'a> {
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[diesel(column_name = type_)]
    pub type_: NoteType,
    pub title: &'a str,
    pub content: Option<&'a str>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub project_id: Uuid,
    pub board_id: Uuid,
    pub sprint_id: Option<Uuid>,
    pub column_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    #[diesel(column_name = type_)]
    pub type_: TaskType,
    pub priority: TaskPriority,
    pub story_points: Option<i32>,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub tag: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = task_comments)]
pub struct NewTaskComment<'a> {
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: &'a str,
}

// ===== Changesets (partial updates) =====
// Option<Option<T>> สำหรับคอลัมน์ที่เป็น NULL ได้: outer None = ไม่แตะต้อง, inner None = set NULL

#[derive(AsChangeset, Default)]
#[diesel(table_name = tasks)]
pub struct TaskChangeset {
    pub title: Option<String>,
    pub description: Option<String>,
    pub column_id: Option<Uuid>,
    #[diesel(column_name = type_)]
    pub type_: Option<TaskType>,
    pub priority: Option<TaskPriority>,
    pub story_points: Option<Option<i32>>,
    pub sprint_id: Option<Option<Uuid>>,
    pub assignee_id: Option<Option<Uuid>>,
    pub tag: Option<Option<String>>,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = sprints)]
pub struct SprintChangeset {
    pub name: Option<String>,
    pub goal: Option<Option<String>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<Option<DateTime<Utc>>>,
    pub is_active: Option<bool>,
}
