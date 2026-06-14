use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, PaginatedData},
    },
    modules::work_logs::{
        dtos::{CreateWorkLogRequest, UpdateWorkLogRequest, WorkLogFilterQuery, WorkLogResponse},
        services::WorkLogService,
    },
};

#[utoipa::path(
    get,
    path = "/work-logs",
    tag = "Work Logs",
    params(WorkLogFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Work logs found successfully", body = ApiResponse<PaginatedData<WorkLogResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_work_logs(
    State(state): State<AppState>,
    Query(filters): Query<WorkLogFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<WorkLogResponse>>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let data = WorkLogService::get_all_work_logs(&mut conn, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", data)))
}

#[utoipa::path(
    get,
    path = "/work-logs/{work_log_id}",
    tag = "Work Logs",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Work log found successfully", body = WorkLogResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_one_work_log(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(work_log_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<WorkLogResponse>>), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let work_log = WorkLogService::find_one_work_log(&mut conn, work_log_id, claims.sub)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            200,
            "Work Log found successfully",
            work_log,
        )),
    ))
}

#[utoipa::path(
    post,
    path = "/work-logs",
    tag = "Work Logs",
    request_body = CreateWorkLogRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Work log created successfully", body = WorkLogResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_work_log(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateWorkLogRequest>,
) -> Result<(StatusCode, Json<ApiResponse<WorkLogResponse>>), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;

    let result = WorkLogService::create_work_log(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "สร้าง Work Log สำเร็จ", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/work-logs/{work_log_id}",
    tag = "Work Logs",
    request_body = CreateWorkLogRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Work log updated successfully", body = WorkLogResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_work_log(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(work_log_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateWorkLogRequest>,
) -> Result<(StatusCode, Json<ApiResponse<WorkLogResponse>>), AppError> {
    if claims.sub != payload.user_id {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์แก้ไข Work Log นี้".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;

    let result = WorkLogService::update_work_log(&mut conn, &payload, claims.sub, work_log_id)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(200, "แก้ไข Work Log สำเร็จ", result)),
    ))
}

#[utoipa::path(
    delete,
    path = "/work-logs/{work_log_id}",
    tag = "Work Logs",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Work log deleted successfully"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_work_log(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(work_log_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;

    WorkLogService::delete_work_log(&mut conn, work_log_id, claims.sub)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success_without_data(200, "ลบ Work Log สำเร็จ")),
    ))
}
