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
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    /// ติ๊กเพื่อบันทึกรายการนี้ลง Transaction (รายจ่าย) ควบคู่ไปด้วย
    #[serde(default)]
    pub sync_to_transaction: bool,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateTransportationExpenseRequest {
    #[validate(custom(function = "validate_category"))]
    pub category: String,
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
    pub amount: i64,
    #[validate(custom(function = "validate_title"))]
    pub title: String,
    #[validate(custom(function = "validate_note"))]
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    /// ติ๊กเพื่อบันทึกรายการนี้ลง Transaction (รายจ่าย) ควบคู่ไปด้วย
    /// ถ้าเดิมเคย sync ไว้แล้วปรับเป็น false ระบบจะลบ Transaction ที่ผูกไว้ให้อัตโนมัติ
    #[serde(default)]
    pub sync_to_transaction: bool,
}

// ===== Responses =====

#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    /// Transaction ที่ถูกสร้างคู่กันไว้ (ถ้าติ๊ก sync_to_transaction ตอนบันทึก)
    pub transaction_id: Option<Uuid>,
    pub category: String,
    pub amount: i64,
    pub title: String,
    pub note: Option<String>,
    pub expense_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
