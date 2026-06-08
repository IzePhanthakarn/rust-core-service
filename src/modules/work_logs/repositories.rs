use crate::{
    modules::work_logs::models::{NewWorkLog, NewWorkLogTag, WorkLog, WorkLogTag},
    schema::{work_log_tags, work_logs},
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};

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
}
