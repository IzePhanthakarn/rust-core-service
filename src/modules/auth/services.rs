use crate::core::jwt::{generate_tokens, verify_refresh_token};
use crate::core::security::verify_password;
use crate::core::{errors::AppError, security::hash_password};
use crate::modules::auth::dtos::{
    AuthResponse, ChangePasswordRequest, LoginRequest, RefreshRequest, ResetPasswordRequest,
};
use crate::modules::auth::repositories::AuthRepository;
use crate::modules::users::models::UserStatus;
use crate::modules::{
    auth::dtos::RegisterRequest,
    users::{models::NewUser, repositories::UserRepository, services::UserService},
};
use diesel::PgConnection;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub fn register(conn: &mut PgConnection, req: RegisterRequest) -> Result<(), AppError> {
        // 1. Check for a duplicate email
        let existing_user = UserRepository::find_by_email(conn, &req.email)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("This email is already in use".to_string()));
        }

        // 2. Hash the password
        let hashed_password = hash_password(&req.password)
            .map_err(|_| AppError::InternalServerError("Unable to hash the password".to_string()))?;

        // 3. Hash the secret word submitted by the user
        let hashed_secret_word = hash_password(&req.secret_word)
            .map_err(|_| AppError::InternalServerError("Unable to hash the secret word".to_string()))?;

        // 4. Bundle the data to prepare for insertion
        let new_user = NewUser {
            email: req.email,
            password_hash: hashed_password,
            secret_word: Some(hashed_secret_word),
        };

        // 5. Save to all 3 tables at once via the repository
        UserRepository::create_user_with_profile(conn, new_user, req.first_name, req.last_name)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub fn login(conn: &mut PgConnection, req: LoginRequest) -> Result<AuthResponse, AppError> {
        // 1. Look up the user by email
        let user = UserRepository::find_by_email(conn, &req.email)
            .map_err(|_| AppError::InternalServerError("A database error occurred".to_string()))?
            .ok_or_else(|| AppError::BadRequest("Invalid email or password".to_string()))?;

        // 2. Check the status (prevent banned users from logging in)
        match user.status {
            UserStatus::Suspended => {
                return Err(AppError::BadRequest("This account has been temporarily suspended".to_string()));
            }
            UserStatus::Banned => return Err(AppError::BadRequest("This account has been permanently banned".to_string())),
            UserStatus::Inactive => return Err(AppError::BadRequest("This account has been deleted".to_string())),
            UserStatus::Active => {}
        }

        // 3. Verify the password
        let is_valid_password =
            verify_password(user.password_hash.as_deref().unwrap_or(""), &req.password);

        if !is_valid_password {
            return Err(AppError::BadRequest("Invalid email or password".to_string()));
        }

        // 4. Issue JWT tokens with the user's current role
        let (access_token, refresh_token) =
            generate_tokens(user.id, user.token_version, user.role.as_str().to_string())
                .map_err(|_| AppError::InternalServerError("Unable to generate token".to_string()))?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900, // 15 minutes
        })
    }

    pub fn refresh(conn: &mut PgConnection, req: RefreshRequest) -> Result<AuthResponse, AppError> {
        // 1. Decode the refresh token
        let claims = verify_refresh_token(&req.refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

        // 2. Check that the token type is correct
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized(
                "Please use a refresh token only".to_string(),
            ));
        }

        // 3. Fetch the user via AuthRepository
        let user = AuthRepository::find_user_by_id(conn, claims.sub)
            .map_err(|_| AppError::Unauthorized("User not found in the system".to_string()))?;

        // 4. Check the user's status
        match user.status {
            UserStatus::Suspended => {
                return Err(AppError::Forbidden("This account has been suspended".to_string()));
            }
            UserStatus::Banned => return Err(AppError::Forbidden("This account has been permanently banned".to_string())),
            _ => {}
        }

        // 5. Check the token version
        if user.token_version != claims.token_version {
            return Err(AppError::Unauthorized(
                "This token has been revoked".to_string(),
            ));
        }

        // 6. Issue a new token pair with the latest role from the DB
        let (access_token, refresh_token) =
            generate_tokens(user.id, user.token_version, user.role.as_str().to_string())
                .map_err(|_| AppError::InternalServerError("Unable to generate token".to_string()))?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900,
        })
    }

    pub fn reset_password(
        conn: &mut PgConnection,
        req: ResetPasswordRequest,
    ) -> Result<(), AppError> {
        let user_result = UserRepository::find_by_email(conn, &req.email)
            .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

        let user = match user_result {
            Some(u) => u,
            None => return Err(AppError::BadRequest("This email was not found in the system".to_string())),
        };

        let stored_secret_word = user.secret_word.as_deref().unwrap_or("");

        if !verify_password(stored_secret_word, &req.secret_word) {
            return Err(AppError::BadRequest("Incorrect secret word".to_string()));
        }

        let hashed_new_password = hash_password(&req.new_password)
            .map_err(|_| AppError::InternalServerError("Unable to hash the new password".to_string()))?;

        // Call UserService instead so it handles the AppError for us
        UserService::update_password(conn, user.id, &hashed_new_password)?;

        Ok(())
    }

    pub fn logout(conn: &mut PgConnection, user_id: Uuid) -> Result<(), AppError> {
        // Call UserService instead
        UserService::increment_token_version(conn, user_id)?;
        Ok(())
    }

    pub fn change_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        req: ChangePasswordRequest,
    ) -> Result<(), AppError> {
        // 1. Fetch the user via AuthRepository
        let user = AuthRepository::find_user_by_id(conn, user_id)
            .map_err(|_| AppError::BadRequest("User data not found in the system".to_string()))?;

        // 2. Verify the old password
        let stored_password_hash = user.password_hash.as_deref().unwrap_or("");
        if !verify_password(stored_password_hash, &req.old_password) {
            return Err(AppError::BadRequest("Old password is incorrect".to_string()));
        }

        // 3. Prevent the new password from matching the old one
        if verify_password(stored_password_hash, &req.new_password) {
            return Err(AppError::BadRequest(
                "The new password must not be the same as the old password".to_string(),
            ));
        }

        // 4. Hash the new password
        let hashed_new_password = hash_password(&req.new_password)
            .map_err(|_| AppError::InternalServerError("Unable to hash the new password".to_string()))?;

        // 5. Update the database by calling UserService
        UserService::update_password(conn, user.id, &hashed_new_password)?;

        Ok(())
    }
}
