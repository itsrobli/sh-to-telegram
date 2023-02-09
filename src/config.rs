use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::de::Error;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub telegram: Telegram,
}

#[derive(Deserialize, Serialize)]
pub struct Telegram {
    pub(crate) token: String,
    pub(crate) current_chat_id: String,
}

pub fn config_path() -> PathBuf {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push("bin");
    config_path.push("sh-to-telegram.toml");
    config_path
}

pub fn get_configs() -> Result<Config, Error> {
    let config_path = config_path();

    match fs::read_to_string(&config_path) {
        Ok(contents) => {
            let package_info: Config = toml::from_str(&*contents)?;
            Ok(package_info)
        }
        Err(_) => {
            panic!("Could not setup new configs.");
        }
    }
}
