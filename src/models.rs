use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}