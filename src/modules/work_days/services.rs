use std::collections::HashSet;

use diesel::Connection;
use diesel::PgConnection;

use crate::{
    core::errors::AppError,
    modules::work_days::{
        dtos::{BotHolidayItem, FetchHolidayResult},
        models::NewHoliday,
        repositories::HolidayRepository,
    },
};

pub struct WorkDayService;

impl WorkDayService {
    pub fn save_holidays(
        conn: &mut PgConnection,
        items: Vec<BotHolidayItem>,
    ) -> Result<FetchHolidayResult, AppError> {
        let mut new_holidays: Vec<NewHoliday> = Vec::new();
        let mut years: HashSet<String> = HashSet::new();

        for item in &items {
            let holiday_date = item.parse_date().ok_or_else(|| {
                AppError::BadRequest(format!(
                    "ไม่สามารถแปลงวันที่ได้: {} {} {}",
                    item.date, item.month, item.year
                ))
            })?;

            let gregorian_year = (item.year.trim().parse::<i32>().unwrap_or(0) - 543).to_string();

            years.insert(gregorian_year.clone());
            new_holidays.push(NewHoliday {
                holiday_description: item.holiday_description.clone(),
                holiday_date,
                holiday_year: gregorian_year,
            });
        }

        let inserted_count = conn.transaction::<i64, AppError, _>(|conn| {
            for year in &years {
                HolidayRepository::delete_by_year(conn, year).map_err(|_| {
                    AppError::InternalServerError("ไม่สามารถลบข้อมูลเก่าได้".to_string())
                })?;
            }

            let inserted = HolidayRepository::insert_batch(conn, new_holidays).map_err(|_| {
                AppError::InternalServerError("ไม่สามารถบันทึกข้อมูลวันหยุดได้".to_string())
            })?;

            Ok(inserted.len() as i64)
        })?;

        Ok(FetchHolidayResult { inserted_count })
    }
}
