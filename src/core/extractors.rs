use crate::core::errors::AppError;
use axum::extract::{FromRequest, Request, rejection::JsonRejection};
use serde::de::DeserializeOwned;
use validator::Validate;

// สร้าง Struct หุ้ม JSON เอาไว้
pub struct ValidatedJson<T>(pub T);

// สั่งให้ Axum รัน Logic นี้ทุกครั้งที่มีการรับ JSON เข้ามา
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. ดักจับ Error ตอนแปลง JSON (เช่น ส่ง String ว่างมาใส่ UUID)
        let axum::Json(value) =
            axum::Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| {
                    // แปลง JsonRejection ของ Axum ให้กลายเป็น AppError ของเรา
                    AppError::BadRequest(format!("ข้อมูลไม่ถูกต้อง หรือชนิดข้อมูลไม่ตรงกัน: {}", rejection))
                })?;

        // 2. รัน Validator อัตโนมัติ (ดักจับรหัสผ่านสั้น, อีเมลผิดรูปแบบ ฯลฯ)
        value
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        Ok(ValidatedJson(value))
    }
}
