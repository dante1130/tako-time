use clap::{Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(AddCommand),
    List(ListCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
    Complete(CompleteCommand),
}

#[derive(Args, Debug)]
pub struct AddCommand {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub time: f64,
}

#[derive(Args, Debug)]
pub struct ListCommand {
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateCommand {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub time: f64,
}

#[derive(Args, Debug)]
pub struct DeleteCommand {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct CompleteCommand {
    #[arg(short, long)]
    pub name: String,
}

