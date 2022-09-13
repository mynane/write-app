use crate::utils::help::get_uid;

use super::TaskStatus;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadTask {
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

impl UploadTask {
    pub fn new() -> Self {
        Self {
            name: "上传任务".to_owned(),
            desc: "上传任务".to_owned(),
            uid: get_uid("upload-task-"),
            status: TaskStatus::Wait,
            progress: Some(0),
        }
    }
}

impl UploadTask {
    fn start(&mut self) -> Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
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

impl Default for UploadTask {
    fn default() -> Self {
        UploadTask::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_upload_task() {
        let upload = UploadTask::new();

        assert_eq!(upload.name, "上传任务");
        assert_eq!(upload.desc, "上传任务");
    }

    #[test]
    fn change_name() {
        let mut upload = UploadTask::new();
        upload.change_name("test").unwrap();

        assert_eq!(upload.name, "test");
    }

    #[test]
    fn change_desc() {
        let mut upload = UploadTask::new();
        upload.change_desc("test").unwrap();

        assert_eq!(upload.desc, "test");
    }

    #[test]
    fn change_status() {
        let mut upload = UploadTask::new();
        upload.change_status(TaskStatus::Pause).unwrap();

        assert_eq!(upload.status, TaskStatus::Pause);
    }

    #[test]
    fn change_progress() {
        let mut upload = UploadTask::new();
        upload.change_progress(100).unwrap();
        assert_eq!(upload.progress, Some(100));

        upload.change_progress(10).unwrap();
        assert_eq!(upload.progress, Some(10));
    }
}
