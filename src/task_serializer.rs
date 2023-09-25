use crate::task::Task;
use anyhow::Result;

pub enum TaskSerializer {}

impl TaskSerializer {
    pub fn serialize(&self, task: &Task) -> Result<String> {
        Ok(serde_json::to_string(&task)?)
    }
}

pub enum TaskDeserializer {}

impl TaskDeserializer {
    pub fn deserialize(&self, file_path: &str) -> Result<Task> {
        let serialized_task = std::fs::read_to_string(file_path)?;
        let task = serde_json::from_str(&serialized_task)?;

        Ok(task)
    }
}
