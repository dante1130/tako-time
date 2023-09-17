use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCommand),
    List(ListCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
    Complete(CompleteCommand),
}

#[derive(Args)]
pub struct AddCommand {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub time: f64,
}

#[derive(Args)]
pub struct ListCommand {
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Args)]
pub struct UpdateCommand {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub time: f64,
}

#[derive(Args)]
pub struct DeleteCommand {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Args)]
pub struct CompleteCommand {
    #[arg(short, long)]
    pub name: String,
}

