use std::sync::{Arc, Mutex};

use crate::utils::{config, dirs};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::Theme;

macro_rules! patch {
    ($lv: expr, $rv: expr, $key: tt) => {
        if ($rv.$key).is_some() {
            $lv.$key = $rv.$key;
        }
    };
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Jira {
    protocol: Option<String>,
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    api_version: Option<String>,
    strict_ssl: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigsInner {
    pub spctl_master_disable: Option<bool>,
    pub theme: Option<String>,
    pub lang: Option<String>,
    pub jira: Option<Jira>,
}

impl Default for ConfigsInner {
    fn default() -> Self {
        ConfigsInner {
            spctl_master_disable: Some(false),
            theme: Some("".to_string()),
            lang: Some("zh".to_string()),
            jira: Some(Jira::default()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configs(pub ConfigsInner);

impl Configs {
    pub fn new() -> Self {
        let config = Self::read_config();
        Self(config)
    }

    /// get config
    pub fn read_config() -> ConfigsInner {
        config::read_yaml::<ConfigsInner>(dirs::config_path())
    }

    /// save the config
    pub fn save_config(&self) -> Result<()> {
        config::save_yaml(
            dirs::config_path(),
            &self.0,
            Some("# Default Config For write app\n\n"),
        )
    }

    /// path item
    pub fn patch_item(&mut self, mut item: ConfigsInner) -> Result<()> {
        let mut clo = self.0.clone();
        patch!(clo, item, spctl_master_disable);
        patch!(clo, item, theme);
        patch!(clo, item, lang);
        patch!(clo, item, jira);

        self.0 = clo;
        return self.save_config();
    }
}

impl Default for Configs {
    fn default() -> Self {
        Configs::new()
    }
}
