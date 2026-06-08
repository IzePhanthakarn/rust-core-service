use axum::{Extension, Json, extract::State, http::StatusCode};

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
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let result = WorkLogService::create_work_log(&mut conn, &payload, claims.sub)
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "สร้าง Work Log สำเร็จ", result)),
    ))
}
