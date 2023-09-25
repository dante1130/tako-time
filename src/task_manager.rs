use std::collections::BTreeMap;

use anyhow::Result;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    task::{State, Task},
    task_serializer::{TaskDeserializer, TaskSerializer},
    time_parser::TimeParser,
};

const TASK_FILE_PATH: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    tasks_map: BTreeMap<u32, Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let task_file = std::fs::read_to_string(TASK_FILE_PATH).unwrap_or_default();

        match TaskDeserializer::deserialize(task_file) {
            Ok(tasks) => tasks,
            Err(_) => TaskManager {
                tasks_map: BTreeMap::new(),
            },
        }
    }

    pub fn save(&self) -> Result<()> {
        let task_file_str = TaskSerializer::serialize(self)?;

        std::fs::write(TASK_FILE_PATH, task_file_str)?;

        Ok(())
    }

    pub fn add_task(&mut self, mut task: Task) {
        let id = self.tasks_map.len() as u32;
        task.id = id;

        self.tasks_map.insert(self.tasks_map.len() as u32, task);
    }

    pub fn list_tasks(&self) {
        for task in self.tasks_map.values() {
            print!("ID: {}, ", task.id);
            print!("Name: {}, ", task.name);
            print!(
                "Time Spent: {}, ",
                TimeParser::duration_to_string(&task.time_spent)
            );
            print!(
                "Time Estimate: {}, ",
                TimeParser::duration_to_string(&task.time_estimated)
            );
            println!("State: {}, ", task.state);
        }
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

    pub fn remove_task(&mut self, id: u32) {
        match self.tasks_map.remove(&id) {
            Some(_) => (),
            None => panic!("Task with id {} does not exist", id),
        }
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
