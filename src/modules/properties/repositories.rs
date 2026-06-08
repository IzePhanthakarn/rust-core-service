use crate::modules::properties::dtos::{PropertyOptionData, PropertyTypeData};
use crate::modules::properties::models::{
    NewPropertyOption, NewPropertyType, PropertyOption, UpdatePropertyType,
};
use crate::schema::property_options;
use crate::{modules::properties::models::PropertyType, schema::property_types};
use diesel::dsl::update;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

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

    pub fn get_all_property(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        name: Option<String>,
        code: Option<String>,
    ) -> QueryResult<(Vec<PropertyTypeData>, i64)> {
        let offset = (page - 1) * limit;

        let mut data_query = property_types::table.into_boxed();
        let mut count_query = property_types::table.into_boxed();

        if let Some(name_text) = name {
            let search_pattern = format!("%{}%", name_text);
            data_query = data_query.filter(property_types::name.ilike(search_pattern.clone()));
            count_query = count_query.filter(property_types::name.ilike(search_pattern));
        }

        if let Some(code_text) = code {
            let search_pattern = format!("%{}%", code_text);
            data_query = data_query.filter(property_types::code.ilike(search_pattern.clone()));
            count_query = count_query.filter(property_types::code.ilike(search_pattern));
        }

        let items = data_query
            .order(property_types::created_at.desc())
            .select(PropertyTypeData::as_select())
            .limit(limit)
            .offset(offset)
            .load::<PropertyTypeData>(conn)?;

        let total: i64 = count_query.count().get_result(conn)?;

        Ok((items, total))
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
        new_property: NewPropertyType,
    ) -> QueryResult<PropertyType> {
        diesel::insert_into(property_types::table)
            .values(&new_property)
            .returning(PropertyType::as_returning())
            .get_result(conn)
    }

    pub fn update_property_type(
        conn: &mut PgConnection,
        updated_property: UpdatePropertyType,
    ) -> QueryResult<PropertyType> {
        update(property_types::table)
            .filter(property_types::id.eq(updated_property.id))
            .set(updated_property)
            .returning(PropertyType::as_returning())
            .get_result(conn)
    }

    pub fn delete_property_type(conn: &mut PgConnection, property_id: Uuid) -> QueryResult<usize> {
        // Implementation for deleting a property type
        diesel::delete(property_types::table.filter(property_types::id.eq(property_id)))
            .execute(conn)
    }

    // ฟังก์ชันเช็คว่า properties_id นั้นมี value ใน property_options หรือยัง
    pub fn check_property_options(
        conn: &mut PgConnection,
        property_id: Uuid,
        value: &str,
    ) -> QueryResult<bool> {
        let count = property_options::table
            .filter(property_options::property_type_id.eq(property_id))
            .filter(property_options::value.eq(value))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    pub fn count_options_by_property_type_id(
        conn: &mut PgConnection,
        property_type_id: Uuid,
    ) -> QueryResult<i64> {
        property_options::table
            .filter(property_options::property_type_id.eq(property_type_id))
            .count()
            .get_result(conn)
    }

    pub fn create_property_option(
        conn: &mut PgConnection,
        new_property_option: NewPropertyOption,
    ) -> QueryResult<PropertyOption> {
        diesel::insert_into(property_options::table)
            .values(&new_property_option)
            .returning(PropertyOption::as_returning())
            .get_result(conn)
    }

    pub fn update_property_is_active(
        conn: &mut PgConnection,
        property_option_id: Uuid,
        is_active: bool,
    ) -> QueryResult<PropertyOptionData> {
        diesel::update(property_options::table.filter(property_options::id.eq(property_option_id)))
            .set(property_options::is_active.eq(is_active))
            .returning(PropertyOptionData::as_returning())
            .get_result(conn)
    }

    pub fn delete_property_option(
        conn: &mut PgConnection,
        property_option_id: Uuid,
    ) -> QueryResult<usize> {
        diesel::delete(property_options::table.filter(property_options::id.eq(property_option_id)))
            .execute(conn)
    }
}
