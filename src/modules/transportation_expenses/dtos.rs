use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let title_len = title.trim().chars().count();

    if title_len == 0 || title_len > 100 {
        return Err(ValidationError::new("invalid_transportation_expense_title"));
    }

    Ok(())
}

fn validate_category(category: &str) -> Result<(), ValidationError> {
    let category_len = category.trim().chars().count();

    if category_len == 0 || category_len > 50 {
        return Err(ValidationError::new(
            "invalid_transportation_expense_category",
        ));
    }

    Ok(())
}

fn validate_note(note: &str) -> Result<(), ValidationError> {
    if note.chars().count() > 3000 {
        return Err(ValidationError::new("invalid_transportation_expense_note"));
    }

    Ok(())
}

// ===== Requests =====

#[derive(Deserialize, IntoParams)]
pub struct TransportationExpenseFilterQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub month: Option<String>,
    pub year: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateTransportationExpenseRequest {
    #[validate(custom(function = "validate_category"))]
    pub category: String,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    /// Check to also save this item as a Transaction (expense)
    #[serde(default)]
    pub sync_to_transaction: bool,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateTransportationExpenseRequest {
    #[validate(custom(function = "validate_category"))]
    pub category: String,
    /// Amount in satang (100.50 baht = 10050)
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    /// Check to also save this item as a Transaction (expense)
    /// If it was previously synced and this is set to false, the system will automatically delete the linked Transaction
    #[serde(default)]
    pub sync_to_transaction: bool,
}

// ===== Responses =====

#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    /// The Transaction created alongside this item (if sync_to_transaction was checked when saving)
    pub transaction_id: Option<Uuid>,
    pub category: String,
    pub amount: i64,
    pub title: String,
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of transportation expenses with summary statistics
/// (stats are calculated from the specified month/year range, independent of the category/keyword filters and pagination)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseListResponse {
    pub items: Vec<TransportationExpenseResponse>,
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub stats: TransportationExpenseStatsResponse,
}

/// Summary statistics for transportation expenses (in satang)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseStatsResponse {
    /// Total expense
    pub total_expense: i64,
    /// Average expense per day with actual spending (total expense / number of days with items)
    pub average_per_active_day: i64,
    /// Total number of items in the calculated range (not just the current page)
    pub expense_count: i64,
    /// Proportion of expense for each category (type), sorted from highest to lowest, summing to 100%
    pub category_split: Vec<TransportationExpenseCategoryStat>,
}

/// Proportion of expense for each category (type)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseCategoryStat {
    pub category: String,
    pub total_amount: i64,
    pub count: i64,
    /// Proportion relative to total expense, in percent (0-100)
    pub percentage: f64,
}
