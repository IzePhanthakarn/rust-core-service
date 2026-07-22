use crate::modules::users::models::User;
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

pub struct AuthRepository;

impl AuthRepository {
    // Moved the query for fetching a user by ID here
    // Much cleaner, since the service no longer needs to know about users::table
    pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        users::table
            .filter(users::id.eq(user_id))
            .filter(users::deleted_at.is_null()) // Exclude users that have already been deleted (soft delete)
            .first::<User>(conn)
    }
}
