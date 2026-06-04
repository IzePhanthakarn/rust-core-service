use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreatePropertyTypeRequest {
    #[validate(length(min = 1, message = "กรุณาระบุชื่อ Property Type ที่ต้องการสร้าง"))]
    pub name: String,
    #[validate(length(min = 1, message = "กรุณาระบุ Code ของ Property Type ที่ต้องการสร้าง"))]
    pub code: String,
    pub description: Option<String>,
}