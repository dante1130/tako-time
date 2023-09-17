use tako_time::{commands::Commands, args::Args};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Add(cmd_args) => {
            println!("Add command: {:?}", cmd_args);
        },
        Commands::List(cmd_args) => {
            println!("List command: {:?}", cmd_args);
        },
        Commands::Delete(cmd_args) => {
            println!("Delete command: {:?}", cmd_args);
        },
        Commands::Update(cmd_args) => {
            println!("Update command: {:?}", cmd_args);
        },
        Commands::Complete(cmd_args) => {
            println!("Complete command: {:?}", cmd_args);
        },
    }

}
