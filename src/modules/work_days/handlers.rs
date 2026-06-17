use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState,
    core::{errors::AppError, response::ApiResponse},
    modules::work_days::{
        dtos::{BotHolidayResponse, FetchHolidayRequest, FetchHolidayResult},
        services::WorkDayService,
    },
};

#[utoipa::path(
    post,
    path = "/work-days/holiday-fetch",
    tag = "Work Days",
    request_body = FetchHolidayRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "บันทึกวันหยุดสำเร็จ", body = ApiResponse<FetchHolidayResult>),
        (status = 400, description = "URL ไม่ถูกต้องหรือรูปแบบข้อมูลผิดพลาด"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn fetch_holidays(
    State(state): State<AppState>,
    Json(payload): Json<FetchHolidayRequest>,
) -> Result<(StatusCode, Json<ApiResponse<FetchHolidayResult>>), AppError> {
    let bot_response = reqwest::get(&payload.path)
        .await
        .map_err(|_| AppError::BadRequest("ไม่สามารถเชื่อมต่อ URL ที่ระบุได้".to_string()))?
        .json::<BotHolidayResponse>()
        .await
        .map_err(|_| {
            AppError::InternalServerError("รูปแบบข้อมูลจาก URL ไม่ถูกต้อง".to_string())
        })?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let result =
        WorkDayService::save_holidays(&mut conn, bot_response.holiday_calendar_lists)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "บันทึกวันหยุดสำเร็จ", result)),
    ))
}
