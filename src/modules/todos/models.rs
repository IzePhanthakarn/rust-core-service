use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{todo_items, todo_lists};

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = todo_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = todo_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoItem {
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

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = todo_lists)]
pub struct NewTodoList<'a> {
    pub user_id: Uuid,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub color: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = todo_items)]
pub struct NewTodoItem<'a> {
    pub list_id: Uuid,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub due_date: Option<DateTime<Utc>>,
}
