use std::collections::HashMap;

use chrono::Utc;
use diesel::Connection;
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::{errors::AppError, response::{normalize_page_limit, PaginatedData}},
    modules::projects::{
        dtos::{
            BoardColumnResponse, BoardResponse, CommentResponse, CreateCommentRequest,
            CreateNoteRequest, CreateProjectRequest, CreateSprintRequest, CreateTaskRequest,
            KanbanColumnResponse, KanbanResponse, MemberResponse, NoteResponse,
            ProjectMemberFilterQuery,
            ProjectDetailResponse, ProjectResponse, SprintResponse, TaskAssigneeResponse,
            TaskResponse, UpdateNoteRequest, UpdateProjectRequest, UpdateSprintRequest,
            UpdateTaskRequest,
        },
        models::{
            NewBoard, NewBoardColumn, NewProject, NewProjectMember, NewProjectNote, NewSprint,
            NewTask, NewTaskComment, NoteType, Project, ProjectNote, ProjectStatus, Sprint,
            SprintChangeset, Task, TaskChangeset, TaskComment,
        },
        repositories::{MemberRow, ProjectRepository},
    },
    modules::users::repositories::UserRepository,
};

/// คอลัมน์ Kanban เริ่มต้น 4 คอลัมน์ (เรียงซ้าย -> ขวา)
const DEFAULT_COLUMNS: [(&str, i32); 4] = [
    ("Todo", 1),
    ("In progress", 2),
    ("Ready to test", 3),
    ("Done", 4),
];

pub struct ProjectService;

impl ProjectService {
    // ===== Projects =====

