use crate::{
    modules::work_logs::models::{NewWorkLog, NewWorkLogTag, WorkLog, WorkLogTag},
    schema::{work_log_tags, work_logs},
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};
use uuid::Uuid;

pub struct WorkLogRepository;

impl WorkLogRepository {
    pub fn create_work_log(
        conn: &mut PgConnection,
        work_log: &NewWorkLog<'_>,
    ) -> QueryResult<WorkLog> {
        diesel::insert_into(work_logs::table)
            .values(work_log)
            .returning(WorkLog::as_returning())
            .get_result(conn)
    }

    pub fn create_work_log_tags(
        conn: &mut PgConnection,
        tags: &[NewWorkLogTag<'_>],
    ) -> QueryResult<Vec<WorkLogTag>> {
        if tags.is_empty() {
            return Ok(Vec::new());
        }

        diesel::insert_into(work_log_tags::table)
            .values(tags)
            .returning(WorkLogTag::as_returning())
            .get_results(conn)
    }

    pub fn delete_work_log_tags(conn: &mut PgConnection, work_log_id: Uuid) -> QueryResult<usize> {
        diesel::delete(work_log_tags::table.filter(work_log_tags::log_id.eq(work_log_id)))
            .execute(conn)
    }

    pub fn update_work_log(
        conn: &mut PgConnection,
        work_log_id: Uuid,
        work_log: &NewWorkLog<'_>,
    ) -> QueryResult<WorkLog> {
        diesel::update(work_logs::table.filter(work_logs::id.eq(work_log_id)))
            .set(work_log)
            .returning(WorkLog::as_returning())
            .get_result(conn)
    }
}
