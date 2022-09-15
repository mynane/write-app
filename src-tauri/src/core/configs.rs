use std::sync::{Arc, Mutex};

use crate::utils::{config, dirs};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigsInner {
    pub spctl_master_disable: bool,
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
}

impl Default for Configs {
    fn default() -> Self {
        Configs::new()
    }
}
