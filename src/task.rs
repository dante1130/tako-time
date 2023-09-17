use chrono::Duration;

#[derive(Debug)]
pub struct Task {
    id: usize,
    name: String,
    time_spent: Duration,
    time_estimated: Duration,
    state: State
}

#[derive(Debug)]
pub enum State {
    InProgress,
    Done
}

