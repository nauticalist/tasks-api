use std::future::Future;
use crate::task::schema::{Task, TaskStore};

use core::errors::{ServiceError, ServiceErrorStatus, SqlxError};
use crate::connection::postgres::DB_POOL;

pub trait TaskUpdater {
    fn update(task: Task) -> impl Future<Output = Result<Task, ServiceError>> + Send;
}

impl TaskUpdater for TaskStore {
    fn update(task: Task) -> impl Future<Output=Result<Task, ServiceError>> + Send {
        update_task(task)
    }
}

async fn update_task(task: Task) -> Result<Task, ServiceError> {
    sqlx::query_as::<_, Task>(
        "UPDATE tasks SET title = $1, status $2 WHERE id = $3 RETURNING *"
    ).bind(task.title)
        .bind(task.status)
        .bind(task.id)
        .fetch_optional(&*DB_POOL)
        .await
        .map_err(|e| {
            ServiceError::new(
                "Something went wrong!".to_string(),
                ServiceErrorStatus::DatabaseError(SqlxError::from(e))
            )
        })?
        .ok_or_else(||
            ServiceError::new(
                format!("Task with id {} not found", task.id),
                ServiceErrorStatus::NotFound
            )
        )
}