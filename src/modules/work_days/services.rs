use std::collections::HashSet;

use chrono::{Datelike, Utc};
use diesel::Connection;
use diesel::PgConnection;

use crate::{
    core::errors::AppError,
    modules::work_days::{
        dtos::{
            BotHolidayItem, FetchHolidayResult, HolidayListResponse, HolidayResponse,
            HolidayStats, NextHolidayInfo,
        },
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

    pub fn get_holidays(
        conn: &mut PgConnection,
        year: Option<String>,
    ) -> Result<HolidayListResponse, AppError> {
        let now = Utc::now();
        let target_year = year.unwrap_or_else(|| now.year().to_string());

        let holidays = HolidayRepository::find_all_by_year(conn, &target_year)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถดึงข้อมูลวันหยุดได้".to_string()))?;

        let current_month = now.month();

        let total_holidays_this_year = holidays.len() as i64;

        let total_holidays_this_month = holidays
            .iter()
            .filter(|h| h.holiday_date.month() == current_month)
            .count() as i64;

        let next_upcoming_holiday = holidays
            .iter()
            .find(|h| h.holiday_date > now)
            .map(|h| {
                let days_until =
                    (h.holiday_date.date_naive() - now.date_naive()).num_days();
                NextHolidayInfo {
                    holiday_description: h.holiday_description.clone(),
                    holiday_date: h.holiday_date,
                    days_until,
                }
            });

        let remaining_holidays_this_year =
            holidays.iter().filter(|h| h.holiday_date > now).count() as i64;

        let items = holidays
            .into_iter()
            .map(|h| HolidayResponse {
                id: h.id,
                holiday_description: h.holiday_description,
                holiday_date: h.holiday_date,
                holiday_year: h.holiday_year,
            })
            .collect();

        Ok(HolidayListResponse {
            items,
            stats: HolidayStats {
                total_holidays_this_year,
                total_holidays_this_month,
                next_upcoming_holiday,
                remaining_holidays_this_year,
            },
        })
    }
}
