use crate::{
    modules::work_logs::models::{NewWorkLog, WorkLog, WorkLogtag},
    schema::{work_log_tags, work_logs},
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};
use uuid::Uuid;

pub struct WorkLogRepository;

impl WorkLogRepository {
    pub fn create_work_log(conn: &mut PgConnection, work_log: &NewWorkLog) -> QueryResult<WorkLog> {
        diesel::insert_into(work_logs::table)
            .values(work_log)
            .returning(WorkLog::as_returning())
            .get_result(conn)
    }

    pub fn create_work_tag(
        conn: &mut PgConnection,
        work_log_id: Uuid,
        work_tag: String,
    ) -> QueryResult<WorkLogtag> {
        diesel::insert_into(work_log_tags::table)
            .values((
                work_log_tags::log_id.eq(work_log_id),
                work_log_tags::work_tag.eq(work_tag),
            ))
            .returning(WorkLogtag::as_returning())
            .get_result(conn)
    }
}
