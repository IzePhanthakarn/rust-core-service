use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

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
    #[param(example = "6")]
    pub month: Option<String>,
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

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateEventRequest {
    #[validate(length(min = 1, max = 100, message = "Title ต้องมี 1-100 ตัวอักษร"))]
    pub title: String,
    #[validate(length(max = 3000, message = "Description ต้องไม่เกิน 3000 ตัวอักษร"))]
    pub description: Option<String>,
    #[schema(value_type = String, example = "2026-06-01T13:00:00+07:00")]
    pub start_date: DateTime<FixedOffset>,
    #[schema(value_type = String, example = "2026-06-03T18:00:00+07:00")]
    pub end_date: DateTime<FixedOffset>,
    #[validate(length(min = 1, max = 20, message = "Tag ต้องมี 1-20 ตัวอักษร"))]
    pub tag: String,
}

#[derive(Serialize, ToSchema)]
pub struct EventResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub tag: String,
    pub date: String,
    pub time: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateEventRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 100, message = "Title ต้องมี 1-100 ตัวอักษร"))]
    pub title: String,
    #[validate(length(max = 3000, message = "Description ต้องไม่เกิน 3000 ตัวอักษร"))]
    pub description: Option<String>,
    #[schema(value_type = String, example = "2026-06-01T13:00:00+07:00")]
    pub start_date: DateTime<FixedOffset>,
    #[schema(value_type = String, example = "2026-06-01T18:00:00+07:00")]
    pub end_date: DateTime<FixedOffset>,
    #[validate(length(min = 1, max = 20, message = "Tag ต้องมี 1-20 ตัวอักษร"))]
    pub tag: String,
}

#[derive(Deserialize, IntoParams)]
pub struct EventFilterQuery {
    #[param(example = "2026")]
    pub year: Option<String>,
    #[param(example = "6")]
    pub month: Option<String>,
    #[param(example = "work")]
    pub tag: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct EventListResponse {
    pub items: Vec<EventResponse>,
    pub total_events: i64,
}
