use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::transportation_expenses;

// ===== Queryable models =====

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = transportation_expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TransportationExpense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub category: String,
    pub amount: i64,
    pub title: String,
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ===== Insertable models =====

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = transportation_expenses)]
pub struct NewTransportationExpense<'a> {
    pub user_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub category: &'a str,
    pub amount: i64,
    pub title: &'a str,
    pub note: Option<&'a str>,
    pub expense_date: DateTime<Utc>,
}
