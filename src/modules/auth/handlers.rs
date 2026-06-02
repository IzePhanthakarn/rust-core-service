use axum::{Json, extract::{State, Extension}, http::StatusCode};

use crate::{
    AppState,
    core::{
        errors::AppError, extractors::ValidatedJson, jwt::Claims, response::{ApiResponse, EmptyData}
    },
    modules::auth::{
        dtos::{
            AuthResponse, ChangePasswordRequest, LoginRequest, RefreshRequest, RegisterRequest, RegisterResponse, ResetPasswordRequest
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
        // เปลี่ยน body คืนค่าเป็น RegisterResponse
        (status = 201, description = "สมัครสมาชิกสำเร็จ", body = ApiResponse<RegisterResponse>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง (Validation Error)", body = ApiResponse<EmptyData>),
        (status = 409, description = "อีเมลนี้ถูกใช้งานแล้ว", body = ApiResponse<EmptyData>)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    // เปลี่ยนจาก Json เป็น ValidatedJson
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<EmptyData>>), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;

    AuthService::register(&mut conn, payload)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success_without_data(201, "สมัครสมาชิกสำเร็จ")),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "เข้าสู่ระบบสำเร็จ", body = ApiResponse<AuthResponse>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง / โดนแบน", body = ApiResponse<EmptyData>)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;

    let response_data = AuthService::login(&mut conn, payload)?;

    Ok(Json(ApiResponse::success(
        200,
        "เข้าสู่ระบบสำเร็จ",
        response_data,
    )))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "ต่ออายุ Token สำเร็จ", body = ApiResponse<AuthResponse>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง", body = ApiResponse<EmptyData>),
        (status = 401, description = "Token ไม่ถูกต้องหรือถูกยกเลิก", body = ApiResponse<EmptyData>)
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RefreshRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let response_data = AuthService::refresh(&mut conn, payload)?;

    Ok(Json(ApiResponse::success(
        200,
        "ต่ออายุ Token สำเร็จ",
        response_data,
    )))
}

#[utoipa::path(
    post,
    path = "/auth/reset-password",
    tag = "Auth",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "เปลี่ยนรหัสผ่านสำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง", body = ApiResponse<EmptyData>)
    )
)]
pub async fn reset_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    AuthService::reset_password(&mut conn, payload)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "เปลี่ยนรหัสผ่านสำเร็จ กรุณาเข้าสู่ระบบใหม่",
    )))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Auth",
    security(("bearerAuth" = [])), // บังคับว่าต้องมี Token
    responses(
        (status = 200, description = "ออกจากระบบสำเร็จ", body = ApiResponse<EmptyData>),
        (status = 401, description = "ยังไม่ได้เข้าสู่ระบบ หรือ Token หมดอายุ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>, // ดึงข้อมูลคนล็อกอินจาก Middleware
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // สั่ง Logout โดยใช้ ID จาก Token
    AuthService::logout(&mut conn, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "ออกจากระบบทุกอุปกรณ์สำเร็จ",
    )))
}

#[utoipa::path(
    put, // ใช้ PUT เพราะเป็นการอัปเดตข้อมูล
    path = "/auth/change-password",
    tag = "Auth",
    request_body = ChangePasswordRequest,
    security(("bearerAuth" = [])), // บังคับว่าต้องมี Token
    responses(
        (status = 200, description = "เปลี่ยนรหัสผ่านสำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "รหัสผ่านเดิมไม่ถูกต้อง", body = ApiResponse<EmptyData>),
        (status = 401, description = "ยังไม่ได้เข้าสู่ระบบ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    
    let mut conn = state.db_pool.get().map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // ส่ง User ID (claims.sub) เข้าไปพร้อมกับ Payload
    AuthService::change_password(&mut conn, claims.sub, payload)?;

    Ok(Json(ApiResponse::success_without_data(200, "เปลี่ยนรหัสผ่านสำเร็จ กรุณาเข้าสู่ระบบใหม่ด้วยรหัสผ่านใหม่")))
}