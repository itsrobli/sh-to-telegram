use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::de::Error;

const DEFAULT_RELATIVE_CONFIG_PATH: &str = "bin/sh-to-telegram.toml";

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
        let config_path = path.unwrap_or(Config::default_config_path());

        match fs::read_to_string(&config_path) {
            Ok(contents) => {
                let package_info: Config = toml::from_str(&*contents)?;
                Ok(package_info)
            }
            Err(err) => {
                panic!("Could not setup new configs. {err}");
            }
        }
    }

    pub fn default_config_path() -> PathBuf {
        let mut default_config_path = dirs::home_dir().unwrap();
        default_config_path.push(PathBuf::from(DEFAULT_RELATIVE_CONFIG_PATH));
        default_config_path
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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]

    #[test]
    fn test_config_file_blank() {

    }
}