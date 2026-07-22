use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::transactions::{
        dtos::{
            CreateSubscriptionRequest, CreateTransactionRequest, SubscriptionFilterQuery,
            SubscriptionListResponse, SubscriptionResponse, TransactionFilterQuery,
            TransactionListResponse, TransactionResponse, UpdateSubscriptionRequest,
            UpdateTransactionRequest,
        },
        services::TransactionService,
    },
};

#[utoipa::path(
    get,
    path = "/transactions",
    tag = "Transactions",
    params(TransactionFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transactions found successfully", body = ApiResponse<TransactionListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_transactions(
    State(state): State<AppState>,
    Query(filters): Query<TransactionFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<TransactionListResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = TransactionService::get_all_transactions(&mut conn, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", data)))
}

#[utoipa::path(
    get,
    path = "/transactions/{transaction_id}",
    tag = "Transactions",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transaction found successfully", body = ApiResponse<TransactionResponse>),
        (status = 404, description = "Transaction not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_one_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(transaction_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TransactionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let transaction = TransactionService::find_one_transaction(&mut conn, transaction_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", transaction))))
}

#[utoipa::path(
    post,
    path = "/transactions",
    tag = "Transactions",
    request_body = CreateTransactionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Transaction created successfully", body = ApiResponse<TransactionResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TransactionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TransactionService::create_transaction(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "สร้าง Transaction สำเร็จ", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/transactions/{transaction_id}",
    tag = "Transactions",
    request_body = UpdateTransactionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transaction updated successfully", body = ApiResponse<TransactionResponse>),
        (status = 404, description = "Transaction not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(transaction_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTransactionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TransactionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result =
        TransactionService::update_transaction(&mut conn, &payload, claims.sub, transaction_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "แก้ไข Transaction สำเร็จ", result))))
}

#[utoipa::path(
    delete,
    path = "/transactions/{transaction_id}",
    tag = "Transactions",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transaction deleted successfully"),
        (status = 404, description = "Transaction not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(transaction_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state.get_conn()?;
    TransactionService::delete_transaction(&mut conn, transaction_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "ลบ Transaction สำเร็จ"))))
}

#[utoipa::path(
    get,
    path = "/subscriptions",
    tag = "Subscriptions",
    params(SubscriptionFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Subscriptions found successfully", body = ApiResponse<SubscriptionListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_subscriptions(
    State(state): State<AppState>,
    Query(filters): Query<SubscriptionFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<SubscriptionListResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = TransactionService::get_all_subscriptions(&mut conn, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", data)))
}

#[utoipa::path(
    post,
    path = "/subscriptions",
    tag = "Subscriptions",
    request_body = CreateSubscriptionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Subscription created successfully", body = ApiResponse<SubscriptionResponse>),
        (status = 400, description = "billing_month ไม่สอดคล้องกับ billing_cycle"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_subscription(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateSubscriptionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<SubscriptionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TransactionService::create_subscription(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "สร้าง Subscription สำเร็จ", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/subscriptions/{subscription_id}",
    tag = "Subscriptions",
    request_body = UpdateSubscriptionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Subscription updated successfully", body = ApiResponse<SubscriptionResponse>),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_subscription(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(subscription_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateSubscriptionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<SubscriptionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result =
        TransactionService::update_subscription(&mut conn, &payload, claims.sub, subscription_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "แก้ไข Subscription สำเร็จ", result))))
}

#[utoipa::path(
    patch,
    path = "/subscriptions/{subscription_id}/toggle",
    tag = "Subscriptions",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Subscription status toggled successfully", body = ApiResponse<SubscriptionResponse>),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn toggle_subscription(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(subscription_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<SubscriptionResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TransactionService::toggle_subscription(&mut conn, subscription_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "เปลี่ยนสถานะ Subscription สำเร็จ", result))))
}

#[utoipa::path(
    delete,
    path = "/subscriptions/{subscription_id}",
    tag = "Subscriptions",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Subscription deleted successfully"),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_subscription(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(subscription_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state.get_conn()?;
    TransactionService::delete_subscription(&mut conn, subscription_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "ลบ Subscription สำเร็จ"))))
}
