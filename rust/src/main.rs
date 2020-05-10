use chrono::prelude::*;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
use termion::{color, style};

#[derive(StructOpt, Debug)]
struct Cli {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    /// Subcommand
    #[structopt(subcommand)]
    cmd: Command,
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

fn main() {
    let args = Cli::from_args();

    if args.debug {
        println!("{:?}", args);
    }

    // Create required folder structure
    create_dir_all(get_report_folder_path()).expect("Cannot create reports folder");

    let mut command_options = HashMap::new();

    match args.cmd {
        Command::Start { task, at, tag, .. } => {
            if let Some(_) = at {
                command_options.insert(String::from("at"), at.unwrap());
            }
            if let Some(_) = tag {
                command_options.insert(String::from("tag"), tag.unwrap());
            }
            write_command(String::from("start"), task, command_options);
        }
        Command::Stop { task, at, tag, .. } => {
            if let Some(_) = at {
                command_options.insert(String::from("at"), at.unwrap());
            }
            if let Some(_) = tag {
                command_options.insert(String::from("tag"), tag.unwrap());
            }
            write_command(String::from("stop"), task, command_options);
        }
        Command::View { at } => {
            view_file_command(at);
        }
    }
}

fn parse_time(value: &str) -> Result<String, &str> {
    match NaiveTime::parse_from_str(value, "%H:%M") {
        Ok(_) => Ok(String::from(value)),
        Err(_) => Err("Cannot parse time. Should use format HH:MM."),
    }
}

fn write_command(action: String, task: String, options: HashMap<String, String>) {
    let date: DateTime<Local>;

    if options.contains_key("at") {
        let time = NaiveTime::parse_from_str(options.get("at").unwrap(), "%H:%M")
            .expect("Cannot parse time");

        let naive_date = NaiveDateTime::new(Local::today().naive_utc(), time);
        date = Local.from_local_datetime(&naive_date).unwrap();
    } else {
        date = Local::now();
    }

    let (year, month, day, hour, minute) = (
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
    );

    // Get report file
    let mut report_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_report_path(year, month))
        .unwrap(); // Avoid match

    let formatted_string;

    if options.contains_key("tag") {
        formatted_string = format!(
            "{}-{:0>2}-{:0>2}@{:0>2}:{:0>2} -> {} ({}) #{}",
            year,
            month,
            day,
            hour,
            minute,
            task,
            action,
            options.get("tag").unwrap()
        );
    } else {
        formatted_string = format!(
            "{}-{:0>2}-{:0>2}@{:0>2}:{:0>2} -> {} ({})",
            year, month, day, hour, minute, task, action
        );
    }
    if let Err(e) = writeln!(report_file, "{}", formatted_string) {
        eprintln!("Cannot write in file: {}", e);
    }

    println!(
        "{}Write:{} {}",
        color::Fg(color::Green),
        style::Reset,
        formatted_string
    );
}

fn view_file_command(at: Option<String>) {
    let local: Date<Local>;

    if let Some(_) = at {
        let mut date_string = at.unwrap();
        date_string.push_str("-01");
        let naive_date =
            NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").expect("Cannot parse date");
        local = Local.from_local_date(&naive_date).unwrap();
    } else {
        local = Local::today();
    }

    let (year, month) = (local.year(), local.month());

    let report_path = get_report_path(year, month);
    let path_clone = report_path.clone();
    let file_name = path_clone.file_name().expect("Cannot retrieve file name");

    let mut file = File::open(report_path).expect("Cannot open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Cannot read file");

    println!(
        "{}View file '{}':{}",
        color::Fg(color::Green),
        file_name.to_string_lossy(),
        style::Reset
    );

    print!("{}", file_content);
}

/**
 * Return path to app dir.
 */
fn get_app_dir() -> PathBuf {
    let mut dir = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    dir.push(".tmg");

    return dir;
}

/**
 * Return report folder path.
 */
fn get_report_folder_path() -> PathBuf {
    let app_dir: PathBuf = get_app_dir();
    let mut folder_path: PathBuf = app_dir;
    folder_path.push("reports");

    return folder_path;
}

/**
 * Return report path.
 */
fn get_report_path(year: i32, month: u32) -> PathBuf {
    let mut report_path = get_report_folder_path();
    report_path.push(format!("{}-{:0>2}", year, month));
    report_path.set_extension("txt");

    return report_path;
}
