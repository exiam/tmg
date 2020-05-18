use chrono::prelude::*;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use termion::{color, style};

pub fn write_command(action: String, task: String, options: HashMap<String, String>) {
  let date: DateTime<Local>;

  if options.contains_key("at") {
    let time =
      NaiveTime::parse_from_str(options.get("at").unwrap(), "%H:%M").expect("Cannot parse time");

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

pub fn view_file_command(at: Option<String>) {
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
  file
    .read_to_string(&mut file_content)
    .expect("Cannot read file");

  println!(
    "{}View file '{}':{}",
    color::Fg(color::Green),
    file_name.to_string_lossy(),
    style::Reset
  );

  print!("{}", file_content);
}

pub fn get_file_lines(at: Option<String>) -> Vec<String> {
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

  let file = File::open(report_path).expect("Cannot open file");
  let buf = BufReader::new(file);
  buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

/**
 * Return path to app dir.
 */
pub fn get_app_dir() -> PathBuf {
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
pub fn get_report_folder_path() -> PathBuf {
  let app_dir: PathBuf = get_app_dir();
  let mut folder_path: PathBuf = app_dir;
  folder_path.push("reports");

  return folder_path;
}

/**
 * Return report path.
 */
pub fn get_report_path(year: i32, month: u32) -> PathBuf {
  let mut report_path = get_report_folder_path();
  report_path.push(format!("{}-{:0>2}", year, month));
  report_path.set_extension("txt");

  return report_path;
}
