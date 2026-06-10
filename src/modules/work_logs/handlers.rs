use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::work_logs::{
        dtos::{CreateWorkLogRequest, WorkLogResponse},
        services::WorkLogService,
    },
};

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
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Work Log".to_string()));
    }

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
    ValidatedJson(payload): ValidatedJson<CreateWorkLogRequest>,
) -> Result<(StatusCode, Json<ApiResponse<WorkLogResponse>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์แก้ไข Work Log".to_string()));
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
