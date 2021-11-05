use chrono::prelude::*;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub task_type: String,
    pub tags: Option<String>,
    pub body: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub due_date: NaiveDateTime,
}
