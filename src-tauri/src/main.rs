#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use diesel::prelude::*;
use fanya::commands::get_tasks;
use fanya::*;
use tauri::api::cli::{get_matches, ArgData, Matches};
use tauri::api::path;

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() {
  let context = tauri::generate_context!();
  let cli_config = context.config().tauri.cli.clone().unwrap();

  match get_matches(&cli_config) {
    // `matches` here is a Struct with { args, subcommand }.
    // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurances }.
    // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
    Ok(matches) => {
      for (key, val) in matches.args {
        if key == "help".to_string() {
          for line in val
            .value
            .to_string()
            .trim_matches(|c| c == '"')
            .split("\\n")
            .collect::<Vec<&str>>()
          {
            println!("{}", line);
          }
          std::process::exit(0);
        }
        // println!("{:?} -> {:?}", key, val);
      }
    }
    Err(_) => {}
  };

  // println!("{:?}", path::data_dir());

  let _ = &mut establish_connection().unwrap();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, get_tasks])
    .run(context)
    .expect("error while running tauri application");
}
