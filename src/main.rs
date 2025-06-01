mod errors;
mod handlers;
mod models;

use axum::routing::post;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    db_pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_axum_sqlite=debug,tower_http=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("DATABASE_URL from env: {}", database_url);

    let db_path_str = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(&database_url);

    let path = Path::new(db_path_str);
    tracing::info!(
        "Attempting to check/create database file at absolute path: {:?}",
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
    );

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            tracing::info!(
                "Parent directory {:?} does not exist, attempting to create.",
                parent_dir
            );
            if let Err(e) = fs::create_dir_all(parent_dir) {
                tracing::error!("Failed to create parent directory {:?}: {}", parent_dir, e);
                panic!("Failed to create parent directory: {}", e);
            }
            tracing::info!("Successfully created parent directory {:?}.", parent_dir);
        }
    }

    match fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(_) => tracing::info!(
            "Successfully touched/opened database file via std::fs: {:?}",
            path
        ),
        Err(e) => {
            tracing::error!(
                "std::fs failed to create/open database file '{:?}': {}",
                path,
                e
            );
            panic!(
                "std::fs check: Failed to create/open database file: {}. Full path: {:?}",
                e,
                path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
            );
        }
    }

    tracing::info!("Connecting to database with sqlx: {}", database_url);
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create SQLite connection pool");

    tracing::info!("Database pool created. Running schema setup...");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create todos table");

    tracing::info!("Schema setup complete.");

    let app_state = AppState { db_pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/todos",
            post(handlers::create_todo).get(handlers::get_todos),
        )
        .route(
            "/todos/{id}",
            get(handlers::get_todo_by_id)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
