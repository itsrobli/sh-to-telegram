use clap::Parser;
use lib::cli::{Cli, Commands, DownloadTask};
use lib::config::{Config, ConfigError};
use lib::logger::log_path;
use lib::telegram;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use lib::app_init::App;

fn main() {
    let mut app = App::default();
    match app.init() {
        Ok(_) => {
            println!("App startup successful!")
        }
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    }
    let cli = Cli::parse();
    let config = match Config::from_file(None) {
        Ok(config) => {
            println!("Config file found...continuing...");
            config
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
                        let message =
                            telegram::format_message_download_finished(false, &task.file_path);
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
