use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::core::response::{ApiResponse, EmptyData};

pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
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