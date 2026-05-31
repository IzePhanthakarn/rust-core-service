use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::core::response::{ApiResponse, EmptyData};

// กำหนดประเภทของ Error ที่อาจเกิดขึ้นในระบบ
pub enum AppError {
    BadRequest(String),
    Conflict(String),
    InternalServerError(String),
}

// แปลง AppError ให้กลายเป็น HTTP Response ของ Axum โดยอัตโนมัติ
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        // ใช้ ApiResponse::error ห่อข้อความอีกที
        let body = Json(ApiResponse::<EmptyData>::error(status.as_u16(), &message));
        
        (status, body).into_response()
    }
}