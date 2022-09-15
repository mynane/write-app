use std::env::temp_dir;
use std::path::{Path, PathBuf};
use tauri::{
    api::path::{home_dir, resource_dir},
    Env, PackageInfo,
};

#[cfg(not(feature = "write-dev"))]
static APP_DIR: &str = "write-app";
#[cfg(feature = "write-dev")]
static APP_DIR: &str = "write-app-dev";

static CONFIG_YAML: &str = "config.yaml";
static VERGE_CONFIG: &str = "app.yaml";
static PROFILE_YAML: &str = "profiles.yaml";
static PROFILE_TEMP: &str = "write-app-runtime.yaml";

/// get the verge app home dir
pub fn app_home_dir() -> PathBuf {
    home_dir()
        .unwrap()
        .join(Path::new(".config"))
        .join(Path::new(APP_DIR))
}

/// get the resources dir
pub fn app_resources_dir(package_info: &PackageInfo) -> PathBuf {
    resource_dir(package_info, &Env::default())
        .unwrap()
        .join("resources")
}

/// profiles dir
pub fn app_profiles_dir() -> PathBuf {
    app_home_dir().join("profiles")
}

/// logs dir
pub fn app_logs_dir() -> PathBuf {
    app_home_dir().join("logs")
}

pub fn config_path() -> PathBuf {
    app_home_dir().join(CONFIG_YAML)
}

pub fn verge_path() -> PathBuf {
    app_home_dir().join(VERGE_CONFIG)
}

pub fn profiles_path() -> PathBuf {
    app_home_dir().join(PROFILE_YAML)
}

pub fn profiles_temp_path() -> PathBuf {
    temp_dir().join(PROFILE_TEMP)
}
