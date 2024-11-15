use std::future::Future;
use crate::task::models::TaskDto;

use core::errors::ServiceError;
use persistence::task::schema::TaskStore;
use persistence::task::transactions::update::TaskUpdater;
use crate::task::service::TaskService;

pub trait TaskUpdaterService {
    fn update_task(task: TaskDto) -> impl Future<Output = Result<TaskDto, ServiceError>> + Send;
}

impl TaskUpdaterService for TaskService {
    fn update_task(task: TaskDto) -> impl Future<Output=Result<TaskDto, ServiceError>> + Send {
        update_task_fn::<TaskStore>(task)
    }
}

async fn update_task_fn<T: TaskUpdater>(task: TaskDto) -> Result<TaskDto, ServiceError> {
    let updated = T::update(task.to_task()).await?;
    TaskDto::from_task(updated)
}