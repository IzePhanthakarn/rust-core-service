use crate::{
    modules::work_logs::{
        dtos::WorkLogResponse,
        models::{NewWorkLog, NewWorkLogTag, WorkLog, WorkLogTag},
    },
    schema::{work_log_tags, work_logs},
};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};
use std::collections::HashMap;
use uuid::Uuid;

pub struct WorkLogRepository;

impl WorkLogRepository {
    pub fn find_all_work_logs(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        user_id: Uuid,
        title: Option<String>,
        month: Option<String>,
        year: Option<String>,
    ) -> QueryResult<(Vec<WorkLogResponse>, i64)> {
        let offset = (page - 1) * limit;

        let mut data_query = work_logs::table
            .filter(work_logs::user_id.eq(user_id))
            .into_boxed();
        let mut count_query = work_logs::table
            .filter(work_logs::user_id.eq(user_id))
            .into_boxed();

        if let Some(title_text) = title {
            let search_pattern = format!("%{}%", title_text);
            data_query = data_query.filter(work_logs::title.ilike(search_pattern.clone()));
            count_query = count_query.filter(work_logs::title.ilike(search_pattern));
        }

        if let Some((start_date, end_date)) =
            Self::month_year_range(month.as_deref(), year.as_deref())
        {
            data_query = data_query
                .filter(work_logs::date_logged.ge(start_date))
                .filter(work_logs::date_logged.lt(end_date));
            count_query = count_query
                .filter(work_logs::date_logged.ge(start_date))
                .filter(work_logs::date_logged.lt(end_date));
        }

        let work_logs = data_query
            .order_by(work_logs::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select(WorkLog::as_select())
            .load::<WorkLog>(conn)?;

        let work_log_ids: Vec<Uuid> = work_logs.iter().map(|work_log| work_log.id).collect();

        let tags = if work_log_ids.is_empty() {
            Vec::new()
        } else {
            work_log_tags::table
                .filter(work_log_tags::log_id.eq_any(&work_log_ids))
                .select(WorkLogTag::as_select())
                .load::<WorkLogTag>(conn)?
        };

        let mut tags_by_log_id: HashMap<Uuid, Vec<WorkLogTag>> = HashMap::new();
        for tag in tags {
            tags_by_log_id.entry(tag.log_id).or_default().push(tag);
        }

        let items = work_logs
            .into_iter()
            .map(|work_log| WorkLogResponse {
                id: work_log.id,
                user_id: work_log.user_id,
                title: work_log.title,
                content: work_log.content,
                mood_score: work_log.mood_score,
                productivity_score: work_log.productivity_score,
                tags: tags_by_log_id.remove(&work_log.id).unwrap_or_default(),
                date_logged: work_log.date_logged,
                created_at: work_log.created_at,
                updated_at: work_log.updated_at,
            })
            .collect();

        let total: i64 = count_query.count().get_result(conn)?;

        Ok((items, total))
    }

    pub fn find_one_work_log(conn: &mut PgConnection, work_log_id: Uuid) -> QueryResult<WorkLog> {
        work_logs::table
            .filter(work_logs::id.eq(work_log_id))
            .first(conn)
    }
    pub fn create_work_log(
        conn: &mut PgConnection,
        work_log: &NewWorkLog<'_>,
    ) -> QueryResult<WorkLog> {
        diesel::insert_into(work_logs::table)
            .values(work_log)
            .returning(WorkLog::as_returning())
            .get_result(conn)
    }

    pub fn find_work_log_tags(
        conn: &mut PgConnection,
        work_log_id: Uuid,
    ) -> QueryResult<Vec<WorkLogTag>> {
        work_log_tags::table
            .filter(work_log_tags::log_id.eq(work_log_id))
            .get_results(conn)
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

    pub fn delete_work_log(conn: &mut PgConnection, work_log_id: Uuid) -> QueryResult<usize> {
        diesel::delete(work_logs::table.filter(work_logs::id.eq(work_log_id))).execute(conn)
    }

    fn month_year_range(
        month: Option<&str>,
        year: Option<&str>,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let month = month?.parse::<u32>().ok()?;
        let year = year?.parse::<i32>().ok()?;

        if !(1..=12).contains(&month) {
            return None;
        }

        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let start_date = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single()?;
        let end_date = Utc
            .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
            .single()?;

        Some((start_date, end_date))
    }
}
