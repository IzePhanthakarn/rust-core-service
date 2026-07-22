use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, Utc};
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::{
        errors::AppError,
        response::{PaginatedData, normalize_page_limit},
    },
    modules::transactions::{
        dtos::{
            CreateSubscriptionRequest, CreateTransactionRequest, SubscriptionCategoryStat,
            SubscriptionCycleSplit, SubscriptionFilterQuery, SubscriptionListResponse,
            SubscriptionMonthlyStat, SubscriptionResponse, SubscriptionStatsResponse,
            SubscriptionTopExpense, TransactionCategoryStat, TransactionFilterQuery,
            TransactionListResponse, TransactionResponse, TransactionStatsResponse,
            UpdateSubscriptionRequest, UpdateTransactionRequest,
        },
        models::{
            BillingCycle, NewSubscription, NewTransaction, Subscription, Transaction,
            TransactionType,
        },
        repositories::TransactionRepository,
    },
};

/// จำนวนรายการค่าใช้จ่ายสูงสุดที่ส่งกลับ
const TOP_EXPENSES_LIMIT: usize = 4;

/// จำนวนหมวดหมู่รายจ่ายสูงสุดที่ส่งกลับใน stats ของ transaction
const TOP_EXPENSE_CATEGORIES_LIMIT: usize = 5;

pub struct TransactionService;

impl TransactionService {
    pub fn get_all_transactions(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: TransactionFilterQuery,
    ) -> Result<TransactionListResponse, AppError> {
        let (page, limit) = normalize_page_limit(filters.page, filters.limit);

        let (transactions, total_items) =
            TransactionRepository::find_all_transactions(conn, page, limit, user_id, &filters)
                .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let items = transactions
            .into_iter()
            .map(TransactionRepository::to_transaction_response)
            .collect();

        let stats_transactions = TransactionRepository::find_transactions_for_stats(
            conn,
            user_id,
            filters.month.as_deref(),
            filters.year.as_deref(),
        )
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let stats = Self::build_transaction_stats(
            &stats_transactions,
            filters.month.as_deref(),
            filters.year.as_deref(),
        );

        let pagination = PaginatedData::new(items, total_items, page, limit);

        Ok(TransactionListResponse {
            items: pagination.items,
            total_items: pagination.total_items,
            total_pages: pagination.total_pages,
            current_page: pagination.current_page,
            stats,
        })
    }

    /// คำนวณสถิติของ transaction ในช่วงเดือน/ปีที่ระบุ (global ต่อผู้ใช้ ไม่ผูกกับ filter
    /// type/category/keyword) หน่วยเป็นสตางค์
    fn build_transaction_stats(
        transactions: &[Transaction],
        month: Option<&str>,
        year: Option<&str>,
    ) -> TransactionStatsResponse {
        let mut total_income: i64 = 0;
        let mut total_expense: i64 = 0;
        // category -> (ยอดรวม, จำนวนรายการ)
        let mut category_map: HashMap<String, (i64, i64)> = HashMap::new();

        for transaction in transactions {
            match transaction.type_ {
                TransactionType::Income => total_income += transaction.amount,
                TransactionType::Expense => {
                    total_expense += transaction.amount;

                    let entry = category_map
                        .entry(transaction.category.clone())
                        .or_insert((0, 0));
                    entry.0 += transaction.amount;
                    entry.1 += 1;
                }
            }
        }

        let mut top_expense_category: Vec<TransactionCategoryStat> = category_map
            .into_iter()
            .map(|(category, (total_amount, count))| TransactionCategoryStat {
                category,
                total_amount,
                count,
            })
            .collect();
        top_expense_category.sort_by_key(|stat| std::cmp::Reverse(stat.total_amount));
        top_expense_category.truncate(TOP_EXPENSE_CATEGORIES_LIMIT);

        let days_in_range = Self::days_in_range(transactions, month, year);
        let average_daily_expense = if days_in_range > 0 {
            total_expense / days_in_range
        } else {
            0
        };

        TransactionStatsResponse {
            total_income,
            total_expense,
            top_expense_category,
            average_daily_expense,
            transaction_count: transactions.len() as i64,
        }
    }

