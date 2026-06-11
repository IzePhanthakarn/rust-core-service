use crate::{
    AppState,
    core::{errors::AppError, response::ApiResponse},
    modules::health::{dtos::HealthData, services::HealthService},
};
use axum::{Json, extract::State};

#[utoipa::path(
    get,
    path = "/health",
    tag = "System Health",
    responses(
        (status = 200, description = "System is healthy", body = ApiResponse<HealthData>)
    )
)]
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<HealthData>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database Connection Error".to_string()))?;

    // === เพิ่ม state.sys.clone() เข้าไปเป็น Parameter ตัวสุดท้าย ===
    let data = HealthService::get_system_health(&mut conn, &state.db_pool, state.start_time)?;

    Ok(Json(ApiResponse::success(200, "Server is healthy", data)))
}
