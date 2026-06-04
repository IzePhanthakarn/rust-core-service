use axum::{
    extract::{Extension, State},
    Json,
};

use crate::{
    AppState, core::{
        errors::AppError,
        extractors::ValidatedJson,
        jwt::Claims,
        response::{ApiResponse, EmptyData},
    }, modules::roles::{
        dtos::{AssignRoleRequest, CreateRoleRequest, RevokeRoleRequest},
        models::Role,
        repositories::RoleRepository,
        services::RoleService,
    }
};

#[utoipa::path(
    get,
    path = "/roles",
    tag = "Roles", // เปลี่ยน Tag เป็น Roles
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
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());

    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ดูรายชื่อ Role (ต้องการสิทธิ์ super_admin หรือ admin_roles)".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // เรียกผ่าน RoleRepository
    let roles = RoleRepository::get_all_roles(&mut conn)
        .map_err(|_| AppError::InternalServerError("ไม่สามารถดึงข้อมูล Role ได้".to_string()))?;

    Ok(Json(ApiResponse::success(200, "ดึงข้อมูล Role สำเร็จ", roles)))
}

#[utoipa::path(
    post,
    path = "/roles/assign", // เปลี่ยน Path ให้คลีนขึ้น
    tag = "Roles",
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

    let target_role_name = RoleRepository::get_role_name_by_id(&mut conn, payload.role_id)
        .map_err(|_| AppError::BadRequest("ไม่พบ Role ID นี้ในระบบ".to_string()))?;

    let is_super_admin = claims.roles.contains(&"super_admin".to_string());
    let is_admin_roles = claims.roles.contains(&"admin_roles".to_string());

    match target_role_name.as_str() {
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

    // เรียกผ่าน RoleService
    RoleService::assign_role(
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
    post,
    path = "/roles/revoke", // เปลี่ยน Path ให้คลีนขึ้น
    tag = "Roles",
    request_body = RevokeRoleRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "ถอดสิทธิ์สำเร็จ", body = ApiResponse<EmptyData>),
        (status = 400, description = "ข้อมูลผิดพลาด", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>)
    )
)]
pub async fn revoke_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<RevokeRoleRequest>,
) -> Result<Json<ApiResponse<EmptyData>>, AppError> {
    let is_admin = claims.roles.contains(&"super_admin".to_string())
        || claims.roles.contains(&"admin_roles".to_string());
    if !is_admin {
        return Err(AppError::Forbidden(
            "คุณไม่มีสิทธิ์ถอดสิทธิ์ผู้ใช้งาน".to_string(),
        ));
    }

    if claims.sub == payload.target_user_id {
        return Err(AppError::BadRequest(
            "ไม่สามารถถอดสิทธิ์ของตัวเองได้".to_string(),
        ));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // เรียกผ่าน RoleService
    RoleService::revoke_role(&mut conn, payload.target_user_id, payload.role_id)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "ถอดสิทธิ์ผู้ใช้งานสำเร็จ",
    )))
}

#[utoipa::path(
    post,
    path = "/roles",
    tag = "Roles",
    request_body = CreateRoleRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "สร้าง Role สำเร็จ", body = ApiResponse<Role>),
        (status = 400, description = "ข้อมูลผิดพลาด", body = ApiResponse<EmptyData>),
        (status = 403, description = "สิทธิ์ไม่เพียงพอ", body = ApiResponse<EmptyData>),
        (status = 409, description = "มี Role นี้อยู่แล้ว", body = ApiResponse<EmptyData>)
    )
)]
pub async fn create_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateRoleRequest>,
) -> Result<Json<ApiResponse<Role>>, AppError> {
    
    // ล็อกสิทธิ์เฉพาะ super_admin
    let is_super_admin = claims.roles.contains(&"super_admin".to_string());
    if !is_super_admin {
        return Err(AppError::Forbidden("คุณไม่มีสิทธิ์สร้าง Role ใหม่ (ต้องการสิทธิ์ super_admin)".to_string()));
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalServerError("DB Error".to_string()))?;

    // แกะเอา payload.description ส่งเข้าไปด้วย
    let new_role = RoleService::create_role(&mut conn, &payload.name, payload.description)?;

    Ok(Json(ApiResponse::success(201, "สร้าง Role ใหม่สำเร็จ", new_role)))
}