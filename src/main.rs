use clap::Parser;
use tako_time::{
    args::Args,
    commands::Commands,
    task_builder::TaskBuilder,
    task_manager::TaskManager,
    time_parser::{TimeParser, TimeParserError},
};

fn main() {
    let args = Args::parse();

    let mut task_manager = TaskManager::new();

    match &args.command {
        Commands::Add(cmd_args) => {
            let task = TaskBuilder::new()
                .name(cmd_args.name.join(" "))
                .time_estimated(&cmd_args.time_estimated)
                .build();

            task_manager.add_task(task);

            println!("{:?}", task_manager);
        }
        Commands::List(_) => {
            println!("{:?}", task_manager);
        }
        Commands::LogTime(cmd_args) => {
            let time_logged = match TimeParser::parse(&cmd_args.time_logged) {
                Ok(time) => time,
                Err(TimeParserError::InvalidTimeFormat) => panic!("Invalid time format"),
                Err(TimeParserError::InvalidNumberFormat) => panic!("Invalid time unit"),
            };

            task_manager.log_time(cmd_args.id, time_logged);
        }
        Commands::Update(cmd_args) => {
            let mut task_builder = TaskBuilder::new();

            if let Some(time_spent) = &cmd_args.time_spent {
                task_builder.time_spent(time_spent);
            }

            if let Some(time_estimated) = &cmd_args.time_estimated {
                task_builder.time_estimated(time_estimated);
            }

            task_manager.update_task(cmd_args.id, task_builder.build());
        }
        Commands::Remove(cmd_args) => {
            task_manager.remove_task(cmd_args.id);
        }
        Commands::Complete(cmd_args) => {
            task_manager.complete_task(cmd_args.id);
        }
    }

    match task_manager.save() {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }
}
