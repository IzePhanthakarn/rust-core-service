use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, PaginatedData},
    },
    modules::properties::{
        dtos::{
            CreatePropertyOptionRequest, CreatePropertyTypeRequest, PropertyFilterQuery,
            PropertyOptionData, PropertyResponse, PropertyTypeData, UpdatePropertyTypeRequest,
            UpdateStatusRequest,
        },
        models::{PropertyOption, PropertyType},
        services::PropertyService,
    },
};

#[utoipa::path(
    get,
    path = "/properties",
    tag = "Properties",
    params(PropertyFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูล Property Type สำเร็จ", body = ApiResponse<PaginatedData<PropertyTypeData>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_property_type(
    State(state): State<AppState>,
    Query(filters): Query<PropertyFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<PropertyTypeData>>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let result = PropertyService::get_property_type(&mut conn, filters)
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", result)))
}

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
    put,
    path = "/properties",
    tag = "Properties",
    request_body = UpdatePropertyTypeRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property type updated successfully", body = PropertyType),
        (status = 404, description = "Property type not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_property_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<UpdatePropertyTypeRequest>,
) -> Result<Json<ApiResponse<PropertyType>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์แก้ไข Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let updated_property = PropertyService::update_property_type(
        &mut conn,
        payload.id,
        &payload.name,
        &payload.code,
        payload.description,
        claims.sub,
    )?;

    Ok(Json(ApiResponse::success(
        200,
        "แก้ไข Property Type สำเร็จ",
        updated_property,
    )))
}

#[utoipa::path(
    delete,
    path = "/properties/{property_type_id}",
    tag = "Properties",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property type deleted successfully"),
        (status = 404, description = "Property type not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_property_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(property_type_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์ลบ Property".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    PropertyService::delete_property_type(&mut conn, property_type_id)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success_without_data(
            200,
            "ลบ Property Type สำเร็จ",
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

#[utoipa::path(
    patch,
    path = "/properties/options/{property_option_id}/status",
    tag = "Properties",
    request_body = UpdateStatusRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property option status updated successfully", body = PropertyOption),
        (status = 404, description = "Property option not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_property_option_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(property_option_id): Path<Uuid>,
    Json(payload): Json<UpdateStatusRequest>, // รับค่า true/false มาจากตรงนี้
) -> Result<(StatusCode, Json<ApiResponse<PropertyOptionData>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์เปลี่ยนสถานะ Property Option".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let result = PropertyService::update_property_is_active(
        &mut conn,
        property_option_id,
        payload.is_active,
    )?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            200,
            "แก้ไขสถานะ Property Option สําเร็จ",
            result,
        )),
    ))
}

#[utoipa::path(
    delete,
    path = "/properties/options/{property_option_id}",
    tag = "Properties",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property option deleted successfully"),
        (status = 404, description = "Property option not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_property_option(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(property_option_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ลบ Property Option".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    PropertyService::delete_property_option(&mut conn, property_option_id)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success_without_data(
            200,
            "ลบ Property Option สำเร็จ",
        )),
    ))
}
