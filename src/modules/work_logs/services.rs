use std::collections::HashSet;

use diesel::Connection;
use diesel::PgConnection;
use uuid::Uuid;

use crate::core::response::PaginatedData;
use crate::modules::work_logs::dtos::UpdateWorkLogRequest;
use crate::modules::work_logs::dtos::WorkLogFilterQuery;
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
    pub fn get_all_work_logs(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: WorkLogFilterQuery,
    ) -> Result<PaginatedData<WorkLogResponse>, AppError> {
        let page = filters.page.unwrap_or(1).max(1);
        let limit = filters.limit.unwrap_or(10).clamp(1, 100);

        let (items, total_items) = WorkLogRepository::find_all_work_logs(
            conn,
            page,
            limit,
            user_id,
            filters.title,
            filters.start_date,
            filters.end_date,
        )
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

        Ok(PaginatedData {
            items,
            total_items,
            total_pages,
            current_page: page,
        })
    }

    pub fn find_one_work_log(
        conn: &mut PgConnection,
        work_log_id: Uuid,
    ) -> Result<WorkLogResponse, AppError> {
        let work_log = WorkLogRepository::find_one_work_log(conn, work_log_id)
            .map_err(|_| AppError::NotFound("Work Log not found".to_string()))?;
        let work_log_tags =
            WorkLogRepository::find_work_log_tags(conn, work_log.id).map_err(|_| {
                AppError::InternalServerError("Failed to find work log tags".to_string())
            })?;

        let work_log_response = WorkLogResponse {
            user_id: work_log.user_id,
            id: work_log.id,
            title: work_log.title,
            content: work_log.content,
            mood_score: work_log.mood_score,
            productivity_score: work_log.productivity_score,
            is_draft: work_log.is_draft,
            date_logged: work_log.date_logged,
            created_at: work_log.created_at,
            updated_at: work_log.updated_at,
            tags: work_log_tags,
        };

        Ok(work_log_response)
    }

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
        work_log: &UpdateWorkLogRequest,
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

    pub fn delete_work_log(
        conn: &mut PgConnection,
        work_log_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let work_log = WorkLogRepository::find_one_work_log(conn, work_log_id)
            .map_err(|_| AppError::NotFound("Work Log not found".to_string()))?;

        if work_log.user_id != user_id {
            return Err(AppError::Forbidden("คุณไม่มีสิทธิ์ลบ Work Log นี้".to_string()));
        }
        conn.transaction::<(), AppError, _>(|conn| {
            WorkLogRepository::delete_work_log(conn, work_log_id)?;
            Ok(())
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
