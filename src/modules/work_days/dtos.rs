use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct FetchHolidayRequest {
    pub path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotHolidayItem {
    pub holiday_description: String,
    pub date: String,
    pub month: String,
    pub year: String,
}

impl BotHolidayItem {
    pub fn parse_date(&self) -> Option<DateTime<Utc>> {
        let day: u32 = self.date.split_whitespace().last()?.parse().ok()?;
        let month_num = thai_month_to_number(self.month.trim())?;
        let gregorian_year: i32 = self.year.trim().parse::<i32>().ok()? - 543;
        Utc.with_ymd_and_hms(gregorian_year, month_num, day, 0, 0, 0)
            .single()
    }
}

fn thai_month_to_number(month: &str) -> Option<u32> {
    match month {
        "มกราคม" => Some(1),
        "กุมภาพันธ์" => Some(2),
        "มีนาคม" => Some(3),
        "เมษายน" => Some(4),
        "พฤษภาคม" => Some(5),
        "มิถุนายน" => Some(6),
        "กรกฎาคม" => Some(7),
        "สิงหาคม" => Some(8),
        "กันยายน" => Some(9),
        "ตุลาคม" => Some(10),
        "พฤศจิกายน" => Some(11),
        "ธันวาคม" => Some(12),
        _ => None,
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotHolidayResponse {
    pub holiday_calendar_lists: Vec<BotHolidayItem>,
}

#[derive(Serialize, ToSchema)]
pub struct FetchHolidayResult {
    pub inserted_count: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct HolidayFilterQuery {
    #[param(example = "2026")]
    pub year: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct HolidayResponse {
    pub id: Uuid,
    pub holiday_description: String,
    pub holiday_date: DateTime<Utc>,
    pub holiday_year: String,
}

#[derive(Serialize, ToSchema)]
pub struct NextHolidayInfo {
    pub holiday_description: String,
    pub holiday_date: DateTime<Utc>,
    pub days_until: i64,
}

#[derive(Serialize, ToSchema)]
pub struct HolidayStats {
    pub total_holidays_this_year: i64,
    pub total_holidays_this_month: i64,
    pub next_upcoming_holiday: Option<NextHolidayInfo>,
    pub remaining_holidays_this_year: i64,
}

#[derive(Serialize, ToSchema)]
pub struct HolidayListResponse {
    pub items: Vec<HolidayResponse>,
    pub stats: HolidayStats,
}
