use std::future::Future;
use persistence::task::transactions::fetch::TaskFetcher;
use core::errors::{ServiceError};
use persistence::task::schema::TaskStore;
use crate::task::models::{TaskDto, Tasks};
use crate::task::service::TaskService;

pub trait TaskFetcherService {
    fn get_tasks() -> impl Future<Output = Result<Tasks, ServiceError>> + Send;
    fn get_tasks_by_id(id: i64) -> impl Future<Output = Result<TaskDto, ServiceError>> + Send;
}

impl TaskFetcherService for TaskService {
    fn get_tasks() -> impl Future<Output=Result<Tasks, ServiceError>> + Send {
        get_tasks_fn::<TaskStore>()
    }

    fn get_tasks_by_id(id: i64) -> impl Future<Output=Result<TaskDto, ServiceError>> + Send {
        get_task_by_id_fn::<TaskStore>(id)
    }
}

async fn get_tasks_fn<T: TaskFetcher>() -> Result<Tasks, ServiceError> {
    let tasks = T::fetch_all().await?;
    Tasks::from_vec(tasks)
}

async fn get_task_by_id_fn<T: TaskFetcher>(id: i64) -> Result<TaskDto, ServiceError> {
    let task = T::fetch_one(id).await?;
    TaskDto::from_task(task)
}