use std::{fs, path::Path};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tauri::{api, App, Manager, State, Window};

use crate::{
    core::{master_disable, ConfigsInner, ExecResult, RepInner, RepItem, Tasks},
    states::{ConfigsState, RepositoriesState, TasksState},
    utils::dirs,
    wrap_err,
};

#[derive(Debug, Default, Deserialize, Serialize)]
struct Version {
    pub app: String,
    pub version: String,
    pub uri: String,
}

#[tauri::command]
pub fn greet(name: String, tasks_state: State<'_, TasksState>) -> String {
    let tasks = tasks_state.0.lock().unwrap();
    // let mut map = HashMap::new();
    // map.insert("app", "commonServices");

    // let res = call_function::<Version>("VersionsServices/lastest", &map)
    //     .await
    //     .unwrap();
    // let version = res.data.unwrap();

    log::info!("[command:greet]: {}", 123123);
    log::error!("[command:greet]: {}", 123123);

    format!("Hello! You've been greeted from Rust! {}", name)
}

#[tauri::command]
pub fn get_all_tasks(tasks_state: State<'_, TasksState>) -> Tasks {
    let tasks = tasks_state.0.lock().unwrap();
    tasks.clone()
}

#[tauri::command]
pub async fn download(tasks_state: State<'_, TasksState>) -> Tasks {
    let tasks = tasks_state.0.lock().unwrap();
    tasks.clone()
}

#[tauri::command]
pub fn spctl_master_disable() -> Result<String, String> {
    let result = master_disable();
    match result {
        ExecResult::Err(err) => Err(err),
        ExecResult::Success(result) => Ok(result),
    }
}

#[tauri::command]
pub fn get_configs(configs_state: State<'_, ConfigsState>) -> ConfigsInner {
    let configs = configs_state.0.lock().unwrap();

    configs.0.clone()
}

#[tauri::command]
pub fn change_theme(theme: String, configs_state: State<'_, ConfigsState>) -> ConfigsInner {
    let mut configs = configs_state.0.lock().unwrap();
    configs.0.theme = theme;
    configs.save_config().unwrap();
    configs.0.clone()
}

#[tauri::command]
pub fn change_lang(lang: String, configs_state: State<'_, ConfigsState>) -> ConfigsInner {
    let mut configs = configs_state.0.lock().unwrap();
    configs.0.lang = lang;
    configs.save_config().unwrap();
    configs.0.clone()
}

/// kill all sidecars when update app
#[tauri::command]
pub fn kill_sidecars() {
    api::process::kill_children();
}

/// open app config dir
#[tauri::command]
pub fn open_app_dir() -> Result<(), String> {
    let app_dir = dirs::app_home_dir();
    wrap_err!(open::that(app_dir))
}

/// open logs dir
#[tauri::command]
pub fn open_logs_dir() -> Result<(), String> {
    let log_dir = dirs::app_logs_dir();
    wrap_err!(open::that(log_dir))
}

// Repository

#[tauri::command]
pub fn get_repositories(rep_state: State<'_, RepositoriesState>) -> RepInner {
    let rep = rep_state.0.lock().unwrap();

    rep.0.clone()
}

// #[tauri::command]
// pub fn get_rep_width_uri(uri: String, rep_state: State<'_, RepositoriesState>) -> RepItem {
//     let mut rep = rep_state.0.lock().unwrap();

//     let result = rep.get_rep_width_uri(uri).unwrap().clone().unwrap();
//     result
// }

#[tauri::command]
pub fn append_rep(item: RepItem, rep_state: State<'_, RepositoriesState>) -> Result<(), String> {
    let mut rep = rep_state.0.lock().unwrap();

    wrap_err!(rep.append_item(item))
}

#[tauri::command]
pub fn create_rep(
    item: RepItem,
    rep_state: State<'_, RepositoriesState>,
) -> Result<String, String> {
    let mut rep = rep_state.0.lock().unwrap();
    let rep_dir = Path::new(&rep.0.basic_dir.clone().unwrap())
        .join(&item.host.unwrap())
        .join(&item.group.unwrap())
        .join(&item.name.unwrap());

    if rep_dir.exists() {
        return Err("repository path exists".to_string());
    }

    fs::create_dir_all(&rep_dir).unwrap();

    Ok(rep_dir.display().to_string())
}

#[tauri::command]
pub fn get_basic_dir(rep_state: State<'_, RepositoriesState>) -> Result<Option<String>, String> {
    let mut rep = rep_state.0.lock().unwrap();

    wrap_err!(rep.get_basic_dir())
}

#[tauri::command]
pub fn set_basic_dir(
    basic_dir: String,
    rep_state: State<'_, RepositoriesState>,
) -> Result<Option<String>, String> {
    let mut rep = rep_state.0.lock().unwrap();

    wrap_err!(rep.set_basic_dir(basic_dir))
}

/// open logs dir
#[tauri::command]
pub fn open_dir(dir: String) -> Result<(), String> {
    let dir = Path::new(&dir);
    wrap_err!(open::that(dir))
}
