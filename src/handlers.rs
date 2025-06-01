use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    errors::{AppError, AppResult},
    models::{CreateTodo, Todo, UpdateTodo},
};

pub async fn create_todo(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> AppResult<(StatusCode, Json<Todo>)> {
    if let Err(validation_errors) = payload.validate() {
        tracing::error!("Validation failed: {:?}", validation_errors);
        return Err(validation_errors.into());
    }

    tracing::info!("Validated payload: {:?}", payload);

    let todo_id = Uuid::new_v4();
    let now = Utc::now();

    let todo_id_string = todo_id.to_string();

    let record = sqlx::query!(
        r#"
        INSERT INTO todos (id, title, description, completed, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            title,
            description,
            completed,
            created_at AS "created_at: DateTime<Utc>", -- Decode as DateTime<Utc>
            updated_at AS "updated_at: DateTime<Utc>"  -- Decode as DateTime<Utc>
        "#,
        todo_id_string,
        payload.title,
        payload.description,
        false,
        now,
        now
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    let new_todo = Todo {
        id: Uuid::parse_str(&record.id)?,
        title: record.title,
        description: record.description,
        completed: record.completed,
        created_at: record.created_at,
        updated_at: record.updated_at,
    };

    tracing::info!("Successfully created todo: {:?}", new_todo);

    Ok((StatusCode::CREATED, Json(new_todo)))
}

pub async fn get_todos(State(app_state): State<AppState>) -> AppResult<Json<Vec<Todo>>> {
    tracing::info!("Fetching all todos");

    let records = sqlx::query!(
        r#"
        SELECT id, title, description, completed, created_at AS "created_at: DateTime<Utc>", updated_at AS "updated_at: DateTime<Utc>"
        FROM todos
        ORDER BY created_at DESC -- Optional: order by creation date
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await?;

    let mut todos = Vec::with_capacity(records.len());
    for record in records {
        todos.push(Todo {
            id: Uuid::parse_str(&record.id)?,
            title: record.title,
            description: record.description,
            completed: record.completed,
            created_at: record.created_at,
            updated_at: record.updated_at,
        });
    }

    tracing::info!("Successfully fetched {} todos", todos.len());
    Ok(Json(todos))
}

pub async fn get_todo_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Todo>> {
    tracing::info!("Fetching todo by id: {}", id);

    let id_as_string = id.to_string();

    let optional_record = sqlx::query!(
        r#"
        SELECT id, title, description, completed, created_at AS "created_at: DateTime<Utc>", updated_at AS "updated_at: DateTime<Utc>"
        FROM todos
        WHERE id = $1
        "#,
        id_as_string
    )
    .fetch_optional(&app_state.db_pool)
    .await?;

    if let Some(record) = optional_record {
        let todo = Todo {
            id: Uuid::parse_str(&record.id)?,
            title: record.title,
            description: record.description,
            completed: record.completed,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };
        tracing::info!("Successfully fetched todo: {:?}", todo);
        Ok(Json(todo))
    } else {
        tracing::warn!("Todo with id {} not found", id);
        Err(AppError::NotFound)
    }
}

pub async fn update_todo(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> AppResult<Json<Todo>> {
    if let Err(validation_errors) = payload.validate() {
        tracing::error!("Update validation failed: {:?}", validation_errors);
        return Err(validation_errors.into());
    }
    tracing::info!("Updating todo id: {} with payload: {:?}", id, payload);

    let id_as_string = id.to_string();

    let current_record = sqlx::query!(
        r#"SELECT title, description, completed, created_at FROM todos WHERE id = $1"#,
        id_as_string
    )
    .fetch_optional(&app_state.db_pool)
    .await?;

    let current_todo_values = match current_record {
        Some(record) => record,
        None => {
            tracing::warn!("Todo with id {} not found for update", id);
            return Err(AppError::NotFound);
        }
    };

    let final_title = payload.title.unwrap_or(current_todo_values.title);
    let final_description = payload.description;
    let final_completed = payload.completed.unwrap_or(current_todo_values.completed);
    let updated_at_ts = Utc::now();

    let updated_record = sqlx::query!(
        r#"
        UPDATE todos
        SET title = $1, description = $2, completed = $3, updated_at = $4
        WHERE id = $5
        RETURNING id, title, description, completed, created_at AS "created_at: DateTime<Utc>", updated_at AS "updated_at: DateTime<Utc>"
        "#,
        final_title,
        final_description,
        final_completed,
        updated_at_ts,
        id_as_string
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    let todo = Todo {
        id: Uuid::parse_str(&updated_record.id)?,
        title: updated_record.title,
        description: updated_record.description,
        completed: updated_record.completed,
        created_at: updated_record.created_at,
        updated_at: updated_record.updated_at,
    };

    tracing::info!("Successfully updated todo: {:?}", todo);
    Ok(Json(todo))
}

pub async fn delete_todo(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    tracing::info!("Attempting to delete todo with id: {}", id);

    let id_as_string = id.to_string();

    let result = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
        id_as_string
    )
    .execute(&app_state.db_pool)
    .await?;

    if result.rows_affected() == 0 {
        tracing::warn!("Todo with id {} not found for deletion", id);
        Err(AppError::NotFound)
    } else {
        tracing::info!("Successfully deleted todo with id: {}", id);
        Ok(StatusCode::NO_CONTENT)
    }
}
