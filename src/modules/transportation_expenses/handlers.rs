use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::transportation_expenses::{
        dtos::{
            CreateTransportationExpenseRequest, TransportationExpenseFilterQuery,
            TransportationExpenseListResponse, TransportationExpenseResponse,
            UpdateTransportationExpenseRequest,
        },
        services::TransportationExpenseService,
    },
};

#[utoipa::path(
    get,
    path = "/transportation-expenses",
    tag = "Transportation Expenses",
    params(TransportationExpenseFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transportation expenses found successfully", body = ApiResponse<TransportationExpenseListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_transportation_expenses(
    State(state): State<AppState>,
    Query(filters): Query<TransportationExpenseFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<TransportationExpenseListResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = TransportationExpenseService::get_all_transportation_expenses(
        &mut conn, claims.sub, filters,
    )?;

    Ok(Json(ApiResponse::success(200, "Data retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/transportation-expenses",
    tag = "Transportation Expenses",
    request_body = CreateTransportationExpenseRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Transportation expense created successfully", body = ApiResponse<TransportationExpenseResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_transportation_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateTransportationExpenseRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TransportationExpenseResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TransportationExpenseService::create_transportation_expense(
        &mut conn, &payload, claims.sub,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Transportation expense created successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/transportation-expenses/{expense_id}",
    tag = "Transportation Expenses",
    request_body = UpdateTransportationExpenseRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transportation expense updated successfully", body = ApiResponse<TransportationExpenseResponse>),
        (status = 404, description = "Transportation expense not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_transportation_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(expense_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTransportationExpenseRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TransportationExpenseResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = TransportationExpenseService::update_transportation_expense(
        &mut conn,
        &payload,
        claims.sub,
        expense_id,
    )?;

    Ok((StatusCode::OK, Json(ApiResponse::success(200, "Transportation expense updated successfully", result))))
}

#[utoipa::path(
    delete,
    path = "/transportation-expenses/{expense_id}",
    tag = "Transportation Expenses",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Transportation expense deleted successfully"),
        (status = 404, description = "Transportation expense not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_transportation_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(expense_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state.get_conn()?;
    TransportationExpenseService::delete_transportation_expense(&mut conn, expense_id, claims.sub)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "Transportation expense deleted successfully"))))
}
