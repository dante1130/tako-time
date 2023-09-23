use std::collections::BTreeMap;

use chrono::Duration;

use crate::task::{Task, State};

#[derive(Debug)]
pub struct TaskManager {
    tasks_map: BTreeMap<u32, Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks_map: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks_map.insert(self.tasks_map.len() as u32, task);
    }

    pub fn log_time(&mut self, id: u32, time: Duration) {
        if let Some(task) = self.tasks_map.get_mut(&id) {
            task.time_spent = task.time_spent + time;
        }
    }

    pub fn update_task(&mut self, id: u32, task: Task) {
        self.tasks_map.insert(id, task);
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

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks_map.get_mut(&id) {
            task.state = State::Done;
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
