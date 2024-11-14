use std::future::Future;
use crate::task::schema::{Task, TaskStore};
use core::errors::{ServiceError, SqlxError, ServiceErrorStatus};
use crate::connection::postgres::DB_POOL;

pub trait TaskFetcher {
    fn fetch_all() -> impl Future<Output = Result<Vec<Task>, ServiceError>> + Send;
    fn fetch_one(id: i64) -> impl Future<Output = Result<Task, ServiceError>> + Send;
}

impl TaskFetcher for TaskStore {
    fn fetch_all() -> impl Future<Output=Result<Vec<Task>, ServiceError>> + Send {
        fetch_all_tasks()
    }

    fn fetch_one(id: i64) -> impl Future<Output=Result<Task, ServiceError>> + Send {
        fetch_task(id)
    }
}

async fn fetch_task(id: i64) -> Result<Task, ServiceError> {
    sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1",
    ).bind(id)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|e| {
            println!("{:?}", e.to_string());
            ServiceError::new(
                "Something went wrong!".to_string(),
                ServiceErrorStatus::DatabaseError(SqlxError::from(e))
            )
        })?
        .ok_or_else(||
            ServiceError::new(
                format!("Task with id {} not found", id),
                ServiceErrorStatus::NotFound
            )
        )
}

async fn fetch_all_tasks() -> Result<Vec<Task>, ServiceError> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks"
    ).fetch_all(&*DB_POOL)
        .await
        .map_err(|e| {
            println!("{:?}", e.to_string());
            ServiceError::new(
                "Something went wrong!".to_string(),
                ServiceErrorStatus::DatabaseError(SqlxError::from(e))
            )
        })?;

    Ok(tasks)
}