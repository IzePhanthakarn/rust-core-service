use std::collections::HashSet;

use diesel::Connection;
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::work_logs::{
        dtos::{CreateWorkLogRequest, WorkLogResponse},
        models::{NewWorkLog, NewWorkLogTag},
        repositories::WorkLogRepository,
    },
};

pub struct WorkLogService;

impl WorkLogService {
    pub fn create_work_log(
        conn: &mut PgConnection,
        work_log: &CreateWorkLogRequest,
        user_id: Uuid,
    ) -> Result<WorkLogResponse, AppError> {
        conn.transaction::<WorkLogResponse, AppError, _>(|conn| {
            let new_work_log = NewWorkLog {
                user_id,
                title: work_log.title.trim(),
                content: work_log.content.trim(),
                mood_score: work_log.mood_score,
                productivity_score: work_log.productivity_score,
                is_draft: work_log.is_draft,
                date_logged: work_log.date_logged,
            };

            let saved_log = WorkLogRepository::create_work_log(conn, &new_work_log)?;

            let tags = Self::normalize_tags(&work_log.tags);
            let new_tags: Vec<NewWorkLogTag<'_>> = tags
                .iter()
                .map(|tag| NewWorkLogTag {
                    log_id: saved_log.id,
                    work_tag: tag,
                })
                .collect();

            let tags = WorkLogRepository::create_work_log_tags(conn, &new_tags)?;

            Ok(WorkLogResponse {
                user_id: saved_log.user_id,
                id: saved_log.id,
                title: saved_log.title,
                content: saved_log.content,
                mood_score: saved_log.mood_score,
                productivity_score: saved_log.productivity_score,
                tags,
                is_draft: saved_log.is_draft,
                date_logged: saved_log.date_logged,
                created_at: saved_log.created_at,
                updated_at: saved_log.updated_at,
            })
        })
    }

    pub fn update_work_log(
        conn: &mut PgConnection,
        work_log: &CreateWorkLogRequest,
        user_id: Uuid,
        work_log_id: Uuid,
    ) -> Result<WorkLogResponse, AppError> {
        conn.transaction::<WorkLogResponse, AppError, _>(|conn| {
            let new_work_log = NewWorkLog {
                user_id,
                title: work_log.title.trim(),
                content: work_log.content.trim(),
                mood_score: work_log.mood_score,
                productivity_score: work_log.productivity_score,
                is_draft: work_log.is_draft,
                date_logged: work_log.date_logged,
            };

            let saved_log = WorkLogRepository::update_work_log(conn, work_log_id, &new_work_log)?;

            let tags = Self::normalize_tags(&work_log.tags);
            WorkLogRepository::delete_work_log_tags(conn, saved_log.id)?;

            let new_tags: Vec<NewWorkLogTag<'_>> = tags
                .iter()
                .map(|tag| NewWorkLogTag {
                    log_id: saved_log.id,
                    work_tag: tag,
                })
                .collect();

            let tags = WorkLogRepository::create_work_log_tags(conn, &new_tags)?;

            Ok(WorkLogResponse {
                user_id: saved_log.user_id,
                id: saved_log.id,
                title: saved_log.title,
                content: saved_log.content,
                mood_score: saved_log.mood_score,
                productivity_score: saved_log.productivity_score,
                tags,
                is_draft: saved_log.is_draft,
                date_logged: saved_log.date_logged,
                created_at: saved_log.created_at,
                updated_at: saved_log.updated_at,
            })
        })
    }

    fn normalize_tags(tags: &[String]) -> Vec<&str> {
        let mut seen = HashSet::with_capacity(tags.len());
        let mut normalized = Vec::with_capacity(tags.len());

        for tag in tags {
            let tag = tag.trim();

            if seen.insert(tag) {
                normalized.push(tag);
            }
        }

        normalized
    }
}
