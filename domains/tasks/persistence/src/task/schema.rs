use serde::{Deserialize, Serialize};

pub struct TaskStore;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub status: String,
}