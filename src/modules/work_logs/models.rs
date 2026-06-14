use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{work_log_tags, work_logs};

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = work_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub mood_score: i32,
    pub productivity_score: i32,
    pub date_logged: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = work_log_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkLogTag {
    pub log_id: Uuid,
    pub work_tag: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = work_logs)]
pub struct NewWorkLog<'a> {
    pub user_id: Uuid,
    pub title: &'a str,
    pub content: &'a str,
    pub mood_score: i32,
    pub productivity_score: i32,
    pub date_logged: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = work_log_tags)]
pub struct NewWorkLogTag<'a> {
    pub log_id: Uuid,
    pub work_tag: &'a str,
}
