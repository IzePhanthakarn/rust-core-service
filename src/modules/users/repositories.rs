use crate::core::errors::AppError;
use diesel::prelude::*;
use uuid::Uuid;
// อย่าลืมอัปเดตบรรทัดนี้นะครับ
use crate::modules::users::models::{NewUser, NewUserProfile, NewUserRole, UpdateProfileRequest, User, UserProfile};
use crate::schema::{roles, user_profiles, user_roles, users};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_email(conn: &mut PgConnection, target_email: &str) -> QueryResult<Option<User>> {
        users::table
            .filter(users::email.eq(target_email))
            .first::<User>(conn)
            .optional()
    }

    pub fn create_user_with_profile(
        conn: &mut PgConnection,
        new_user: NewUser,
        first_name: String,
        last_name: String,
    ) -> QueryResult<User> {
        conn.transaction(|conn| {
            // 1. Insert ลงตาราง users
            let inserted_user: User = diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)?;

            // 2. Insert ลงตาราง user_profiles
            let new_profile = NewUserProfile {
                user_id: inserted_user.id,
                first_name,
                last_name,
            };
            diesel::insert_into(user_profiles::table)
                .values(&new_profile)
                .execute(conn)?;

            // 3. ค้นหา Role ID ของคำว่า "user" (จากที่เรา Seed ไว้)
            let default_role_id: Uuid = roles::table
                .filter(roles::name.eq("user"))
                .select(roles::id)
                .first(conn)?;

            // 4. ผูก Role ให้กับ User ใหม่
            let new_user_role = NewUserRole {
                user_id: inserted_user.id,
                role_id: default_role_id,
                assigned_by: None,
            };
            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .execute(conn)?;

            Ok(inserted_user)
        })
    }

    pub fn get_user_roles(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Vec<String>> {
        user_roles::table
            .inner_join(roles::table) // Join ตาราง user_roles กับ roles
            .filter(user_roles::user_id.eq(user_id))
            .select(roles::name) // เอาเฉพาะชื่อ role
            .load::<String>(conn)
    }

    pub fn get_all_paginated(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
    ) -> QueryResult<(Vec<User>, i64)> {
        // คืนค่าเป็น (รายการข้อมูล, จำนวนทั้งหมด)
        let offset = (page - 1) * limit;

        // 1. ดึงข้อมูล
        let items = users::table
            .order_by(users::created_at)
            .limit(limit)
            .offset(offset)
            .load::<User>(conn)?;

        // 2. นับจำนวนทั้งหมด
        let total: i64 = users::table.count().get_result(conn)?;

        Ok((items, total))
    }

    pub fn get_role_name_by_id(conn: &mut PgConnection, role_id: Uuid) -> QueryResult<String> {
        roles::table
            .filter(roles::id.eq(role_id))
            .select(roles::name)
            .first(conn)
    }

    pub fn assign_role(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        role_id: Uuid, // << รับ Uuid ตรงๆ
        assigned_by: Uuid,
    ) -> Result<(), AppError> {
        let new_user_role = NewUserRole {
            user_id: target_user_id,
            role_id,
            assigned_by: Some(assigned_by),
        };

        diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .on_conflict_do_nothing()
            .execute(conn)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถกำหนดสิทธิ์ได้".to_string()))?;

        Ok(())
    }

    // ดึง User คู่กับ Profile
    pub fn get_user_with_profile(
        conn: &mut PgConnection, 
        user_id: Uuid
    ) -> QueryResult<(User, UserProfile)> {
        users::table
            .inner_join(user_profiles::table.on(users::id.eq(user_profiles::user_id)))
            .filter(users::id.eq(user_id))
            // === เพิ่มบรรทัดนี้ เพื่อบอก Diesel ว่าจะผูกข้อมูลเข้า Struct ตัวไหนบ้าง ===
            .select((User::as_select(), UserProfile::as_select()))
            // =========================================================
            .first::<(User, UserProfile)>(conn)
    }

    // ฟังก์ชันอัปเดตรหัสผ่านและอัปเดต Token Version
    pub fn update_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> Result<(), AppError> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::password_hash.eq(new_password_hash),
                // เพิ่ม token_version ขึ้น 1 เพื่อบังคับให้ Token เก่าใช้งานไม่ได้ทันที
                users::token_version.eq(users::token_version + 1), 
                users::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเปลี่ยนรหัสผ่านได้".to_string()))?;
            
        Ok(())
    }

    pub fn increment_token_version(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::token_version.eq(users::token_version + 1))
            .execute(conn)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถออกจากระบบได้".to_string()))?;
            
        Ok(())
    }

    pub fn update_profile(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        req: &UpdateProfileRequest,
    ) -> Result<UserProfile, AppError> {
        let updated_profile = diesel::update(user_profiles::table.filter(user_profiles::user_id.eq(target_user_id)))
            .set((
                user_profiles::first_name.eq(&req.first_name),
                user_profiles::last_name.eq(&req.last_name),
            ))
            // === เพิ่มบรรทัดนี้เพื่อบังคับให้คืนค่ามาแค่ 3 ฟิลด์ที่ตรงกับ Struct ===
            .returning(UserProfile::as_returning()) 
            // ========================================================
            .get_result(conn) // ลบ ::<UserProfile> ออกได้เลย เพราะ as_returning บอก Type ไปแล้ว
            .optional() 
            .map_err(|_| AppError::InternalServerError("เกิดข้อผิดพลาดที่ระบบฐานข้อมูล ไม่สามารถอัปเดตได้".to_string()))?;

        match updated_profile {
            Some(profile) => Ok(profile),
            None => Err(AppError::BadRequest("ไม่พบข้อมูลโปรไฟล์ของคุณในระบบ (อาจถูกลบไปแล้ว)".to_string())),
        }
    }
}
