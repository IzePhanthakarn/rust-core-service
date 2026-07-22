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

/// รายการค่าใช้จ่ายเดินทางแบบแบ่งหน้า พร้อมสถิติสรุป
/// (stats คำนวณจากช่วงเดือน/ปีที่ระบุ ไม่ผูกกับ filter category/keyword และไม่ผูกกับ pagination)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseListResponse {
    pub items: Vec<TransportationExpenseResponse>,
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub stats: TransportationExpenseStatsResponse,
}

/// สรุปสถิติของค่าใช้จ่ายเดินทาง (หน่วยเป็นสตางค์)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseStatsResponse {
    /// ค่าใช้จ่ายรวม
    pub total_expense: i64,
    /// ค่าใช้จ่ายเฉลี่ยต่อวันที่มีการใช้จ่ายจริง (ค่าใช้จ่ายรวม / จำนวนวันที่มีรายการ)
    pub average_per_active_day: i64,
    /// จำนวนรายการทั้งหมดในช่วงที่คำนวณ (ไม่ใช่แค่หน้าปัจจุบัน)
    pub expense_count: i64,
    /// สัดส่วนค่าใช้จ่ายของแต่ละหมวด (type) เรียงจากมากไปน้อย รวมกันได้ 100%
    pub category_split: Vec<TransportationExpenseCategoryStat>,
}

/// สัดส่วนค่าใช้จ่ายของแต่ละหมวด (type)
#[derive(Serialize, ToSchema)]
pub struct TransportationExpenseCategoryStat {
    pub category: String,
    pub total_amount: i64,
    pub count: i64,
    /// สัดส่วนเทียบกับค่าใช้จ่ายรวม หน่วยเปอร์เซ็นต์ (0-100)
    pub percentage: f64,
}
