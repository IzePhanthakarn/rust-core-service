use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    core::{errors::AppError, extractors::ValidatedJson, jwt::Claims, response::{ApiResponse, PaginatedData}},
    modules::projects::{
        dtos::{
            AddProjectMemberRequest, BoardResponse, CommentResponse, CreateCommentRequest,
            CreateNoteRequest, CreateProjectRequest, CreateSprintRequest, CreateTaskRequest,
            KanbanResponse, MemberResponse, NoteResponse, ProjectDetailResponse, ProjectResponse,
            ProjectMemberFilterQuery, SprintResponse, TaskResponse, UpdateNoteRequest, UpdateProjectRequest,
            UpdateSprintRequest, UpdateTaskRequest,
        },
        services::ProjectService,
    },
};

// ===== Projects =====

#[utoipa::path(
    get,
    path = "/projects",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Projects found successfully", body = ApiResponse<Vec<ProjectResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_projects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<ProjectResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::list_projects(&mut conn, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Projects retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/projects",
    tag = "Projects",
    request_body = CreateProjectRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Project created successfully", body = ApiResponse<ProjectDetailResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ProjectDetailResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::create_project(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Project created successfully", result)),
    ))
}

#[utoipa::path(
    get,
    path = "/projects/{id}",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Project detail found successfully", body = ApiResponse<ProjectDetailResponse>),
        (status = 404, description = "Project not found")
    )
)]
pub async fn get_project_detail(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProjectDetailResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_project_detail(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(
        200,
        "Project details retrieved successfully",
        data,
    )))
}

#[utoipa::path(
    put,
    path = "/projects/{id}",
    tag = "Projects",
    request_body = UpdateProjectRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Project updated successfully", body = ApiResponse<ProjectResponse>),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateProjectRequest>,
) -> Result<Json<ApiResponse<ProjectResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::update_project(&mut conn, &payload, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Project updated successfully", result)))
}

#[utoipa::path(
    delete,
    path = "/projects/{id}",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Project deleted successfully"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut conn = state.get_conn()?;
    ProjectService::delete_project(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "Project deleted successfully",
    )))
}

// ===== Members =====

#[utoipa::path(
    get,
    path = "/projects/{id}/members",
    tag = "Projects",
    params(ProjectMemberFilterQuery),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Project members found successfully", body = ApiResponse<PaginatedData<MemberResponse>>),
        (status = 403, description = "Only project members can view members")
    )
)]
pub async fn list_members(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Query(filters): Query<ProjectMemberFilterQuery>,
) -> Result<Json<ApiResponse<PaginatedData<MemberResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::list_members(&mut conn, id, claims.sub, filters)?;

    Ok(Json(ApiResponse::success(200, "Project members retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/projects/{id}/members",
    tag = "Projects",
    request_body = AddProjectMemberRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Member added successfully", body = ApiResponse<MemberResponse>),
        (status = 403, description = "Only the project owner can add members"),
        (status = 404, description = "User not found")
    )
)]
pub async fn add_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<AddProjectMemberRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MemberResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::add_member(&mut conn, &payload.email, id, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Project member added successfully", result)),
    ))
}

#[utoipa::path(
    delete,
    path = "/projects/{id}/members/{member_id}",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Member removed successfully"),
        (status = 403, description = "Only the project owner can remove members"),
        (status = 404, description = "Member not found")
    )
)]
pub async fn remove_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut conn = state.get_conn()?;
    ProjectService::remove_member(&mut conn, id, member_id, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "Member removed from project successfully",
    )))
}

// ===== Notes =====

#[utoipa::path(
    get,
    path = "/projects/{id}/notes",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Notes tree found successfully", body = ApiResponse<Vec<NoteResponse>>)
    )
)]
pub async fn get_notes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<NoteResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_notes_tree(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Notes structure retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/projects/{id}/notes",
    tag = "Projects",
    request_body = CreateNoteRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Note created successfully", body = ApiResponse<NoteResponse>)
    )
)]
pub async fn create_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateNoteRequest>,
) -> Result<(StatusCode, Json<ApiResponse<NoteResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::create_note(&mut conn, &payload, id, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Note created successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/projects/{id}/notes/{note_id}",
    tag = "Projects",
    request_body = UpdateNoteRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Note updated successfully", body = ApiResponse<NoteResponse>)
    )
)]
pub async fn update_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, note_id)): Path<(Uuid, Uuid)>,
    ValidatedJson(payload): ValidatedJson<UpdateNoteRequest>,
) -> Result<Json<ApiResponse<NoteResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::update_note(&mut conn, &payload, id, note_id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Note updated successfully", result)))
}

#[utoipa::path(
    delete,
    path = "/projects/{id}/notes/{note_id}",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Note deleted successfully")
    )
)]
pub async fn delete_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, note_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut conn = state.get_conn()?;
    ProjectService::delete_note(&mut conn, id, note_id, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(200, "Note deleted successfully")))
}

// ===== Boards, Kanban & Backlogs =====

