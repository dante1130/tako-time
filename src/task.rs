use chrono::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    InProgress,
    Done,
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
