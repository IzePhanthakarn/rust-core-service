use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{events, holidays};

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = holidays)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Holiday {
    pub id: Uuid,
    pub holiday_description: String,
    pub holiday_date: DateTime<Utc>,
    pub holiday_year: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = holidays)]
pub struct NewHoliday {
    pub holiday_description: String,
    pub holiday_date: DateTime<Utc>,
    pub holiday_year: String,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub tag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub tag: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = events)]
pub struct UpdateEvent {
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub tag: String,
    pub updated_at: DateTime<Utc>,
}
