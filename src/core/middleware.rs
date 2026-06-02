use axum::{extract::Request, http::header, middleware::Next, response::Response};
use crate::core::{errors::AppError, jwt::verify_token};

pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    // 1. ดึง Authorization Header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("ไม่พบ Authorization Header".to_string()))?;

    // 2. เช็คคำว่า "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("รูปแบบ Token ไม่ถูกต้อง".to_string()));
    }

    let token = &auth_header[7..]; // ตัดคำว่า "Bearer " ออก

    // 3. ถอดรหัส Token
    let claims = verify_token(token)
        .map_err(|_| AppError::Unauthorized("Token ไม่ถูกต้องหรือหมดอายุ".to_string()))?;

    // 4. เช็คว่าเป็น Access Token เท่านั้น (ห้ามเอา Refresh Token มายิง API)
    if claims.token_type != "access" {
        return Err(AppError::Unauthorized("กรุณาใช้ Access Token".to_string()));
    }

    // 5. ฝังข้อมูล Claims ลงใน Request เพื่อให้ Handler เอาไปใช้ต่อได้ (เช่น เช็ค Role)
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}