// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod utils;

use commands::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      problem_cmds::choose_directory,
      problem_cmds::get_problems_info,
      problem_cmds::run_problem,
      problem_cmds::load_problem,
      reactions::show_error
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
