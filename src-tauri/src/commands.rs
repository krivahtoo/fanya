use serde::{Deserialize, Serialize};
use tauri::{command, State};

use super::models::Task;
use super::*;

#[derive(Debug, Serialize)]
pub struct Tasks {
  pub tasks: Vec<Task>,
}

#[command]
pub fn get_tasks() -> Tasks {
  use super::schema::tasks::dsl::*;

  let connection = establish_connection().unwrap();
  let result = tasks
    .load::<Task>(&connection)
    .expect("Error loading tasks");
  Tasks { tasks: result }
}
