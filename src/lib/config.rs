use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::de::Error;
use thiserror::Error;

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
    pub fn new(path: Option<PathBuf>) -> Result<Config, ConfigError> {
        let config_path = path.unwrap_or(
            Config::default_config_path()
                .ok_or(ConfigError::FileNotFound)?
        );

        match fs::read_to_string(&config_path) {
            Ok(contents) => {
                let package_info: Config = toml::from_str(&*contents)
                    .map_err(
                        |_| ConfigError::FileParseError
                    )?;
                Ok(package_info)
            }
            Err(_) => {
                Err(ConfigError::FileReadError)
            }
        }
    }

    pub fn default_config_path() -> Option<PathBuf> {
        let mut default_config_path = dirs::home_dir()?;
        default_config_path.push(PathBuf::from(DEFAULT_RELATIVE_CONFIG_PATH));
        Some(default_config_path)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config file not found")]
    FileNotFound,
    #[error("config file could not be parsed")]
    FileParseError,
    #[error("config file could not be read")]
    FileReadError,
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