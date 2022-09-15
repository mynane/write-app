use anyhow::bail;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    core::{master_disable, ConfigsInner, ExecResult, Tasks},
    states::{ConfigsState, TasksState},
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
    println!("{:?}", tasks_state.inner());
    let tasks = tasks_state.0.lock().unwrap();
    // let mut map = HashMap::new();
    // map.insert("app", "commonServices");

    // let res = call_function::<Version>("VersionsServices/lastest", &map)
    //     .await
    //     .unwrap();
    // let version = res.data.unwrap();

    log::info!("[command:greet]: {}", 123123);
    log::error!("[command:greet]: {}", 123123);

    println!("{:?}", tasks);

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
