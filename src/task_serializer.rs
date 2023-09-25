use anyhow::Result;

use crate::task_manager::TaskManager;

pub enum TaskSerializer {}

impl TaskSerializer {
    pub fn serialize(task: &TaskManager) -> Result<String> {
        Ok(serde_json::to_string(&task)?)
    }
}

pub enum TaskDeserializer {}

impl TaskDeserializer {
    pub fn deserialize(task_file_str: String) -> Result<TaskManager> {
        Ok(serde_json::from_str(&task_file_str)?)
    }
}
