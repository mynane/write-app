#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// mod management;
mod cmds;
mod core;
mod events;
mod states;
mod tasks;
mod utils;

use events::resolve_events;

use crate::cmds::{
    change_lang, change_theme, get_all_tasks, get_configs, greet, spctl_master_disable,
};
use crate::utils::resolve;

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::init())
        .manage(states::TasksState::default())
        .manage(states::ConfigsState::default())
        .setup(|app| Ok(resolve::resolve_setup(app)))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_all_tasks,
            spctl_master_disable,
            change_theme,
            get_configs,
            change_lang,
        ]);

    builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(resolve_events);
}
