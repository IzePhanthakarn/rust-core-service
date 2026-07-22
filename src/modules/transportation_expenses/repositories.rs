use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};
use uuid::Uuid;

use crate::{
    modules::transportation_expenses::{
        dtos::{TransportationExpenseFilterQuery, TransportationExpenseResponse},
        models::{NewTransportationExpense, TransportationExpense},
    },
    schema::transportation_expenses,
};

pub struct TransportationExpenseRepository;

impl TransportationExpenseRepository {
    pub fn find_all_transportation_expenses(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        user_id: Uuid,
        filters: &TransportationExpenseFilterQuery,
    ) -> QueryResult<(Vec<TransportationExpense>, i64)> {
        let offset = (page - 1) * limit;

        let mut data_query = transportation_expenses::table
            .filter(transportation_expenses::user_id.eq(user_id))
            .into_boxed();
        let mut count_query = transportation_expenses::table
            .filter(transportation_expenses::user_id.eq(user_id))
            .into_boxed();

        if let Some(category_text) = normalize_filter(filters.category.as_deref()) {
            data_query =
                data_query.filter(transportation_expenses::category.eq(category_text.to_string()));
            count_query = count_query
                .filter(transportation_expenses::category.eq(category_text.to_string()));
        }

        if let Some(keyword_text) = normalize_filter(filters.keyword.as_deref()) {
            let search_pattern = format!("%{}%", keyword_text);
            data_query =
                data_query.filter(transportation_expenses::title.ilike(search_pattern.clone()));
            count_query = count_query.filter(transportation_expenses::title.ilike(search_pattern));
        }

        if let Some((start_date, end_date)) =
            Self::month_year_range(filters.month.as_deref(), filters.year.as_deref())
        {
            data_query = data_query
                .filter(transportation_expenses::expense_date.ge(start_date))
                .filter(transportation_expenses::expense_date.lt(end_date));
            count_query = count_query
                .filter(transportation_expenses::expense_date.ge(start_date))
                .filter(transportation_expenses::expense_date.lt(end_date));
        }

        let items = data_query
            .order_by(transportation_expenses::expense_date.desc())
            .limit(limit)
            .offset(offset)
            .select(TransportationExpense::as_select())
            .load::<TransportationExpense>(conn)?;

        let total: i64 = count_query.count().get_result(conn)?;

        Ok((items, total))
    }

    /// รายการค่าใช้จ่ายเดินทางทั้งหมดของผู้ใช้ในช่วงเดือน/ปีที่ระบุ (ไม่ผูก filter category/keyword
    /// และไม่แบ่งหน้า) ใช้สำหรับคำนวณ stats — ถ้าไม่ระบุเดือน/ปีจะคืนทุกรายการของผู้ใช้
    pub fn find_transportation_expenses_for_stats(
        conn: &mut PgConnection,
        user_id: Uuid,
        month: Option<&str>,
        year: Option<&str>,
    ) -> QueryResult<Vec<TransportationExpense>> {
        let mut query = transportation_expenses::table
            .filter(transportation_expenses::user_id.eq(user_id))
            .into_boxed();

        if let Some((start_date, end_date)) = Self::month_year_range(month, year) {
            query = query
                .filter(transportation_expenses::expense_date.ge(start_date))
                .filter(transportation_expenses::expense_date.lt(end_date));
        }

        query
            .select(TransportationExpense::as_select())
            .load::<TransportationExpense>(conn)
    }

    pub fn find_one_transportation_expense(
        conn: &mut PgConnection,
        expense_id: Uuid,
    ) -> QueryResult<TransportationExpense> {
        transportation_expenses::table
            .filter(transportation_expenses::id.eq(expense_id))
            .first(conn)
    }

    pub fn create_transportation_expense(
        conn: &mut PgConnection,
        expense: &NewTransportationExpense<'_>,
    ) -> QueryResult<TransportationExpense> {
        diesel::insert_into(transportation_expenses::table)
            .values(expense)
            .returning(TransportationExpense::as_returning())
            .get_result(conn)
    }

    pub fn update_transportation_expense(
        conn: &mut PgConnection,
        expense_id: Uuid,
        expense: &NewTransportationExpense<'_>,
    ) -> QueryResult<TransportationExpense> {
        diesel::update(
            transportation_expenses::table.filter(transportation_expenses::id.eq(expense_id)),
        )
        .set((
            transportation_expenses::transaction_id.eq(expense.transaction_id),
            transportation_expenses::category.eq(expense.category),
            transportation_expenses::amount.eq(expense.amount),
            transportation_expenses::title.eq(expense.title),
            transportation_expenses::note.eq(expense.note),
            transportation_expenses::expense_date.eq(expense.expense_date),
            transportation_expenses::updated_at.eq(diesel::dsl::now),
        ))
        .returning(TransportationExpense::as_returning())
        .get_result(conn)
    }

    pub fn delete_transportation_expense(
        conn: &mut PgConnection,
        expense_id: Uuid,
    ) -> QueryResult<usize> {
        diesel::delete(
            transportation_expenses::table.filter(transportation_expenses::id.eq(expense_id)),
        )
        .execute(conn)
    }

    pub fn to_transportation_expense_response(
        expense: TransportationExpense,
    ) -> TransportationExpenseResponse {
        TransportationExpenseResponse {
            id: expense.id,
            user_id: expense.user_id,
            transaction_id: expense.transaction_id,
            category: expense.category,
            amount: expense.amount,
            title: expense.title,
            note: expense.note,
            expense_date: expense.expense_date,
            created_at: expense.created_at,
            updated_at: expense.updated_at,
        }
    }

    fn month_year_range(
        month: Option<&str>,
        year: Option<&str>,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let month = month?.parse::<u32>().ok()?;
        let year = year?.parse::<i32>().ok()?;

        if !(1..=12).contains(&month) {
            return None;
        }

        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let start_date = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single()?;
        let end_date = Utc
            .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
            .single()?;

        Some((start_date, end_date))
    }
}

fn normalize_filter(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}
