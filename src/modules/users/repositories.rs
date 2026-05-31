use diesel::prelude::*;
use uuid::Uuid;
// อย่าลืมอัปเดตบรรทัดนี้นะครับ
use crate::modules::users::models::{NewUser, NewUserProfile, NewUserRole, User};
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
}