mod base;
mod download;
mod upload;

use serde::Deserialize;
use serde::Serialize;

pub use self::base::*;
pub use self::download::*;
pub use self::upload::*;

pub type AllTasksType = DownloadTask;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskTypes {
    Download(DownloadTask),
    Upload(UploadTask),
}
