use std::collections::{HashMap, HashSet};

use chrono::{DateTime, NaiveDate, Utc};
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::{
        errors::AppError,
        response::{PaginatedData, normalize_page_limit},
    },
    modules::transactions::{
        models::{NewTransaction, Transaction, TransactionType},
        repositories::TransactionRepository,
    },
    modules::transportation_expenses::{
        dtos::{
            CreateTransportationExpenseRequest, TransportationExpenseCategoryStat,
            TransportationExpenseFilterQuery, TransportationExpenseListResponse,
            TransportationExpenseResponse, TransportationExpenseStatsResponse,
            UpdateTransportationExpenseRequest,
        },
        models::{NewTransportationExpense, TransportationExpense},
        repositories::TransportationExpenseRepository,
    },
};

/// category ของ Transaction ที่ sync มาจากค่าเดินทาง (ตรงกับ property_options
/// property_type: TRANSACTION_EXPENSE_CATEGORY, value: transport)
const TRANSPORTATION_TRANSACTION_CATEGORY: &str = "transport";

pub struct TransportationExpenseService;

impl TransportationExpenseService {
    pub fn get_all_transportation_expenses(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: TransportationExpenseFilterQuery,
    ) -> Result<TransportationExpenseListResponse, AppError> {
        let (page, limit) = normalize_page_limit(filters.page, filters.limit);

        let (expenses, total_items) =
            TransportationExpenseRepository::find_all_transportation_expenses(
                conn, page, limit, user_id, &filters,
            )
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let items = expenses
            .into_iter()
            .map(TransportationExpenseRepository::to_transportation_expense_response)
            .collect();

        let stats_expenses =
            TransportationExpenseRepository::find_transportation_expenses_for_stats(
                conn,
                user_id,
                filters.month.as_deref(),
                filters.year.as_deref(),
            )
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let stats = Self::build_transportation_expense_stats(&stats_expenses);

        let pagination = PaginatedData::new(items, total_items, page, limit);

        Ok(TransportationExpenseListResponse {
            items: pagination.items,
            total_items: pagination.total_items,
            total_pages: pagination.total_pages,
            current_page: pagination.current_page,
            stats,
        })
    }

    /// คำนวณสถิติของค่าใช้จ่ายเดินทางในช่วงเดือน/ปีที่ระบุ (global ต่อผู้ใช้ ไม่ผูกกับ filter
    /// category/keyword) หน่วยเป็นสตางค์
    fn build_transportation_expense_stats(
        expenses: &[TransportationExpense],
    ) -> TransportationExpenseStatsResponse {
        let total_expense: i64 = expenses.iter().map(|expense| expense.amount).sum();

        // category -> (ยอดรวม, จำนวนรายการ)
        let mut category_map: HashMap<String, (i64, i64)> = HashMap::new();
        let mut active_days: HashSet<NaiveDate> = HashSet::new();

        for expense in expenses {
            active_days.insert(expense.expense_date.date_naive());

            let entry = category_map
                .entry(expense.category.clone())
                .or_insert((0, 0));
            entry.0 += expense.amount;
            entry.1 += 1;
        }

        let mut category_split: Vec<TransportationExpenseCategoryStat> = category_map
            .into_iter()
            .map(
                |(category, (total_amount, count))| TransportationExpenseCategoryStat {
                    category,
                    total_amount,
                    count,
                    percentage: Self::percentage_of(total_amount, total_expense),
                },
            )
            .collect();
        category_split.sort_by_key(|stat| std::cmp::Reverse(stat.total_amount));

        let average_per_active_day = if active_days.is_empty() {
            0
        } else {
            total_expense / active_days.len() as i64
        };

        TransportationExpenseStatsResponse {
            total_expense,
            average_per_active_day,
            expense_count: expenses.len() as i64,
            category_split,
        }
    }

    /// สัดส่วนของ part เทียบกับ total เป็นเปอร์เซ็นต์ ปัดเศษ 2 ตำแหน่ง
    fn percentage_of(part: i64, total: i64) -> f64 {
        if total == 0 {
            return 0.0;
        }

        Self::round_to_two_decimal_places(part as f64 / total as f64 * 100.0)
    }

