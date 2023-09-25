use std::fmt::{Display, Formatter, self};

use chrono::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    InProgress,
    Done,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::InProgress => write!(f, "In Progress"),
            State::Done => write!(f, "Done"),
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub time_spent: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub time_estimated: Duration,
    pub state: State
}

impl Task {
    pub fn new(id: u32, name: String, time_spent: Duration, time_estimated: Duration, state: State) -> Self {
        Task {
            id,
            name,
            time_spent,
            time_estimated,
            state
        }
    }
}
