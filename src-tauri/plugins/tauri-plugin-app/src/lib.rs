use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod cmds;
mod states;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app")
        .invoke_handler(generate_handler![cmds::do_something])
        .setup(|app_handle| {
            app_handle.manage(states::MyState::default());
            Ok(())
        })
        .build()
}
