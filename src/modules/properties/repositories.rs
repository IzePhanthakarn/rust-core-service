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
    ) -> QueryResult<(PropertyType, PropertyOption)> {
        property_types::table.inner_join(property_options::table.on(property_types::id.eq(property_id)))
        .filter(property_types::id.eq(property_id))
        .select((PropertyType::as_select(), PropertyOption::as_select()))
        .first::<(PropertyType, PropertyOption)>(conn)
    }

    pub fn create_property_type(
        conn: &mut PgConnection,
        name: &str,
        code: &str,
        description: Option<String>,
        created_by: Uuid,
    ) -> QueryResult<PropertyType> {
        
        let new_property = NewPropertyType {
            name: name.to_string(),
            code: code.to_string(),
            description,
            created_by,
            updated_by: created_by,
        };

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

    pub fn create_property_option(
        conn: &mut PgConnection,
        newPropertyOption: NewPropertyOption
    ) -> QueryResult<PropertyOption> {
        diesel::insert_into(property_options::table)
            .values(&newPropertyOption)
            .returning(PropertyOption::as_returning())
            .get_result(conn)
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
