use tauri::{AppHandle, Runtime, State};

use crate::states::MyState;

#[tauri::command]
pub fn do_something<R: Runtime>(_app: AppHandle<R>, _state: State<'_, MyState>) {
    println!("hello");
}
