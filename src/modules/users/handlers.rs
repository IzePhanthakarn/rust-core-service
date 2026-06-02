use crate::{
    AppState,
    core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData, PaginatedData, PaginationQuery},
    },
    modules::users::{
        models::{
            AssignRoleRequest, MeResponse, RevokeRoleRequest, Role, UpdateProfileRequest, UpdateUserStatusRequest, User, UserDetailResponse, UserFilterQuery
        },
        repositories::UserRepository,
    },
    schema::users,
};
use axum::{
    Json,
    extract::{Extension, Path, Query, State},
};
use diesel::prelude::*;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    params(UserFilterQuery), // << เปลี่ยนตรงนี้ให้ใช้ Schema ใหม่ เพื่อให้โผล่ใน Swagger
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูลสำเร็จ", body = ApiResponse<PaginatedData<User>>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(filters): Query<UserFilterQuery>, // << รับค่าเป็น UserFilterQuery
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PaginatedData<User>>>, AppError> {
    // 1. เช็คสิทธิ์ (RBAC) อนุญาตแค่ super_admin และ iam_admin
    let allowed_roles = ["super_admin", "iam_admin"];
    let has_permission = claims
        .roles
        .iter()
        .any(|role| allowed_roles.contains(&role.as_str()));

    if !has_permission {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์เข้าถึงข้อมูลนี้".to_string()));
    }

    // 2. จัดการค่า Page & Limit
    let page = filters.page.unwrap_or(1).max(1);
    let limit = filters.limit.unwrap_or(10).clamp(1, 100);

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

    // 3. ดึงข้อมูลผ่าน Repository (ส่ง email และ status เข้าไปด้วย)
    let (items, total_items) = UserRepository::get_all_paginated(
        &mut conn,
        page,
        limit,
        filters.email,  // ส่ง Option<String>
        filters.status, // ส่ง Option<UserStatus>
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

#[utoipa::path(
    get,
    path = "/users/roles",
    tag = "Users",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ดึงข้อมูล Role สำเร็จ", body = ApiResponse<Vec<Role>>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn get_roles(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Role>>>, AppError> {
    // 1. เช็คสิทธิ์ (อนุญาตเฉพาะ super_admin และ iam_admin)
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"iam_admin".to_string());

    if !is_admin {
        // แยกระบุ Error ให้ชัดเจนไปเลย
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ดูรายชื่อ Role (ต้องการสิทธิ์ super_admin หรือ iam_admin)".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 2. ดึงข้อมูล
    let roles = UserRepository::get_all_roles(&mut conn)
        .map_err(|_| AppError::InternalServerError("ไม่สามารถดึงข้อมูล Role ได้".to_string()))?;

    // 3. ส่งข้อมูลกลับ
    Ok(Json(ApiResponse::success(200, "ดึงข้อมูล Role สำเร็จ", roles)))
}

#[utoipa::path(
    patch, // ใช้ PATCH เพราะเราอัปเดตแค่ฟิลด์เดียว (status) ไม่ได้อัปเดตทั้งก้อน
    path = "/users/{id}/status",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "ID ของผู้ใช้งานที่ต้องการเปลี่ยนสถานะ")
    ),
    request_body = UpdateUserStatusRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "เปลี่ยนสถานะผู้ใช้สำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ข้อมูลผิดพลาด (เช่น ไม่พบผู้ใช้ หรือ พยายามแบนตัวเอง)", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn update_user_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(target_user_id): Path<Uuid>, // ดึง ID จาก URL
    ValidatedJson(payload): ValidatedJson<UpdateUserStatusRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    // 1. เช็คสิทธิ์ (อนุญาตเฉพาะ super_admin และ iam_admin)
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"iam_admin".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์เปลี่ยนสถานะผู้ใช้งาน (ต้องการสิทธิ์ super_admin หรือ iam_admin)".to_string(),
        ));
    }

    // 2. Business Logic: ป้องกันไม่ให้แอดมินเผลอแบน/ระงับบัญชีของตัวเอง
    if claims.sub == target_user_id {
        return Err(AppError::BadRequest(
            "คุณไม่สามารถเปลี่ยนสถานะบัญชีของตัวเองได้".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 3. สั่งเปลี่ยนสถานะ
    UserRepository::update_user_status(&mut conn, target_user_id, &payload.status)?;

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
    // 1. เช็คสิทธิ์ (อนุญาตเฉพาะ super_admin และ iam_admin)
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"iam_admin".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ดูข้อมูลผู้ใช้อื่น (ต้องการสิทธิ์ super_admin หรือ iam_admin)".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 2. ดึงข้อมูล User และ Profile (ใช้ฟังก์ชันเดิมเลย)
    // แยก Error ชัดเจนว่าหาไม่เจอ
    let (user, profile) = UserRepository::get_user_with_profile(&mut conn, target_user_id)
        .map_err(|_| AppError::BadRequest("ไม่พบข้อมูลผู้ใช้งานนี้ในระบบ".to_string()))?;

    // 3. ดึง Roles (ใช้ฟังก์ชันเดิม)
    let roles = UserRepository::get_user_roles(&mut conn, target_user_id).unwrap_or_default();

    // 4. ประกอบร่างส่งกลับ
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
    delete, // ใช้ Method DELETE
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
    // 1. เช็คสิทธิ์ (การลบข้อมูลควรให้เฉพาะระดับสูงสุดอย่าง super_admin เท่านั้น)
    let is_super_admin = claims.roles.contains(&"super_admin".to_string());
    if !is_super_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ลบบัญชีผู้ใช้งาน (ต้องการสิทธิ์ super_admin)".to_string(),
        ));
    }

    // 2. Business Logic: ป้องกันไม่ให้แอดมินลบบัญชีของตัวเอง (สำคัญมาก!)
    if claims.sub == target_user_id {
        return Err(AppError::BadRequest(
            "คุณไม่สามารถลบบัญชีของตัวเองได้".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 3. สั่งลบ
    UserRepository::delete_user(&mut conn, target_user_id)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "ลบบัญชีผู้ใช้งานออกจากระบบสำเร็จ",
    )))
}

#[utoipa::path(
    post,
    path = "/users/revoke-role",
    tag = "Users",
    request_body = RevokeRoleRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ถอดสิทธิ์สำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ข้อมูลผิดพลาด (เช่น พยายามถอดสิทธิ์ตัวเอง)", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn revoke_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<RevokeRoleRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {

    // 1. เช็คสิทธิ์ (อนุญาตเฉพาะ super_admin หรือ iam_admin)
    let is_admin = claims.roles.contains(&"super_admin".to_string()) || claims.roles.contains(&"iam_admin".to_string());
    if !is_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์ถอดสิทธิ์ผู้ใช้งาน (ต้องการสิทธิ์ super_admin หรือ iam_admin)".to_string()));
    }

    // 2. ป้องกันแอดมินถอดสิทธิ์ตัวเอง (กันเหนียวไว้ก่อน)
    if claims.sub == payload.target_user_id {
        return Err(AppError::BadRequest("ไม่สามารถถอดสิทธิ์ของตัวเองได้ เพื่อป้องกันการไม่สามารถเข้าถึงระบบ".to_string()));
    }

    let mut conn = state.db_pool.get().map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // 3. สั่งถอดสิทธิ์
    UserRepository::revoke_role(&mut conn, payload.target_user_id, payload.role_id)?;

    Ok(Json(ApiResponse::success_without_data(200, "ถอดสิทธิ์ผู้ใช้งานสำเร็จ")))
}