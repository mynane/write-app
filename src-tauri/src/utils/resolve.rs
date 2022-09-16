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

    let mut task = task_state.0.lock().unwrap();
    task.append_item(TaskTypes::Download(DownloadTask::new()))
        .unwrap();

    let mut configs = config_state.0.lock().unwrap();

    let handle = app.handle();

    /// check update
    // tauri::async_runtime::spawn(async move {
    //     println!("2");
    //     match handle.updater().check().await {
    //         Ok(update) => {
    //             if update.is_update_available() {
    //                 log::info!(
    //                     "success to update: {} to {}",
    //                     update.current_version(),
    //                     update.latest_version()
    //                 );
    //                 update.download_and_install().await.unwrap();
    //             }
    //         }
    //         Err(e) => {
    //             log::info!("failed to update: {}", e);
    //         }
    //     }
    //     println!("3");
    // });
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
