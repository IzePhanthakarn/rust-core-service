use chrono::{DateTime, Utc};
use diesel::dsl::{count_star, exists};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper, select};
use uuid::Uuid;

use crate::{
    modules::projects::models::{
        Board, BoardColumn, NewBoard, NewBoardColumn, NewProject, NewProjectMember, NewProjectNote,
        NewSprint, NewTask, NewTaskComment, Project, ProjectNote, ProjectStatus, Sprint,
        SprintChangeset, Task, TaskChangeset, TaskComment,
    },
    schema::{
        board_columns, boards, project_members, project_notes, projects, sprints, task_comments,
        tasks, user_profiles, users,
    },
};

pub type MemberRow = (
    Uuid,
    Option<String>,
    Option<String>,
    Option<String>,
    DateTime<Utc>,
);

pub struct ProjectRepository;

impl ProjectRepository {
    // ===== Projects =====

    pub fn find_projects_for_user(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> QueryResult<Vec<Project>> {
        projects::table
            .inner_join(project_members::table.on(project_members::project_id.eq(projects::id)))
            .filter(project_members::user_id.eq(user_id))
            .order(projects::created_at.desc())
            .select(Project::as_select())
            .load::<Project>(conn)
    }

    pub fn find_project(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<Project> {
        projects::table
            .filter(projects::id.eq(project_id))
            .select(Project::as_select())
            .first(conn)
    }

    pub fn insert_project(conn: &mut PgConnection, new: &NewProject<'_>) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(new)
            .returning(Project::as_returning())
            .get_result(conn)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_project(
        conn: &mut PgConnection,
        project_id: Uuid,
        title: &str,
        description: &str,
        status: ProjectStatus,
        start_date: DateTime<Utc>,
        finish_date: Option<DateTime<Utc>>,
    ) -> QueryResult<Project> {
        diesel::update(projects::table.filter(projects::id.eq(project_id)))
            .set((
                projects::title.eq(title),
                projects::description.eq(description),
                projects::status.eq(status),
                projects::start_date.eq(start_date),
                projects::finish_date.eq(finish_date),
                projects::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Project::as_returning())
            .get_result(conn)
    }

    pub fn delete_project(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<usize> {
        diesel::delete(projects::table.filter(projects::id.eq(project_id))).execute(conn)
    }

    // ===== Members =====

    pub fn is_member(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<bool> {
        select(exists(
            project_members::table
                .filter(project_members::project_id.eq(project_id))
                .filter(project_members::user_id.eq(user_id)),
        ))
        .get_result(conn)
    }

    pub fn insert_member(conn: &mut PgConnection, new: &NewProjectMember) -> QueryResult<usize> {
        diesel::insert_into(project_members::table)
            .values(new)
            .execute(conn)
    }

    pub fn delete_member(
        conn: &mut PgConnection,
        project_id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<usize> {
        diesel::delete(
            project_members::table
                .filter(project_members::project_id.eq(project_id))
                .filter(project_members::user_id.eq(user_id)),
        )
        .execute(conn)
    }

    /// นับจำนวนสมาชิกของหลายโปรเจกต์พร้อมกันในคิวรีเดียว (กัน N+1)
    pub fn count_members_by_projects(
        conn: &mut PgConnection,
        project_ids: &[Uuid],
    ) -> QueryResult<Vec<(Uuid, i64)>> {
        if project_ids.is_empty() {
            return Ok(Vec::new());
        }

        project_members::table
            .filter(project_members::project_id.eq_any(project_ids))
            .group_by(project_members::project_id)
            .select((project_members::project_id, count_star()))
            .load::<(Uuid, i64)>(conn)
    }

    pub fn count_members(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<i64> {
        project_members::table
            .filter(project_members::project_id.eq(project_id))
            .select(count_star())
            .get_result(conn)
    }

    pub fn find_members_detailed(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Vec<MemberRow>> {
        project_members::table
            .inner_join(users::table.on(users::id.eq(project_members::user_id)))
            .left_join(user_profiles::table.on(user_profiles::user_id.eq(project_members::user_id)))
            .filter(project_members::project_id.eq(project_id))
            .order(project_members::joined_at.asc())
            .select((
                project_members::user_id,
                users::email,
                user_profiles::first_name.nullable(),
                user_profiles::last_name.nullable(),
                project_members::joined_at,
            ))
            .load::<MemberRow>(conn)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn find_members_paginated(
        conn: &mut PgConnection,
        project_id: Uuid,
        page: i64,
        limit: i64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
    ) -> QueryResult<(Vec<MemberRow>, i64)> {
        let offset = (page - 1) * limit;
        let mut data_query = project_members::table
            .inner_join(users::table.on(users::id.eq(project_members::user_id)))
            .left_join(user_profiles::table.on(user_profiles::user_id.eq(project_members::user_id)))
            .filter(project_members::project_id.eq(project_id))
            .filter(users::deleted_at.is_null())
            .into_boxed();
        let mut count_query = project_members::table
            .inner_join(users::table.on(users::id.eq(project_members::user_id)))
            .left_join(user_profiles::table.on(user_profiles::user_id.eq(project_members::user_id)))
            .filter(project_members::project_id.eq(project_id))
            .filter(users::deleted_at.is_null())
            .into_boxed();

        if let Some(value) = first_name.filter(|value| !value.trim().is_empty()) {
            let pattern = format!("%{}%", value.trim());
            data_query = data_query.filter(user_profiles::first_name.ilike(pattern.clone()));
            count_query = count_query.filter(user_profiles::first_name.ilike(pattern));
        }
        if let Some(value) = last_name.filter(|value| !value.trim().is_empty()) {
            let pattern = format!("%{}%", value.trim());
            data_query = data_query.filter(user_profiles::last_name.ilike(pattern.clone()));
            count_query = count_query.filter(user_profiles::last_name.ilike(pattern));
        }
        if let Some(value) = email.filter(|value| !value.trim().is_empty()) {
            let pattern = format!("%{}%", value.trim());
            data_query = data_query.filter(users::email.ilike(pattern.clone()));
            count_query = count_query.filter(users::email.ilike(pattern));
        }

        let items = data_query
            .order(project_members::joined_at.asc())
            .limit(limit)
            .offset(offset)
            .select((
                project_members::user_id,
                users::email,
                user_profiles::first_name.nullable(),
                user_profiles::last_name.nullable(),
                project_members::joined_at,
            ))
            .load::<MemberRow>(conn)?;
        let total_items = count_query.select(count_star()).first::<i64>(conn)?;

        Ok((items, total_items))
    }

    // ===== Boards & columns =====

    pub fn insert_board(conn: &mut PgConnection, new: &NewBoard<'_>) -> QueryResult<Board> {
        diesel::insert_into(boards::table)
            .values(new)
            .returning(Board::as_returning())
            .get_result(conn)
    }

    pub fn find_board_by_project(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<Board> {
        boards::table
            .filter(boards::project_id.eq(project_id))
            .select(Board::as_select())
            .first(conn)
    }

    pub fn find_boards_by_project(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Vec<Board>> {
        boards::table
            .filter(boards::project_id.eq(project_id))
            .order(boards::name.asc())
            .select(Board::as_select())
            .load::<Board>(conn)
    }

    /// ดึงคอลัมน์ของหลายบอร์ดในคิวรีเดียว (กัน N+1) เรียงตาม order_index
    pub fn find_columns_by_boards(
        conn: &mut PgConnection,
        board_ids: &[Uuid],
    ) -> QueryResult<Vec<BoardColumn>> {
        if board_ids.is_empty() {
            return Ok(Vec::new());
        }

        board_columns::table
            .filter(board_columns::board_id.eq_any(board_ids))
            .order((board_columns::board_id.asc(), board_columns::order_index.asc()))
            .select(BoardColumn::as_select())
            .load::<BoardColumn>(conn)
    }

    pub fn insert_columns(
        conn: &mut PgConnection,
        new: &[NewBoardColumn<'_>],
    ) -> QueryResult<usize> {
        diesel::insert_into(board_columns::table)
            .values(new)
            .execute(conn)
    }

    pub fn find_columns_by_board(
        conn: &mut PgConnection,
        board_id: Uuid,
    ) -> QueryResult<Vec<BoardColumn>> {
        board_columns::table
            .filter(board_columns::board_id.eq(board_id))
            .order(board_columns::order_index.asc())
            .select(BoardColumn::as_select())
            .load::<BoardColumn>(conn)
    }

    pub fn find_first_column(conn: &mut PgConnection, board_id: Uuid) -> QueryResult<BoardColumn> {
        board_columns::table
            .filter(board_columns::board_id.eq(board_id))
            .order(board_columns::order_index.asc())
            .select(BoardColumn::as_select())
            .first(conn)
    }

    pub fn column_belongs_to_board(
        conn: &mut PgConnection,
        column_id: Uuid,
        board_id: Uuid,
    ) -> QueryResult<bool> {
        select(exists(
            board_columns::table
                .filter(board_columns::id.eq(column_id))
                .filter(board_columns::board_id.eq(board_id)),
        ))
        .get_result(conn)
    }

    // ===== Sprints =====

    pub fn insert_sprint(conn: &mut PgConnection, new: &NewSprint<'_>) -> QueryResult<Sprint> {
        diesel::insert_into(sprints::table)
            .values(new)
            .returning(Sprint::as_returning())
            .get_result(conn)
    }

    pub fn find_sprints_by_project(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Vec<Sprint>> {
        sprints::table
            .filter(sprints::project_id.eq(project_id))
            .order(sprints::start_date.asc())
            .select(Sprint::as_select())
            .load::<Sprint>(conn)
    }

    pub fn find_sprint(conn: &mut PgConnection, sprint_id: Uuid) -> QueryResult<Sprint> {
        sprints::table
            .filter(sprints::id.eq(sprint_id))
            .select(Sprint::as_select())
            .first(conn)
    }

    pub fn find_active_sprint(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Option<Sprint>> {
        sprints::table
            .filter(sprints::project_id.eq(project_id))
            .filter(sprints::is_active.eq(true))
            .order(sprints::start_date.asc())
            .select(Sprint::as_select())
            .first(conn)
            .optional()
    }

    pub fn update_sprint(
        conn: &mut PgConnection,
        sprint_id: Uuid,
        changeset: &SprintChangeset,
    ) -> QueryResult<Sprint> {
        diesel::update(sprints::table.filter(sprints::id.eq(sprint_id)))
            .set(changeset)
            .returning(Sprint::as_returning())
            .get_result(conn)
    }

    // ===== Notes =====

    pub fn find_notes_by_project(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Vec<ProjectNote>> {
        project_notes::table
            .filter(project_notes::project_id.eq(project_id))
            .order((project_notes::title.asc(), project_notes::created_at.asc()))
            .select(ProjectNote::as_select())
            .load::<ProjectNote>(conn)
    }

    pub fn find_note(conn: &mut PgConnection, note_id: Uuid) -> QueryResult<ProjectNote> {
        project_notes::table
            .filter(project_notes::id.eq(note_id))
            .select(ProjectNote::as_select())
            .first(conn)
    }

    pub fn insert_note(
        conn: &mut PgConnection,
        new: &NewProjectNote<'_>,
    ) -> QueryResult<ProjectNote> {
        diesel::insert_into(project_notes::table)
            .values(new)
            .returning(ProjectNote::as_returning())
            .get_result(conn)
    }

    pub fn update_note(
        conn: &mut PgConnection,
        note_id: Uuid,
        title: &str,
        content: Option<&str>,
        updated_by: Uuid,
    ) -> QueryResult<ProjectNote> {
        diesel::update(project_notes::table.filter(project_notes::id.eq(note_id)))
            .set((
                project_notes::title.eq(title),
                project_notes::content.eq(content),
                project_notes::updated_by.eq(updated_by),
                project_notes::updated_at.eq(diesel::dsl::now),
            ))
            .returning(ProjectNote::as_returning())
            .get_result(conn)
    }

    pub fn delete_note(conn: &mut PgConnection, note_id: Uuid) -> QueryResult<usize> {
        diesel::delete(project_notes::table.filter(project_notes::id.eq(note_id))).execute(conn)
    }

    // ===== Tasks =====

    pub fn insert_task(conn: &mut PgConnection, new: &NewTask<'_>) -> QueryResult<Task> {
        diesel::insert_into(tasks::table)
            .values(new)
            .returning(Task::as_returning())
            .get_result(conn)
    }

    pub fn find_task(conn: &mut PgConnection, task_id: Uuid) -> QueryResult<Task> {
        tasks::table
            .filter(tasks::id.eq(task_id))
            .select(Task::as_select())
            .first(conn)
    }

    pub fn find_tasks_by_sprint(
        conn: &mut PgConnection,
        sprint_id: Uuid,
    ) -> QueryResult<Vec<Task>> {
        tasks::table
            .filter(tasks::sprint_id.eq(sprint_id))
            .order(tasks::created_at.asc())
            .select(Task::as_select())
            .load::<Task>(conn)
    }

    pub fn find_backlog_tasks(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<Vec<Task>> {
        tasks::table
            .filter(tasks::project_id.eq(project_id))
            .filter(tasks::sprint_id.is_null())
            .order(tasks::created_at.asc())
            .select(Task::as_select())
            .load::<Task>(conn)
    }

    pub fn update_task(
        conn: &mut PgConnection,
        task_id: Uuid,
        changeset: &TaskChangeset,
    ) -> QueryResult<Task> {
        diesel::update(tasks::table.filter(tasks::id.eq(task_id)))
            .set((changeset, tasks::updated_at.eq(diesel::dsl::now)))
            .returning(Task::as_returning())
            .get_result(conn)
    }

    pub fn delete_task(conn: &mut PgConnection, task_id: Uuid) -> QueryResult<usize> {
        diesel::delete(tasks::table.filter(tasks::id.eq(task_id))).execute(conn)
    }

    pub fn sprint_belongs_to_project(
        conn: &mut PgConnection,
        sprint_id: Uuid,
        project_id: Uuid,
    ) -> QueryResult<bool> {
        select(exists(
            sprints::table
                .filter(sprints::id.eq(sprint_id))
                .filter(sprints::project_id.eq(project_id)),
        ))
        .get_result(conn)
    }

    // ===== Comments =====

    pub fn find_comments_by_task(
        conn: &mut PgConnection,
        task_id: Uuid,
    ) -> QueryResult<Vec<TaskComment>> {
        task_comments::table
            .filter(task_comments::task_id.eq(task_id))
            .order(task_comments::created_at.asc())
            .select(TaskComment::as_select())
            .load::<TaskComment>(conn)
    }

    pub fn insert_comment(
        conn: &mut PgConnection,
        new: &NewTaskComment<'_>,
    ) -> QueryResult<TaskComment> {
        diesel::insert_into(task_comments::table)
            .values(new)
            .returning(TaskComment::as_returning())
            .get_result(conn)
    }
}
