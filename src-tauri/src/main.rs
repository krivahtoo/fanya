#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use diesel::prelude::*;
use fanya::*;

#[tauri::command]
fn my_custom_command() -> String {
    "Hello from Rust!".into()
}

fn main() {
    use fanya::schema::tasks::dsl::*;

    let _ = &mut establish_connection();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