#[utoipa::path(
    get,
    path = "/projects/{id}/boards",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Boards with columns found successfully", body = ApiResponse<Vec<BoardResponse>>)
    )
)]
pub async fn get_boards(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<BoardResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_boards(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Board data retrieved successfully", data)))
}

#[utoipa::path(
    get,
    path = "/projects/{id}/kanban",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Kanban board found successfully", body = ApiResponse<KanbanResponse>)
    )
)]
pub async fn get_kanban(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<KanbanResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_kanban(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Kanban data retrieved successfully", data)))
}

#[utoipa::path(
    get,
    path = "/projects/{id}/backlogs",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Backlog tasks found successfully", body = ApiResponse<Vec<TaskResponse>>)
    )
)]
pub async fn get_backlogs(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TaskResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_backlogs(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(
        200,
        "Backlogs data retrieved successfully",
        data,
    )))
}

// ===== Sprints =====

#[utoipa::path(
    get,
    path = "/projects/{id}/sprints",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Sprints found successfully", body = ApiResponse<Vec<SprintResponse>>)
    )
)]
pub async fn list_sprints(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<SprintResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::list_sprints(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Sprints retrieved successfully", data)))
}

#[utoipa::path(
    get,
    path = "/projects/{id}/sprints/{sprint_id}/tasks",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Sprint tasks found successfully", body = ApiResponse<Vec<TaskResponse>>)
    )
)]
pub async fn get_sprint_tasks(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, sprint_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<Vec<TaskResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::get_sprint_tasks(&mut conn, id, sprint_id, claims.sub)?;

    Ok(Json(ApiResponse::success(
        200,
        "Sprint tasks retrieved successfully",
        data,
    )))
}

#[utoipa::path(
    post,
    path = "/projects/{id}/sprints",
    tag = "Projects",
    request_body = CreateSprintRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Sprint created successfully", body = ApiResponse<SprintResponse>)
    )
)]
pub async fn create_sprint(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateSprintRequest>,
) -> Result<(StatusCode, Json<ApiResponse<SprintResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::create_sprint(&mut conn, &payload, id, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Sprint created successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/projects/{id}/sprints/{sprint_id}",
    tag = "Projects",
    request_body = UpdateSprintRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Sprint updated successfully", body = ApiResponse<SprintResponse>)
    )
)]
pub async fn update_sprint(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, sprint_id)): Path<(Uuid, Uuid)>,
    ValidatedJson(payload): ValidatedJson<UpdateSprintRequest>,
) -> Result<Json<ApiResponse<SprintResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::update_sprint(&mut conn, &payload, id, sprint_id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Sprint updated successfully", result)))
}

#[utoipa::path(
    delete,
    path = "/projects/{id}/sprints/{sprint_id}",
    tag = "Projects",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Sprint deleted successfully")
    )
)]
pub async fn delete_sprint(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, sprint_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut conn = state.get_conn()?;
    ProjectService::delete_sprint(&mut conn, id, sprint_id, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(200, "Sprint deleted successfully")))
}

// ===== Tasks =====

#[utoipa::path(
    post,
    path = "/tasks",
    tag = "Tasks",
    request_body = CreateTaskRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Task created successfully", body = ApiResponse<TaskResponse>)
    )
)]
pub async fn create_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(payload): ValidatedJson<CreateTaskRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TaskResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::create_task(&mut conn, &payload, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Task created successfully", result)),
    ))
}

#[utoipa::path(
    put,
    path = "/tasks/{id}",
    tag = "Tasks",
    request_body = UpdateTaskRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Task updated successfully", body = ApiResponse<TaskResponse>)
    )
)]
pub async fn update_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateTaskRequest>,
) -> Result<Json<ApiResponse<TaskResponse>>, AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::update_task(&mut conn, &payload, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Task updated successfully", result)))
}

#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    tag = "Tasks",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Task deleted successfully")
    )
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let mut conn = state.get_conn()?;
    ProjectService::delete_task(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success_without_data(
        200,
        "Task deleted successfully",
    )))
}

// ===== Comments =====

#[utoipa::path(
    get,
    path = "/tasks/{id}/comments",
    tag = "Tasks",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Comments found successfully", body = ApiResponse<Vec<CommentResponse>>)
    )
)]
pub async fn list_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<CommentResponse>>>, AppError> {
    let mut conn = state.get_conn()?;
    let data = ProjectService::list_comments(&mut conn, id, claims.sub)?;

    Ok(Json(ApiResponse::success(200, "Comments retrieved successfully", data)))
}

#[utoipa::path(
    post,
    path = "/tasks/{id}/comments",
    tag = "Tasks",
    request_body = CreateCommentRequest,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Comment created successfully", body = ApiResponse<CommentResponse>)
    )
)]
pub async fn create_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateCommentRequest>,
) -> Result<(StatusCode, Json<ApiResponse<CommentResponse>>), AppError> {
    let mut conn = state.get_conn()?;
    let result = ProjectService::create_comment(&mut conn, &payload, id, claims.sub)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(201, "Comment added successfully", result)),
    ))
}
