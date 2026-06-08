use crate::core::response::{ApiResponse, EmptyData};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        // ==== ปรับ Logic การเลือก Status ====
        // .is_client_error() จะเป็น true ถ้าเป็นเลข 400-499
        let body = if status.is_client_error() {
            Json(ApiResponse::<EmptyData>::fail(status.as_u16(), &message))
        } else {
            Json(ApiResponse::<EmptyData>::error(status.as_u16(), &message))
        };
        // ===================================

        (status, body).into_response()
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            // ถ้าเป็น Not Found ให้แปลงเป็น AppError::NotFound
            diesel::result::Error::NotFound => {
                AppError::NotFound("ข้อมูลที่ต้องการไม่พบในระบบ".to_string())
            }
            // ถ้าเป็น Unique Violation (เช่น อีเมลซ้ำ) ให้แปลงเป็น Conflict
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => AppError::Conflict("ข้อมูลนี้มีอยู่แล้วในระบบ".to_string()),
            // อื่นๆ ให้ถือว่าเป็น Internal Server Error
            _ => {
                // แนะนำ: ทำการ log ตัว err จริงๆ ไว้ด้วย เพื่อให้ Debug ง่ายขึ้น
                eprintln!("Database Error: {:?}", err);
                AppError::InternalServerError("เกิดข้อผิดพลาดจากฐานข้อมูล".to_string())
            }
        }
    }
}