    fn round_to_two_decimal_places(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    pub fn create_transportation_expense(
        conn: &mut PgConnection,
        payload: &CreateTransportationExpenseRequest,
        user_id: Uuid,
    ) -> Result<TransportationExpenseResponse, AppError> {
        let transaction_id = if payload.sync_to_transaction {
            let linked_transaction = Self::create_linked_transaction(
                conn,
                user_id,
                payload.amount,
                payload.title.trim(),
                payload.note.as_deref(),
                payload.expense_date,
            )?;
            Some(linked_transaction.id)
        } else {
            None
        };

        let new_expense = NewTransportationExpense {
            user_id,
            transaction_id,
            category: payload.category.trim(),
            amount: payload.amount,
            title: payload.title.trim(),
            note: normalize_optional(payload.note.as_deref()),
            expense_date: payload.expense_date,
        };

        let saved_expense =
            TransportationExpenseRepository::create_transportation_expense(conn, &new_expense)?;

        Ok(TransportationExpenseRepository::to_transportation_expense_response(saved_expense))
    }

    pub fn update_transportation_expense(
        conn: &mut PgConnection,
        payload: &UpdateTransportationExpenseRequest,
        user_id: Uuid,
        expense_id: Uuid,
    ) -> Result<TransportationExpenseResponse, AppError> {
        let existing = Self::find_owned_transportation_expense(conn, expense_id, user_id)?;

        let transaction_id = Self::resolve_transaction_link(
            conn,
            existing.transaction_id,
            payload.sync_to_transaction,
            user_id,
            payload.amount,
            payload.title.trim(),
            payload.note.as_deref(),
            payload.expense_date,
        )?;

        let new_expense = NewTransportationExpense {
            user_id,
            transaction_id,
            category: payload.category.trim(),
            amount: payload.amount,
            title: payload.title.trim(),
            note: normalize_optional(payload.note.as_deref()),
            expense_date: payload.expense_date,
        };

        let saved_expense = TransportationExpenseRepository::update_transportation_expense(
            conn,
            expense_id,
            &new_expense,
        )?;

        Ok(TransportationExpenseRepository::to_transportation_expense_response(saved_expense))
    }

    pub fn delete_transportation_expense(
        conn: &mut PgConnection,
        expense_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let existing = Self::find_owned_transportation_expense(conn, expense_id, user_id)?;

        TransportationExpenseRepository::delete_transportation_expense(conn, expense_id)?;

        // ลบ Transaction ที่เคย sync ไว้ด้วย กันรายการค้างเป็น ghost transaction
        if let Some(transaction_id) = existing.transaction_id {
            TransactionRepository::delete_transaction(conn, transaction_id)?;
        }

        Ok(())
    }

    /// เทียบสถานะ sync เดิมกับที่ส่งมาใหม่ แล้วสร้าง/แก้ไข/ลบ Transaction ที่ผูกไว้ให้ตรงกัน
    #[allow(clippy::too_many_arguments)]
    fn resolve_transaction_link(
        conn: &mut PgConnection,
        current_transaction_id: Option<Uuid>,
        sync_to_transaction: bool,
        user_id: Uuid,
        amount: i64,
        title: &str,
        note: Option<&str>,
        expense_date: DateTime<Utc>,
    ) -> Result<Option<Uuid>, AppError> {
        match (current_transaction_id, sync_to_transaction) {
            (Some(transaction_id), true) => {
                let new_transaction = NewTransaction {
                    user_id,
                    type_: TransactionType::Expense,
                    amount,
                    category: TRANSPORTATION_TRANSACTION_CATEGORY,
                    title,
                    note,
                    transaction_date: expense_date,
                };
                TransactionRepository::update_transaction(conn, transaction_id, &new_transaction)?;

                Ok(Some(transaction_id))
            }
            (None, true) => {
                let linked_transaction =
                    Self::create_linked_transaction(conn, user_id, amount, title, note, expense_date)?;

                Ok(Some(linked_transaction.id))
            }
            (Some(transaction_id), false) => {
                TransactionRepository::delete_transaction(conn, transaction_id)?;

                Ok(None)
            }
            (None, false) => Ok(None),
        }
    }

    fn create_linked_transaction(
        conn: &mut PgConnection,
        user_id: Uuid,
        amount: i64,
        title: &str,
        note: Option<&str>,
        expense_date: DateTime<Utc>,
    ) -> Result<Transaction, AppError> {
        let new_transaction = NewTransaction {
            user_id,
            type_: TransactionType::Expense,
            amount,
            category: TRANSPORTATION_TRANSACTION_CATEGORY,
            title,
            note,
            transaction_date: expense_date,
        };

        Ok(TransactionRepository::create_transaction(conn, &new_transaction)?)
    }

    fn find_owned_transportation_expense(
        conn: &mut PgConnection,
        expense_id: Uuid,
        user_id: Uuid,
    ) -> Result<TransportationExpense, AppError> {
        let expense =
            TransportationExpenseRepository::find_one_transportation_expense(conn, expense_id)
                .map_err(|_| AppError::NotFound("Transportation expense not found".to_string()))?;

        if expense.user_id != user_id {
            return Err(AppError::Forbidden("คุณไม่มีสิทธิ์เข้าถึงรายการนี้".to_string()));
        }

        Ok(expense)
    }
}

fn normalize_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}
