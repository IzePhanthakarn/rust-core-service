use diesel::PgConnection;
use log::debug;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::properties::{models::PropertyType, repositories::PropertyRepository},
};

pub struct PropertyService;

impl PropertyService {
    pub fn get_property_type() {
        // Implementation for retrieving a property type
    }

    pub fn get_one_property_type() {
        // Implementation for retrieving a property type
    }

    pub fn create_property_type(
        conn: &mut PgConnection,
        name: &str,
        code: &str,
        description: Option<String>,
        created_by: Uuid,
    ) -> Result<PropertyType, AppError> {
        // Implementation for creating a property type
        let upper_code = code.trim().to_ascii_uppercase();

        let existing_name = PropertyRepository::find_by_name(conn, name)
            .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

        if existing_name.is_some() {
            return Err(AppError::Conflict(format!(
                "มีชื่อ Property Type '{}' อยู่ในระบบแล้ว",
                name
            )));
        }

        let existing_code = PropertyRepository::find_by_code(conn, &upper_code)
            .map_err(|_| AppError::InternalServerError("Database Error".to_string()))?;

        if existing_code.is_some() {
            return Err(AppError::Conflict(format!(
                "มีรหัส Property Type '{}' อยู่ในระบบแล้ว",
                upper_code
            )));
        }

        let new_property =
            PropertyRepository::create_property_type(conn, name, &upper_code, description, created_by)
                .map_err(|e| {
                    println!("Database Error: {:?}", e);

                    AppError::InternalServerError("ไม่สามารถสร้าง Property Type ใหม่ได้".to_string())
                })?;
        Ok(new_property)
    }

    pub fn update_property_type() {
        // Implementation for updating a property type
    }

    pub fn delete_property_type() {
        // Implementation for deleting a property type
    }

    pub fn create_property_option() {
        // Implementation for creating a property option
    }

    pub fn get_property_option() {
        // Implementation for retrieving a property option
    }

    pub fn update_property_is_active() {
        // Implementation for updating a property option
    }

    pub fn delete_property_option() {
        // Implementation for deleting a property option
    }
}
