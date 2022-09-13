#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// mod management;
mod cmds;
mod core;
mod states;
mod tasks;
mod utils;

use crate::cmds::{get_all_tasks, greet};
use crate::utils::resolve;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(states::TasksState::default())
        .setup(|app| Ok(resolve::resolve_setup(app)))
        .invoke_handler(tauri::generate_handler![greet, get_all_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
