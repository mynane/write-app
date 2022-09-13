use crate::utils::{config, dirs, help};
use anyhow::{bail, Context, Result};

// web 应用
#[derive(Debug)]
pub struct WebApp {
    // web 应用名称
    pub name: String,
    // web 应用版本
    pub version: String,
    // web 更新时间
    pub update_at: String,
}

// 应用
#[derive(Default, Debug)]
pub struct App {
    pub items: Option<Vec<WebApp>>,
}

impl App {
    /// read the config from the file
    pub fn read_file() -> Self {
        let mut profiles = config::read_yaml::<Self>(dirs::verge_path());

        if profiles.items.is_none() {
            profiles.items = Some(vec![]);
        }

        profiles.items.as_mut().map(|items| {
            for mut item in items.iter_mut() {
                if item.uid.is_none() {
                    item.uid = Some(help::get_uid("d"));
                }
            }
        });

        profiles
    }

    /// save the config to the file
    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(
            dirs::profiles_path(),
            self,
            Some("# Profiles Config for Clash Verge\n\n"),
        )
    }

    /// sync the config between file and memory
    pub fn sync_file(&mut self) -> Result<()> {
        let data = Self::read_file();
        if data.current.is_none() && data.items.is_none() {
            bail!("failed to read profiles.yaml");
        }

        self.items = data.items;
        Ok(())
    }
}
