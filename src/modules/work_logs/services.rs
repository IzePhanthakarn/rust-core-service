use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::work_logs::{
        dtos::{CreateWorkLogRequest, WorkLogResponse},
        models::{NewWorkLog, WorkLogtag},
        repositories::WorkLogRepository,
    },
};
use diesel::Connection;

pub struct WorkLogService;

impl WorkLogService {
    pub fn create_work_log(
        conn: &mut PgConnection,
        work_log: &CreateWorkLogRequest,
        user_id: Uuid,
    ) -> Result<WorkLogResponse, AppError> {
        // เปลี่ยน Return Type ให้สอดคล้องกับ AppError

        conn.transaction::<WorkLogResponse, AppError, _>(|conn| {
            let new_work_log = NewWorkLog {
                user_id,
                title: work_log.title.clone(),
                content: Some(work_log.content.clone()),
                mood_score: Some(work_log.mood_score),
                productivity_score: Some(work_log.productivity_score),
                is_draft: Some(work_log.is_draft),
                date_logged: Some(work_log.date_logged),
            };

            // 1. สร้าง WorkLog
            let saved_log = WorkLogRepository::create_work_log(conn, &new_work_log)
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

            // 2. สร้าง Tags และเก็บผลลัพธ์เข้า Vec<WorkLogtag>
            // เราใช้ .into_iter() -> .map() -> .collect() เพื่อความสะอาด
            let log_tag_result: Vec<WorkLogtag> = work_log
                .tags
                .clone()
                .into_iter()
                .map(|tag| {
                    WorkLogRepository::create_work_tag(conn, saved_log.id, tag)
                        .map_err(|e| AppError::InternalServerError(e.to_string()))
                })
                .collect::<Result<Vec<WorkLogtag>, AppError>>()?; // ถ้า tag ไหนพังจะหยุดและคืน Err

            // 3. ปั้น Response
            let response = WorkLogResponse {
                user_id: saved_log.user_id,
                id: saved_log.id,
                title: saved_log.title,
                content: saved_log.content,
                mood_score: saved_log.mood_score,
                productivity_score: saved_log.productivity_score,
                tags: log_tag_result,
                is_draft: saved_log.is_draft,
                date_logged: saved_log.date_logged,
                created_at: saved_log.created_at,
                updated_at: saved_log.updated_at,
            };

            Ok(response)
        })
    }
}
