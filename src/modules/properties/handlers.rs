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
            PropertyOptionData, PropertyResponse, PropertyTypeData, UpdatePropertyOptionRequest,
            UpdatePropertyTypeRequest, UpdateStatusRequest,
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
        (status = 200, description = "Property Type retrieved successfully", body = ApiResponse<PaginatedData<PropertyTypeData>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_property_type(
    State(state): State<AppState>,
    Query(filters): Query<PropertyFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<PropertyTypeData>>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to view Property data".to_string()));
    }

    let mut conn = state.get_conn()?;
    let result = PropertyService::get_property_type(&mut conn, filters)?;

    Ok(Json(ApiResponse::success(200, "Data retrieved successfully", result)))
}

#[utoipa::path(
    get,
    path = "/properties/{property_type_id}",
    tag = "Properties",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property Type retrieved successfully", body = ApiResponse<PropertyResponse>),
        (status = 404, description = "Specified Property Type not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_one_property_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(property_type_id): Path<Uuid>,
) -> Result<Json<ApiResponse<PropertyResponse>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to view Property data".to_string()));
    }

    let mut conn = state.get_conn()?;
    let property_data = PropertyService::get_one_property_type(&mut conn, property_type_id)?;

    Ok(Json(ApiResponse::success(200, "Property Type retrieved successfully", property_data)))
}

#[utoipa::path(
    get,
    path = "/properties/code/{code}",
    tag = "Properties",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property Type retrieved successfully", body = ApiResponse<PropertyResponse>),
        (status = 404, description = "Specified Property Type not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_property_type_by_code(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(code): Path<String>,
) -> Result<Json<ApiResponse<PropertyResponse>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to view Property data".to_string()));
    }

    let mut conn = state.get_conn()?;
    let property_data = PropertyService::get_one_property_type_by_code(&mut conn, &code)?;

    Ok(Json(ApiResponse::success(200, "Property Type retrieved successfully", property_data)))
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
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to create Property".to_string()));
    }

    let mut conn = state.get_conn()?;
    let new_property = PropertyService::create_property_type(
        &mut conn,
        &payload.name,
        &payload.code,
        payload.description,
        claims.sub,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Property Type created successfully", new_property)),
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
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to edit Property".to_string()));
    }

    let mut conn = state.get_conn()?;
    let updated_property = PropertyService::update_property_type(
        &mut conn,
        payload.id,
        &payload.name,
        &payload.code,
        payload.description,
        claims.sub,
    )?;

    Ok(Json(ApiResponse::success(200, "Property Type updated successfully", updated_property)))
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
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to delete Property".to_string()));
    }

    let mut conn = state.get_conn()?;
    PropertyService::delete_property_type(&mut conn, property_type_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "Property Type deleted successfully"))))
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
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to create Property Option".to_string()));
    }

    let mut conn = state.get_conn()?;
    let new_option = PropertyService::create_property_option(
        &mut conn,
        payload.property_type_id,
        payload.label,
        payload.value,
        claims.sub,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Property Option created successfully", new_option)),
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
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PropertyOptionData>>), AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "You do not have permission to change Property Option status".to_string(),
        ));
    }

    let mut conn = state.get_conn()?;
    let result =
        PropertyService::update_property_is_active(&mut conn, property_option_id, payload.is_active)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(200, "Property Option status updated successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/properties/options",
    tag = "Properties",
    request_body = UpdatePropertyOptionRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Property option updated successfully", body = ApiResponse<PropertyOptionData>),
        (status = 404, description = "Property option not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_property_option(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<UpdatePropertyOptionRequest>,
) -> Result<Json<ApiResponse<PropertyOptionData>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to edit Property Option".to_string()));
    }

    let mut conn = state.get_conn()?;
    let updated_option = PropertyService::update_property_option(
        &mut conn,
        payload.id,
        &payload.label,
        &payload.value,
        payload.sort_order,
        payload.is_active,
    )?;

    Ok(Json(ApiResponse::success(200, "Property Option updated successfully", updated_option)))
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
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to delete Property Option".to_string()));
    }

    let mut conn = state.get_conn()?;
    PropertyService::delete_property_option(&mut conn, property_option_id)?;

    Ok((StatusCode::OK, Json(ApiResponse::success_without_data(200, "Property Option deleted successfully"))))
}