    /// จำนวนวันของช่วงที่ใช้คำนวณค่าเฉลี่ยต่อวัน
    /// - ถ้าระบุเดือน/ปี ใช้จำนวนวันจริงของเดือนนั้น
    /// - ถ้าไม่ระบุ ใช้จำนวนวันระหว่างรายการแรกสุดถึงล่าสุดของผลลัพธ์ (อย่างน้อย 1 วัน)
    fn days_in_range(transactions: &[Transaction], month: Option<&str>, year: Option<&str>) -> i64 {
        if let (Some(month), Some(year)) = (
            month.and_then(|m| m.parse::<u32>().ok()),
            year.and_then(|y| y.parse::<i32>().ok()),
        ) && (1..=12).contains(&month)
        {
            return last_day_of_month(year, month) as i64;
        }

        let dates: Vec<NaiveDate> = transactions
            .iter()
            .map(|transaction| transaction.transaction_date.date_naive())
            .collect();

        match (dates.iter().min(), dates.iter().max()) {
            (Some(min_date), Some(max_date)) => (*max_date - *min_date).num_days() + 1,
            _ => 1,
        }
    }

    pub fn find_one_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
        user_id: Uuid,
    ) -> Result<TransactionResponse, AppError> {
        let transaction = Self::find_owned_transaction(conn, transaction_id, user_id)?;

        Ok(TransactionRepository::to_transaction_response(transaction))
    }

    pub fn create_transaction(
        conn: &mut PgConnection,
        payload: &CreateTransactionRequest,
        user_id: Uuid,
    ) -> Result<TransactionResponse, AppError> {
        let new_transaction = NewTransaction {
            user_id,
            type_: payload.type_,
            amount: payload.amount,
            category: payload.category.trim(),
            title: payload.title.trim(),
            note: normalize_optional(payload.note.as_deref()),
            transaction_date: payload.transaction_date,
        };

        let saved_transaction = TransactionRepository::create_transaction(conn, &new_transaction)?;

        Ok(TransactionRepository::to_transaction_response(
            saved_transaction,
        ))
    }

    pub fn update_transaction(
        conn: &mut PgConnection,
        payload: &UpdateTransactionRequest,
        user_id: Uuid,
        transaction_id: Uuid,
    ) -> Result<TransactionResponse, AppError> {
        Self::find_owned_transaction(conn, transaction_id, user_id)?;

        let new_transaction = NewTransaction {
            user_id,
            type_: payload.type_,
            amount: payload.amount,
            category: payload.category.trim(),
            title: payload.title.trim(),
            note: normalize_optional(payload.note.as_deref()),
            transaction_date: payload.transaction_date,
        };

        let saved_transaction =
            TransactionRepository::update_transaction(conn, transaction_id, &new_transaction)?;

        Ok(TransactionRepository::to_transaction_response(
            saved_transaction,
        ))
    }

    pub fn delete_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::find_owned_transaction(conn, transaction_id, user_id)?;
        TransactionRepository::delete_transaction(conn, transaction_id)?;

        Ok(())
    }

    pub fn get_all_subscriptions(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: SubscriptionFilterQuery,
    ) -> Result<SubscriptionListResponse, AppError> {
        let subscriptions = TransactionRepository::find_all_subscriptions(conn, user_id, &filters)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let active_subscriptions =
            TransactionRepository::find_active_subscriptions(conn, user_id)
                .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let total_count = TransactionRepository::count_subscriptions(conn, user_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let items = subscriptions
            .into_iter()
            .map(TransactionRepository::to_subscription_response)
            .collect();

        let stats = Self::build_subscription_stats(&active_subscriptions, total_count);

        Ok(SubscriptionListResponse { items, stats })
    }

    /// คำนวณสถิติจากรายการ active ทั้งหมด (global ไม่ผูกกับ filter) หน่วยเป็นสตางค์
    fn build_subscription_stats(
        active_subscriptions: &[Subscription],
        total_count: i64,
    ) -> SubscriptionStatsResponse {
        let today = Utc::now().date_naive();

        let mut monthly_recurring: i64 = 0;
        let mut yearly_estimate: i64 = 0;
        let mut remaining_count: i64 = 0;
        let mut remaining_amount: i64 = 0;
        let mut passed_count: i64 = 0;
        let mut passed_amount: i64 = 0;
        let mut monthly_count: i64 = 0;
        let mut yearly_count: i64 = 0;
        // category -> (ยอดรายเดือน normalize, จำนวนรายการ)
        let mut category_map: HashMap<Option<String>, (i64, i64)> = HashMap::new();

        for subscription in active_subscriptions {
            let monthly_amount = normalize_monthly(subscription);
            monthly_recurring += monthly_amount;

            match subscription.billing_cycle {
                BillingCycle::Monthly => {
                    yearly_estimate += subscription.amount * 12;
                    monthly_count += 1;
                }
                BillingCycle::Yearly => {
                    yearly_estimate += subscription.amount;
                    yearly_count += 1;
                }
            }

            // เฉพาะรายการที่ครบกำหนดตัดเงินในเดือนนี้ (รายเดือนทุกตัว + รายปีที่ตรงเดือน)
            // ถ้าวันตัดผ่านไปแล้ว = จ่ายแล้ว, ยังไม่ถึง = ยังเหลือ
            if let Some(billing_date) = current_month_billing_date(subscription, today) {
                if billing_date < today {
                    passed_count += 1;
                    passed_amount += subscription.amount;
                } else {
                    remaining_count += 1;
                    remaining_amount += subscription.amount;
                }
            }

            let entry = category_map
                .entry(subscription.category.clone())
                .or_insert((0, 0));
            entry.0 += monthly_amount;
            entry.1 += 1;
        }

        let mut category_breakdown: Vec<SubscriptionCategoryStat> = category_map
            .into_iter()
            .map(|(category, (monthly_amount, count))| SubscriptionCategoryStat {
                category,
                monthly_amount,
                count,
            })
            .collect();
        category_breakdown.sort_by_key(|stat| std::cmp::Reverse(stat.monthly_amount));

        let mut top_expenses: Vec<SubscriptionTopExpense> = active_subscriptions
            .iter()
            .map(|subscription| SubscriptionTopExpense {
                name: subscription.name.clone(),
                monthly_amount: normalize_monthly(subscription),
            })
            .collect();
        top_expenses.sort_by_key(|expense| std::cmp::Reverse(expense.monthly_amount));
        top_expenses.truncate(TOP_EXPENSES_LIMIT);

        SubscriptionStatsResponse {
            monthly_recurring,
            yearly_estimate,
            active_count: active_subscriptions.len() as i64,
            total_count,
            remaining_this_month: SubscriptionMonthlyStat {
                count: remaining_count,
                amount: remaining_amount,
            },
            passed_this_month: SubscriptionMonthlyStat {
                count: passed_count,
                amount: passed_amount,
            },
            category_breakdown,
            cycle_split: SubscriptionCycleSplit {
                monthly_count,
                yearly_count,
            },
            top_expenses,
        }
    }

    pub fn create_subscription(
        conn: &mut PgConnection,
        payload: &CreateSubscriptionRequest,
        user_id: Uuid,
    ) -> Result<SubscriptionResponse, AppError> {
        let billing_month = Self::resolve_billing_month(payload.billing_cycle, payload.billing_month)?;

        let new_subscription = NewSubscription {
            user_id,
            name: payload.name.trim(),
            amount: payload.amount,
            billing_cycle: payload.billing_cycle,
            billing_day: payload.billing_day,
            billing_month,
            category: normalize_optional(payload.category.as_deref()),
            note: normalize_optional(payload.note.as_deref()),
            start_date: payload.start_date,
            end_date: payload.end_date,
        };

        let saved_subscription =
            TransactionRepository::create_subscription(conn, &new_subscription)?;

        Ok(TransactionRepository::to_subscription_response(
            saved_subscription,
        ))
    }

    pub fn update_subscription(
        conn: &mut PgConnection,
        payload: &UpdateSubscriptionRequest,
        user_id: Uuid,
        subscription_id: Uuid,
    ) -> Result<SubscriptionResponse, AppError> {
        Self::find_owned_subscription(conn, subscription_id, user_id)?;

        let billing_month = Self::resolve_billing_month(payload.billing_cycle, payload.billing_month)?;

        let new_subscription = NewSubscription {
            user_id,
            name: payload.name.trim(),
            amount: payload.amount,
            billing_cycle: payload.billing_cycle,
            billing_day: payload.billing_day,
            billing_month,
            category: normalize_optional(payload.category.as_deref()),
            note: normalize_optional(payload.note.as_deref()),
            start_date: payload.start_date,
            end_date: payload.end_date,
        };

        let saved_subscription =
            TransactionRepository::update_subscription(conn, subscription_id, &new_subscription)?;

        Ok(TransactionRepository::to_subscription_response(
            saved_subscription,
        ))
    }

    pub fn toggle_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
        user_id: Uuid,
    ) -> Result<SubscriptionResponse, AppError> {
        let subscription = Self::find_owned_subscription(conn, subscription_id, user_id)?;

        let updated_subscription = TransactionRepository::set_subscription_active(
            conn,
            subscription_id,
            !subscription.is_active,
        )?;

        Ok(TransactionRepository::to_subscription_response(
            updated_subscription,
        ))
    }

    pub fn delete_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::find_owned_subscription(conn, subscription_id, user_id)?;
        TransactionRepository::delete_subscription(conn, subscription_id)?;

        Ok(())
    }

    /// รายปีต้องระบุเดือนที่ตัดเงิน ส่วนรายเดือนต้องไม่มีเดือน (ตรงกับ CHECK constraint ฝั่ง DB)
    fn resolve_billing_month(
        billing_cycle: BillingCycle,
        billing_month: Option<i32>,
    ) -> Result<Option<i32>, AppError> {
        match billing_cycle {
            BillingCycle::Yearly => billing_month.map(Some).ok_or_else(|| {
                AppError::BadRequest("ต้องระบุ billing_month เมื่อจ่ายแบบรายปี".to_string())
            }),
            BillingCycle::Monthly => {
                if billing_month.is_some() {
                    return Err(AppError::BadRequest(
                        "จ่ายแบบรายเดือนต้องไม่ระบุ billing_month".to_string(),
                    ));
                }

                Ok(None)
            }
        }
    }

    fn find_owned_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
        user_id: Uuid,
    ) -> Result<Transaction, AppError> {
        let transaction = TransactionRepository::find_one_transaction(conn, transaction_id)
            .map_err(|_| AppError::NotFound("Transaction not found".to_string()))?;

        if transaction.user_id != user_id {
            return Err(AppError::Forbidden("คุณไม่มีสิทธิ์เข้าถึง Transaction นี้".to_string()));
        }

        Ok(transaction)
    }

    fn find_owned_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
        user_id: Uuid,
    ) -> Result<Subscription, AppError> {
        let subscription = TransactionRepository::find_one_subscription(conn, subscription_id)
            .map_err(|_| AppError::NotFound("Subscription not found".to_string()))?;

        if subscription.user_id != user_id {
            return Err(AppError::Forbidden("คุณไม่มีสิทธิ์เข้าถึง Subscription นี้".to_string()));
        }

        Ok(subscription)
    }
}

