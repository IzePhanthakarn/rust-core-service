use axum::{Extension, Json, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::work_days::{
        dtos::{
            BotHolidayResponse, CreateEventRequest, EventFilterQuery, EventListResponse,
            EventResponse, FetchHolidayRequest, FetchHolidayResult, HolidayFilterQuery,
            HolidayListResponse, UpdateEventRequest,
        },
        services::WorkDayService,
    },
};

#[utoipa::path(
    get,
    path = "/work-days/holidays",
    tag = "Work Days",
    params(HolidayFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูลวันหยุดสำเร็จ", body = ApiResponse<HolidayListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_holidays(
    State(state): State<AppState>,
    Query(filters): Query<HolidayFilterQuery>,
) -> Result<Json<ApiResponse<HolidayListResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let data = WorkDayService::get_holidays(&mut conn, filters.year)?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลวันหยุดสำเร็จ", data)))
}

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

#[utoipa::path(
    delete,
    path = "/work-days/events/{event_id}",
    tag = "Work Days",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    security(("bearerAuth" = [])),
    responses(
        (status = 204, description = "ลบ event สำเร็จ"),
        (status = 403, description = "ไม่มีสิทธิ์ลบ"),
        (status = 404, description = "ไม่พบ event"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_event(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(event_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    WorkDayService::delete_event(&mut conn, event_id, claims.sub)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/work-days/events/{event_id}",
    tag = "Work Days",
    request_body = UpdateEventRequest,
    params(("event_id" = Uuid, Path, description = "Event ID")),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "แก้ไข event สำเร็จ", body = ApiResponse<EventResponse>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง"),
        (status = 403, description = "ไม่มีสิทธิ์แก้ไข"),
        (status = 404, description = "ไม่พบ event"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_event(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(event_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateEventRequest>,
) -> Result<Json<ApiResponse<EventResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let result = WorkDayService::update_event(&mut conn, event_id, &payload, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "แก้ไข event สำเร็จ", result)))
}

#[utoipa::path(
    get,
    path = "/work-days/events",
    tag = "Work Days",
    params(EventFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูล events สำเร็จ", body = ApiResponse<EventListResponse>),
        (status = 400, description = "ข้อมูล filter ไม่ถูกต้อง"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_events(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<EventFilterQuery>,
) -> Result<Json<ApiResponse<EventListResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let data = WorkDayService::get_events(&mut conn, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูล events สำเร็จ", data)))
}

#[utoipa::path(
    post,
    path = "/work-days/events",
    tag = "Work Days",
    request_body = CreateEventRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "สร้าง event สำเร็จ", body = ApiResponse<Vec<EventResponse>>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_events(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateEventRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<EventResponse>>>), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database connection error".to_string()))?;

    let result = WorkDayService::create_events(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "สร้าง event สำเร็จ", result)),
    ))
}