    pub fn list_projects(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Vec<ProjectResponse>, AppError> {
        let projects = ProjectRepository::find_projects_for_user(conn, user_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let project_ids: Vec<Uuid> = projects.iter().map(|project| project.id).collect();
        let counts = ProjectRepository::count_members_by_projects(conn, &project_ids)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        let count_by_project: HashMap<Uuid, i64> = counts.into_iter().collect();

        Ok(projects
            .into_iter()
            .map(|project| {
                let member_count = count_by_project.get(&project.id).copied().unwrap_or(0);
                to_project_response(project, member_count)
            })
            .collect())
    }

    /// สร้างโปรเจกต์ใหม่ภายใต้ Transaction เดียว:
    /// projects -> project_members(owner) -> boards -> board_columns(4) -> sprints("Sprint 1")
    /// ถ้าพังจุดใดจุดหนึ่ง Rollback ทั้งหมด
    pub fn create_project(
        conn: &mut PgConnection,
        payload: &CreateProjectRequest,
        user_id: Uuid,
    ) -> Result<ProjectDetailResponse, AppError> {
        conn.transaction::<ProjectDetailResponse, AppError, _>(|conn| {
            let status = payload.status.unwrap_or(ProjectStatus::Planning);

            let new_project = NewProject {
                title: payload.title.trim(),
                description: payload.description.trim(),
                owner_id: user_id,
                status,
                start_date: payload.start_date,
                finish_date: payload.finish_date,
            };
            let project = ProjectRepository::insert_project(conn, &new_project)?;

            // เจ้าของเป็นสมาชิกคนแรก
            ProjectRepository::insert_member(
                conn,
                &NewProjectMember {
                    project_id: project.id,
                    user_id,
                },
            )?;

            // บอร์ดตั้งชื่อตามชื่อโปรเจกต์
            let board = ProjectRepository::insert_board(
                conn,
                &NewBoard {
                    project_id: project.id,
                    name: &project.title,
                },
            )?;

            // 4 คอลัมน์เริ่มต้น
            let new_columns: Vec<NewBoardColumn<'_>> = DEFAULT_COLUMNS
                .iter()
                .map(|&(name, order_index)| NewBoardColumn {
                    board_id: board.id,
                    name,
                    order_index,
                })
                .collect();
            ProjectRepository::insert_columns(conn, &new_columns)?;

            // Sprint 1 (เริ่มวันนี้ ไม่มีวันสิ้นสุด)
            ProjectRepository::insert_sprint(
                conn,
                &NewSprint {
                    project_id: project.id,
                    name: "Sprint 1",
                    goal: None,
                    start_date: Utc::now(),
                    end_date: None,
                    is_active: true,
                },
            )?;

            let members = ProjectRepository::find_members_detailed(conn, project.id)?;
            Ok(to_project_detail_response(project, members))
        })
    }

    pub fn get_project_detail(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<ProjectDetailResponse, AppError> {
        let project = Self::ensure_member(conn, project_id, user_id)?;
        let members = ProjectRepository::find_members_detailed(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(to_project_detail_response(project, members))
    }

    pub fn update_project(
        conn: &mut PgConnection,
        payload: &UpdateProjectRequest,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<ProjectResponse, AppError> {
        Self::ensure_owner(conn, project_id, user_id)?;

        let project = ProjectRepository::update_project(
            conn,
            project_id,
            payload.title.trim(),
            payload.description.trim(),
            payload.status,
            payload.start_date,
            payload.finish_date,
        )?;

        let member_count = ProjectRepository::count_members(conn, project_id)?;
        Ok(to_project_response(project, member_count))
    }

    pub fn delete_project(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::ensure_owner(conn, project_id, user_id)?;
        ProjectRepository::delete_project(conn, project_id)?;
        Ok(())
    }

    // ===== Members =====

    pub fn list_members(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
        filters: ProjectMemberFilterQuery,
    ) -> Result<PaginatedData<MemberResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;
        let (page, limit) = normalize_page_limit(filters.page, filters.limit);
        let (members, total_items) = ProjectRepository::find_members_paginated(
            conn,
            project_id,
            page,
            limit,
            filters.first_name,
            filters.last_name,
            filters.email,
        )
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(PaginatedData::new(
            members.into_iter().map(to_member_response).collect(),
            total_items,
            page,
            limit,
        ))
    }

    pub fn add_member(
        conn: &mut PgConnection,
        email: &str,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<MemberResponse, AppError> {
        Self::ensure_owner(conn, project_id, user_id)?;

        let member = UserRepository::find_by_email(conn, email.trim())
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?
            .ok_or_else(|| AppError::NotFound("ไม่พบผู้ใช้งานจากอีเมลนี้".to_string()))?;

        if member.deleted_at.is_some() {
            return Err(AppError::NotFound("ไม่พบผู้ใช้งานจากอีเมลนี้".to_string()));
        }

        let is_member = ProjectRepository::is_member(conn, project_id, member.id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        if is_member {
            return Err(AppError::BadRequest(
                "ผู้ใช้งานนี้เป็นสมาชิกของโปรเจกต์อยู่แล้ว".to_string(),
            ));
        }

        ProjectRepository::insert_member(
            conn,
            &NewProjectMember {
                project_id,
                user_id: member.id,
            },
        )?;

        let members = ProjectRepository::find_members_detailed(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        members
            .into_iter()
            .find(|(member_id, _, _, _, _)| *member_id == member.id)
            .map(to_member_response)
            .ok_or_else(|| AppError::InternalServerError("ไม่พบสมาชิกที่เพิ่มใหม่".to_string()))
    }

    pub fn remove_member(
        conn: &mut PgConnection,
        project_id: Uuid,
        member_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let project = Self::ensure_owner(conn, project_id, user_id)?;

        if project.owner_id == member_id {
            return Err(AppError::BadRequest(
                "ไม่สามารถลบเจ้าของโปรเจกต์ออกจากสมาชิกได้".to_string(),
            ));
        }

        let deleted = ProjectRepository::delete_member(conn, project_id, member_id)?;
        if deleted == 0 {
            return Err(AppError::NotFound("ไม่พบสมาชิกในโปรเจกต์นี้".to_string()));
        }

        Ok(())
    }

    // ===== Notes =====

    pub fn get_notes_tree(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<NoteResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;
        let notes = ProjectRepository::find_notes_by_project(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(build_note_tree(notes))
    }

    pub fn create_note(
        conn: &mut PgConnection,
        payload: &CreateNoteRequest,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<NoteResponse, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        if let Some(parent_id) = payload.parent_id {
            let parent = ProjectRepository::find_note(conn, parent_id)
                .map_err(|_| AppError::NotFound("ไม่พบโฟลเดอร์ปลายทาง".to_string()))?;

            if parent.project_id != project_id {
                return Err(AppError::BadRequest(
                    "โฟลเดอร์ปลายทางไม่ได้อยู่ในโปรเจกต์นี้".to_string(),
                ));
            }
            if parent.type_ != NoteType::Folder {
                return Err(AppError::BadRequest(
                    "สามารถสร้างโน้ตไว้ภายใต้ folder เท่านั้น".to_string(),
                ));
            }
        }

        // folder ไม่มี content เสมอ
        let content = if payload.type_ == NoteType::Folder {
            None
        } else {
            normalize_optional(payload.content.as_deref())
        };

        let new_note = NewProjectNote {
            project_id,
            parent_id: payload.parent_id,
            type_: payload.type_,
            title: payload.title.trim(),
            content,
            created_by: user_id,
            updated_by: user_id,
        };

        let note = ProjectRepository::insert_note(conn, &new_note)?;
        Ok(to_note_response(note))
    }

    pub fn update_note(
        conn: &mut PgConnection,
        payload: &UpdateNoteRequest,
        project_id: Uuid,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<NoteResponse, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let note = ProjectRepository::find_note(conn, note_id)
            .map_err(|_| AppError::NotFound("ไม่พบโน้ตนี้".to_string()))?;
        if note.project_id != project_id {
            return Err(AppError::NotFound("ไม่พบโน้ตนี้ในโปรเจกต์".to_string()));
        }

        let content = if note.type_ == NoteType::Folder {
            None
        } else {
            normalize_optional(payload.content.as_deref())
        };

        let updated =
            ProjectRepository::update_note(conn, note_id, payload.title.trim(), content, user_id)?;
        Ok(to_note_response(updated))
    }

    pub fn delete_note(
        conn: &mut PgConnection,
        project_id: Uuid,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let note = ProjectRepository::find_note(conn, note_id)
            .map_err(|_| AppError::NotFound("ไม่พบโน้ตนี้".to_string()))?;
        if note.project_id != project_id {
            return Err(AppError::NotFound("ไม่พบโน้ตนี้ในโปรเจกต์".to_string()));
        }

        ProjectRepository::delete_note(conn, note_id)?;
        Ok(())
    }

    // ===== Kanban & Backlogs =====

    pub fn get_kanban(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<KanbanResponse, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let board = ProjectRepository::find_board_by_project(conn, project_id)
            .map_err(|_| AppError::NotFound("ไม่พบบอร์ดของโปรเจกต์นี้".to_string()))?;
        let columns = ProjectRepository::find_columns_by_board(conn, board.id)?;
        let active_sprint = ProjectRepository::find_active_sprint(conn, project_id)?;

        let tasks = match &active_sprint {
            Some(sprint) => ProjectRepository::find_tasks_by_sprint(conn, sprint.id)?,
            None => Vec::new(),
        };
        let assignees = get_task_assignees(conn, project_id)?;

        let mut tasks_by_column: HashMap<Uuid, Vec<TaskResponse>> = HashMap::new();
        for task in tasks {
            tasks_by_column
                .entry(task.column_id)
                .or_default()
                .push(to_task_response(task, &assignees));
        }

        let columns = columns
            .into_iter()
            .map(|column| KanbanColumnResponse {
                tasks: tasks_by_column.remove(&column.id).unwrap_or_default(),
                id: column.id,
                name: column.name,
                order_index: column.order_index,
            })
            .collect();

        Ok(KanbanResponse {
            board_id: board.id,
            board_name: board.name,
            active_sprint: active_sprint.map(to_sprint_response),
            columns,
        })
    }

    pub fn get_boards(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<BoardResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let boards = ProjectRepository::find_boards_by_project(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        let board_ids: Vec<Uuid> = boards.iter().map(|board| board.id).collect();
        let columns = ProjectRepository::find_columns_by_boards(conn, &board_ids)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        // columns ถูกเรียงตาม (board_id, order_index) จาก DB แล้ว -> push ตามลำดับได้เลย
        let mut columns_by_board: HashMap<Uuid, Vec<BoardColumnResponse>> = HashMap::new();
        for column in columns {
            columns_by_board
                .entry(column.board_id)
                .or_default()
                .push(BoardColumnResponse {
                    id: column.id,
                    name: column.name,
                    order_index: column.order_index,
                });
        }

        let boards = boards
            .into_iter()
            .map(|board| BoardResponse {
                columns: columns_by_board.remove(&board.id).unwrap_or_default(),
                id: board.id,
                project_id: board.project_id,
                name: board.name,
            })
            .collect();

        Ok(boards)
    }

    pub fn get_backlogs(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<TaskResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;
        let tasks = ProjectRepository::find_backlog_tasks(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        let assignees = get_task_assignees(conn, project_id)?;

        Ok(tasks
            .into_iter()
            .map(|task| to_task_response(task, &assignees))
            .collect())
    }

    // ===== Sprints =====

    pub fn list_sprints(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<SprintResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;
        let sprints = ProjectRepository::find_sprints_by_project(conn, project_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(sprints.into_iter().map(to_sprint_response).collect())
    }

    pub fn get_sprint_tasks(
        conn: &mut PgConnection,
        project_id: Uuid,
        sprint_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<TaskResponse>, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let sprint = ProjectRepository::find_sprint(conn, sprint_id)
            .map_err(|_| AppError::NotFound("ไม่พบสปรินต์นี้".to_string()))?;
        if sprint.project_id != project_id {
            return Err(AppError::NotFound("ไม่พบสปรินต์นี้ในโปรเจกต์".to_string()));
        }

        let tasks = ProjectRepository::find_tasks_by_sprint(conn, sprint_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        let assignees = get_task_assignees(conn, project_id)?;

        Ok(tasks
            .into_iter()
            .map(|task| to_task_response(task, &assignees))
            .collect())
    }

    pub fn create_sprint(
        conn: &mut PgConnection,
        payload: &CreateSprintRequest,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<SprintResponse, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;
        let has_active_sprint = ProjectRepository::find_active_sprint(conn, project_id)?.is_some();

        let new_sprint = NewSprint {
            project_id,
            name: payload.name.trim(),
            goal: normalize_optional(payload.goal.as_deref()),
            start_date: payload.start_date,
            end_date: payload.end_date,
            is_active: !has_active_sprint,
        };

        let sprint = ProjectRepository::insert_sprint(conn, &new_sprint)?;
        Ok(to_sprint_response(sprint))
    }

    pub fn update_sprint(
        conn: &mut PgConnection,
        payload: &UpdateSprintRequest,
        project_id: Uuid,
        sprint_id: Uuid,
        user_id: Uuid,
    ) -> Result<SprintResponse, AppError> {
        Self::ensure_member(conn, project_id, user_id)?;

        let sprint = ProjectRepository::find_sprint(conn, sprint_id)
            .map_err(|_| AppError::NotFound("ไม่พบสปรินต์นี้".to_string()))?;
        if sprint.project_id != project_id {
            return Err(AppError::NotFound("ไม่พบสปรินต์นี้ในโปรเจกต์".to_string()));
        }

        if payload.is_active == Some(true)
            && let Some(active_sprint) = ProjectRepository::find_active_sprint(conn, project_id)?
            && active_sprint.id != sprint_id
        {
            return Err(AppError::BadRequest(
                "ไม่สามารถเปิดสปรินต์ได้ เพราะมีสปรินต์อื่นกำลังทำงานอยู่".to_string(),
            ));
        }

        let changeset = SprintChangeset {
            name: payload.name.as_ref().map(|name| name.trim().to_string()),
            goal: payload
                .goal
                .as_ref()
                .map(|goal| goal.as_ref().map(|text| text.trim().to_string())),
            start_date: payload.start_date,
            end_date: payload.end_date,
            is_active: payload.is_active,
        };

        let updated = ProjectRepository::update_sprint(conn, sprint_id, &changeset)?;
        Ok(to_sprint_response(updated))
    }

    pub fn delete_sprint(
        conn: &mut PgConnection,
        project_id: Uuid,
        sprint_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::ensure_owner(conn, project_id, user_id)?;

        let sprint = ProjectRepository::find_sprint(conn, sprint_id)
            .map_err(|_| AppError::NotFound("ไม่พบสปรินต์นี้".to_string()))?;
        if sprint.project_id != project_id {
            return Err(AppError::NotFound("ไม่พบสปรินต์นี้ในโปรเจกต์".to_string()));
        }

        ProjectRepository::delete_sprint(conn, sprint_id)?;
        Ok(())
    }

    // ===== Tasks =====

    pub fn create_task(
        conn: &mut PgConnection,
        payload: &CreateTaskRequest,
        user_id: Uuid,
    ) -> Result<TaskResponse, AppError> {
        let project = Self::ensure_member(conn, payload.project_id, user_id)?;
        let board = ProjectRepository::find_board_by_project(conn, project.id)
            .map_err(|_| AppError::NotFound("ไม่พบบอร์ดของโปรเจกต์นี้".to_string()))?;

        // เลือกคอลัมน์: ถ้าไม่ส่งมา ใช้คอลัมน์แรกสุด
        let column_id = match payload.column_id {
            Some(column_id) => {
                if !ProjectRepository::column_belongs_to_board(conn, column_id, board.id)? {
                    return Err(AppError::BadRequest(
                        "column_id ไม่ได้อยู่ในบอร์ดของโปรเจกต์นี้".to_string(),
                    ));
                }
                column_id
            }
            None => {
                ProjectRepository::find_first_column(conn, board.id)
                    .map_err(|_| AppError::NotFound("ไม่พบคอลัมน์ในบอร์ดนี้".to_string()))?
                    .id
            }
        };

        if let Some(sprint_id) = payload.sprint_id
            && !ProjectRepository::sprint_belongs_to_project(conn, sprint_id, project.id)?
        {
            return Err(AppError::BadRequest(
                "sprint_id ไม่ได้อยู่ในโปรเจกต์นี้".to_string(),
            ));
        }

        let new_task = NewTask {
            project_id: project.id,
            board_id: board.id,
            sprint_id: payload.sprint_id,
            column_id,
            title: payload.title.trim(),
            description: payload.description.trim(),
            type_: payload.type_,
            priority: payload.priority,
            story_points: payload.story_points,
            assignee_id: payload.assignee_id,
            reporter_id: user_id,
            tag: normalize_optional(payload.tag.as_deref()),
        };

        let task = ProjectRepository::insert_task(conn, &new_task)?;
        let assignees = get_task_assignees(conn, project.id)?;
        Ok(to_task_response(task, &assignees))
    }

    pub fn update_task(
        conn: &mut PgConnection,
        payload: &UpdateTaskRequest,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<TaskResponse, AppError> {
        let task = ProjectRepository::find_task(conn, task_id)
            .map_err(|_| AppError::NotFound("ไม่พบ Task นี้".to_string()))?;
        Self::ensure_member(conn, task.project_id, user_id)?;

        if let Some(column_id) = payload.column_id
            && !ProjectRepository::column_belongs_to_board(conn, column_id, task.board_id)?
        {
            return Err(AppError::BadRequest(
                "column_id ไม่ได้อยู่ในบอร์ดของ Task นี้".to_string(),
            ));
        }

        if let Some(Some(sprint_id)) = payload.sprint_id
            && !ProjectRepository::sprint_belongs_to_project(conn, sprint_id, task.project_id)?
        {
            return Err(AppError::BadRequest(
                "sprint_id ไม่ได้อยู่ในโปรเจกต์ของ Task นี้".to_string(),
            ));
        }

        let changeset = TaskChangeset {
            title: payload.title.as_ref().map(|title| title.trim().to_string()),
            description: payload
                .description
                .as_ref()
                .map(|description| description.trim().to_string()),
            column_id: payload.column_id,
            type_: payload.type_,
            priority: payload.priority,
            story_points: payload.story_points,
            sprint_id: payload.sprint_id,
            assignee_id: payload.assignee_id,
            tag: payload
                .tag
                .as_ref()
                .map(|tag| tag.as_ref().map(|text| text.trim().to_string())),
        };

        let updated = ProjectRepository::update_task(conn, task_id, &changeset)?;
        let assignees = get_task_assignees(conn, updated.project_id)?;
        Ok(to_task_response(updated, &assignees))
    }

    pub fn delete_task(
        conn: &mut PgConnection,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let task = ProjectRepository::find_task(conn, task_id)
            .map_err(|_| AppError::NotFound("ไม่พบ Task นี้".to_string()))?;
        Self::ensure_member(conn, task.project_id, user_id)?;

        ProjectRepository::delete_task(conn, task_id)?;
        Ok(())
    }

    // ===== Comments =====

    pub fn list_comments(
        conn: &mut PgConnection,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<CommentResponse>, AppError> {
        let task = ProjectRepository::find_task(conn, task_id)
            .map_err(|_| AppError::NotFound("ไม่พบ Task นี้".to_string()))?;
        Self::ensure_member(conn, task.project_id, user_id)?;

        let comments = ProjectRepository::find_comments_by_task(conn, task_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

        Ok(comments.into_iter().map(to_comment_response).collect())
    }

    pub fn create_comment(
        conn: &mut PgConnection,
        payload: &CreateCommentRequest,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<CommentResponse, AppError> {
        let task = ProjectRepository::find_task(conn, task_id)
            .map_err(|_| AppError::NotFound("ไม่พบ Task นี้".to_string()))?;
        Self::ensure_member(conn, task.project_id, user_id)?;

        let new_comment = NewTaskComment {
            task_id,
            user_id,
            content: payload.content.trim(),
        };

        let comment = ProjectRepository::insert_comment(conn, &new_comment)?;
        Ok(to_comment_response(comment))
    }

    // ===== Access helpers =====

    /// ผู้ใช้ต้องเป็นสมาชิกของโปรเจกต์ (คืนค่า Project เมื่อผ่าน)
    fn ensure_member(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Project, AppError> {
        let project = ProjectRepository::find_project(conn, project_id)
            .map_err(|_| AppError::NotFound("ไม่พบโปรเจกต์นี้".to_string()))?;

        let is_member = ProjectRepository::is_member(conn, project_id, user_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;
        if !is_member {
            return Err(AppError::Forbidden("คุณไม่ใช่สมาชิกของโปรเจกต์นี้".to_string()));
        }

        Ok(project)
    }

    /// ผู้ใช้ต้องเป็นเจ้าของโปรเจกต์ (คืนค่า Project เมื่อผ่าน)
    fn ensure_owner(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Project, AppError> {
        let project = ProjectRepository::find_project(conn, project_id)
            .map_err(|_| AppError::NotFound("ไม่พบโปรเจกต์นี้".to_string()))?;

        if project.owner_id != user_id {
            return Err(AppError::Forbidden(
                "เฉพาะเจ้าของโปรเจกต์เท่านั้นที่ทำรายการนี้ได้".to_string(),
            ));
        }

        Ok(project)
    }
}

// ===== Mappers =====

fn to_project_response(project: Project, member_count: i64) -> ProjectResponse {
    ProjectResponse {
        id: project.id,
        title: project.title,
        description: project.description,
        owner_id: project.owner_id,
        status: project.status,
        start_date: project.start_date,
        finish_date: project.finish_date,
        member_count,
        created_at: project.created_at,
        updated_at: project.updated_at,
    }
}

fn to_project_detail_response(project: Project, members: Vec<MemberRow>) -> ProjectDetailResponse {
    ProjectDetailResponse {
        id: project.id,
        title: project.title,
        description: project.description,
        owner_id: project.owner_id,
        status: project.status,
        start_date: project.start_date,
        finish_date: project.finish_date,
        created_at: project.created_at,
        updated_at: project.updated_at,
        members: members.into_iter().map(to_member_response).collect(),
    }
}

fn to_member_response(row: MemberRow) -> MemberResponse {
    let (user_id, email, first_name, last_name, joined_at) = row;
    MemberResponse {
        user_id,
        email,
        first_name,
        last_name,
        joined_at,
    }
}

fn to_sprint_response(sprint: Sprint) -> SprintResponse {
    SprintResponse {
        id: sprint.id,
        project_id: sprint.project_id,
        name: sprint.name,
        goal: sprint.goal,
        start_date: sprint.start_date,
        end_date: sprint.end_date,
        is_active: sprint.is_active,
    }
}

fn get_task_assignees(
    conn: &mut PgConnection,
    project_id: Uuid,
) -> Result<HashMap<Uuid, TaskAssigneeResponse>, AppError> {
    let members = ProjectRepository::find_members_detailed(conn, project_id)
        .map_err(|_| AppError::InternalServerError("Query Error".to_string()))?;

    Ok(members
        .into_iter()
        .map(|(user_id, _, first_name, last_name, _)| {
            (
                user_id,
                TaskAssigneeResponse {
                    user_id,
                    first_name,
                    last_name,
                },
            )
        })
        .collect())
}

fn to_task_response(
    task: Task,
    assignees: &HashMap<Uuid, TaskAssigneeResponse>,
) -> TaskResponse {
    TaskResponse {
        id: task.id,
        project_id: task.project_id,
        board_id: task.board_id,
        sprint_id: task.sprint_id,
        column_id: task.column_id,
        title: task.title,
        description: task.description,
        type_: task.type_,
        priority: task.priority,
        story_points: task.story_points,
        assignee_id: task.assignee_id,
        assignee: task
            .assignee_id
            .and_then(|assignee_id| assignees.get(&assignee_id).cloned()),
        reporter_id: task.reporter_id,
        tag: task.tag,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }
}

fn to_comment_response(comment: TaskComment) -> CommentResponse {
    CommentResponse {
        id: comment.id,
        task_id: comment.task_id,
        user_id: comment.user_id,
        content: comment.content,
        created_at: comment.created_at,
        updated_at: comment.updated_at,
    }
}

fn to_note_response_flat(note: ProjectNote, children: Vec<NoteResponse>) -> NoteResponse {
    NoteResponse {
        id: note.id,
        project_id: note.project_id,
        parent_id: note.parent_id,
        type_: note.type_,
        title: note.title,
        content: note.content,
        created_by: note.created_by,
        updated_by: note.updated_by,
        created_at: note.created_at,
        updated_at: note.updated_at,
        children,
    }
}

fn to_note_response(note: ProjectNote) -> NoteResponse {
    to_note_response_flat(note, Vec::new())
}

/// ประกอบ Vec<ProjectNote> แบบแบนให้เป็นโครงสร้างต้นไม้ (folder ซ้อน folder)
fn build_note_tree(notes: Vec<ProjectNote>) -> Vec<NoteResponse> {
    let mut children_map: HashMap<Option<Uuid>, Vec<ProjectNote>> = HashMap::new();
    for note in notes {
        children_map.entry(note.parent_id).or_default().push(note);
    }

    build_children(None, &mut children_map)
}

fn build_children(
    parent_id: Option<Uuid>,
    children_map: &mut HashMap<Option<Uuid>, Vec<ProjectNote>>,
) -> Vec<NoteResponse> {
    let nodes = children_map.remove(&parent_id).unwrap_or_default();
    nodes
        .into_iter()
        .map(|note| {
            let children = build_children(Some(note.id), children_map);
            to_note_response_flat(note, children)
        })
        .collect()
}

fn normalize_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}
