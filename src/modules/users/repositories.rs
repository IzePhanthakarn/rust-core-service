use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{roles, user_profiles, user_roles, users};
use crate::modules::users::models::{NewUser, NewUserProfile, User, UserProfile, UserStatus};
use crate::modules::roles::models::NewUserRole;

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
            let inserted_user: User = diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)?;

            let new_profile = NewUserProfile {
                user_id: inserted_user.id,
                first_name,
                last_name,
            };
            diesel::insert_into(user_profiles::table)
                .values(&new_profile)
                .execute(conn)?;

            let default_role_id: Uuid = roles::table
                .filter(roles::name.eq("user"))
                .select(roles::id)
                .first(conn)?;

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

    pub fn get_all_paginated(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        email_filter: Option<String>,
        status_filter: Option<UserStatus>,
    ) -> QueryResult<(Vec<User>, i64)> {
        let offset = (page - 1) * limit;

        let mut data_query = users::table.filter(users::deleted_at.is_null()).into_boxed();
        let mut count_query = users::table.filter(users::deleted_at.is_null()).into_boxed();

        if let Some(email_text) = email_filter {
            let search_pattern = format!("%{}%", email_text);
            data_query = data_query.filter(users::email.ilike(search_pattern.clone()));
            count_query = count_query.filter(users::email.ilike(search_pattern));
        }

        if let Some(status_enum) = status_filter {
            data_query = data_query.filter(users::status.eq(status_enum.clone()));
            count_query = count_query.filter(users::status.eq(status_enum));
        }

        let items = data_query
            .order_by(users::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<User>(conn)?;

        let total: i64 = count_query.count().get_result(conn)?;

        Ok((items, total))
    }

    pub fn get_user_with_profile(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> QueryResult<(User, UserProfile)> {
        users::table
            .inner_join(user_profiles::table.on(users::id.eq(user_profiles::user_id)))
            .filter(users::id.eq(user_id))
            .select((User::as_select(), UserProfile::as_select()))
            .first::<(User, UserProfile)>(conn)
    }

    pub fn update_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> QueryResult<usize> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::password_hash.eq(new_password_hash),
                users::token_version.eq(users::token_version + 1),
                users::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)
    }

    pub fn increment_token_version(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::token_version.eq(users::token_version + 1))
            .execute(conn)
    }

    pub fn update_profile(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        first_name: &str,
        last_name: &str,
    ) -> QueryResult<Option<UserProfile>> {
        diesel::update(user_profiles::table.filter(user_profiles::user_id.eq(target_user_id)))
            .set((
                user_profiles::first_name.eq(first_name),
                user_profiles::last_name.eq(last_name),
            ))
            .returning(UserProfile::as_returning())
            .get_result(conn)
            .optional()
    }

    pub fn update_user_status(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        new_status: &UserStatus,
    ) -> QueryResult<usize> {
        diesel::update(users::table.filter(users::id.eq(target_user_id)))
            .set((
                users::status.eq(new_status),
                users::token_version.eq(users::token_version + 1),
                users::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, target_user_id: Uuid) -> QueryResult<usize> {
        diesel::update(
            users::table
                .filter(users::id.eq(target_user_id))
                .filter(users::deleted_at.is_null()),
        )
        .set((
            users::status.eq(UserStatus::Inactive),
            users::deleted_at.eq(diesel::dsl::now),
            users::token_version.eq(users::token_version + 1),
        ))
        .execute(conn)
    }
}