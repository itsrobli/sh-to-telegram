mod config;
mod telegram;
mod logger;

use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::io::prelude::*;
use clap::Parser;
use toml;
use crate::config::{Config, config_path, Telegram};
use crate::logger::log_path;


/// Parse from the CLI
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Passed in from caller whether file was moved.
    #[clap(short, long, action)]
    has_moved: bool,
    /// Path of file whether moved or not.
    #[clap(short, long)]
    file_path: String,
}

fn init_check() {
    let mut bin_path = dirs::home_dir().unwrap();
    bin_path.push("bin");
    match Path::new(&bin_path).exists() {
        true => {
            println!("Bin dir found")
        }
        false => {
            fs::create_dir(&bin_path)
                .expect("Could not create bin dir")
        }
    }

    let config_path = config_path();
    let log_path = log_path();
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
                eprintln!("Couldn't write to file: {}", e);
            }
            panic!("New user. Please setup configs at {:?}", &config_path);
        }
    }
}

fn main() {
    init_check();
    let cli = Cli::parse();
    let config = config::get_configs().expect("Couldn't get configs");

    let token = config.telegram.token;
    let current_chat_id = config.telegram.current_chat_id;
    let message = telegram::format_message(cli.has_moved, cli.file_path);
    telegram::send_message(message, token, current_chat_id);
}

