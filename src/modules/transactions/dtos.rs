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
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
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
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
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
    /// ค้นหาจากชื่อ subscription แบบ contains (ไม่สนตัวพิมพ์เล็กใหญ่)
    pub keyword: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateSubscriptionRequest {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    #[validate(range(min = 1, max = 31, message = "วันที่ตัดเงินต้องอยู่ระหว่าง 1-31"))]
    pub billing_day: i32,
    /// เดือนที่ตัดเงิน (1-12) ใช้เฉพาะ billing_cycle = yearly
    #[validate(range(min = 1, max = 12, message = "เดือนที่ตัดเงินต้องอยู่ระหว่าง 1-12"))]
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
    /// จำนวนเงินหน่วยสตางค์ (100.50 บาท = 10050)
    #[validate(range(min = 1, message = "จำนวนเงินต้องมากกว่า 0"))]
    pub amount: i64,
    pub billing_cycle: BillingCycle,
    #[validate(range(min = 1, max = 31, message = "วันที่ตัดเงินต้องอยู่ระหว่าง 1-31"))]
    pub billing_day: i32,
    /// เดือนที่ตัดเงิน (1-12) ใช้เฉพาะ billing_cycle = yearly
    #[validate(range(min = 1, max = 12, message = "เดือนที่ตัดเงินต้องอยู่ระหว่าง 1-12"))]
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

/// รายการ subscription พร้อมสถิติสรุป (stats คำนวณจากรายการที่ active ทั้งหมด ไม่ผูกกับ filter)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionListResponse {
    pub items: Vec<SubscriptionResponse>,
    pub stats: SubscriptionStatsResponse,
}

/// สรุปสถิติของ subscription (จำนวนเงินหน่วยสตางค์)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionStatsResponse {
    /// ค่าใช้จ่ายรวมต่อเดือน (รายเดือน + รายปีหาร 12) เฉพาะรายการ active
    pub monthly_recurring: i64,
    /// ประมาณการค่าใช้จ่ายรวมต่อปี (รายเดือนคูณ 12 + รายปี) เฉพาะรายการ active
    pub yearly_estimate: i64,
    /// จำนวนรายการที่ active
    pub active_count: i64,
    /// จำนวนรายการทั้งหมด (รวม inactive)
    pub total_count: i64,
    /// รายการที่ครบกำหนดตัดเงินในเดือนนี้แต่ยังไม่ถึงวันตัด (ยังไม่จ่าย)
    pub remaining_this_month: SubscriptionMonthlyStat,
    /// รายการที่ถึงวันตัดเงินของเดือนนี้ไปแล้ว (จ่ายแล้ว)
    pub passed_this_month: SubscriptionMonthlyStat,
    /// ยอดรายเดือน (normalize) แยกตามหมวด เรียงมากไปน้อย เฉพาะรายการ active
    pub category_breakdown: Vec<SubscriptionCategoryStat>,
    /// สัดส่วนจำนวนรายการตามรอบบิล เฉพาะรายการ active
    pub cycle_split: SubscriptionCycleSplit,
    /// รายการที่แพงที่สุด 4 อันดับ (คิดเป็นรายเดือน) เฉพาะรายการ active
    pub top_expenses: Vec<SubscriptionTopExpense>,
}

/// สรุปรายการที่ครบกำหนดตัดเงินในเดือนนี้ (ใช้ทั้งฝั่งจ่ายแล้วและยังไม่จ่าย)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionMonthlyStat {
    pub count: i64,
    /// ยอดเงินจริง (ไม่ normalize)
    pub amount: i64,
}

/// ยอดรายเดือน (normalize) ของแต่ละหมวด
#[derive(Serialize, ToSchema)]
pub struct SubscriptionCategoryStat {
    pub category: Option<String>,
    pub monthly_amount: i64,
    pub count: i64,
}

/// จำนวนรายการแยกตามรอบบิล
#[derive(Serialize, ToSchema)]
pub struct SubscriptionCycleSplit {
    pub monthly_count: i64,
    pub yearly_count: i64,
}

/// รายการค่าใช้จ่ายสูงสุด (คิดเป็นรายเดือน)
#[derive(Serialize, ToSchema)]
pub struct SubscriptionTopExpense {
    pub name: String,
    pub monthly_amount: i64,
}
