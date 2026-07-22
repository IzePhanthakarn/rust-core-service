use std::collections::HashSet;

use chrono::{Datelike, DateTime, Duration, FixedOffset, NaiveTime, TimeZone, Utc};
use diesel::Connection;
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::calendar::{
        dtos::{
            BotHolidayItem, CreateEventRequest, EventFilterQuery, EventListResponse, EventResponse,
            FetchHolidayResult, HolidayListResponse, HolidayResponse, HolidayStats,
            NextHolidayInfo, UpdateEventRequest,
        },
        models::{Event, NewEvent, NewHoliday, UpdateEvent},
        repositories::{EventRepository, HolidayRepository},
    },
};

pub struct CalendarService;

impl CalendarService {
    pub fn save_holidays(
        conn: &mut PgConnection,
        items: Vec<BotHolidayItem>,
    ) -> Result<FetchHolidayResult, AppError> {
        let mut new_holidays: Vec<NewHoliday> = Vec::new();
        let mut years: HashSet<String> = HashSet::new();

        for item in &items {
            let holiday_date = item.parse_date().ok_or_else(|| {
                AppError::BadRequest(format!(
                    "Unable to parse date: {} {} {}",
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
                    AppError::InternalServerError("Unable to delete old data".to_string())
                })?;
            }

            let inserted = HolidayRepository::insert_batch(conn, new_holidays).map_err(|_| {
                AppError::InternalServerError("Unable to save holiday data".to_string())
            })?;

            Ok(inserted.len() as i64)
        })?;

        Ok(FetchHolidayResult { inserted_count })
    }

