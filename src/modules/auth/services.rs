use crate::core::jwt::{generate_tokens, verify_refresh_token};
use crate::core::security::verify_password;
use crate::core::{errors::AppError, security::hash_password};
use crate::modules::auth::dtos::{AuthResponse, LoginRequest, RefreshRequest, ResetPasswordRequest};
use crate::modules::users::models::UserStatus;
use crate::modules::{
    auth::dtos::RegisterRequest,
    users::{models::NewUser, repositories::UserRepository},
};
use diesel::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

pub struct AuthService;

impl AuthService {
    pub fn register(conn: &mut PgConnection, req: RegisterRequest) -> Result<(), AppError> {
        // 1. เช็คอีเมลซ้ำ
        let existing_user = UserRepository::find_by_email(conn, &req.email)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("อีเมลนี้ถูกใช้งานแล้ว".to_string()));
        }

        // 2. แฮชรหัสผ่าน
        let hashed_password = hash_password(&req.password)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเข้ารหัสผ่านได้".to_string()))?;

        // 3. แฮชคำลับ (secret_word) ที่ผู้ใช้ส่งมา
        let hashed_secret_word = hash_password(&req.secret_word)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเข้ารหัสคำลับได้".to_string()))?;

        // 4. ผูกข้อมูลเพื่อเตรียม Insert
        let new_user = NewUser {
            email: req.email,
            password_hash: hashed_password,
            secret_word: Some(hashed_secret_word), // ใส่คำลับที่ผ่านการ Hash แล้วลงไป
        };

        // 5. บันทึกลง 3 ตารางรวดเดียวผ่าน Repository
        UserRepository::create_user_with_profile(
            conn, 
            new_user, 
            req.first_name, 
            req.last_name
        )
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 6. คืนค่าข้อความสำเร็จ
        Ok(())
    }

    pub fn login(conn: &mut PgConnection, req: LoginRequest) -> Result<AuthResponse, AppError> {
        
        // 1. ค้นหา User จากอีเมล
        let user = UserRepository::find_by_email(conn, &req.email)
            .map_err(|_| AppError::InternalServerError("เกิดข้อผิดพลาดที่ระบบฐานข้อมูล".to_string()))?
            .ok_or_else(|| AppError::BadRequest("อีเมลหรือรหัสผ่านไม่ถูกต้อง".to_string()))?; 
            // ^ Security Tip: เราจะไม่บอกตรงๆ ว่า "ไม่พบอีเมล" เพื่อป้องกันคนสุ่มหาอีเมลในระบบ

        // 2. เช็คสถานะ (ป้องกัน User ที่โดนแบนเข้าสู่ระบบ)
        match user.status {
            UserStatus::Suspended => return Err(AppError::BadRequest("บัญชีนี้ถูกระงับการใช้งานชั่วคราว".to_string())),
            UserStatus::Banned => return Err(AppError::BadRequest("บัญชีนี้ถูกแบนถาวร".to_string())),
            UserStatus::Active => {} // ผ่านได้
        }

        // 3. ตรวจสอบรหัสผ่าน
        let is_valid_password = verify_password(
            user.password_hash.as_deref().unwrap_or(""), 
            &req.password
        );

        if !is_valid_password {
            return Err(AppError::BadRequest("อีเมลหรือรหัสผ่านไม่ถูกต้อง".to_string()));
        }

        // 4. ดึง Roles ทั้งหมดของ User
        let roles = UserRepository::get_user_roles(conn, user.id)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถดึงข้อมูลสิทธิ์การใช้งานได้".to_string()))?;

        // 5. ออก JWT Tokens
        let (access_token, refresh_token) = generate_tokens(user.id, user.token_version, roles)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถสร้าง Token ได้".to_string()))?;

        // 6. คืนค่า AuthResponse 
        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900, // 15 นาที = 900 วินาที
        })
    }
    
    pub fn refresh(conn: &mut PgConnection, req: RefreshRequest) -> Result<AuthResponse, AppError> {
        // 1. ถอดรหัส Refresh Token
        let claims = verify_refresh_token(&req.refresh_token)
            .map_err(|_| AppError::Unauthorized("Refresh Token ไม่ถูกต้องหรือหมดอายุ".to_string()))?;

        // 2. เช็คว่าประเภท Token ถูกต้องไหม
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("กรุณาใช้ Refresh Token เท่านั้น".to_string()));
        }

        // 3. ดึงข้อมูล User จาก Database
        let user = users::table
            .filter(users::id.eq(claims.sub))
            .first::<crate::modules::users::models::User>(conn)
            .map_err(|_| AppError::Unauthorized("ไม่พบผู้ใช้งานในระบบ".to_string()))?;

        // 4. ตรวจสอบสถานะ User
        match user.status {
            crate::modules::users::models::UserStatus::Suspended => return Err(AppError::Forbidden("บัญชีนี้ถูกระงับการใช้งาน".to_string())),
            crate::modules::users::models::UserStatus::Banned => return Err(AppError::Forbidden("บัญชีนี้ถูกแบนถาวร".to_string())),
            _ => {}
        }

        // 5. ตรวจสอบ Token Version (ป้องกัน Token ที่ถูก Force Logout)
        if user.token_version != claims.token_version {
            return Err(AppError::Unauthorized("Token นี้ถูกยกเลิกการใช้งานแล้ว".to_string()));
        }

        // 6. ดึง Roles ใหม่ล่าสุดจาก DB (เผื่อมีการเพิ่ม/ลดสิทธิ์ระหว่างที่ Token ใบเก่าทำงานอยู่)
        let roles = UserRepository::get_user_roles(conn, user.id)
            .unwrap_or_default();

        // 7. ออก Token คู่ใหม่ให้
        let (access_token, refresh_token) = generate_tokens(user.id, user.token_version, roles)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถสร้าง Token ได้".to_string()))?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900, // 15 นาที
        })
    }

    pub fn reset_password(conn: &mut PgConnection, req: ResetPasswordRequest) -> Result<(), AppError> {
        // 1. ค้นหา User จากอีเมล
        let user_result = UserRepository::find_by_email(conn, &req.email)
            .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

        // 2. เช็คว่าเจออีเมลไหม (ถ้าไม่เจอ ดีดกลับทันที)
        let user = match user_result {
            Some(u) => u,
            None => return Err(AppError::BadRequest("ไม่พบอีเมลนี้ในระบบ".to_string())),
        };

        // 3. ดึง Hash ของคำลับออกมา
        let stored_secret_word = user.secret_word.as_deref().unwrap_or("");

        // 4. ตรวจสอบคำลับ (ถ้ามาถึงตรงนี้แปลว่าอีเมลถูกแล้ว เช็คแค่คำลับ)
        if !verify_password(stored_secret_word, &req.secret_word) {
            return Err(AppError::BadRequest("คำลับไม่ถูกต้อง".to_string()));
        }

        // 5. แฮชรหัสผ่านใหม่
        let hashed_new_password = hash_password(&req.new_password)
            .map_err(|_| AppError::InternalServerError("ไม่สามารถเข้ารหัสผ่านใหม่ได้".to_string()))?;

        // 6. อัปเดตลง Database
        UserRepository::update_password(conn, user.id, &hashed_new_password)?;

        Ok(())
    }

    pub fn logout(conn: &mut PgConnection, user_id: Uuid) -> Result<(), AppError> {
        UserRepository::increment_token_version(conn, user_id)?;
        Ok(())
    }

}

