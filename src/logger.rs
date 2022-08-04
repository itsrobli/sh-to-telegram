use dirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;

pub fn log_this(msg: String) {
    let mut log_path = dirs::home_dir().unwrap();
    log_path.push("Dropbox");
    log_path.push("bridge_to_overlord");
    log_path.push("staging_log.txt");

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
    let log_line = "[".to_owned() + &*now + "] " + &*msg;
    return log_line;
}