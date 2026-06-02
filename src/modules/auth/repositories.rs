use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;
use crate::modules::users::models::User;

pub struct AuthRepository;

impl AuthRepository {
    // ย้าย Query การดึง User ด้วย ID มาไว้ที่นี่
    // คลีนสุดๆ เพราะ Service จะไม่ต้องรู้จัก users::table อีกต่อไป
    pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        users::table
            .filter(users::id.eq(user_id))
            .filter(users::deleted_at.is_null()) // ดักคนที่ถูกลบไปแล้ว (Soft Delete)
            .first::<User>(conn)
    }
}