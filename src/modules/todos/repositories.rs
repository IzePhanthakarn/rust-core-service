use std::collections::HashMap;

use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, SelectableHelper, dsl::max};
use uuid::Uuid;

use crate::{
    modules::todos::{
        dtos::{TodoItemResponse, TodoListResponse},
        models::{NewTodoItem, NewTodoList, TodoItem, TodoList},
    },
    schema::{todo_items, todo_lists},
};

pub struct TodoRepository;

impl TodoRepository {
    pub fn find_all_lists_with_items(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> QueryResult<Vec<TodoListResponse>> {
        let lists = Self::find_lists_by_user(conn, user_id)?;

        let list_ids: Vec<Uuid> = lists.iter().map(|list| list.id).collect();

        let items = if list_ids.is_empty() {
            Vec::new()
        } else {
            todo_items::table
                .filter(todo_items::list_id.eq_any(&list_ids))
                .order_by((todo_items::position.asc(), todo_items::created_at.asc()))
                .select(TodoItem::as_select())
                .load::<TodoItem>(conn)?
        };

        let mut items_by_list_id: HashMap<Uuid, Vec<TodoItemResponse>> = HashMap::new();
        for item in items {
            items_by_list_id
                .entry(item.list_id)
                .or_default()
                .push(Self::to_item_response(item));
        }

        let result = lists
            .into_iter()
            .map(|list| {
                let items = items_by_list_id.remove(&list.id).unwrap_or_default();
                Self::to_list_response(list, items)
            })
            .collect();

        Ok(result)
    }

    pub fn find_lists_by_user(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> QueryResult<Vec<TodoList>> {
        todo_lists::table
            .filter(todo_lists::user_id.eq(user_id))
            .order_by((todo_lists::position.asc(), todo_lists::created_at.desc()))
            .select(TodoList::as_select())
            .load::<TodoList>(conn)
    }

    pub fn find_one_list(conn: &mut PgConnection, list_id: Uuid) -> QueryResult<TodoList> {
        todo_lists::table
            .filter(todo_lists::id.eq(list_id))
            .first(conn)
    }

    pub fn next_list_position(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<i32> {
        let current_max: Option<i32> = todo_lists::table
            .filter(todo_lists::user_id.eq(user_id))
            .select(max(todo_lists::position))
            .first(conn)?;

        Ok(current_max.map(|position| position + 1).unwrap_or(0))
    }

    pub fn create_list(
        conn: &mut PgConnection,
        list: &NewTodoList<'_>,
        position: i32,
    ) -> QueryResult<TodoList> {
        diesel::insert_into(todo_lists::table)
            .values((
                todo_lists::user_id.eq(list.user_id),
                todo_lists::title.eq(list.title),
                todo_lists::description.eq(list.description),
                todo_lists::color.eq(list.color),
                todo_lists::position.eq(position),
            ))
            .returning(TodoList::as_returning())
            .get_result(conn)
    }

    pub fn update_list_position(
        conn: &mut PgConnection,
        list_id: Uuid,
        position: i32,
    ) -> QueryResult<usize> {
        diesel::update(todo_lists::table.filter(todo_lists::id.eq(list_id)))
            .set((
                todo_lists::position.eq(position),
                todo_lists::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)
    }

    pub fn update_list(
        conn: &mut PgConnection,
        list_id: Uuid,
        list: &NewTodoList<'_>,
    ) -> QueryResult<TodoList> {
        diesel::update(todo_lists::table.filter(todo_lists::id.eq(list_id)))
            .set((
                todo_lists::title.eq(list.title),
                todo_lists::description.eq(list.description),
                todo_lists::color.eq(list.color),
                todo_lists::updated_at.eq(diesel::dsl::now),
            ))
            .returning(TodoList::as_returning())
            .get_result(conn)
    }

    pub fn delete_list(conn: &mut PgConnection, list_id: Uuid) -> QueryResult<usize> {
        diesel::delete(todo_lists::table.filter(todo_lists::id.eq(list_id))).execute(conn)
    }

    pub fn find_one_item(conn: &mut PgConnection, item_id: Uuid) -> QueryResult<TodoItem> {
        todo_items::table
            .filter(todo_items::id.eq(item_id))
            .first(conn)
    }

    pub fn next_item_position(conn: &mut PgConnection, list_id: Uuid) -> QueryResult<i32> {
        let current_max: Option<i32> = todo_items::table
            .filter(todo_items::list_id.eq(list_id))
            .select(max(todo_items::position))
            .first(conn)?;

        Ok(current_max.map(|position| position + 1).unwrap_or(0))
    }

    pub fn create_item(
        conn: &mut PgConnection,
        item: &NewTodoItem<'_>,
        position: i32,
    ) -> QueryResult<TodoItem> {
        diesel::insert_into(todo_items::table)
            .values((
                todo_items::list_id.eq(item.list_id),
                todo_items::title.eq(item.title),
                todo_items::description.eq(item.description),
                todo_items::due_date.eq(item.due_date),
                todo_items::position.eq(position),
            ))
            .returning(TodoItem::as_returning())
            .get_result(conn)
    }

    pub fn find_items_by_list(
        conn: &mut PgConnection,
        list_id: Uuid,
    ) -> QueryResult<Vec<TodoItem>> {
        todo_items::table
            .filter(todo_items::list_id.eq(list_id))
            .order_by((todo_items::position.asc(), todo_items::created_at.asc()))
            .select(TodoItem::as_select())
            .load::<TodoItem>(conn)
    }

    pub fn update_item_position(
        conn: &mut PgConnection,
        item_id: Uuid,
        position: i32,
    ) -> QueryResult<usize> {
        diesel::update(todo_items::table.filter(todo_items::id.eq(item_id)))
            .set((
                todo_items::position.eq(position),
                todo_items::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)
    }

    pub fn set_item_completed(
        conn: &mut PgConnection,
        item_id: Uuid,
        is_completed: bool,
    ) -> QueryResult<TodoItem> {
        diesel::update(todo_items::table.filter(todo_items::id.eq(item_id)))
            .set((
                todo_items::is_completed.eq(is_completed),
                todo_items::updated_at.eq(diesel::dsl::now),
            ))
            .returning(TodoItem::as_returning())
            .get_result(conn)
    }

    pub fn delete_item(conn: &mut PgConnection, item_id: Uuid) -> QueryResult<usize> {
        diesel::delete(todo_items::table.filter(todo_items::id.eq(item_id))).execute(conn)
    }

    pub fn to_item_response(item: TodoItem) -> TodoItemResponse {
        TodoItemResponse {
            id: item.id,
            list_id: item.list_id,
            title: item.title,
            description: item.description,
            is_completed: item.is_completed,
            due_date: item.due_date,
            position: item.position,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }

    pub fn to_list_response(list: TodoList, items: Vec<TodoItemResponse>) -> TodoListResponse {
        TodoListResponse {
            id: list.id,
            user_id: list.user_id,
            title: list.title,
            description: list.description,
            color: list.color,
            position: list.position,
            items,
            created_at: list.created_at,
            updated_at: list.updated_at,
        }
    }
}
