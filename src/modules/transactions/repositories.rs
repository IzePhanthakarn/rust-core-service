use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper};
use uuid::Uuid;

use crate::{
    modules::transactions::{
        dtos::{
            SubscriptionFilterQuery, SubscriptionResponse, TransactionFilterQuery,
            TransactionResponse,
        },
        models::{NewSubscription, NewTransaction, Subscription, Transaction},
    },
    schema::{subscriptions, transactions},
};

pub struct TransactionRepository;

impl TransactionRepository {
    pub fn find_all_transactions(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        user_id: Uuid,
        filters: &TransactionFilterQuery,
    ) -> QueryResult<(Vec<Transaction>, i64)> {
        let offset = (page - 1) * limit;

        let mut data_query = transactions::table
            .filter(transactions::user_id.eq(user_id))
            .into_boxed();
        let mut count_query = transactions::table
            .filter(transactions::user_id.eq(user_id))
            .into_boxed();

        if let Some(transaction_type) = filters.type_ {
            data_query = data_query.filter(transactions::type_.eq(transaction_type));
            count_query = count_query.filter(transactions::type_.eq(transaction_type));
        }

        if let Some(category_text) = normalize_filter(filters.category.as_deref()) {
            data_query = data_query.filter(transactions::category.eq(category_text.to_string()));
            count_query = count_query.filter(transactions::category.eq(category_text.to_string()));
        }

        if let Some(keyword_text) = normalize_filter(filters.keyword.as_deref()) {
            let search_pattern = format!("%{}%", keyword_text);
            data_query = data_query.filter(transactions::title.ilike(search_pattern.clone()));
            count_query = count_query.filter(transactions::title.ilike(search_pattern));
        }

        if let Some((start_date, end_date)) =
            Self::month_year_range(filters.month.as_deref(), filters.year.as_deref())
        {
            data_query = data_query
                .filter(transactions::transaction_date.ge(start_date))
                .filter(transactions::transaction_date.lt(end_date));
            count_query = count_query
                .filter(transactions::transaction_date.ge(start_date))
                .filter(transactions::transaction_date.lt(end_date));
        }

        let items = data_query
            .order_by(transactions::transaction_date.desc())
            .limit(limit)
            .offset(offset)
            .select(Transaction::as_select())
            .load::<Transaction>(conn)?;

        let total: i64 = count_query.count().get_result(conn)?;

        Ok((items, total))
    }

    /// รายการ transaction ทั้งหมดของผู้ใช้ในช่วงเดือน/ปีที่ระบุ (ไม่ผูก filter type/category/keyword
    /// และไม่แบ่งหน้า) ใช้สำหรับคำนวณ stats — ถ้าไม่ระบุเดือน/ปีจะคืนทุกรายการของผู้ใช้
    pub fn find_transactions_for_stats(
        conn: &mut PgConnection,
        user_id: Uuid,
        month: Option<&str>,
        year: Option<&str>,
    ) -> QueryResult<Vec<Transaction>> {
        let mut query = transactions::table
            .filter(transactions::user_id.eq(user_id))
            .into_boxed();

        if let Some((start_date, end_date)) = Self::month_year_range(month, year) {
            query = query
                .filter(transactions::transaction_date.ge(start_date))
                .filter(transactions::transaction_date.lt(end_date));
        }

        query
            .select(Transaction::as_select())
            .load::<Transaction>(conn)
    }

