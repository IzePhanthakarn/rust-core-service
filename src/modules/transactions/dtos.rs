use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::modules::transactions::models::{BillingCycle, TransactionType};

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let title_len = title.trim().chars().count();

    if title_len == 0 || title_len > 100 {
        return Err(ValidationError::new("invalid_transaction_title"));
    }

    Ok(())
}

fn validate_category(category: &str) -> Result<(), ValidationError> {
    let category_len = category.trim().chars().count();

    if category_len == 0 || category_len > 50 {
        return Err(ValidationError::new("invalid_transaction_category"));
    }

    Ok(())
}

fn validate_note(note: &str) -> Result<(), ValidationError> {
    if note.chars().count() > 3000 {
        return Err(ValidationError::new("invalid_transaction_note"));
    }

    Ok(())
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    let name_len = name.trim().chars().count();

    if name_len == 0 || name_len > 100 {
        return Err(ValidationError::new("invalid_subscription_name"));
    }

    Ok(())
}

// ===== Transaction requests =====

#[derive(Deserialize, IntoParams)]
pub struct TransactionFilterQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<TransactionType>,
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub month: Option<String>,
    pub year: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateTransactionRequest {
    #[serde(rename = "type")]
    pub type_: TransactionType,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_category"))]
    pub category: String,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub transaction_date: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateTransactionRequest {
    #[serde(rename = "type")]
    pub type_: TransactionType,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_category"))]
    pub category: String,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub transaction_date: DateTime<Utc>,
}

// ===== Subscription requests =====

#[derive(Deserialize, IntoParams)]
pub struct SubscriptionFilterQuery {
    pub billing_cycle: Option<BillingCycle>,
    pub is_active: Option<bool>,
    /// Search by subscription name using contains matching (case-insensitive)
    pub keyword: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateSubscriptionRequest {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    #[validate(range(min = 1, max = 31, message = "Billing day must be between 1 and 31"))]
    pub billing_day: i32,
    /// Billing month (1-12), used only when billing_cycle = yearly
    #[validate(range(min = 1, max = 12, message = "Billing month must be between 1 and 12"))]
    pub billing_month: Option<i32>,
    #[validate(custom(function = "validate_category"))]
    pub category: Option<String>,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateSubscriptionRequest {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    #[validate(range(min = 1, max = 31, message = "Billing day must be between 1 and 31"))]
    pub billing_day: i32,
    /// Billing month (1-12), used only when billing_cycle = yearly
    #[validate(range(min = 1, max = 12, message = "Billing month must be between 1 and 12"))]
    pub billing_month: Option<i32>,
    #[validate(custom(function = "validate_category"))]
    pub category: Option<String>,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

// ===== Responses =====

#[derive(Serialize, ToSchema)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "type")]
    pub type_: TransactionType,
    pub amount: i64,
    pub category: String,
    pub title: String,
    pub note: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of transactions with summary statistics
/// (stats are calculated from the specified month/year range, independent of the type/category/keyword filters and pagination)
#[derive(Serialize, ToSchema)]
pub struct TransactionListResponse {
    pub items: Vec<TransactionResponse>,
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub stats: TransactionStatsResponse,
}

/// Summary statistics for transactions (in satang)
#[derive(Serialize, ToSchema)]
pub struct TransactionStatsResponse {
    /// Total income
    pub total_income: i64,
    /// Total expense
    pub total_expense: i64,
    /// Top expense categories, sorted from highest to lowest
    pub top_expense_category: Vec<TransactionCategoryStat>,
    /// Average daily expense (total expense / number of days in range)
    pub average_daily_expense: i64,
    /// Total number of items in the calculated range (not just the current page)
    pub transaction_count: i64,
}

/// Total expense amount for each category
#[derive(Serialize, ToSchema)]
pub struct TransactionCategoryStat {
    pub category: String,
    pub total_amount: i64,
    pub count: i64,
}

#[derive(Serialize, ToSchema)]
pub struct SubscriptionResponse {
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

/// List of subscriptions with summary statistics (stats are calculated from all active items, independent of the filter)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionListResponse {
    pub items: Vec<SubscriptionResponse>,
    pub stats: SubscriptionStatsResponse,
}

/// Summary statistics for subscriptions (amount in satang)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionStatsResponse {
    /// Total monthly expense (monthly items + yearly items divided by 12), active items only
    pub monthly_recurring: i64,
    /// Estimated total yearly expense (monthly items times 12 + yearly items), active items only
    pub yearly_estimate: i64,
    /// Number of active items
    pub active_count: i64,
    /// Total number of items (including inactive)
    pub total_count: i64,
    /// Items due for billing this month but not yet billed (not yet paid)
    pub remaining_this_month: SubscriptionMonthlyStat,
    /// Items whose billing date this month has already passed (already paid)
    pub passed_this_month: SubscriptionMonthlyStat,
    /// Monthly amount (normalized) broken down by category, sorted from highest to lowest, active items only
    pub category_breakdown: Vec<SubscriptionCategoryStat>,
    /// Proportion of items by billing cycle, active items only
    pub cycle_split: SubscriptionCycleSplit,
    /// Top 4 most expensive items (calculated monthly), active items only
    pub top_expenses: Vec<SubscriptionTopExpense>,
}

/// Summary of items due for billing this month (covers both paid and unpaid items)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionMonthlyStat {
    pub count: i64,
    /// Actual amount (not normalized)
    pub amount: i64,
}

/// Monthly amount (normalized) for each category
#[derive(Serialize, ToSchema)]
pub struct SubscriptionCategoryStat {
    pub category: Option<String>,
    pub monthly_amount: i64,
    pub count: i64,
}

/// Number of items broken down by billing cycle
#[derive(Serialize, ToSchema)]
pub struct SubscriptionCycleSplit {
    pub monthly_count: i64,
    pub yearly_count: i64,
}

/// Most expensive items (calculated monthly)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionTopExpense {
    pub name: String,
    pub monthly_amount: i64,
}
