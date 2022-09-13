use crate::utils::{dirs, tmpl};
use chrono::Local;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::process::{Command, CommandEvent};
use tauri::PackageInfo;

/// initialize this instance's log file
fn init_log(log_dir: &PathBuf) {
    let local_time = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
    let log_file = format!("{}.log", local_time);
    let log_file = log_dir.join(log_file);

    let time_format = "{d(%Y-%m-%d %H:%M:%S)} - {m}{n}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(time_format)))
        .build();
    let tofile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(time_format)))
        .build(log_file)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(tofile)))
        .build(
            Root::builder()
                .appenders(["stdout", "file"])
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

/// Initialize all the files from resources
fn init_config(app_dir: &PathBuf) -> std::io::Result<()> {
    // target path
    let clash_path = app_dir.join("config.yaml");
    let app_path = app_dir.join("app.yaml");
    let profile_path = app_dir.join("profiles.yaml");

    if !clash_path.exists() {
        fs::File::create(clash_path)?.write(tmpl::CLASH_CONFIG)?;
    }
    if !app_path.exists() {
        fs::File::create(app_path)?.write(tmpl::APP_CONFIG)?;
    }
    if !profile_path.exists() {
        fs::File::create(profile_path)?.write(tmpl::PROFILES_CONFIG)?;
    }
    Ok(())
}

/// initialize app
pub fn init_app(package_info: &PackageInfo) {
    // #[cfg(target_os = "macos")]
    // tauri::async_runtime::spawn(async move {
    //     let (mut rx, mut child) = Command::new("node")
    //         .args(["--master-disable"])
    //         .spawn()
    //         .expect("Failed to spawn cargo");

    //     let mut i = 0;
    //     while let Some(event) = rx.recv().await {
    //         if let CommandEvent::Stdout(line) = event {
    //             println!("got: {}", line);
    //             i += 1;
    //             if i == 4 {
    //                 child.write("message from Rust\n".as_bytes()).unwrap();
    //                 i = 0;
    //             }
    //         }
    //     }
    // });
    // create app dir
    let app_dir = dirs::app_home_dir();
    let log_dir = dirs::app_logs_dir();
    let profiles_dir = dirs::app_profiles_dir();

    let res_dir = dirs::app_resources_dir(package_info);

    println!("{:?}", res_dir);
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).unwrap();
    }
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir).unwrap();
    }

    init_log(&log_dir);
    if let Err(err) = init_config(&app_dir) {
        log::error!("{err}");
    }
}
