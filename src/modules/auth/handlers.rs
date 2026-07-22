use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};

use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData},
    },
    modules::auth::{
        dtos::{
            AuthResponse, ChangePasswordRequest, LoginRequest, RefreshRequest, RegisterRequest,
            RegisterResponse, ResetPasswordRequest,
        },
        services::AuthService,
    },
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = ApiResponse<RegisterResponse>),
        (status = 400, description = "Invalid data (validation error)", body = ApiResponse<EmptyData>),
        (status = 409, description = "This email is already in use", body = ApiResponse<EmptyData>)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<EmptyData>>), AppError> {
    let mut conn = state.get_conn()?;

    AuthService::register(&mut conn, payload)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success_without_data(201, "Registration successful")),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<AuthResponse>),
        (status = 400, description = "Invalid data / account banned", body = ApiResponse<EmptyData>)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let mut conn = state.get_conn()?;

    let response_data = AuthService::login(&mut conn, payload)?;

    Ok(Json(ApiResponse::success(200, "Login successful", response_data)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = ApiResponse<AuthResponse>),
        (status = 400, description = "Invalid data", body = ApiResponse<EmptyData>),
        (status = 401, description = "Invalid or revoked token", body = ApiResponse<EmptyData>)
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RefreshRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let mut conn = state.get_conn()?;

    let response_data = AuthService::refresh(&mut conn, payload)?;

    Ok(Json(ApiResponse::success(200, "Token refreshed successfully", response_data)))
}

#[utoipa::path(
    post,
    path = "/auth/reset-password",
    tag = "Auth",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = ApiResponse<EmptyData>),
        (status = 400, description = "Invalid data", body = ApiResponse<EmptyData>)
    )
)]
pub async fn reset_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state.get_conn()?;

    AuthService::reset_password(&mut conn, payload)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "Password changed successfully, please log in again",
    )))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Auth",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse<EmptyData>),
        (status = 401, description = "Not logged in or token expired", body = ApiResponse<EmptyData>)
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state.get_conn()?;

    AuthService::logout(&mut conn, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(200, "Logged out from all devices successfully")))
}

#[utoipa::path(
    put,
    path = "/auth/change-password",
    tag = "Auth",
    request_body = ChangePasswordRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Password changed successfully", body = ApiResponse<EmptyData>),
        (status = 400, description = "Old password is incorrect", body = ApiResponse<EmptyData>),
        (status = 401, description = "Not logged in", body = ApiResponse<EmptyData>)
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state.get_conn()?;

    AuthService::change_password(&mut conn, claims.sub, payload)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "Password changed successfully, please log in again with your new password",
    )))
}
