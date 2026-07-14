use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::{
        errors::AppError,
        response::{PaginatedData, normalize_page_limit},
    },
    modules::transactions::{
        dtos::{
            CreateSubscriptionRequest, CreateTransactionRequest, SubscriptionFilterQuery,
            SubscriptionResponse, SubscriptionSummaryResponse, TransactionFilterQuery,
            TransactionResponse, UpdateSubscriptionRequest, UpdateTransactionRequest,
        },
        models::{BillingCycle, NewSubscription, NewTransaction, Subscription, Transaction},
        repositories::TransactionRepository,
    },
};

pub struct TransactionService;

impl TransactionService {
    pub fn get_all_transactions(
        conn: &mut PgConnection,
        user_id: Uuid,
        filters: TransactionFilterQuery,
    ) -> Result<PaginatedData<TransactionResponse>, AppError> {
        let (page, limit) = normalize_page_limit(filters.page, filters.limit);

        let (transactions, total_items) =
            TransactionRepository::find_all_transactions(conn, page, limit, user_id, &filters)
                .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let items = transactions
            .into_iter()
            .map(TransactionRepository::to_transaction_response)
            .collect();

        Ok(PaginatedData::new(items, total_items, page, limit))
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
    ) -> Result<Vec<SubscriptionResponse>, AppError> {
        let subscriptions = TransactionRepository::find_all_subscriptions(conn, user_id, &filters)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(subscriptions
            .into_iter()
            .map(TransactionRepository::to_subscription_response)
            .collect())
    }

    pub fn get_subscription_summary(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<SubscriptionSummaryResponse, AppError> {
        let (monthly_total, yearly_total, active_count) =
            TransactionRepository::sum_active_subscriptions(conn, user_id)
                .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(SubscriptionSummaryResponse {
            monthly_total,
            yearly_total,
            estimated_monthly_total: monthly_total + yearly_total / 12,
            estimated_yearly_total: monthly_total * 12 + yearly_total,
            active_count,
        })
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
