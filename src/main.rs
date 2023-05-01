use lib::cli::{Cli, Commands, DownloadTask};
use lib::config::{Config, ConfigError};
use lib::logger::log_path;
use clap::Parser;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use lib::telegram;

fn init_check() {
    let mut bin_path = dirs::home_dir().unwrap();
    bin_path.push("bin");
    match Path::new(&bin_path).exists() {
        true => {
            println!("binary dir found")
        }
        false => fs::create_dir(&bin_path).expect("Could not create binary dir"),
    }


    let log_path = log_path();
    match Path::new(&log_path).exists() {
        true => {
            println!("Log file found")
        }
        false => {
            let mut file =
                fs::File::create(&log_path).expect("Failed Could not setup new log file.");
            if let Err(e) = write!(file, "") {
                eprintln!("Couldn't write to log file: {}", e);
            }
        }
    }
}

fn main() {
    init_check();
    let cli = Cli::parse();
    let config =
        match Config::from_file(None) {
            Ok(config) => {
                println!("Config file found...continuing...");
                config
            },
            Err(ConfigError::FileNotFound) => {
                match Config::create_template_config_file() {
                    Ok(_) => {
                        println!("New user. Please setup configs in you user dir {}", Config::print_default_config_path());
                        println!("Then run this program again. \n Goodbye!");
                        std::process::exit(1)
                    }
                    Err(err) => {
                        println!("{}", err);
                        std::process::exit(1)
                    }
                }
            }
            Err(_) => {
                println!("{}", ConfigError::Unknown);
                std::process::exit(1)
            }
        };

    let token = config.telegram.token;
    let current_chat_id = config.telegram.current_chat_id;

    match cli.command {
        Commands::Download(command) => {
            let download_command = command.download_commands;
            match download_command {
                DownloadTask::Finished(task) => {
                    if task.has_moved {
                        let message = telegram::format_message_download_finished(
                            task.has_moved,
                            &task.file_path,
                        );
                        telegram::send_message(message, &token, &current_chat_id);
                    } else {
                        let message = telegram::format_message_download_finished(
                            false,
                            &task.file_path,
                        );
                        telegram::send_message(message, &token, &current_chat_id);
                    }
                }
                DownloadTask::Started(task) => {
                    let message = telegram::format_message_download_started(&task.file_path);
                    telegram::send_message(message, &token, &current_chat_id);
                }
            }
        }
    }
}
