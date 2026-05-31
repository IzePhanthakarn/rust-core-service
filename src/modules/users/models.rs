use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{users, user_profiles, user_roles}; // << เพิ่ม user_roles

#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatus"]
pub enum UserStatus {
    Active,
    Suspended,
    Banned,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    #[serde(skip_serializing)] 
    pub secret_word: Option<String>, // << เพิ่มฟิลด์นี้ (ซ่อนไว้ไม่ให้เผลอหลุดไปกับ JSON)
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
    pub secret_word: Option<String>, // << เพิ่มฟิลด์นี้
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

// === เพิ่ม Struct สำหรับการ Insert ผูก Role ===
#[derive(Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_by: Option<Uuid>, // ให้เป็น None เพราะระบบเป็นคนผูกให้
}