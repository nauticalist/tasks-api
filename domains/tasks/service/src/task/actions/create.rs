use crate::task::models::{NewTask, TaskDto};
use std::future::Future;

use crate::task::service::TaskService;
use core::errors::ServiceError;
use persistence::task::schema::TaskStore;
use persistence::task::transactions::create::TaskCreator;

pub trait TaskCreatorService {
    fn create_task(task: NewTask) -> impl Future<Output = Result<TaskDto, ServiceError>> + Send;
}

impl TaskCreatorService for TaskService {
    fn create_task(new_task: NewTask) -> impl Future<Output = Result<TaskDto, ServiceError>> + Send {
        create_task_fn::<TaskStore>(new_task)
    }
}

async fn create_task_fn<T: TaskCreator>(new_task: NewTask) -> Result<TaskDto, ServiceError> {
    let task = T::create(new_task.to_task()?).await?;
    TaskDto::from_task(task)
}