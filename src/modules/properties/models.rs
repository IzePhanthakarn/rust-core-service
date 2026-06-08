use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{property_options, property_types};

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = property_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PropertyType {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Clone, Debug, ToSchema)]
#[diesel(table_name = property_options)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PropertyOption {
    pub id: Uuid,
    pub property_type_id: Uuid,
    pub sort_order: i32,
    pub label: String,
    pub value: String,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = property_types)]
pub struct NewPropertyType {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = property_types)]
pub struct UpdatePropertyType {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub updated_by: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = property_options)]
pub struct NewPropertyOption {
    pub property_type_id: Uuid,
    pub sort_order: i32,
    pub label: String,
    pub value: String,
    pub is_active: bool,
    pub created_by: Uuid,
}
