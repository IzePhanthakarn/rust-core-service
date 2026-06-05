use crate::modules::properties::dtos::{PropertyOptionData, PropertyTypeData};
use crate::modules::properties::models::{NewPropertyOption, NewPropertyType, PropertyOption};
use crate::schema::property_options;
use uuid::Uuid;
use crate::{modules::properties::models::PropertyType, schema::property_types};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

pub struct PropertyRepository;

impl PropertyRepository {
    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Option<PropertyType>> {
        property_types::table
            .filter(property_types::name.eq(name))
            .select(PropertyType::as_select())
            .first::<PropertyType>(conn)
            .optional()
    }

    pub fn find_by_code(conn: &mut PgConnection, code: &str) -> QueryResult<Option<PropertyType>> {
        property_types::table
            .filter(property_types::code.eq(code))
            .select(PropertyType::as_select())
            .first::<PropertyType>(conn)
            .optional()
    }

    pub fn get_all_property_type(conn: &mut PgConnection) -> QueryResult<Vec<PropertyType>> {
        property_types::table
            .select(PropertyType::as_select())
            .load::<PropertyType>(conn)
    }

    pub fn get_one_property_type(
        conn: &mut PgConnection,
        property_id: Uuid,
    ) -> QueryResult<(PropertyTypeData, Vec<PropertyOptionData>)> {
        let property_type = property_types::table
            .filter(property_types::id.eq(property_id))
            .select(PropertyTypeData::as_select())
            .first::<PropertyTypeData>(conn)?;

        let options = property_options::table
            .filter(property_options::property_type_id.eq(property_id))
            .order(property_options::sort_order.asc())
            .select(PropertyOptionData::as_select())
            .load::<PropertyOptionData>(conn)?;

        Ok((property_type, options))
    }

    pub fn create_property_type(
        conn: &mut PgConnection,
        new_property: NewPropertyType
    ) -> QueryResult<PropertyType> {

        diesel::insert_into(property_types::table)
        .values(&new_property)
        .returning(PropertyType::as_returning())
        .get_result(conn)
    }

    pub fn update_property_type() {
        // Implementation for updating a property type
    }

    pub fn delete_property_type() {
        // Implementation for deleting a property type
    }

    // ฟังก์ชันเช็คว่า properties_id นั้นมี value ใน property_options หรือยัง
    pub fn check_property_options(conn: &mut PgConnection, property_id: Uuid, value: &str) -> QueryResult<bool> {
        let count = property_options::table
            .filter(property_options::property_type_id.eq(property_id))
            .filter(property_options::value.eq(value))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    pub fn count_options_by_property_type_id(conn: &mut PgConnection, property_type_id: Uuid) -> QueryResult<i64> {
        property_options::table
            .filter(property_options::property_type_id.eq(property_type_id))
            .count()
            .get_result(conn)
    }

    pub fn create_property_option(
        conn: &mut PgConnection,
        newPropertyOption: NewPropertyOption
    ) -> QueryResult<PropertyOption> {
        diesel::insert_into(property_options::table)
            .values(&newPropertyOption)
            .returning(PropertyOption::as_returning())
            .get_result(conn)
    }

    pub fn delete_property_option() {
        // Implementation for deleting a property option
    }
}
