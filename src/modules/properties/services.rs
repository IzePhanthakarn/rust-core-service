use diesel::PgConnection;
use diesel::result::Error as DieselError;
use log::debug;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::properties::{
        dtos::PropertyResponse, models::{NewPropertyOption, NewPropertyType, PropertyOption, PropertyType}, repositories::PropertyRepository
    },
};

pub struct PropertyService;

impl PropertyService {
    pub fn get_property_type() {
        // Implementation for retrieving a property type
    }

    pub fn get_one_property_type(conn: &mut PgConnection, property_id: Uuid) -> Result<PropertyResponse, AppError> {
        let property_data = PropertyRepository::get_one_property_type(conn, property_id).map_err(|e| {
            debug!("Database Error: {:?}", e);
            match e {
                DieselError::NotFound => AppError::NotFound("ไม่พบ Property Type ที่ระบุ".to_string()),
                _ => AppError::InternalServerError("ไม่สามารถดึงข้อมูล Property Type ได้".to_string()),
            }
        })?;

        Ok(PropertyResponse::from_tuple(property_data))
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

        let new_property = NewPropertyType {
            name: name.to_string(),
            code: upper_code,
            description,
            created_by,
            updated_by: created_by,
        };

        let result = PropertyRepository::create_property_type(conn, new_property).map_err(|e| {
            println!("Database Error: {:?}", e);

            AppError::InternalServerError("ไม่สามารถสร้าง Property Type ใหม่ได้".to_string())
        })?;
        Ok(result)
    }

    pub fn update_property_type() {
        // Implementation for updating a property type
    }

    pub fn delete_property_type() {
        // Implementation for deleting a property type
    }

    pub fn create_property_option(
        conn: &mut PgConnection,
        property_type_id: Uuid,
        label: String,
        value: String,
        created_by: Uuid,
    ) -> Result<PropertyOption, AppError> {
        let is_have_option_value = PropertyRepository::check_property_options(conn, property_type_id, &value)
            .map_err(|e| {
                println!("Database Error: {:?}", e);
                AppError::InternalServerError("ไม่สามารถตรวจสอบ Property Option ได้".to_string())
            })?;

        if is_have_option_value {
            return Err(AppError::Conflict(format!(
                "มี Property Option value '{}' อยู่ในระบบแล้ว",
                value
            )));
        }

        let count_options =
            PropertyRepository::count_options_by_property_type_id(conn, property_type_id).map_err(
                |e| {
                    println!("Database Error: {:?}", e);
                    AppError::InternalServerError("ไม่สามารถนับจำนวน Property Option ได้".to_string())
                },
            )?;

        let new_option = NewPropertyOption {
            property_type_id,
            sort_order: count_options as i32 + 1,
            label,
            value,
            is_active: true,
            created_by,
        };
        let result = PropertyRepository::create_property_option(conn, new_option).map_err(|e| {
            println!("Database Error: {:?}", e);
            AppError::InternalServerError("ไม่สามารถสร้าง Property Option ใหม่ได้".to_string())
        })?;
        Ok(result)
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
