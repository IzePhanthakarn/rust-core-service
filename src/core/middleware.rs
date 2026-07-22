use crate::core::{errors::AppError, jwt::verify_token};
use axum::{extract::Request, http::header, middleware::Next, response::Response};

pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    // 1. Get the Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Authorization header not found".to_string()))?;

    // 2. Check for the "Bearer " prefix
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid token format".to_string()));
    }

    let token = &auth_header[7..]; // Strip the "Bearer " prefix

    // 3. Decode the token
    let claims = verify_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    // 4. Check that it is an access token only (refresh tokens must not be used to call the API)
    if claims.token_type != "access" {
        return Err(AppError::Unauthorized("Please use an access token".to_string()));
    }

    // 5. Embed the claims into the request so the handler can use them later (e.g. to check role)
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
