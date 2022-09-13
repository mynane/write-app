use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum TaskStatus {
    // 等待中
    Wait,
    // 执行中
    Running,
    // 暂停
    Pause,
    // 成功
    Success,
    // 失败
    Fail,
}
