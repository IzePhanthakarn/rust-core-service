use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData, PaginatedData},
    },
    modules::{
        roles::repositories::RoleRepository,
        users::{
            dtos::{
                MeResponse, UpdateProfileRequest, UpdateUserStatusRequest, UserDetailResponse,
                UserFilterQuery,
            },
            models::User,
            repositories::UserRepository,
            services::UserService,
        },
    },
    AppState,
};

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    params(UserFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูลสำเร็จ", body = ApiResponse<PaginatedData<User>>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(filters): Query<UserFilterQuery>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<User>>>, AppError> {
    let allowed_roles = ["super_admin", "admin_roles"];
    let has_permission = claims
        .roles
        .iter()
        .any(|role| allowed_roles.contains(&role.as_str()));

    if !has_permission {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์เข้าถึงข้อมูลนี้".to_string()));
    }

    let page = filters.page.unwrap_or(1).max(1);
    let limit = filters.limit.unwrap_or(10).clamp(1, 100);

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

    let (items, total_items) = UserRepository::get_all_users(
        &mut conn,
        page,
        limit,
        filters.email,
        filters.status,
    )
    .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

    let data = PaginatedData {
        items,
        total_items,
        total_pages,
        current_page: page,
    };

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลสำเร็จ", data)))
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "Users",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "ดึงข้อมูลส่วนตัวสำเร็จ", body = ApiResponse<MeResponse>))
)]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, claims.sub)
        .map_err(|_| AppError::BadRequest("ไม่พบข้อมูลผู้ใช้งาน".to_string()))?;

    // เรียกข้ามไปใช้ RoleRepository ที่เราแยกไว้
    let roles = RoleRepository::get_user_roles(&mut conn, claims.sub).unwrap_or_default();

    let data = MeResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: profile.first_name,
        last_name: profile.last_name,
        roles,
    };

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลส่วนตัวสำเร็จ", data)))
}

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "Users",
    request_body = UpdateProfileRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "อัปเดตข้อมูลสำเร็จ", body = ApiResponse<MeResponse>),
        (status = 400, description = "ข้อมูลไม่ถูกต้อง / ไม่พบผู้ใช้", body = ApiResponse<EmptyData>)
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // ใช้ UserService ที่เราคลีนไว้แล้ว
    let updated_profile = UserService::update_profile(&mut conn, claims.sub, &payload)?;

    // ใช้ Repository ดึงข้อมูล User กลับมา (แทนการเขียน diesel filter ตรงๆ ใน Handler)
    let (user, _) = UserRepository::get_user_with_profile(&mut conn, claims.sub)
        .map_err(|_| AppError::BadRequest("ไม่พบบัญชีผู้ใช้งานของคุณในระบบ".to_string()))?;

    let roles = RoleRepository::get_user_roles(&mut conn, claims.sub).unwrap_or_default();

    let data = MeResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: updated_profile.first_name,
        last_name: updated_profile.last_name,
        roles,
    };

    Ok(Json(ApiResponse::success(
        200,
        "อัปเดตข้อมูลโปรไฟล์สำเร็จ",
        data,
    )))
}

#[utoipa::path(
    patch,
    path = "/users/{id}/status",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID ของผู้ใช้งานที่ต้องการเปลี่ยนสถานะ")
    ),
    request_body = UpdateUserStatusRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "เปลี่ยนสถานะผู้ใช้สำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ข้อมูลผิดพลาด", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn update_user_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateUserStatusRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์เปลี่ยนสถานะผู้ใช้งาน".to_string(),
        ));
    }

    if claims.sub == target_user_id {
        return Err(AppError::BadRequest(
            "คุณไม่สามารถเปลี่ยนสถานะบัญชีของตัวเองได้".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // เรียกผ่าน UserService
    UserService::update_user_status(&mut conn, target_user_id, &payload.status)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "เปลี่ยนสถานะผู้ใช้สำเร็จ",
    )))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID ของผู้ใช้งานที่ต้องการดูข้อมูล")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูลสำเร็จ", body = ApiResponse<UserDetailResponse>),
        (status = 400, description = "ไม่พบผู้ใช้งาน", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserDetailResponse>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ดูข้อมูลผู้ใช้อื่น".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, target_user_id)
        .map_err(|_| AppError::BadRequest("ไม่พบข้อมูลผู้ใช้งานนี้ในระบบ".to_string()))?;

    let roles = RoleRepository::get_user_roles(&mut conn, target_user_id).unwrap_or_default();

    let data = UserDetailResponse {
        id: user.id,
        email: user.email.unwrap_or_default(),
        first_name: profile.first_name,
        last_name: profile.last_name,
        status: user.status,
        roles,
    };

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลผู้ใช้งานสำเร็จ", data)))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID ของผู้ใช้งานที่ต้องการลบ")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ลบบัญชีผู้ใช้งานสำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ไม่พบผู้ใช้ / พยายามลบตัวเอง", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn delete_user_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let is_super_admin = claims.roles.contains(&"super_admin".to_string());
    if !is_super_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ลบบัญชีผู้ใช้งาน".to_string(),
        ));
    }

    if claims.sub == target_user_id {
        return Err(AppError::BadRequest(
            "คุณไม่สามารถลบบัญชีของตัวเองได้".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // เรียกผ่าน UserService
    UserService::delete_user(&mut conn, target_user_id)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "ลบบัญชีผู้ใช้งานออกจากระบบสำเร็จ",
    )))
}