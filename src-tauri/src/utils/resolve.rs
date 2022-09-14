use crate::{
    core::master_disable,
    states,
    tasks::{DownloadTask, TaskTypes, UploadTask},
    utils::init,
};
use tauri::{App, Manager};

/// handle something when start app
pub fn resolve_setup(app: &App) {
    // init app config
    init::init_app(app.package_info());

    // init states
    let task_state = app.state::<states::TasksState>();

    let mut task = task_state.0.lock().unwrap();
    task.append_item(TaskTypes::Download(DownloadTask::new()))
        .unwrap();

    master_disable();
}
