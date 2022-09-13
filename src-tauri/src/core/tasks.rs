use std::fmt::Debug;

use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::tasks::TaskTypes;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tasks {
    pub items: Option<Vec<TaskTypes>>,
}

impl Tasks {
    pub fn new() -> Self {
        Self {
            items: Some(vec![]),
        }
    }

    // 添加item
    pub fn append_item(&mut self, task: TaskTypes) -> Result<()> {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        self.items.as_mut().map(|items| items.push(task));
        Ok(())
    }

    // 删除item
    pub fn delete_item(&mut self, uid: String) -> Result<bool> {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }
        let mut items = self.items.take().unwrap_or(vec![]);
        let mut index = None;

        // get the index
        for i in 0..items.len() {
            match items[i].clone() {
                TaskTypes::Download(task) => {
                    if task.uid == uid.clone() {
                        index = Some(i);
                        break;
                    }
                }
                TaskTypes::Upload(task) => {
                    if task.uid == uid.clone() {
                        index = Some(i);
                        break;
                    }
                }
            }
        }

        if let Some(index) = index {
            items.remove(index);
        }

        self.items = Some(items);
        Ok(true)
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Tasks::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::DownloadTask;

    #[test]
    fn new_tasks() {
        let mut tasks = Tasks::new();
        let items = tasks.items.unwrap();

        assert_eq!(items.len(), 0);
    }

    #[test]
    fn append_task() {
        let mut tasks = Tasks::new();
        tasks
            .append_item(TaskTypes::Download(DownloadTask::new()))
            .unwrap();
        let items = tasks.items.unwrap();

        assert_eq!(items.len(), 1);
    }
}
