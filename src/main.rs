mod config;

use std::env;
use clap::Parser;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::Api;
use chrono::prelude::*;


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

fn send_message(msg: String, token: String, current_chat_id: String) {
    let api = Api::new(token.as_str());
    let send_message_params = SendMessageParams::builder()
        .chat_id(current_chat_id)
        .text(msg)
        .build();
    if let Err(err) = api.send_message(&send_message_params) {
        println!("Failed to send message: {:?}", err);
    }
}

fn format_message(has_moved: bool, file_path: String) -> String {
    let mut message = format!("{}\n", file_path);
    if has_moved {
        message.push_str("moved to Plex ");
        let now = Local::now();
        message.push_str(now.format("%Y/%m/%d").to_string().as_str());
    }
    return message
}


fn main() {
    let cli = Cli::parse();
    config::get_configs();

    // let token = env::var("TOKEN")
    //     .expect("TOKEN not found");
    // let current_chat_id = env::var("CURRENT_CHAT_ID")
    //     .expect("CURRENT_CHAT_ID not found");
    // let message = format_message(cli.has_moved, cli.file_path);
    // send_message(message, token, current_chat_id);


}