use axum::{
    Json,
    extract::{Extension, Path, Query, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData, PaginatedData},
    },
    modules::users::{
        dtos::{
            MeResponse, UpdateProfileRequest, UpdateUserStatusRequest, UserDetailResponse,
            UserFilterQuery,
        },
        models::User,
        repositories::UserRepository,
        services::UserService,
    },
};

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    params(UserFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<PaginatedData<User>>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(filters): Query<UserFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<User>>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to access this data".to_string()));
    }

    let mut conn = state.get_conn()?;
    let data = UserService::get_all_users(&mut conn, filters)?;

    Ok(Json(ApiResponse::success(200, "Data retrieved successfully", data)))
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "Users",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Personal data retrieved successfully", body = ApiResponse<MeResponse>))
)]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let mut conn = state.get_conn()?;

    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, claims.sub)
        .map_err(|_| AppError::BadRequest("User data not found".to_string()))?;

    let data = MeResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: profile.first_name,
        last_name: profile.last_name,
        role: user.role,
    };

    Ok(Json(ApiResponse::success(200, "Personal data retrieved successfully", data)))
}

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "Users",
    request_body = UpdateProfileRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Data updated successfully", body = ApiResponse<MeResponse>),
        (status = 400, description = "Invalid data / user not found", body = ApiResponse<EmptyData>)
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let mut conn = state.get_conn()?;

    let updated_profile = UserService::update_profile(&mut conn, claims.sub, &payload)?;

    let (user, _) = UserRepository::get_user_with_profile(&mut conn, claims.sub)
        .map_err(|_| AppError::BadRequest("Your user account was not found in the system".to_string()))?;

    let data = MeResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: updated_profile.first_name,
        last_name: updated_profile.last_name,
        role: user.role,
    };

    Ok(Json(ApiResponse::success(200, "Profile updated successfully", data)))
}

#[utoipa::path(
    patch,
    path = "/users/{id}/status",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID of the user whose status should be changed")
    ),
    request_body = UpdateUserStatusRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "User status changed successfully", body = ApiResponse<EmptyData>),
        (status = 400, description = "Invalid data", body = ApiResponse<EmptyData>),
        (status = 403, description = "Insufficient permissions", body = ApiResponse<EmptyData>)
    )
)]
pub async fn update_user_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateUserStatusRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to change user status".to_string()));
    }

    if claims.sub == target_user_id {
        return Err(AppError::BadRequest(
            "You cannot change the status of your own account".to_string(),
        ));
    }

    let mut conn = state.get_conn()?;
    UserService::update_user_status(&mut conn, target_user_id, &payload.status)?;

    Ok(Json(ApiResponse::success_without_data(200, "User status changed successfully")))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID of the user whose data to view")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<UserDetailResponse>),
        (status = 400, description = "User not found", body = ApiResponse<EmptyData>),
        (status = 403, description = "Insufficient permissions", body = ApiResponse<EmptyData>)
    )
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserDetailResponse>>, AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("You do not have permission to view other users' data".to_string()));
    }

    let mut conn = state.get_conn()?;

    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, target_user_id)
        .map_err(|_| AppError::BadRequest("This user's data was not found in the system".to_string()))?;

    let data = UserDetailResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: profile.first_name,
        last_name: profile.last_name,
        status: user.status,
        role: user.role,
    };

    Ok(Json(ApiResponse::success(200, "User data retrieved successfully", data)))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID of the user to delete")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "User account deleted successfully", body = ApiResponse<EmptyData>),
        (status = 400, description = "User not found / attempted to delete self", body = ApiResponse<EmptyData>),
        (status = 403, description = "Insufficient permissions", body = ApiResponse<EmptyData>)
    )
)]
pub async fn delete_user_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    if !claims.is_super_admin() {
        return Err(AppError::Forbidden("You do not have permission to delete user accounts".to_string()));
    }

    if claims.sub == target_user_id {
        return Err(AppError::BadRequest("You cannot delete your own account".to_string()));
    }

    let mut conn = state.get_conn()?;
    UserService::delete_user(&mut conn, target_user_id)?;

    Ok(Json(ApiResponse::success_without_data(200, "User account deleted from the system successfully")))
}
