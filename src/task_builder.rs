use chrono::Duration;

use crate::{
    task::{State, Task},
    time_parser::{TimeParser, TimeParserError},
};

pub struct TaskBuilder {
    id: u32,
    name: String,
    time_spent: Duration,
    time_estimated: Duration,
}

impl TaskBuilder {
    pub fn new() -> TaskBuilder {
        TaskBuilder {
            id: 0,
            name: String::new(),
            time_spent: Duration::zero(),
            time_estimated: Duration::zero(),
        }
    }

    pub fn id(&mut self, id: u32) -> &mut TaskBuilder {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: String) -> &mut TaskBuilder {
        self.name = name;
        self
    }

    pub fn time_spent(&mut self, time_spent: &Vec<String>) -> &mut TaskBuilder {
        self.time_spent = match TimeParser::parse(time_spent) {
            Ok(time) => time,
            Err(TimeParserError::InvalidTimeFormat) => panic!("Invalid time format"),
            Err(TimeParserError::InvalidNumberFormat) => panic!("Invalid time unit"),
        };

        self
    }

    pub fn time_estimated(&mut self, time_estimated: &Vec<String>) -> &mut TaskBuilder {
        self.time_estimated = match TimeParser::parse(time_estimated) {
            Ok(time) => time,
            Err(TimeParserError::InvalidTimeFormat) => panic!("Invalid time format"),
            Err(TimeParserError::InvalidNumberFormat) => panic!("Invalid time unit"),
        };

        self
    }

    pub fn build(&self) -> Task {
        Task {
            id: self.id,
            name: self.name.clone(),
            time_spent: self.time_spent,
            time_estimated: self.time_estimated,
            state: State::InProgress,
        }
    }
}

impl Default for TaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
