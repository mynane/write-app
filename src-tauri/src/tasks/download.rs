use std::{
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::utils::{error::AnyError, help::get_uid};

use super::TaskStatus;
use anyhow::{Ok, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadTask {
    // 任务uid
    pub uid: String,
    // 任务名
    pub name: String,
    // 任务描述
    pub desc: String,
    // 任务状态
    pub status: TaskStatus,
    // 进度
    pub progress: Option<i8>,
}

impl DownloadTask {
    pub fn new() -> Self {
        let mut download = Self {
            name: "下载任务".to_owned(),
            desc: "下载任务".to_owned(),
            uid: get_uid("download-task-"),
            status: TaskStatus::Wait,
            progress: Some(0),
        };

        download.start().unwrap();

        download
    }

    pub async fn download_base_binary(
        &self,
        output_directory: &Path,
        binary_path_suffix: &str,
    ) -> Result<(), AnyError> {
        println!("{:?}", output_directory);
        let download_url = format!(
            "http://xdfe-new.oss-cn-hangzhou.aliyuncs.com/Dev/{}",
            binary_path_suffix
        );

        let client_builder = Client::builder();
        let client = client_builder.build()?;

        println!("Checking {}", &download_url);

        let res = client.get(&download_url).send().await?;

        let binary_content = if res.status().is_success() {
            println!("Download has been found");
            res.bytes().await?.to_vec()
        } else {
            println!("Download could not be found, aborting");
            std::process::exit(1)
        };

        std::fs::create_dir_all(&output_directory)?;
        let output_path = output_directory.join(binary_path_suffix);
        std::fs::create_dir_all(&output_path.parent().unwrap())?;
        tokio::fs::write(output_path, binary_content).await?;
        Ok(())
    }
}

impl DownloadTask {
    fn start(&mut self) -> Result<(), AnyError> {
        self.change_status(TaskStatus::Running).unwrap();
        println!("{:?}", self);
        // self.download();
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.change_status(TaskStatus::Pause).unwrap();
        Ok(())
    }

    async fn download(&mut self) -> Result<(), AnyError> {
        self.download_base_binary(Path::new("./"), "dist.zip")
            .await
            .unwrap();
        Ok(())
    }

    fn change_name(&mut self, name: &str) -> Result<()> {
        self.name = name.to_owned();
        Ok(())
    }

    fn change_desc(&mut self, desc: &str) -> Result<()> {
        self.desc = desc.to_owned();
        Ok(())
    }

    fn change_status(&mut self, status: TaskStatus) -> Result<()> {
        self.status = status;
        Ok(())
    }

    fn change_progress(&mut self, progress: i8) -> Result<()> {
        self.progress = Some(progress);
        Ok(())
    }
}

impl Default for DownloadTask {
    fn default() -> Self {
        DownloadTask::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_download_task() {
        let mut download = DownloadTask::new();

        assert_eq!(download.name, "下载任务");
        assert_eq!(download.desc, "下载任务");
    }

    #[test]
    fn change_name() {
        let mut download = DownloadTask::new();
        download.change_name("test").unwrap();

        assert_eq!(download.name, "test");
    }

    #[test]
    fn change_desc() {
        let mut download = DownloadTask::new();
        download.change_desc("test").unwrap();

        assert_eq!(download.desc, "test");
    }

    #[test]
    fn change_status() {
        let mut download = DownloadTask::new();
        download.change_status(TaskStatus::Pause).unwrap();

        assert_eq!(download.status, TaskStatus::Pause);
    }

    #[test]
    fn change_progress() {
        let mut download = DownloadTask::new();
        download.change_progress(100).unwrap();
        assert_eq!(download.progress, Some(100));

        download.change_progress(10).unwrap();
        assert_eq!(download.progress, Some(10));
    }
}
