use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    core::{master_disable, ExecResult, Tasks},
    states::TasksState,
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
pub async fn spctl_master_disable() -> String {
    let result = master_disable();
    match result {
        ExecResult::Err(err) => {
            return err;
        }
        ExecResult::Success(result) => {
            return result;
        }
    }
}
