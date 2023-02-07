mod config;
mod telegram;
mod logger;
mod cli;

use std::fs;
use std::path::Path;
use std::io::prelude::*;
use clap::Parser;
use toml;
use crate::cli::{Commands, DownloadSubCommands, DownloadTask};
use crate::config::{Config, config_path, Telegram};
use crate::logger::log_path;

fn init_check() {
    let mut bin_path = dirs::home_dir().unwrap();
    bin_path.push("bin");
    match Path::new(&bin_path).exists() {
        true => {
            println!("binary dir found")
        }
        false => {
            fs::create_dir(&bin_path)
                .expect("Could not create binary dir")
        }
    }

    let config_path = config_path();
    match Path::new(&config_path).exists() {
        true => {
            println!("Config file found")
        }
        false => {
            let new_config = Config {
                telegram: Telegram {
                    token: "".to_string(),
                    current_chat_id: "".to_string()
                }
            };
            let mut file = fs::File::create(&config_path)
                .expect("Failed Could not setup new configs.");
            if let Err(e) = writeln!(file, "{}", toml::to_string(&new_config).unwrap()) {
                eprintln!("Couldn't write to config file: {}", e);
            }
            panic!("New user. Please setup configs at {:?}", &config_path);
        }
    }

    let log_path = log_path();
    match Path::new(&log_path).exists() {
        true => {
            println!("Log file found")
        }
        false => {
            let mut file = fs::File::create(&log_path)
                .expect("Failed Could not setup new log file.");
            if let Err(e) = write!(file, "") {
                eprintln!("Couldn't write to log file: {}", e);
            }
        }
    }
}

fn main() {
    init_check();
    let cli = cli::Cli::parse();
    let config = config::get_configs().expect("Couldn't get configs");

    let token = config.telegram.token;
    let current_chat_id = config.telegram.current_chat_id;

    match cli.command {
        Commands::Download(command) => {
            let download_command = command.download_commands;
            match download_command {
                DownloadTask::Finished(task) => {
                    if task.has_moved && task.file_path.parse().unwrap() {
                        let message = telegram::format_message(task.has_moved, &task.file_path);
                        telegram::send_message(message, &token, &current_chat_id);
                    }
                    if task.file_path.parse().unwrap() {
                        let message = telegram::format_message(false, &task.file_path);
                        telegram::send_message(message, &token, &current_chat_id);
                    }
                }
                _ => {}
            }
        }
    }
}

