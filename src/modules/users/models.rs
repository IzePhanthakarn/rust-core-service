use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{user_profiles, users};

#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatus"]
pub enum UserStatus {
    Active,
    Suspended,
    Banned,
    Inactive,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub secret_word: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub status: UserStatus,
    pub token_version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub secret_word: Option<String>,
    pub password_hash: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, ToSchema)]
#[diesel(table_name = user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfile {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}