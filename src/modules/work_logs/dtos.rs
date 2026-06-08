use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::modules::work_logs::models::WorkLogtag;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateWorkLogRequest {
    pub title: String,
    pub content: String,
    pub mood_score: i32,
    pub productivity_score: i32,
    pub tags: Vec<String>,
    pub is_draft: bool,
    pub date_logged: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct WorkLogResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub mood_score: i32,
    pub productivity_score: i32,
    pub tags: Vec<WorkLogtag>,
    pub is_draft: bool,
    pub date_logged: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
