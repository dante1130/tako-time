use chrono::Duration;

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub time_spent: Duration,
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

#[derive(Debug)]
pub enum State {
    InProgress,
    Done
}

