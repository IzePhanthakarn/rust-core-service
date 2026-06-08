// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_provider"))]
    pub struct UserProvider;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;
}

diesel::table! {
    property_options (id) {
        id -> Uuid,
        property_type_id -> Uuid,
        sort_order -> Int4,
        #[max_length = 100]
        label -> Varchar,
        #[max_length = 50]
        value -> Varchar,
        is_active -> Bool,
        created_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    property_types (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_by -> Uuid,
        updated_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserProvider;

    social_accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider -> UserProvider,
        #[max_length = 255]
        provider_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_profiles (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        assigned_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        secret_word -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Nullable<Varchar>,
        status -> UserStatus,
        token_version -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    work_log_tags (log_id, work_tag) {
        log_id -> Uuid,
        #[max_length = 50]
        work_tag -> Varchar,
    }
}

diesel::table! {
    work_logs (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        content -> Text,
        mood_score -> Int4,
        productivity_score -> Int4,
        is_draft -> Bool,
        date_logged -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(property_options -> property_types (property_type_id));
diesel::joinable!(property_options -> users (created_by));
diesel::joinable!(social_accounts -> users (user_id));
diesel::joinable!(user_profiles -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(work_log_tags -> work_logs (log_id));
diesel::joinable!(work_logs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    property_options,
    property_types,
    roles,
    social_accounts,
    user_profiles,
    user_roles,
    users,
    work_log_tags,
    work_logs,
);
