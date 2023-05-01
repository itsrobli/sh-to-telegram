use chrono::prelude::*;
use dirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn log_path() -> PathBuf {
    let mut log_path = dirs::home_dir().unwrap();
    log_path.push("bin");
    log_path.push("sh-to-telegram-log.txt");
    log_path
}

pub fn log_this(msg: String) {
    let log_path = log_path();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log_formatter(msg)) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn log_formatter(msg: String) -> String {
    let now = Local::now().format("%Y/%m/%d").to_string();
    "[".to_owned() + &*now + "] " + &*msg
}
