use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{roles, user_roles};
use crate::modules::roles::models::{Role, NewUserRole};

pub struct RoleRepository;

impl RoleRepository {
    pub fn get_all_roles(conn: &mut PgConnection) -> QueryResult<Vec<Role>> {
        roles::table.select(Role::as_select()).load::<Role>(conn)
    }

    pub fn get_user_roles(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Vec<String>> {
        user_roles::table
            .inner_join(roles::table)
            .filter(user_roles::user_id.eq(user_id))
            .select(roles::name)
            .load::<String>(conn)
    }

    pub fn get_role_name_by_id(conn: &mut PgConnection, role_id: Uuid) -> QueryResult<String> {
        roles::table
            .filter(roles::id.eq(role_id))
            .select(roles::name)
            .first(conn)
    }

    pub fn assign_role(conn: &mut PgConnection, target_user_id: Uuid, role_id: Uuid, assigned_by: Uuid) -> QueryResult<usize> {
        let new_user_role = NewUserRole {
            user_id: target_user_id,
            role_id,
            assigned_by: Some(assigned_by),
        };

        diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .on_conflict_do_nothing()
            .execute(conn)
    }

    pub fn revoke_role(conn: &mut PgConnection, target_user_id: Uuid, role_id_to_revoke: Uuid) -> QueryResult<usize> {
        diesel::delete(
            user_roles::table
                .filter(user_roles::user_id.eq(target_user_id))
                .filter(user_roles::role_id.eq(role_id_to_revoke)),
        )
        .execute(conn)
    }
}