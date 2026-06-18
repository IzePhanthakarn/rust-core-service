use crate::{
    modules::calendar::models::{Event, Holiday, NewEvent, NewHoliday, UpdateEvent},
    schema::{events, holidays},
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};

pub struct HolidayRepository;

impl HolidayRepository {
    pub fn delete_by_year(conn: &mut PgConnection, year: &str) -> QueryResult<usize> {
        diesel::delete(holidays::table.filter(holidays::holiday_year.eq(year))).execute(conn)
    }

    pub fn find_all_by_year(conn: &mut PgConnection, year: &str) -> QueryResult<Vec<Holiday>> {
        holidays::table
            .filter(holidays::holiday_year.eq(year))
            .order_by(holidays::holiday_date.asc())
            .select(Holiday::as_select())
            .load::<Holiday>(conn)
    }

    pub fn insert_batch(
        conn: &mut PgConnection,
        holidays: Vec<NewHoliday>,
    ) -> QueryResult<Vec<Holiday>> {
        diesel::insert_into(holidays::table)
            .values(&holidays)
            .returning(Holiday::as_returning())
            .get_results(conn)
    }
}

pub struct EventRepository;

impl EventRepository {
    pub fn insert_batch(
        conn: &mut PgConnection,
        new_events: Vec<NewEvent>,
    ) -> QueryResult<Vec<Event>> {
        diesel::insert_into(events::table)
            .values(&new_events)
            .returning(Event::as_returning())
            .get_results(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, event_id: uuid::Uuid) -> QueryResult<Event> {
        events::table
            .filter(events::id.eq(event_id))
            .select(Event::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        event_id: uuid::Uuid,
        changes: UpdateEvent,
    ) -> QueryResult<Event> {
        diesel::update(events::table.filter(events::id.eq(event_id)))
            .set(changes)
            .returning(Event::as_returning())
            .get_result(conn)
    }

    pub fn delete_by_id(conn: &mut PgConnection, event_id: uuid::Uuid) -> QueryResult<usize> {
        diesel::delete(events::table.filter(events::id.eq(event_id))).execute(conn)
    }

    pub fn find_all_by_user(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
        year: Option<i32>,
        month: Option<u32>,
        tag: Option<String>,
    ) -> QueryResult<Vec<Event>> {
        use diesel::dsl::sql;
        use diesel::pg::Pg;
        use diesel::sql_types::Bool;

        let mut query = events::table
            .filter(events::user_id.eq(user_id))
            .into_boxed::<Pg>();

        if let Some(y) = year {
            query = query.filter(sql::<Bool>(&format!(
                "EXTRACT(YEAR FROM start_date) = {}",
                y
            )));
        }

        if let Some(m) = month {
            query = query.filter(sql::<Bool>(&format!(
                "EXTRACT(MONTH FROM start_date) = {}",
                m
            )));
        }

        if let Some(ref t) = tag {
            query = query.filter(events::tag.eq(t.as_str()));
        }

        query
            .order_by((events::start_date.asc(), events::end_date.asc()))
            .select(Event::as_select())
            .load::<Event>(conn)
    }
}
