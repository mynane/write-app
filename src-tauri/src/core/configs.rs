use crate::utils::{config, dirs};
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configs {
    pub spctl_master_disable: bool,
}

impl Configs {
    pub fn new() -> Self {
        let config = Self::read_config();
        config
    }

    /// get config
    pub fn read_config() -> Configs {
        config::read_yaml::<Configs>(dirs::config_path())
    }

    /// save the config
    pub fn save_config(&self) -> Result<()> {
        config::save_yaml(
            dirs::config_path(),
            &self,
            Some("# Default Config For write app\n\n"),
        )
    }
}
