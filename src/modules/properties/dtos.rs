use diesel::{Selectable, deserialize::Queryable, pg};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::schema::{property_options, property_types};

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreatePropertyTypeRequest {
    #[validate(length(min = 1, message = "Please provide the name of the Property Type to create"))]
    pub name: String,
    #[validate(length(min = 1, message = "Please provide the Code of the Property Type to create"))]
    pub code: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdatePropertyTypeRequest {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Please provide the name of the Property Type to update"))]
    pub name: String,
    #[validate(length(min = 1, message = "Please provide the Code of the Property Type to update"))]
    pub code: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreatePropertyOptionRequest {
    #[validate(length(min = 1, message = "Please provide the name of the Property Option to create"))]
    pub label: String,
    #[validate(length(min = 1, message = "Please provide the Value of the Property Option to create"))]
    pub value: String,
    pub property_type_id: Uuid,
}

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = property_types)]
#[diesel(check_for_backend(pg::Pg))]
pub struct PropertyTypeData {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct PropertyFilterQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub name: Option<String>,
    pub code: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = property_options)]
#[diesel(check_for_backend(pg::Pg))]
pub struct PropertyOptionData {
    pub id: Uuid,
    pub sort_order: i32,
    pub label: String,
    pub value: String,
    pub is_active: bool,
}

#[derive(Serialize, ToSchema)]
pub struct PropertyResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub options: Vec<PropertyOptionData>,
}

impl PropertyResponse {
    pub fn from_tuple(data: (PropertyTypeData, Vec<PropertyOptionData>)) -> Self {
        let (pt, opts) = data;
        Self {
            id: pt.id,
            name: pt.name,
            code: pt.code,
            description: pt.description,
            options: opts,
        }
    }
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdatePropertyOptionRequest {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Please provide the Label of the Property Option to update"))]
    pub label: String,
    #[validate(length(min = 1, message = "Please provide the Value of the Property Option to update"))]
    pub value: String,
    pub sort_order: i32,
    pub is_active: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateStatusRequest {
    pub is_active: bool,
}
