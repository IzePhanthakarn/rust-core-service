use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::modules::work_logs::models::WorkLogTag;

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let title_len = title.trim().chars().count();

    if title_len == 0 || title_len > 100 {
        return Err(ValidationError::new("invalid_work_log_title"));
    }

    Ok(())
}

fn validate_content(content: &str) -> Result<(), ValidationError> {
    if content.trim().is_empty() {
        return Err(ValidationError::new("invalid_work_log_content"));
    }

    Ok(())
}

fn validate_tags(tags: &[String]) -> Result<(), ValidationError> {
    for tag in tags {
        let tag = tag.trim();

        if tag.is_empty() || tag.chars().count() > 50 {
            return Err(ValidationError::new("invalid_work_log_tag"));
        }
    }

    Ok(())
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateWorkLogRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_content"))]
    pub content: String,
    #[validate(range(min = 1, max = 5, message = "Mood score ต้องอยู่ระหว่าง 1-5"))]
    pub mood_score: i32,
    #[validate(range(min = 1, max = 5, message = "Productivity score ต้องอยู่ระหว่าง 1-5"))]
    pub productivity_score: i32,
    #[validate(length(max = 10, message = "Tags ต้องไม่เกิน 10 รายการ"))]
    #[validate(custom(function = "validate_tags"))]
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
    pub tags: Vec<WorkLogTag>,
    pub is_draft: bool,
    pub date_logged: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
