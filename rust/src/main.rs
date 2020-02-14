use std::env;
use std::path::PathBuf;
use dirs::home_dir;
use chrono::prelude::*;
use std::fs::{ File, OpenOptions, create_dir_all };
use std::io::prelude::*;
use termion::{ color, style };
use regex::Regex;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    /// Start a task
    Start {
        task: String
    },
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);

    // Create required folder structure
    create_dir_all(get_report_folder_path()).expect("Cannot create reports folder");

    match args {
        Cli::Start { task } => {
            let mut options = Vec::new();
            options.push(task);
            write_command("start".to_string(), options);
        }
    }
}

fn write_command(action: String, options: Vec<String>) {
    if options.len() < 1 {
        panic!("Missing value argument");
    }

    let (value, command_options) = parse_command_arguments(&options);

    let date: DateTime<Local>;

    if command_options.contains_key("at") {
        let time = NaiveTime::parse_from_str(
            command_options.get("at").unwrap(),
            "%H:%M"
        ).expect("Cannot parse time");

        let naive_date = NaiveDateTime::new(Local::today().naive_utc(), time);
        date = Local.from_local_datetime(&naive_date).unwrap();
    } else {
        date = Local::now();
    }

    let (
        year, 
        month, 
        day,
        hour,
        minute
    ) = (date.year(), date.month(), date.day(), date.hour(), date.minute());

    // Get report file
    let mut report_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_report_path(year, month))
        .unwrap(); // Avoid match

    let formatted_string = format!(
        "{}-{:0>2}-{:0>2}@{:0>2}:{:0>2} -> {} ({})", 
        year, 
        month,
        day,
        hour,
        minute,
        value,
        action
    );
    
    if let Err(e) = writeln!(report_file, "{}", formatted_string) {
        eprintln!("Cannot write in file: {}", e);
    }

    println!("{}Write:{} {}", color::Fg(color::Green), style::Reset, formatted_string);
}

fn view_file_command() {
    let local: DateTime<Local> = Local::now();
    let (
        year, 
        month
    ) = (local.year(), local.month());

    let report_path = get_report_path(year, month);
    let path_clone = report_path.clone();
    
    let file_name = path_clone.file_name().expect("Cannot retrieve file name");

    let mut file = File::open(report_path).expect("Cannot open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Cannot read file");

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
        None => PathBuf::from("")
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

fn parse_command_arguments(options: &Vec<String>) -> (String, HashMap<String, String>) {
    let mut command_options = HashMap::new();
    let mut command_value: Option<String> = None;

    let re = Regex::new(r"--?(\w+)=?([^$\s]*)?").unwrap();

    for option in options {
        /*if re.is_match(option) {
            let cap = re.captures(option).unwrap();
            command_options.insert(
                String::from(&cap[1]),
                String::from(&cap[2]) 
            );

            continue;
        }*/

        if let Some(_value) = command_value {
            panic!("Too many arguments");
        }

        command_value = Some(option.to_string());
    }

    if let None = command_value {
        panic!("Missing value argument");
    }

    return (command_value.unwrap(), command_options);
}