use axum::{Extension, Json, extract::State, http::StatusCode};

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::properties::{
        dtos::CreatePropertyTypeRequest, models::PropertyType, services::PropertyService,
    },
};

#[utoipa::path(
    post,
    path = "/properties",
    tag = "Properties",
    request_body = CreatePropertyTypeRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Property type created successfully", body = PropertyType),
        (status = 409, description = "Property type already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_property_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreatePropertyTypeRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PropertyType>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let new_property = PropertyService::create_property_type(
        &mut conn,
        &payload.name,
        &payload.code,
        payload.description,
        claims.sub,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            201,
            "สร้าง Property Type สำเร็จ",
            new_property,
        )),
    ))
}
