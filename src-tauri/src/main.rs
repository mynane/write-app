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
    append_rep, change_lang, change_theme, create_rep, get_all_tasks, get_basic_dir, get_configs,
    get_repositories, greet, kill_sidecars, open_app_dir, open_dir, open_logs_dir, set_basic_dir,
    spctl_master_disable,
};
use crate::utils::resolve;

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    fix_path_env::fix();
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::init())
        .manage(states::TasksState::default())
        .manage(states::ConfigsState::default())
        .manage(states::RepositoriesState::default())
        .setup(|app| Ok(resolve::resolve_setup(app)))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_all_tasks,
            spctl_master_disable,
            change_theme,
            get_configs,
            change_lang,
            kill_sidecars,
            open_app_dir,
            open_logs_dir,
            // Rep
            get_repositories,
            append_rep,
            get_basic_dir,
            set_basic_dir,
            create_rep,
            // common
            open_dir
        ]);

    builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(resolve_events);
}
