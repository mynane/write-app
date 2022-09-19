use tauri::{api, AppHandle, Manager, RunEvent};

/// handle events
pub fn resolve_events(app_handle: &AppHandle, event: RunEvent) {
    match event {
        // tauri::RunEvent::WindowEvent { label, event, .. } => match event {
        //     tauri::WindowEvent::CloseRequested { api, .. } => {
        //         let app_handle = app_handle.clone();
        //         api.prevent_close();
        //         app_handle.get_window(&label).unwrap().hide().unwrap();
        //     }
        //     _ => {}
        // },
        tauri::RunEvent::ExitRequested { .. } => {
            api::process::kill_children();
        }
        tauri::RunEvent::Updater(updater_event) => match updater_event {
            tauri::UpdaterEvent::DownloadProgress {
                chunk_length,
                content_length,
            } => {}
            tauri::UpdaterEvent::UpdateAvailable {
                body,
                date,
                version,
            } => {
                log::info!("update available {}", version);
            }
            tauri::UpdaterEvent::Pending => {
                log::info!("update is pending!");
            }
            tauri::UpdaterEvent::Downloaded => {
                log::info!("update has been downloaded!");
            }
            tauri::UpdaterEvent::Updated => {
                log::info!("app has been updated");
            }
            tauri::UpdaterEvent::AlreadyUpToDate => {
                log::info!("app is already up to date");
            }
            tauri::UpdaterEvent::Error(error) => {
                println!("failed to update: {}", error);
            }
            _ => (),
        },
        _ => {}
    }
}
