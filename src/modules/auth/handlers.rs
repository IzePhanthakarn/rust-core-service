use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    core::{errors::AppError, response::{ApiResponse, EmptyData}}, 
    modules::auth::{dtos::{RegisterRequest, RegisterResponse}, services::AuthService}, // นำเข้า RegisterResponse
    AppState,
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
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<RegisterResponse>>, AppError> { // เปลี่ยน EmptyData เป็น RegisterResponse
    
    if let Err(errors) = payload.validate() {
        return Err(AppError::BadRequest(errors.to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("ไม่สามารถเชื่อมต่อฐานข้อมูลได้".to_string()))?;


    AuthService::register(&mut conn, payload)?;

    // ส่ง data กลับไปหา Client
    Ok(Json(ApiResponse::success_without_data(201, "สมัครสมาชิกสำเร็จ")))
}