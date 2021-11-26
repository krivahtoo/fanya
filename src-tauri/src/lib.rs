#[macro_use]
extern crate diesel;

pub mod commands;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::{env, fs, io::Error, path::PathBuf};
use tauri::api::path;

use self::models::Task;

pub fn establish_connection() -> Result<SqliteConnection, Error> {
  dotenv().ok();

  let mut path: PathBuf = path::config_dir().unwrap();
  path.push("fanya");
  fs::create_dir_all(&path)?;
  path.push(env::var("DATABASE_URL").unwrap_or("fanya.db".into()));

  let database_url = path.to_str().unwrap();
  Ok(
    SqliteConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
  )
}
