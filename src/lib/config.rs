use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::de::Error;

#[derive(Debug, Deserialize, Serialize, PartialOrd, PartialEq, Clone)]
pub struct Config {
    pub telegram: Telegram,
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, PartialEq, Clone)]
pub struct Telegram {
    pub token: String,
    pub current_chat_id: String,
}

impl Config {
    pub fn new(path: Option<PathBuf>) -> Result<Config, Error> {
        let config_path = path.unwrap_or(config_path());

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
}

impl Telegram {
    pub fn new() -> Telegram {
        Telegram {
            token: "".to_string(),
            current_chat_id: "".to_string(),
        }
    }
}

impl Default for Telegram {
    fn default() -> Self {
        Telegram::new()
    }
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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::message::*;
    use crate::types::{PrivMsg, Target};

    #[test]
    fn test_config_file_blank() {

    }
}