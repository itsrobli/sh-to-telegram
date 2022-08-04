mod config;
mod telegram;
mod logger;

use clap::Parser;



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

fn main() {
    let cli = Cli::parse();
    let config = config::get_configs().expect("Couldn't get configs");

    let token = config.telegram.token;
    let current_chat_id = config.telegram.current_chat_id;
    let message = telegram::format_message(cli.has_moved, cli.file_path);
    telegram::send_message(message, token, current_chat_id);
}
