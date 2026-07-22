use std::collections::HashSet;

use diesel::Connection;
use diesel::PgConnection;
use uuid::Uuid;

use crate::{
    core::errors::AppError,
    modules::todos::{
        dtos::{
            CreateTodoItemRequest, CreateTodoListRequest, ReorderTodoItemsRequest,
            TodoItemResponse, TodoListResponse, UpdateTodoListRequest,
        },
        models::{NewTodoItem, NewTodoList, TodoItem, TodoList},
        repositories::TodoRepository,
    },
};

pub struct TodoService;

impl TodoService {
    pub fn get_all_todo_lists(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Vec<TodoListResponse>, AppError> {
        TodoRepository::find_all_lists_with_items(conn, user_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))
    }

    pub fn create_todo_list(
        conn: &mut PgConnection,
        payload: &CreateTodoListRequest,
        user_id: Uuid,
    ) -> Result<TodoListResponse, AppError> {
        let new_list = NewTodoList {
            user_id,
            title: payload.title.trim(),
            description: normalize_optional(payload.description.as_deref()),
            color: normalize_optional(payload.color.as_deref()),
        };

        let position = TodoRepository::next_list_position(conn, user_id)?;
        let saved_list = TodoRepository::create_list(conn, &new_list, position)?;

        Ok(TodoRepository::to_list_response(saved_list, Vec::new()))
    }

    /// Moves the selected list to the top (position 0) and shifts the other lists down, preserving their original order.
    pub fn move_todo_list_to_top(
        conn: &mut PgConnection,
        list_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<TodoListResponse>, AppError> {
        Self::find_owned_list(conn, list_id, user_id)?;

        conn.transaction::<(), AppError, _>(|conn| {
            // Sort the current lists by position, then move the target list to the front
            let mut ordered_ids: Vec<Uuid> = TodoRepository::find_lists_by_user(conn, user_id)?
                .into_iter()
                .map(|list| list.id)
                .filter(|id| *id != list_id)
                .collect();
            ordered_ids.insert(0, list_id);

            for (position, id) in ordered_ids.iter().enumerate() {
                TodoRepository::update_list_position(conn, *id, position as i32)?;
            }

            Ok(())
        })?;

        TodoRepository::find_all_lists_with_items(conn, user_id)
            .map_err(|_| AppError::InternalServerError("Query Error".to_string()))
    }

    pub fn update_todo_list(
        conn: &mut PgConnection,
        payload: &UpdateTodoListRequest,
        user_id: Uuid,
        list_id: Uuid,
    ) -> Result<TodoListResponse, AppError> {
        Self::find_owned_list(conn, list_id, user_id)?;

        let new_list = NewTodoList {
            user_id,
            title: payload.title.trim(),
            description: normalize_optional(payload.description.as_deref()),
            color: normalize_optional(payload.color.as_deref()),
        };

        let saved_list = TodoRepository::update_list(conn, list_id, &new_list)?;

        let items = TodoRepository::find_all_lists_with_items(conn, user_id)?
            .into_iter()
            .find(|list| list.id == saved_list.id)
            .map(|list| list.items)
            .unwrap_or_default();

        Ok(TodoRepository::to_list_response(saved_list, items))
    }

    pub fn delete_todo_list(
        conn: &mut PgConnection,
        list_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::find_owned_list(conn, list_id, user_id)?;
        TodoRepository::delete_list(conn, list_id)?;
        Ok(())
    }

    pub fn create_todo_item(
        conn: &mut PgConnection,
        payload: &CreateTodoItemRequest,
        user_id: Uuid,
        list_id: Uuid,
    ) -> Result<TodoItemResponse, AppError> {
        Self::find_owned_list(conn, list_id, user_id)?;

        let new_item = NewTodoItem {
            list_id,
            title: payload.title.trim(),
            description: normalize_optional(payload.description.as_deref()),
            due_date: payload.due_date,
        };

        let position = TodoRepository::next_item_position(conn, list_id)?;
        let saved_item = TodoRepository::create_item(conn, &new_item, position)?;

        Ok(TodoRepository::to_item_response(saved_item))
    }

    pub fn reorder_todo_items(
        conn: &mut PgConnection,
        payload: &ReorderTodoItemsRequest,
        user_id: Uuid,
        list_id: Uuid,
    ) -> Result<Vec<TodoItemResponse>, AppError> {
        Self::find_owned_list(conn, list_id, user_id)?;

        // Prevent duplicate ids in the payload
        let unique_ids: HashSet<Uuid> = payload.item_ids.iter().copied().collect();
        if unique_ids.len() != payload.item_ids.len() {
            return Err(AppError::BadRequest("The request contains duplicate item ids.".to_string()));
        }

        let existing_items = TodoRepository::find_items_by_list(conn, list_id)?;
        let existing_ids: HashSet<Uuid> = existing_items.iter().map(|item| item.id).collect();

        // The payload must reference exactly all items in this list, no more and no fewer
        if unique_ids != existing_ids {
            return Err(AppError::BadRequest(
                "The submitted item list does not match all items in this list.".to_string(),
            ));
        }

        conn.transaction::<Vec<TodoItemResponse>, AppError, _>(|conn| {
            for (position, item_id) in payload.item_ids.iter().enumerate() {
                TodoRepository::update_item_position(conn, *item_id, position as i32)?;
            }

            let items = TodoRepository::find_items_by_list(conn, list_id)?
                .into_iter()
                .map(TodoRepository::to_item_response)
                .collect();

            Ok(items)
        })
    }

    pub fn toggle_todo_item(
        conn: &mut PgConnection,
        item_id: Uuid,
        user_id: Uuid,
    ) -> Result<TodoItemResponse, AppError> {
        let item = Self::find_owned_item(conn, item_id, user_id)?;

        let updated_item =
            TodoRepository::set_item_completed(conn, item_id, !item.is_completed)?;

        Ok(TodoRepository::to_item_response(updated_item))
    }

    pub fn delete_todo_item(
        conn: &mut PgConnection,
        item_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        Self::find_owned_item(conn, item_id, user_id)?;
        TodoRepository::delete_item(conn, item_id)?;
        Ok(())
    }

    fn find_owned_list(
        conn: &mut PgConnection,
        list_id: Uuid,
        user_id: Uuid,
    ) -> Result<TodoList, AppError> {
        let list = TodoRepository::find_one_list(conn, list_id)
            .map_err(|_| AppError::NotFound("Todo list not found".to_string()))?;

        if list.user_id != user_id {
            return Err(AppError::Forbidden("You do not have permission to access this Todo list.".to_string()));
        }

        Ok(list)
    }

    fn find_owned_item(
        conn: &mut PgConnection,
        item_id: Uuid,
        user_id: Uuid,
    ) -> Result<TodoItem, AppError> {
        let item = TodoRepository::find_one_item(conn, item_id)
            .map_err(|_| AppError::NotFound("Todo item not found".to_string()))?;

        Self::find_owned_list(conn, item.list_id, user_id)?;

        Ok(item)
    }
}

fn normalize_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}
