use std::fmt;
use std::fmt::{Formatter};
use serde::{Deserialize, Serialize};

use core::errors::{ServiceError, ServiceErrorStatus};
use persistence::task::schema::Task;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {write!(f, "DONE")},
            &Self::PENDING => {write!(f, "PENDING")}
         }
    }
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, ServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(
                ServiceError::new(
                    "Invalid task status".to_string(),
                    ServiceErrorStatus::BadRequest
                )
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTask {
    pub title: String,
    pub status: TaskStatus
}

impl fmt::Display for NewTask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.status {
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
            TaskStatus::DONE => write!(f, "Done: {}", self.title)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDto {
    pub id: i64,
    pub title: String,
    pub status: TaskStatus
}

impl TaskDto {
    pub fn from_task(task: Task) -> Result<TaskDto, ServiceError> {
        let dto = TaskDto {
            id: task.id,
            title: task.title,
            status: TaskStatus::from_string(&task.status)?
        };

        Ok(dto)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub pending: Vec<TaskDto>,
    pub done: Vec<TaskDto>
}

impl Tasks {
    pub fn from_vec(all_tasks: Vec<Task>) -> Result<Tasks, ServiceError> {
        let mut pending= Vec::new();
        let mut done = Vec::new();

        for item in all_tasks {
            match TaskStatus::from_string(&item.status)? {
                TaskStatus::PENDING => pending.push(TaskDto::from_task(item)?),
                TaskStatus::DONE => done.push(TaskDto::from_task(item)?)
            }
        }

        Ok(Tasks {
            pending,
            done
        })
    }
}