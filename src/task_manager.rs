use std::collections::HashMap;

use crate::task::Task;

#[derive(Debug)]
pub struct TaskManager {
    tasks_map: HashMap<u32, Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks_map: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks_map.insert(self.tasks_map.len() as u32, task);
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks_map.get(&id)
    }

    pub fn get_tasks(&self) -> Vec<&Task> {
        self.tasks_map.values().collect()
    }

    pub fn remove_task(&mut self, id: u32) -> Option<Task> {
        self.tasks_map.remove(&id)
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
