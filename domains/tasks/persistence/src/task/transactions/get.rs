use std::future::Future;
use crate::task::schema::{Task, TaskStore};
use core::errors::{ServiceError, ServiceErrorStatus};
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
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1"
    ).bind(id)
        .fetch_optional(&*DB_POOL)
        .await?
        .map_err(|e| {
           ServiceError::new(
               e.to_string(),
               ServiceErrorStatus::Unknown
           )
        });

    match task {
        Some(task) => Ok(task),
        None => {
            ServiceError::new(
                "Task not found".to_string(),
                ServiceErrorStatus::NotFound
            )
        }
    }
}

async fn fetch_all_tasks() -> Result<Vec<Task>, ServiceError> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks"
    ).fetch_all(&*DB_POOL)
        .await
        .map_err(|e| {
            ServiceError::new(
                e.to_string(),
                ServiceErrorStatus::Unknown
            )
        })?;

    Ok(tasks)
}