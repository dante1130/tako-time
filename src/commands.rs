use clap::{Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(AddCommand),
    List(ListCommand),
    LogTime(LogTimeCommand),
    Update(UpdateCommand),
    Remove(RemoveCommand),
    Complete(CompleteCommand),
}

const TIME_ARGS_SIZE: usize = 4;

#[derive(Args, Debug)]
pub struct AddCommand {
    #[arg(long, num_args = 1.., required = true)]
    pub name: Vec<String>,
    #[arg(long, num_args = 1..=TIME_ARGS_SIZE, required = true)]
    pub time_estimated: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ListCommand {}

#[derive(Args, Debug)]
pub struct LogTimeCommand {
    #[arg(long, required = true)]
    pub id: u32,
    #[arg(long, num_args = 1..=TIME_ARGS_SIZE, required = true)]
    pub time_logged: Vec<String>,
}

#[derive(Args, Debug)]
pub struct UpdateCommand {
    #[arg(long, required = true)]
    pub id: u32,
    #[arg(long, num_args = 0..=TIME_ARGS_SIZE)]
    pub time_spent: Option<Vec<String>>,
    #[arg(long, num_args = 0..=TIME_ARGS_SIZE)]
    pub time_estimated: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct RemoveCommand {
    #[arg(long, required = true)]
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct CompleteCommand {
    #[arg(long, required = true)]
    pub id: u32,
}

