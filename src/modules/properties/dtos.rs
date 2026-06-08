use diesel::{Selectable, deserialize::Queryable, pg};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::schema::{property_options, property_types};

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreatePropertyTypeRequest {
    #[validate(length(min = 1, message = "กรุณาระบุชื่อ Property Type ที่ต้องการสร้าง"))]
    pub name: String,
    #[validate(length(min = 1, message = "กรุณาระบุ Code ของ Property Type ที่ต้องการสร้าง"))]
    pub code: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdatePropertyTypeRequest {
    pub id: Uuid,
    #[validate(length(min = 1, message = "กรุณาระบุชื่อ Property Type ที่ต้องการแก้ไข"))]
    pub name: String,
    #[validate(length(min = 1, message = "กรุณาระบุ Code ของ Property Type ที่ต้องการแก้ไข"))]
    pub code: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreatePropertyOptionRequest {
    #[validate(length(min = 1, message = "กรุณาระบุชื่อ Property Option ที่ต้องการสร้าง"))]
    pub label: String,
    #[validate(length(min = 1, message = "กรุณาระบุ Value ของ Property Option ที่ต้องการสร้าง"))]
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
}

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = property_options)]
#[diesel(check_for_backend(pg::Pg))]
pub struct PropertyOptionData {
    pub id: Uuid,
    pub label: String,
    pub value: String,
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
