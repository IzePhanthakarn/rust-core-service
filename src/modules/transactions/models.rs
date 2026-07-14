use chrono::{DateTime, Utc};
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{subscriptions, transactions};

// ===== Enums =====

#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::TransactionType"]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::BillingCycle"]
#[serde(rename_all = "snake_case")]
pub enum BillingCycle {
    Monthly,
    Yearly,
}

// ===== Queryable models =====

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "type")]
    #[diesel(column_name = type_)]
    pub type_: TransactionType,
    pub amount: i64,
    pub category: String,
    pub title: String,
    pub note: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    pub billing_day: i32,
    pub billing_month: Option<i32>,
    pub category: Option<String>,
    pub note: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ===== Insertable models =====

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub user_id: Uuid,
    #[diesel(column_name = type_)]
    pub type_: TransactionType,
    pub amount: i64,
    pub category: &'a str,
    pub title: &'a str,
    pub note: Option<&'a str>,
    pub transaction_date: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = subscriptions)]
pub struct NewSubscription<'a> {
    pub user_id: Uuid,
    pub name: &'a str,
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    pub billing_day: i32,
    pub billing_month: Option<i32>,
    pub category: Option<&'a str>,
    pub note: Option<&'a str>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}
