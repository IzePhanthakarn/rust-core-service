// src/docs.rs
use crate::{core, modules};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health Check Route
        modules::health::handlers::health_check,

        // Auth Routes
        modules::auth::handlers::register,
        modules::auth::handlers::login,
        modules::auth::handlers::refresh_token,
        modules::auth::handlers::change_password,
        modules::auth::handlers::reset_password,
        modules::auth::handlers::logout,

        // User Routes
        modules::users::handlers::get_users,
        modules::users::handlers::update_me,
        modules::users::handlers::get_me,
        modules::users::handlers::update_user_status,
        modules::users::handlers::delete_user_by_id,
        modules::users::handlers::get_user_by_id,

        // Properties Routes
        modules::properties::handlers::get_all_property_type,
        modules::properties::handlers::get_one_property_type,
        modules::properties::handlers::get_property_type_by_code,
        modules::properties::handlers::create_property_type,
        modules::properties::handlers::update_property_type,
        modules::properties::handlers::delete_property_type,
        modules::properties::handlers::create_property_option,
        modules::properties::handlers::update_property_option_status,
        modules::properties::handlers::update_property_option,
        modules::properties::handlers::delete_property_option,

        // Work Logs Routes
        modules::work_logs::handlers::get_all_work_logs,
        modules::work_logs::handlers::get_one_work_log,
        modules::work_logs::handlers::create_work_log,
        modules::work_logs::handlers::update_work_log,
        modules::work_logs::handlers::delete_work_log,

        // Calendar Routes
        modules::calendar::handlers::get_holidays,
        modules::calendar::handlers::fetch_holidays,
        modules::calendar::handlers::get_events,
        modules::calendar::handlers::create_events,
        modules::calendar::handlers::update_event,
        modules::calendar::handlers::delete_event,

        // Todo Routes
        modules::todos::handlers::get_all_todo_lists,
        modules::todos::handlers::create_todo_list,
        modules::todos::handlers::update_todo_list,
        modules::todos::handlers::delete_todo_list,
        modules::todos::handlers::move_todo_list_to_top,
        modules::todos::handlers::create_todo_item,
        modules::todos::handlers::reorder_todo_items,
        modules::todos::handlers::toggle_todo_item,
        modules::todos::handlers::delete_todo_item,

        // Project Routes
        modules::projects::handlers::list_projects,
        modules::projects::handlers::create_project,
        modules::projects::handlers::get_project_detail,
        modules::projects::handlers::update_project,
        modules::projects::handlers::delete_project,
        modules::projects::handlers::list_members,
        modules::projects::handlers::add_member,
        modules::projects::handlers::remove_member,
        modules::projects::handlers::get_notes,
        modules::projects::handlers::create_note,
        modules::projects::handlers::update_note,
        modules::projects::handlers::delete_note,
        modules::projects::handlers::get_boards,
        modules::projects::handlers::get_kanban,
        modules::projects::handlers::get_backlogs,
        modules::projects::handlers::list_sprints,
        modules::projects::handlers::get_sprint_tasks,
        modules::projects::handlers::create_sprint,
        modules::projects::handlers::update_sprint,
        modules::projects::handlers::delete_sprint,

        // Task Routes
        modules::projects::handlers::create_task,
        modules::projects::handlers::update_task,
        modules::projects::handlers::delete_task,
        modules::projects::handlers::list_comments,
        modules::projects::handlers::create_comment,

        // Transaction Routes
        modules::transactions::handlers::get_all_transactions,
        modules::transactions::handlers::get_one_transaction,
        modules::transactions::handlers::create_transaction,
        modules::transactions::handlers::update_transaction,
        modules::transactions::handlers::delete_transaction,

        // Subscription Routes
        modules::transactions::handlers::get_all_subscriptions,
        modules::transactions::handlers::create_subscription,
        modules::transactions::handlers::update_subscription,
        modules::transactions::handlers::toggle_subscription,
        modules::transactions::handlers::delete_subscription,

        // Transportation Expense Routes
        modules::transportation_expenses::handlers::get_all_transportation_expenses,
        modules::transportation_expenses::handlers::create_transportation_expense,
        modules::transportation_expenses::handlers::update_transportation_expense,
        modules::transportation_expenses::handlers::delete_transportation_expense
    ),
    components(schemas(
        // ==== Common Response Schemas ===
        core::response::ApiResponse<modules::health::dtos::HealthData>,
        core::response::ApiResponse<core::response::EmptyData>,
        core::response::ApiResponse<modules::auth::dtos::AuthResponse>,
        core::response::EmptyData,
        // ================================

        // ==== Health ====
        modules::health::dtos::HealthData,
        modules::health::dtos::DbHealth,
        // ================================

        // ==== Auth ====
        modules::auth::dtos::RegisterRequest,
        modules::auth::dtos::LoginRequest,
        modules::auth::dtos::AuthResponse,
        modules::auth::dtos::RefreshRequest,
        modules::auth::dtos::ChangePasswordRequest,
        modules::auth::dtos::ResetPasswordRequest,
        // ================================

        // ==== Users ====
        modules::users::models::User,
        modules::users::models::UserStatus,
        modules::users::models::UserRole,
        modules::users::models::UserProfile,

        modules::users::dtos::MeResponse,
        modules::users::dtos::UpdateProfileRequest,
        modules::users::dtos::UpdateUserStatusRequest,
        modules::users::dtos::UserDetailResponse,

        core::response::PaginatedData<modules::users::models::User>,
        core::response::ApiResponse<core::response::PaginatedData<modules::users::models::User>>,
        core::response::ApiResponse<modules::users::dtos::MeResponse>,
        core::response::ApiResponse<modules::users::dtos::UserDetailResponse>,
        // ================================

        // ==== Properties ====
        modules::properties::models::PropertyType,
        modules::properties::models::PropertyOption,
        modules::properties::dtos::CreatePropertyTypeRequest,
        modules::properties::dtos::UpdatePropertyTypeRequest,
        modules::properties::dtos::CreatePropertyOptionRequest,
        modules::properties::dtos::PropertyResponse,
        modules::properties::dtos::PropertyTypeData,
        modules::properties::dtos::PropertyOptionData,
        modules::properties::dtos::PropertyFilterQuery,
        modules::properties::dtos::UpdateStatusRequest,
        modules::properties::dtos::UpdatePropertyOptionRequest,

        core::response::ApiResponse<modules::properties::models::PropertyType>,
        core::response::ApiResponse<modules::properties::models::PropertyOption>,
        core::response::ApiResponse<modules::properties::dtos::PropertyResponse>,
        // ================================

        // ==== Work Logs ====
        modules::work_logs::models::WorkLog,
        modules::work_logs::models::WorkLogTag,
        modules::work_logs::dtos::CreateWorkLogRequest,
        modules::work_logs::dtos::WorkLogListResponse,
        modules::work_logs::dtos::WorkLogResponse,
        core::response::ApiResponse<modules::work_logs::dtos::WorkLogListResponse>,
        core::response::ApiResponse<modules::work_logs::dtos::WorkLogResponse>,
        // ================================

        // ==== Calendar ====
        modules::calendar::dtos::FetchHolidayRequest,
        modules::calendar::dtos::FetchHolidayResult,
        modules::calendar::dtos::HolidayResponse,
        modules::calendar::dtos::HolidayListResponse,
        modules::calendar::dtos::HolidayStats,
        modules::calendar::dtos::NextHolidayInfo,
        modules::calendar::dtos::CreateEventRequest,
        modules::calendar::dtos::UpdateEventRequest,
        modules::calendar::dtos::EventResponse,
        modules::calendar::dtos::EventListResponse,
        modules::calendar::models::Holiday,
        core::response::ApiResponse<modules::calendar::dtos::FetchHolidayResult>,
        core::response::ApiResponse<modules::calendar::dtos::HolidayListResponse>,
        core::response::ApiResponse<Vec<modules::calendar::dtos::EventResponse>>,
        core::response::ApiResponse<modules::calendar::dtos::EventListResponse>,
        core::response::ApiResponse<modules::calendar::dtos::EventResponse>,
        // ================================

        // ==== Todos ====
        modules::todos::models::TodoList,
        modules::todos::models::TodoItem,
        modules::todos::dtos::CreateTodoListRequest,
        modules::todos::dtos::UpdateTodoListRequest,
        modules::todos::dtos::CreateTodoItemRequest,
        modules::todos::dtos::ReorderTodoItemsRequest,
        modules::todos::dtos::TodoListResponse,
        modules::todos::dtos::TodoItemResponse,
        core::response::ApiResponse<Vec<modules::todos::dtos::TodoListResponse>>,
        core::response::ApiResponse<modules::todos::dtos::TodoListResponse>,
        core::response::ApiResponse<modules::todos::dtos::TodoItemResponse>,
        // ================================

        // ==== Projects ====
        modules::projects::models::ProjectStatus,
        modules::projects::models::NoteType,
        modules::projects::models::TaskType,
        modules::projects::models::TaskPriority,

        modules::projects::dtos::CreateProjectRequest,
        modules::projects::dtos::UpdateProjectRequest,
        modules::projects::dtos::AddProjectMemberRequest,
        modules::projects::dtos::CreateNoteRequest,
        modules::projects::dtos::UpdateNoteRequest,
        modules::projects::dtos::CreateSprintRequest,
        modules::projects::dtos::UpdateSprintRequest,
        modules::projects::dtos::CreateTaskRequest,
        modules::projects::dtos::UpdateTaskRequest,
        modules::projects::dtos::CreateCommentRequest,

        modules::projects::dtos::ProjectResponse,
        modules::projects::dtos::ProjectDetailResponse,
        modules::projects::dtos::BoardResponse,
        modules::projects::dtos::BoardColumnResponse,
        modules::projects::dtos::MemberResponse,
        modules::projects::dtos::NoteResponse,
        modules::projects::dtos::SprintResponse,
        modules::projects::dtos::TaskAssigneeResponse,
        modules::projects::dtos::TaskResponse,
        modules::projects::dtos::KanbanResponse,
        modules::projects::dtos::KanbanColumnResponse,
        modules::projects::dtos::CommentResponse,

        core::response::ApiResponse<Vec<modules::projects::dtos::ProjectResponse>>,
        core::response::ApiResponse<modules::projects::dtos::ProjectDetailResponse>,
        core::response::ApiResponse<modules::projects::dtos::MemberResponse>,
        core::response::PaginatedData<modules::projects::dtos::MemberResponse>,
        core::response::ApiResponse<core::response::PaginatedData<modules::projects::dtos::MemberResponse>>,
        core::response::ApiResponse<Vec<modules::projects::dtos::NoteResponse>>,
        core::response::ApiResponse<modules::projects::dtos::NoteResponse>,
        core::response::ApiResponse<Vec<modules::projects::dtos::BoardResponse>>,
        core::response::ApiResponse<modules::projects::dtos::KanbanResponse>,
        core::response::ApiResponse<Vec<modules::projects::dtos::TaskResponse>>,
        core::response::ApiResponse<modules::projects::dtos::TaskResponse>,
        core::response::ApiResponse<Vec<modules::projects::dtos::SprintResponse>>,
        core::response::ApiResponse<modules::projects::dtos::SprintResponse>,
        core::response::ApiResponse<Vec<modules::projects::dtos::CommentResponse>>,
        core::response::ApiResponse<modules::projects::dtos::CommentResponse>,
        // ================================

        // ==== Transactions & Subscriptions ====
        modules::transactions::models::TransactionType,
        modules::transactions::models::BillingCycle,

        modules::transactions::dtos::CreateTransactionRequest,
        modules::transactions::dtos::UpdateTransactionRequest,
        modules::transactions::dtos::TransactionResponse,
        modules::transactions::dtos::CreateSubscriptionRequest,
        modules::transactions::dtos::UpdateSubscriptionRequest,
        modules::transactions::dtos::SubscriptionResponse,
        modules::transactions::dtos::SubscriptionListResponse,
        modules::transactions::dtos::SubscriptionStatsResponse,
        modules::transactions::dtos::SubscriptionMonthlyStat,
        modules::transactions::dtos::SubscriptionCategoryStat,
        modules::transactions::dtos::SubscriptionCycleSplit,
        modules::transactions::dtos::SubscriptionTopExpense,

        core::response::PaginatedData<modules::transactions::dtos::TransactionResponse>,
        core::response::ApiResponse<core::response::PaginatedData<modules::transactions::dtos::TransactionResponse>>,
        core::response::ApiResponse<modules::transactions::dtos::TransactionResponse>,
        core::response::ApiResponse<modules::transactions::dtos::SubscriptionListResponse>,
        core::response::ApiResponse<modules::transactions::dtos::SubscriptionResponse>,
        // ================================

        // ==== Transportation Expenses ====
        modules::transportation_expenses::dtos::CreateTransportationExpenseRequest,
        modules::transportation_expenses::dtos::UpdateTransportationExpenseRequest,
        modules::transportation_expenses::dtos::TransportationExpenseResponse,

        core::response::PaginatedData<modules::transportation_expenses::dtos::TransportationExpenseResponse>,
        core::response::ApiResponse<core::response::PaginatedData<modules::transportation_expenses::dtos::TransportationExpenseResponse>>,
        core::response::ApiResponse<modules::transportation_expenses::dtos::TransportationExpenseResponse>,
        // ================================
    )),
    tags(
        (name = "System Health", description = "Endpoints for monitoring server status"),
        (name = "Auth", description = "Authentication & User Management") ,
        (name = "Users", description = "User Management"),
        (name = "Properties", description = "Property Type and Option Management"),
        (name = "Work Logs", description = "Work Log Management"),
        (name = "Calendar", description = "Calendar & Holiday Management"),
        (name = "Todos", description = "Todo List and Item Management"),
        (name = "Projects", description = "Project, Notes, Kanban, Sprint & Member Management"),
        (name = "Tasks", description = "Task (Kanban Card / Backlog) and Comment Management"),
        (name = "Transactions", description = "Income & Expense Transaction Management"),
        (name = "Subscriptions", description = "Recurring Monthly/Yearly Expense Management"),
        (name = "Transportation Expenses", description = "Travel Expense History Management (Fuel, Electric Train, Public Transport)")
    ),
    servers(
        (url = "/v1", description = "Core API v1")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
