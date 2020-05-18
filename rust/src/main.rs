mod actions;
mod ui;
mod util;

use crate::actions::{get_report_folder_path, view_file_command, write_command};
use crate::ui::render_ui;
use chrono::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::create_dir_all;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    /// Subcommand
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Start a task
    Start {
        /// Task name
        task: String,

        /// Option to apply specific time
        #[structopt(short, long, parse(try_from_str = parse_time))]
        at: Option<String>,

        /// Option to apply specific tag
        #[structopt(short, long)]
        tag: Option<String>,
    },
    /// Stop a task
    Stop {
        /// Task name
        task: String,

        /// Option to apply specific time
        #[structopt(short, long, parse(try_from_str = parse_time))]
        at: Option<String>,

        /// Option to apply specific tag
        #[structopt(short, long)]
        tag: Option<String>,
    },
    /// View all tasks
    View {
        /// Specific month to display
        #[structopt(short, long)]
        at: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    if args.debug {
        println!("{:?}", args);
    }

    // Create required folder structure
    create_dir_all(get_report_folder_path()).expect("Cannot create reports folder");

    let mut command_options = HashMap::new();

    match args.cmd {
        Some(Command::Start { task, at, tag, .. }) => {
            if let Some(_) = at {
                command_options.insert(String::from("at"), at.unwrap());
            }
            if let Some(_) = tag {
                command_options.insert(String::from("tag"), tag.unwrap());
            }
            write_command(String::from("start"), task, command_options);
        }
        Some(Command::Stop { task, at, tag, .. }) => {
            if let Some(_) = at {
                command_options.insert(String::from("at"), at.unwrap());
            }
            if let Some(_) = tag {
                command_options.insert(String::from("tag"), tag.unwrap());
            }
            write_command(String::from("stop"), task, command_options);
        }
        Some(Command::View { at }) => {
            view_file_command(at);
        }
        None => {
            render_ui()?;
        }
    }
    Ok(())
}

fn parse_time(value: &str) -> Result<String, &str> {
    match NaiveTime::parse_from_str(value, "%H:%M") {
        Ok(_) => Ok(String::from(value)),
        Err(_) => Err("Cannot parse time. Should use format HH:MM."),
    }
}