    pub fn get_holidays(
        conn: &mut PgConnection,
        year: Option<String>,
        month: Option<String>,
    ) -> Result<HolidayListResponse, AppError> {
        let now = Utc::now();
        let target_year = year.unwrap_or_else(|| now.year().to_string());

        let target_month = month
            .as_deref()
            .map(|m| m.parse::<u32>())
            .transpose()
            .map_err(|_| AppError::BadRequest("month must be a number between 1 and 12".to_string()))?;

        if let Some(m) = target_month {
            if !(1..=12).contains(&m) {
                return Err(AppError::BadRequest("month must be between 1 and 12".to_string()));
            }
        }

        let holidays = HolidayRepository::find_all_by_year(conn, &target_year)
            .map_err(|_| AppError::InternalServerError("Unable to retrieve holiday data".to_string()))?;

        let stat_month = target_month.unwrap_or_else(|| now.month());
        let total_holidays_this_year = holidays.len() as i64;

        let total_holidays_this_month = holidays
            .iter()
            .filter(|h| h.holiday_date.month() == stat_month)
            .count() as i64;

        let next_upcoming_holiday = holidays.iter().find(|h| h.holiday_date > now).map(|h| {
            let days_until = (h.holiday_date.date_naive() - now.date_naive()).num_days();
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
            .filter(|h| target_month.map_or(true, |m| h.holiday_date.month() == m))
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

    pub fn update_event(
        conn: &mut PgConnection,
        event_id: Uuid,
        payload: &UpdateEventRequest,
        claims_user_id: Uuid,
    ) -> Result<EventResponse, AppError> {
        if payload.user_id != claims_user_id {
            return Err(AppError::Forbidden("You do not have permission to edit this event.".to_string()));
        }

        if payload.end_date <= payload.start_date {
            return Err(AppError::BadRequest("end_date must be after start_date".to_string()));
        }

        let existing = EventRepository::find_by_id(conn, event_id)
            .map_err(|_| AppError::NotFound("The event to edit was not found.".to_string()))?;

        if existing.user_id != claims_user_id {
            return Err(AppError::Forbidden("You do not have permission to edit this event.".to_string()));
        }

        let changes = UpdateEvent {
            title: payload.title.clone(),
            description: payload.description.clone(),
            start_date: payload.start_date.with_timezone(&Utc),
            end_date: payload.end_date.with_timezone(&Utc),
            tag: payload.tag.clone(),
            updated_at: Utc::now(),
        };

        let updated = EventRepository::update(conn, event_id, changes)
            .map_err(|_| AppError::InternalServerError("Unable to update the event.".to_string()))?;

        Ok(event_to_response(updated))
    }

    pub fn delete_event(
        conn: &mut PgConnection,
        event_id: Uuid,
        claims_user_id: Uuid,
    ) -> Result<(), AppError> {
        let existing = EventRepository::find_by_id(conn, event_id)
            .map_err(|_| AppError::NotFound("The event to delete was not found.".to_string()))?;

        if existing.user_id != claims_user_id {
            return Err(AppError::Forbidden("You do not have permission to delete this event.".to_string()));
        }

        EventRepository::delete_by_id(conn, event_id)
            .map_err(|_| AppError::InternalServerError("Unable to delete the event.".to_string()))?;

        Ok(())
    }

    pub fn get_events(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: EventFilterQuery,
    ) -> Result<EventListResponse, AppError> {
        let year = filters
            .year
            .as_deref()
            .map(|y| y.parse::<i32>())
            .transpose()
            .map_err(|_| AppError::BadRequest("year must be a number".to_string()))?;

        let month = filters
            .month
            .as_deref()
            .map(|m| m.parse::<u32>())
            .transpose()
            .map_err(|_| AppError::BadRequest("month must be a number between 1 and 12".to_string()))?;

        if let Some(m) = month {
            if !(1..=12).contains(&m) {
                return Err(AppError::BadRequest("month must be between 1 and 12".to_string()));
            }
        }

        let events = EventRepository::find_all_by_user(conn, user_id, year, month, filters.tag)
            .map_err(|_| AppError::InternalServerError("Unable to retrieve event data.".to_string()))?;

        let total_events = events.len() as i64;
        let items = events.into_iter().map(event_to_response).collect();

        Ok(EventListResponse { items, total_events })
    }

    pub fn create_events(
        conn: &mut PgConnection,
        payload: &CreateEventRequest,
        user_id: Uuid,
    ) -> Result<Vec<EventResponse>, AppError> {
        if payload.end_date <= payload.start_date {
            return Err(AppError::BadRequest("end_date must be after start_date".to_string()));
        }

        let new_events = split_into_daily_events(user_id, payload);

        let created = EventRepository::insert_batch(conn, new_events)
            .map_err(|_| AppError::InternalServerError("Unable to save the event.".to_string()))?;

        Ok(created.into_iter().map(event_to_response).collect())
    }
}

fn event_to_response(e: Event) -> EventResponse {
    let bkk = FixedOffset::east_opt(7 * 3600).unwrap();
    EventResponse {
        date: e.end_date.with_timezone(&bkk).format("%Y-%m-%d").to_string(),
        time: e.start_date.with_timezone(&bkk).format("%H:%M").to_string(),
        id: e.id,
        user_id: e.user_id,
        title: e.title,
        description: e.description,
        start_date: e.start_date,
        end_date: e.end_date,
        tag: e.tag,
    }
}

fn split_into_daily_events(user_id: Uuid, payload: &CreateEventRequest) -> Vec<NewEvent> {
    let start = payload.start_date;
    let end = payload.end_date;
    let offset = *start.offset();

    let start_local_date = start.date_naive();
    let end_local_date = end.date_naive();

    let mut result = Vec::new();
    let mut current = start_local_date;

    let eod_time = NaiveTime::from_hms_opt(23, 59, 0).unwrap();
    let sod_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    while current <= end_local_date {
        let seg_start: DateTime<Utc> = if current == start_local_date {
            start.with_timezone(&Utc)
        } else {
            offset
                .from_local_datetime(&current.and_time(sod_time))
                .single()
                .unwrap()
                .with_timezone(&Utc)
        };

        let seg_end: DateTime<Utc> = if current == end_local_date {
            end.with_timezone(&Utc)
        } else {
            offset
                .from_local_datetime(&current.and_time(eod_time))
                .single()
                .unwrap()
                .with_timezone(&Utc)
        };

        result.push(NewEvent {
            user_id,
            title: payload.title.clone(),
            description: payload.description.clone(),
            start_date: seg_start,
            end_date: seg_end,
            tag: payload.tag.clone(),
        });

        current = current + Duration::days(1);
    }

    result
}
