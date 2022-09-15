use crate::{
    core::{master_disable, Configs},
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
    let config_state = app.state::<states::ConfigsState>();

    println!("{:?}", config_state);

    let mut task = task_state.0.lock().unwrap();
    task.append_item(TaskTypes::Download(DownloadTask::new()))
        .unwrap();

    let mut configs = config_state.0.lock().unwrap();

    if !configs.0.spctl_master_disable {
        let result = master_disable();
        match result {
            crate::core::ExecResult::Err(_) => {}
            crate::core::ExecResult::Success(_) => {
                configs.0.spctl_master_disable = true;
                configs.save_config().unwrap();
            }
        }
    }
}
