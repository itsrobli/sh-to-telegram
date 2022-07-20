use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::Api;
use chrono::prelude::*;

pub fn send_message(msg: String, token: String, current_chat_id: String) {
    let api = Api::new(token.as_str());
    let send_message_params = SendMessageParams::builder()
        .chat_id(current_chat_id)
        .text(msg)
        .build();
    if let Err(err) = api.send_message(&send_message_params) {
        println!("Failed to send message: {:?}", err);
    }
}

pub fn format_message(has_moved: bool, file_path: String) -> String {
    let mut message = format!("{}\n", file_path);
    if has_moved {
        message.push_str("moved to Plex ");
        let now = Local::now();
        message.push_str(now.format("%Y/%m/%d").to_string().as_str());
    } else {
        message.push_str("did not move ");
        let now = Local::now();
        message.push_str(now.format("%Y/%m/%d").to_string().as_str());
    }
    return message
}