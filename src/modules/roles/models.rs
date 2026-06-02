use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{roles, user_roles};

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_by: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}