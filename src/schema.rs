// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "billing_cycle"))]
    pub struct BillingCycle;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "note_type"))]
    pub struct NoteType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "project_status"))]
    pub struct ProjectStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_priority"))]
    pub struct TaskPriority;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_type"))]
    pub struct TaskType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_provider"))]
    pub struct UserProvider;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    board_columns (id) {
        id -> Uuid,
        board_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        order_index -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    boards (id) {
        id -> Uuid,
        project_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    events (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        #[max_length = 20]
        tag -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    holidays (id) {
        id -> Uuid,
        #[max_length = 1000]
        holiday_description -> Varchar,
        holiday_date -> Timestamptz,
        #[max_length = 10]
        holiday_year -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    project_members (id) {
        id -> Uuid,
        project_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::NoteType;

    project_notes (id) {
        id -> Uuid,
        project_id -> Uuid,
        parent_id -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> NoteType,
        #[max_length = 255]
        title -> Varchar,
        content -> Nullable<Text>,
        created_by -> Uuid,
        updated_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::ProjectStatus;

    projects (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        owner_id -> Uuid,
        status -> ProjectStatus,
        start_date -> Timestamptz,
        finish_date -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

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
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

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
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
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
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    sprints (id) {
        id -> Uuid,
        project_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        goal -> Nullable<Text>,
        start_date -> Timestamptz,
        end_date -> Nullable<Timestamptz>,
        is_active -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::BillingCycle;

    subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        amount -> Int8,
        billing_cycle -> BillingCycle,
        billing_day -> Int4,
        billing_month -> Nullable<Int4>,
        #[max_length = 50]
        category -> Nullable<Varchar>,
        #[max_length = 3000]
        note -> Nullable<Varchar>,
        start_date -> Timestamptz,
        end_date -> Nullable<Timestamptz>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    task_comments (id) {
        id -> Uuid,
        task_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::TaskType;
    use super::sql_types::TaskPriority;

    tasks (id) {
        id -> Uuid,
        project_id -> Uuid,
        board_id -> Uuid,
        sprint_id -> Nullable<Uuid>,
        column_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[sql_name = "type"]
        type_ -> TaskType,
        priority -> TaskPriority,
        story_points -> Nullable<Int4>,
        assignee_id -> Nullable<Uuid>,
        reporter_id -> Uuid,
        #[max_length = 20]
        tag -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    todo_items (id) {
        id -> Uuid,
        list_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        is_completed -> Bool,
        due_date -> Nullable<Timestamptz>,
        position -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    todo_lists (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        #[max_length = 20]
        color -> Nullable<Varchar>,
        position -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::TransactionType;

    transactions (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> TransactionType,
        amount -> Int8,
        #[max_length = 50]
        category -> Varchar,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 3000]
        note -> Nullable<Varchar>,
        transaction_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    transportation_expenses (id) {
        id -> Uuid,
        user_id -> Uuid,
        transaction_id -> Nullable<Uuid>,
        #[max_length = 50]
        category -> Varchar,
        amount -> Int8,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 3000]
        note -> Nullable<Varchar>,
        expense_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

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
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};
    use super::sql_types::UserRole;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        secret_word -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Nullable<Varchar>,
        role -> UserRole,
        status -> UserStatus,
        token_version -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    work_log_tags (log_id, work_tag) {
        log_id -> Uuid,
        #[max_length = 50]
        work_tag -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Int8, Nullable, Text, Timestamp, Timestamptz, Uuid, Varchar};

    work_logs (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        content -> Text,
        mood_score -> Int4,
        productivity_score -> Int4,
        date_logged -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(board_columns -> boards (board_id));
diesel::joinable!(boards -> projects (project_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(project_members -> projects (project_id));
diesel::joinable!(project_members -> users (user_id));
diesel::joinable!(project_notes -> projects (project_id));
diesel::joinable!(projects -> users (owner_id));
diesel::joinable!(property_options -> property_types (property_type_id));
diesel::joinable!(property_options -> users (created_by));
diesel::joinable!(social_accounts -> users (user_id));
diesel::joinable!(sprints -> projects (project_id));
diesel::joinable!(subscriptions -> users (user_id));
diesel::joinable!(task_comments -> tasks (task_id));
diesel::joinable!(task_comments -> users (user_id));
diesel::joinable!(tasks -> board_columns (column_id));
diesel::joinable!(tasks -> boards (board_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> sprints (sprint_id));
diesel::joinable!(todo_items -> todo_lists (list_id));
diesel::joinable!(todo_lists -> users (user_id));
diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(transportation_expenses -> transactions (transaction_id));
diesel::joinable!(transportation_expenses -> users (user_id));
diesel::joinable!(user_profiles -> users (user_id));
diesel::joinable!(work_log_tags -> work_logs (log_id));
diesel::joinable!(work_logs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    board_columns,
    boards,
    events,
    holidays,
    project_members,
    project_notes,
    projects,
    property_options,
    property_types,
    social_accounts,
    sprints,
    subscriptions,
    task_comments,
    tasks,
    todo_items,
    todo_lists,
    transactions,
    transportation_expenses,
    user_profiles,
    users,
    work_log_tags,
    work_logs,
);
