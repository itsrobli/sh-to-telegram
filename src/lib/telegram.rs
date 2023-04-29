use crate::logger::log_this;
use chrono::prelude::*;
use frankenstein::Api;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use std::path::Path;

pub fn send_message(msg: String, token: &String, current_chat_id: &String) {
    let api = Api::new(token.as_str());
    let send_message_params = SendMessageParams::builder()
        .chat_id(current_chat_id.clone())
        .text(msg)
        .build();
    if let Err(err) = api.send_message(&send_message_params) {
        println!("Failed to send message: {:?}", err);
    }
}

pub fn format_message_download_finished(has_moved: bool, file_path: &String) -> String {
    let filename = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let mut message = format!("{:}\n", filename);
    if has_moved {
        message.push_str("moved to Plex ");
        let now = Local::now();
        message.push_str(now.format("%Y/%m/%d").to_string().as_str());
        log_this("moved ".to_owned() + filename)
    } else {
        message.push_str("did not move ");
        let now = Local::now();
        message.push_str(now.format("%Y/%m/%d").to_string().as_str());
        log_this("no-move ".to_owned() + filename)
    }
    return message;
}

pub fn format_message_download_started(file_path: &String) -> String {
    let filename = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let mut message = format!("{:}\n", filename);
    message.push_str("started downloading ");
    let now = Local::now();
    message.push_str(now.format("%Y/%m/%d").to_string().as_str());
    return message;
}
