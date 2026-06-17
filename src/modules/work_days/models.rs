use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::holiday;

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = holiday)]
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
#[diesel(table_name = holiday)]
pub struct NewHoliday {
    pub holiday_description: String,
    pub holiday_date: DateTime<Utc>,
    pub holiday_year: String,
}
