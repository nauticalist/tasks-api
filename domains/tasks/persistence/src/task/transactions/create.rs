use std::future::Future;
use crate::task::schema::{Task, TaskStore};
use core::errors::{ServiceError, ServiceErrorStatus};
use crate::connection::postgres::DB_POOL;

pub trait TaskCreator {
    fn create(item: Task) -> impl Future<Output = Result<Task, ServiceError>> + Send;
}

impl TaskCreator for TaskStore {
    fn create(item: Task) -> impl Future<Output=Result<Task, ServiceError>> + Send {
        create_task(item)
    }
}

async fn create_task(item: Task) -> Result<Task, ServiceError> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title, status) VALUES ($1, $2) RETURNING *"
    ).bind(item.title)
        .bind(item.status)
        .fetch_one(&*DB_POOL)
        .await
        .map_err(|e| {
            ServiceError::new(
                e.to_string(),
                ServiceErrorStatus::Unknown
            )
        })?;

    Ok(task)
}