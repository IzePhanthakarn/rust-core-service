use crate::{
    modules::work_days::models::{Holiday, NewHoliday},
    schema::holiday,
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};

pub struct HolidayRepository;

impl HolidayRepository {
    pub fn delete_by_year(conn: &mut PgConnection, year: &str) -> QueryResult<usize> {
        diesel::delete(holiday::table.filter(holiday::holiday_year.eq(year))).execute(conn)
    }

    pub fn insert_batch(
        conn: &mut PgConnection,
        holidays: Vec<NewHoliday>,
    ) -> QueryResult<Vec<Holiday>> {
        diesel::insert_into(holiday::table)
            .values(&holidays)
            .returning(Holiday::as_returning())
            .get_results(conn)
    }
}
