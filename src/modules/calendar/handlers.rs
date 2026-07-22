use axum::{Extension, Json, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::calendar::{
        dtos::{
            BotHolidayResponse, CreateEventRequest, EventFilterQuery, EventListResponse,
            EventResponse, FetchHolidayRequest, FetchHolidayResult, HolidayFilterQuery,
            HolidayListResponse, UpdateEventRequest,
        },
        services::CalendarService,
    },
};

#[utoipa::path(
    get,
    path = "/calendar/holidays",
    tag = "Calendar",
    params(HolidayFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Holiday data retrieved successfully", body = ApiResponse<HolidayListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_holidays(
    State(state): State<AppState>,
    Query(filters): Query<HolidayFilterQuery>,
) -> Result<Json<ApiResponse<HolidayListResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = CalendarService::get_holidays(&mut conn, filters.year, filters.month)?;

    Ok(Json(ApiResponse::success(200, "Holiday data retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/calendar/holiday-fetch",
    tag = "Calendar",
    request_body = FetchHolidayRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Holidays saved successfully", body = ApiResponse<FetchHolidayResult>),
        (status = 400, description = "Invalid URL or malformed data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn fetch_holidays(
    State(state): State<AppState>,
    Json(payload): Json<FetchHolidayRequest>,
) -> Result<(StatusCode, Json<ApiResponse<FetchHolidayResult>>), AppError> {
    let bot_response = reqwest::get(&payload.path)
        .await
        .map_err(|_| AppError::BadRequest("Unable to connect to the specified URL.".to_string()))?
        .json::<BotHolidayResponse>()
        .await
        .map_err(|_| AppError::InternalServerError("The data format from the URL is invalid.".to_string()))?;

    let mut conn = state.get_conn()?;
    let result = CalendarService::save_holidays(&mut conn, bot_response.holiday_calendar_lists)?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(201, "Holidays saved successfully", result))))
}

#[utoipa::path(
    delete,
    path = "/calendar/events/{event_id}",
    tag = "Calendar",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    security(("bearerAuth" = [])),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 403, description = "No permission to delete"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_event(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(event_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.get_conn()?;
    CalendarService::delete_event(&mut conn, event_id, claims.sub)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/calendar/events/{event_id}",
    tag = "Calendar",
    request_body = UpdateEventRequest,
    params(("event_id" = Uuid, Path, description = "Event ID")),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Event updated successfully", body = ApiResponse<EventResponse>),
        (status = 400, description = "Invalid data"),
        (status = 403, description = "No permission to edit"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_event(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(event_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateEventRequest>,
) -> Result<Json<ApiResponse<EventResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let result = CalendarService::update_event(&mut conn, event_id, &payload, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Event updated successfully", result)))
}

#[utoipa::path(
    get,
    path = "/calendar/events",
    tag = "Calendar",
    params(EventFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Events retrieved successfully", body = ApiResponse<EventListResponse>),
        (status = 400, description = "Invalid filter data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_events(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(filters): Query<EventFilterQuery>,
) -> Result<Json<ApiResponse<EventListResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = CalendarService::get_events(&mut conn, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "Events retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/calendar/events",
    tag = "Calendar",
    request_body = CreateEventRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Event created successfully", body = ApiResponse<Vec<EventResponse>>),
        (status = 400, description = "Invalid data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_events(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateEventRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<EventResponse>>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = CalendarService::create_events(&mut conn, &payload, claims.sub)?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(201, "Event created successfully", result))))
}