fn normalize_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}

/// แปลงยอดของ subscription ให้เป็นค่าใช้จ่ายต่อเดือน (รายปีหาร 12)
fn normalize_monthly(subscription: &Subscription) -> i64 {
    match subscription.billing_cycle {
        BillingCycle::Monthly => subscription.amount,
        BillingCycle::Yearly => subscription.amount / 12,
    }
}

/// จำนวนวันสูงสุดของเดือน ใช้ clamp billing_day ที่เกิน (เช่น 31 ในเดือน ก.พ.)
fn last_day_of_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let first_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .expect("month is within 1..=12");

    (first_of_next_month - Duration::days(1)).day()
}

/// สร้างวันตัดเงินของเดือนที่กำหนด โดย clamp วันให้ไม่เกินวันสิ้นเดือน
fn build_billing_date(year: i32, month: u32, billing_day: i32) -> NaiveDate {
    let day = (billing_day.max(1) as u32).min(last_day_of_month(year, month));

    NaiveDate::from_ymd_opt(year, month, day).expect("day is clamped within the month")
}

/// วันตัดเงินของเดือนปัจจุบัน — คืน None ถ้ารายการนี้ไม่ได้ครบกำหนดในเดือนนี้
/// (รายเดือนครบทุกเดือน, รายปีครบเฉพาะเดือนที่ตรง billing_month)
fn current_month_billing_date(subscription: &Subscription, today: NaiveDate) -> Option<NaiveDate> {
    match subscription.billing_cycle {
        BillingCycle::Monthly => Some(build_billing_date(
            today.year(),
            today.month(),
            subscription.billing_day,
        )),
        BillingCycle::Yearly => {
            let month = subscription.billing_month.unwrap_or(1) as u32;

            if month == today.month() {
                Some(build_billing_date(
                    today.year(),
                    today.month(),
                    subscription.billing_day,
                ))
            } else {
                None
            }
        }
    }
}
