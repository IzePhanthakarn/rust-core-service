use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData, PaginatedData, PaginationQuery},
    },
    modules::users::{
        models::{AssignRoleRequest, MeResponse, UpdateProfileRequest, User},
        repositories::UserRepository,
    },
    schema::users,
};
use axum::{
    Json,
    extract::{Extension, Query, State},
};
use diesel::prelude::*;

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    params(PaginationQuery),
    security(("bearerAuth" = [])), // บอก Scalar ว่าเส้นนี้ต้องใช้ Token
    responses(
        (status = 200, description = "ดึงข้อมูลสำเร็จ", body = ApiResponse<PaginatedData<User>>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>, // << ดึงข้อมูลคนที่ยิง API มาจาก Middleware
) -> Result<Json<ApiResponse<PaginatedData<User>>>, AppError> {
    // 1. เช็คสิทธิ์ (RBAC): อนุญาตแค่ super_admin และ admin_roles
    let allowed_roles = ["super_admin", "admin_roles"];
    let has_permission = claims
        .roles
        .iter()
        .any(|role| allowed_roles.contains(&role.as_str()));

    if !has_permission {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ใช้งาน (ต้องการสิทธิ์ super_admin หรือ admin_roles)".to_string(),
        ));
    }

    // 2. จัดการค่า Page & Limit
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(10).clamp(1, 100); // ห้ามดึงเกิน 100

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

    // 3. ดึงข้อมูลผ่าน Repository
    let (items, total_items) = UserRepository::get_all_paginated(&mut conn, page, limit)
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
    post,
    path = "/users/assign-role",
    tag = "Users",
    request_body = AssignRoleRequest,
    security(("bearerAuth" = [])),
    responses((status = 200, description = "กำหนดสิทธิ์สำเร็จ", body = ApiResponse<EmptyData>))
)]
pub async fn assign_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<AssignRoleRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 1. ดึงชื่อ Role จาก ID ที่ Frontend ส่งมา
    let target_role_name = UserRepository::get_role_name_by_id(&mut conn, payload.role_id)
        .map_err(|_| AppError::BadRequest("ไม่พบ Role ID นี้ในระบบ".to_string()))?;

    // 2. เช็ค Business Logic (ใช้โค้ดเดิมได้เลย แค่เปลี่ยนตัวแปรเปรียบเทียบ)
    let is_super_admin = claims.roles.contains(&"super_admin".to_string());
    let is_admin_roles = claims.roles.contains(&"admin_roles".to_string());

    match target_role_name.as_str() {
        // << เอา target_role_name มาเช็ค
        "admin_roles" => {
            if !is_super_admin {
                return Err(AppError::Forbidden(
                    "มีเพียง Super Admin เท่านั้นที่มอบสิทธิ์นี้ได้".to_string(),
                ));
            }
        }
        "admin_shop" => {
            if !is_super_admin && !is_admin_roles {
                return Err(AppError::Forbidden("คุณไม่มีสิทธิ์กำหนด Shop Admin".to_string()));
            }
        }
        _ => {
            if !is_super_admin && !is_admin_roles {
                return Err(AppError::Forbidden("คุณไม่มีสิทธิ์กำหนดสิทธิ์นี้".to_string()));
            }
        }
    }

    // 3. สั่ง Assign (ส่ง role_id ตรงๆ เข้าไปได้เลย)
    UserRepository::assign_role(
        &mut conn,
        payload.target_user_id,
        payload.role_id,
        claims.sub,
    )?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "กำหนดสิทธิ์สำเร็จ",
    )))
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

    // 1. ดึงข้อมูล User และ Profile จาก DB
    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, claims.sub)
        .map_err(|_| AppError::BadRequest("ไม่พบข้อมูลผู้ใช้งาน".to_string()))?;

    // 2. ดึง Roles (ใช้ฟังก์ชันเดิมที่มีอยู่แล้ว)
    let roles = UserRepository::get_user_roles(&mut conn, claims.sub).unwrap_or_default(); // ถ้าหาไม่เจอ ให้เป็น Array ว่าง

    // 3. ประกอบร่างส่งกลับ
    let data = MeResponse {
        id: user.id,
        // user.email เป็น Option<String> จึงต้อง unwrap_or_default()
        email: user.email.unwrap_or_default(),
        first_name: profile.first_name,
        last_name: profile.last_name,
        roles,
    };

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูลส่วนตัวสำเร็จ", data)))
}

#[utoipa::path(
    put, // ใช้ PUT สำหรับการอัปเดตข้อมูล
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

    // 1. อัปเดตโปรไฟล์
    // ถ้าพัง (เช่น หา Profile ไม่เจอ) มันจะพ่น Error จาก Repository กลับไปหา User ทันที
    let updated_profile = UserRepository::update_profile(&mut conn, claims.sub, &payload)?;

    // 2. ดึงข้อมูล User (เพื่อเอา Email มาตอบกลับ)
    // ตรงนี้เราแยก Error ชัดเจนเลยว่า User หายไปจากระบบแล้ว
    let user = users::table
        .filter(users::id.eq(claims.sub))
        .first::<User>(&mut conn)
        .map_err(|_| AppError::BadRequest("ไม่พบบัญชีผู้ใช้งานของคุณในระบบ".to_string()))?;

    // 3. ดึง Roles
    let roles = UserRepository::get_user_roles(&mut conn, claims.sub).unwrap_or_default();

    // 4. ประกอบร่างข้อมูลอัปเดตใหม่ส่งกลับไป
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
