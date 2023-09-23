use clap::Parser;
use tako_time::{
    args::Args, commands::Commands, task_builder::TaskBuilder, task_manager::TaskManager,
};

fn main() {
    let args = Args::parse();

    let mut task_manager = TaskManager::new();
    let mut task_builder = TaskBuilder::new();

    match &args.command {
        Commands::Add(cmd_args) => {
            let task = task_builder
                .name(cmd_args.name.join(" "))
                .time_estimated(&cmd_args.time_estimated)
                .build();

            task_manager.add_task(task);

            println!("Task manager: {:?}", task_manager);
        }
        Commands::List(cmd_args) => {
            println!("List command: {:?}", cmd_args);
        }
        Commands::Update(cmd_args) => {
            println!("Update command: {:?}", cmd_args);
        }
        Commands::Remove(cmd_args) => {
            println!("Remove command: {:?}", cmd_args);
        }
        Commands::Complete(cmd_args) => {
            println!("Complete command: {:?}", cmd_args);
        }
    }
}