    pub fn find_one_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
    ) -> QueryResult<Transaction> {
        transactions::table
            .filter(transactions::id.eq(transaction_id))
            .first(conn)
    }

    pub fn create_transaction(
        conn: &mut PgConnection,
        transaction: &NewTransaction<'_>,
    ) -> QueryResult<Transaction> {
        diesel::insert_into(transactions::table)
            .values(transaction)
            .returning(Transaction::as_returning())
            .get_result(conn)
    }

    pub fn update_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
        transaction: &NewTransaction<'_>,
    ) -> QueryResult<Transaction> {
        diesel::update(transactions::table.filter(transactions::id.eq(transaction_id)))
            .set((
                transactions::type_.eq(transaction.type_),
                transactions::amount.eq(transaction.amount),
                transactions::category.eq(transaction.category),
                transactions::title.eq(transaction.title),
                transactions::note.eq(transaction.note),
                transactions::transaction_date.eq(transaction.transaction_date),
                transactions::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Transaction::as_returning())
            .get_result(conn)
    }

    pub fn delete_transaction(
        conn: &mut PgConnection,
        transaction_id: Uuid,
    ) -> QueryResult<usize> {
        diesel::delete(transactions::table.filter(transactions::id.eq(transaction_id)))
            .execute(conn)
    }

    pub fn find_all_subscriptions(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: &SubscriptionFilterQuery,
    ) -> QueryResult<Vec<Subscription>> {
        let mut query = subscriptions::table
            .filter(subscriptions::user_id.eq(user_id))
            .into_boxed();

        if let Some(cycle) = filters.billing_cycle {
            query = query.filter(subscriptions::billing_cycle.eq(cycle));
        }

        if let Some(active) = filters.is_active {
            query = query.filter(subscriptions::is_active.eq(active));
        }

        if let Some(keyword_text) = normalize_filter(filters.keyword.as_deref()) {
            query = query.filter(subscriptions::name.ilike(format!("%{}%", keyword_text)));
        }

        query
            .order_by((
                subscriptions::created_at.desc(),
            ))
            .select(Subscription::as_select())
            .load::<Subscription>(conn)
    }

    pub fn find_one_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
    ) -> QueryResult<Subscription> {
        subscriptions::table
            .filter(subscriptions::id.eq(subscription_id))
            .first(conn)
    }

    pub fn create_subscription(
        conn: &mut PgConnection,
        subscription: &NewSubscription<'_>,
    ) -> QueryResult<Subscription> {
        diesel::insert_into(subscriptions::table)
            .values(subscription)
            .returning(Subscription::as_returning())
            .get_result(conn)
    }

    pub fn update_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
        subscription: &NewSubscription<'_>,
    ) -> QueryResult<Subscription> {
        diesel::update(subscriptions::table.filter(subscriptions::id.eq(subscription_id)))
            .set((
                subscriptions::name.eq(subscription.name),
                subscriptions::amount.eq(subscription.amount),
                subscriptions::billing_cycle.eq(subscription.billing_cycle),
                subscriptions::billing_day.eq(subscription.billing_day),
                subscriptions::billing_month.eq(subscription.billing_month),
                subscriptions::category.eq(subscription.category),
                subscriptions::note.eq(subscription.note),
                subscriptions::start_date.eq(subscription.start_date),
                subscriptions::end_date.eq(subscription.end_date),
                subscriptions::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Subscription::as_returning())
            .get_result(conn)
    }

    pub fn set_subscription_active(
        conn: &mut PgConnection,
        subscription_id: Uuid,
        is_active: bool,
    ) -> QueryResult<Subscription> {
        diesel::update(subscriptions::table.filter(subscriptions::id.eq(subscription_id)))
            .set((
                subscriptions::is_active.eq(is_active),
                subscriptions::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Subscription::as_returning())
            .get_result(conn)
    }

    pub fn delete_subscription(
        conn: &mut PgConnection,
        subscription_id: Uuid,
    ) -> QueryResult<usize> {
        diesel::delete(subscriptions::table.filter(subscriptions::id.eq(subscription_id)))
            .execute(conn)
    }

    /// รายการ subscription ที่ยัง active ทั้งหมด (ไม่ผูก filter) ใช้สำหรับคำนวณ stats
    pub fn find_active_subscriptions(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> QueryResult<Vec<Subscription>> {
        subscriptions::table
            .filter(subscriptions::user_id.eq(user_id))
            .filter(subscriptions::is_active.eq(true))
            .select(Subscription::as_select())
            .load::<Subscription>(conn)
    }

    /// จำนวน subscription ทั้งหมดของผู้ใช้ (รวมรายการที่ inactive)
    pub fn count_subscriptions(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<i64> {
        subscriptions::table
            .filter(subscriptions::user_id.eq(user_id))
            .count()
            .get_result(conn)
    }

    pub fn to_transaction_response(transaction: Transaction) -> TransactionResponse {
        TransactionResponse {
            id: transaction.id,
            user_id: transaction.user_id,
            type_: transaction.type_,
            amount: transaction.amount,
            category: transaction.category,
            title: transaction.title,
            note: transaction.note,
            transaction_date: transaction.transaction_date,
            created_at: transaction.created_at,
            updated_at: transaction.updated_at,
        }
    }

    pub fn to_subscription_response(subscription: Subscription) -> SubscriptionResponse {
        SubscriptionResponse {
            id: subscription.id,
            user_id: subscription.user_id,
            name: subscription.name,
            amount: subscription.amount,
            billing_cycle: subscription.billing_cycle,
            billing_day: subscription.billing_day,
            billing_month: subscription.billing_month,
            category: subscription.category,
            note: subscription.note,
            start_date: subscription.start_date,
            end_date: subscription.end_date,
            is_active: subscription.is_active,
            created_at: subscription.created_at,
            updated_at: subscription.updated_at,
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
