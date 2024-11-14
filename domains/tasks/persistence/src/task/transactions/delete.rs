use std::future::Future;
use crate::task::schema::{Task, TaskStore};
use core::errors::{ServiceError, ServiceErrorStatus, SqlxError};
use crate::connection::postgres::DB_POOL;

pub trait TaskDeleter {
    fn delete(id: i64) -> impl Future<Output = Result<Task, ServiceError>> + Send;
}

impl TaskDeleter for TaskStore {
    fn delete(id: i64) -> impl Future<Output=Result<Task, ServiceError>> + Send {
        delete_task(id)
    }
}

async fn delete_task(id: i64) -> Result<Task, ServiceError> {
    sqlx::query_as::<_, Task>(
        "DELETE FROM tasks WHERE id = $1 RETURNING *"
    ).bind(id)
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
                format!("Task with id {} not found", id),
                ServiceErrorStatus::NotFound
            )
        )

}