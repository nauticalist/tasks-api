use std::future::Future;
use core::errors::{ServiceError};
use persistence::task::schema::TaskStore;
use persistence::task::transactions::delete::TaskDeleter;
use crate::task::service::TaskService;

pub trait TaskDeleterService {
    fn delete_task(id: i64) -> impl Future<Output = Result<(), ServiceError>> + Send;
}

impl TaskDeleterService for TaskService {
    fn delete_task(id: i64) -> impl Future<Output=Result<(), ServiceError>> + Send {
        delete::<TaskStore>(id)
    }
}

async fn delete<T: TaskDeleter>(id: i64) -> Result<(), ServiceError> {
    let _ = T::delete(id).await?;
    Ok(())
}