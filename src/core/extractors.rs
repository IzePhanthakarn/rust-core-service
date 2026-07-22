use crate::core::errors::AppError;
use axum::extract::{FromRequest, Request, rejection::JsonRejection};
use serde::de::DeserializeOwned;
use validator::Validate;

// Struct that wraps JSON
pub struct ValidatedJson<T>(pub T);

// Tell Axum to run this logic every time JSON is received
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Catch errors while converting JSON (e.g. an empty string sent for a UUID)
        let axum::Json(value) =
            axum::Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| {
                    // Convert Axum's JsonRejection into our AppError
                    AppError::BadRequest(format!("Invalid data or data type mismatch: {}", rejection))
                })?;

        // 2. Run the validator automatically (catches short passwords, malformed emails, etc.)
        value
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        Ok(ValidatedJson(value))
    }
}
