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

        let body = if status.is_client_error() {
            Json(ApiResponse::<EmptyData>::fail(status.as_u16(), &message))
        } else {
            Json(ApiResponse::<EmptyData>::error(status.as_u16(), &message))
        };

        (status, body).into_response()
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                AppError::NotFound("ข้อมูลที่ต้องการไม่พบในระบบ".to_string())
            }
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => AppError::Conflict("ข้อมูลนี้มีอยู่แล้วในระบบ".to_string()),
            _ => {
                eprintln!("Database Error: {:?}", err);
                AppError::InternalServerError("เกิดข้อผิดพลาดจากฐานข้อมูล".to_string())
            }
        }
    }
}

pub fn map_diesel_error(
    not_found_msg: impl Into<String>,
    internal_msg: impl Into<String>,
) -> impl Fn(diesel::result::Error) -> AppError {
    let not_found = not_found_msg.into();
    let internal = internal_msg.into();
    move |e| match e {
        diesel::result::Error::NotFound => AppError::NotFound(not_found.clone()),
        _ => {
            eprintln!("Database Error: {:?}", e);
            AppError::InternalServerError(internal.clone())
        }
    }
}
