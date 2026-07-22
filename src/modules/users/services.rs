use crate::core::{errors::AppError, response::{PaginatedData, normalize_page_limit}};
use crate::modules::users::dtos::{UpdateProfileRequest, UserFilterQuery};
use crate::modules::users::models::{User, UserProfile, UserStatus};
use crate::modules::users::repositories::UserRepository;
use diesel::PgConnection;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn get_all_users(
        conn: &mut PgConnection,
        filters: UserFilterQuery,
    ) -> Result<PaginatedData<User>, AppError> {
        let (page, limit) = normalize_page_limit(filters.page, filters.limit);

        let (items, total_items) =
            UserRepository::get_all_users(conn, page, limit, filters.email, filters.status)
                .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(PaginatedData::new(items, total_items, page, limit))
    }

    pub fn update_profile(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        req: &UpdateProfileRequest,
    ) -> Result<UserProfile, AppError> {
        let profile =
            UserRepository::update_profile(conn, target_user_id, &req.first_name, &req.last_name)
                .map_err(|_| {
                    AppError::InternalServerError(
                        "A database error occurred, unable to update".to_string(),
                    )
                })?;

        match profile {
            Some(p) => Ok(p),
            None => Err(AppError::BadRequest(
                "Your profile data was not found in the system (it may have been deleted)".to_string(),
            )),
        }
    }

    pub fn update_user_status(
        conn: &mut PgConnection,
        target_user_id: Uuid,
        new_status: &UserStatus,
    ) -> Result<(), AppError> {
        let updated_rows = UserRepository::update_user_status(conn, target_user_id, new_status)
            .map_err(|_| {
                AppError::InternalServerError(
                    "A database error occurred, unable to change status".to_string(),
                )
            })?;

        if updated_rows == 0 {
            return Err(AppError::BadRequest(
                "The user account to change status for was not found (it may have been deleted)".to_string(),
            ));
        }

        Ok(())
    }

    pub fn delete_user(conn: &mut PgConnection, target_user_id: Uuid) -> Result<(), AppError> {
        let updated_rows = UserRepository::delete_user(conn, target_user_id)
            .map_err(|_| AppError::InternalServerError("Unable to delete the account".to_string()))?;

        if updated_rows == 0 {
            return Err(AppError::BadRequest(
                "The user account to delete was not found, or it has already been deleted".to_string(),
            ));
        }

        Ok(())
    }

    pub fn update_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> Result<(), AppError> {
        UserRepository::update_password(conn, user_id, new_password_hash)
            .map_err(|_| AppError::InternalServerError("Unable to change the password".to_string()))?;
        Ok(())
    }

    pub fn increment_token_version(conn: &mut PgConnection, user_id: Uuid) -> Result<(), AppError> {
        UserRepository::increment_token_version(conn, user_id)
            .map_err(|_| AppError::InternalServerError("Unable to log out".to_string()))?;
        Ok(())
    }
}
