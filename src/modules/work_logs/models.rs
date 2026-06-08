use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
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
    pub is_draft: bool,
    pub date_logged: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = work_log_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkLogtag {
    pub log_id: Uuid,
    pub work_tag: String,
}

#[derive(Insertable)]
#[diesel(table_name = work_logs)]
pub struct NewWorkLog {
    pub user_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub mood_score: Option<i32>,
    pub productivity_score: Option<i32>,
    pub is_draft: Option<bool>,
    pub date_logged: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = work_log_tags)]
pub struct NewWorkLogTag {
    pub log_id: Uuid,
    pub work_tag: String,
}
