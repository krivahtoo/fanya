use super::schema::tasks;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub task_type: String,
  pub tags: Option<String>,
  pub body: String,
  pub done: bool,
  pub created_at: NaiveDateTime,
  pub due_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
  pub title: String,
  pub task_type: String,
  pub tags: Option<String>,
  pub body: String,
  pub due_at: NaiveDateTime,
}
