use crate::utils::{config, dirs};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Protocol {
    Https,
    Http,
    Git,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RepItem {
    pub protocol: Option<String>,
    pub uri: Option<String>,
    pub host: Option<String>,
    pub group: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_cloned: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RepInner {
    pub basic_dir: Option<String>,
    pub items: Option<Vec<RepItem>>,
}

impl RepInner {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repositories(pub RepInner);

impl Repositories {
    pub fn new() -> Self {
        let config = Self::read_file();
        Self(config)
    }

    /// get config
    pub fn read_file() -> RepInner {
        config::read_yaml::<RepInner>(dirs::repository_path())
    }

    /// save the config
    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(
            dirs::repository_path(),
            &self.0,
            Some("# Default repository For write app\n\n"),
        )
    }

    /// get basic dir
    pub fn get_basic_dir(&self) -> Result<Option<String>> {
        Ok(self.0.basic_dir.clone())
    }

    /// set basic dir
    pub fn set_basic_dir(&mut self, basic_dir: String) -> Result<Option<String>> {
        self.0.basic_dir = Some(basic_dir);
        self.save_file();

        println!("{:?}", self.0.basic_dir);
        Ok(self.0.basic_dir.clone())
    }

    /// append new item
    pub fn append_item(&mut self, mut item: RepItem) -> Result<()> {
        if item.uri.is_none() {
            bail!("the uri should not be null");
        }

        let uri = item.uri.clone().unwrap();

        let rep = self.get_rep_width_uri(uri).unwrap();
        if !rep.is_none() {
            bail!("the uri should has exest");
        }

        if self.0.items.is_none() {
            self.0.items = Some(vec![]);
        }

        self.0.items.as_mut().map(|items| items.push(item));
        self.save_file();

        Ok(())
    }

    /// find item with uri
    pub fn get_rep_width_uri(&mut self, uri: String) -> Result<Option<RepItem>> {
        if self.0.items.is_none() {
            self.0.items = Some(vec![]);
        }

        let items = self.0.items.as_ref().unwrap();
        let some_uri = Some(uri.clone());

        for each in items.iter() {
            if each.uri == some_uri {
                return Ok(Some(each.clone()));
            }
        }

        Ok(None)
    }
}

impl Default for Repositories {
    fn default() -> Self {
        Repositories::new()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn uri() {
        let rep_url = url::Url::parse("git@github.com:mynane/rust-swc.git");

        println!("{:?}", rep_url);
    }
}
