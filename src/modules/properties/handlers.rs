use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::ApiResponse},
    modules::properties::{
        dtos::{CreatePropertyOptionRequest, CreatePropertyTypeRequest, PropertyResponse},
        models::{PropertyOption, PropertyType},
        services::PropertyService,
    },
};

#[utoipa::path(
    get,
    path = "/properties/{property_type_id}",
    tag = "Properties",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูล Property Type สำเร็จ", body = ApiResponse<PropertyResponse>),
        (status = 404, description = "ไม่พบ Property Type ที่ระบุ"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_one_property_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(property_type_id): Path<Uuid>,
) -> Result<Json<ApiResponse<PropertyResponse>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let property_data = PropertyService::get_one_property_type(&mut conn, property_type_id)?;

    Ok(Json(ApiResponse::success(
        200,
        "ดึงข้อมูล Property Type สำเร็จ",
        property_data,
    )))
}

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

#[utoipa::path(
    post,
    path = "/properties/options",
    tag = "Properties",
    request_body = CreatePropertyOptionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Property option created successfully", body = PropertyOption),
        (status = 409, description = "Property option already exists"),
        (status = 500, description = "Internal server error")
    )
)]

pub async fn create_property_option(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreatePropertyOptionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PropertyOption>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์สร้าง Property Option".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let new_option = PropertyService::create_property_option(
        &mut conn,
        payload.property_type_id,
        payload.label,
        payload.value,
        claims.sub,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            201,
            "สร้าง Property Option สำเร็จ",
            new_option,
        )),
    ))
}
