use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::PathBuf;
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
    pub fn from_file(path: Option<PathBuf>) -> Result<Config, ConfigError> {
        let config_path = path.unwrap_or(Config::default_config_path()?);

        match fs::read_to_string(config_path) {
            Ok(contents) => {
                let package_info: Config =
                    toml::from_str(&contents).map_err(|_| ConfigError::FileParseError)?;
                Ok(package_info)
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Err(ConfigError::FileNotFound),
                _ => Err(ConfigError::FileReadError),
            },
        }
    }

    pub fn new() -> Self {
        Self {
            telegram: Default::default(),
        }
    }

    pub fn default_config_path() -> Result<PathBuf, ConfigError> {
        let mut default_config_path = dirs::home_dir().ok_or(ConfigError::HomePathNotFound)?;
        default_config_path.push(PathBuf::from(DEFAULT_RELATIVE_CONFIG_PATH));
        Ok(default_config_path)
    }

    pub fn create_template_config_file() -> Result<(), ConfigError> {
        let new_config = Config::default();
        let mut file = fs::File::create(Config::default_config_path()?)
            .map_err(|_| ConfigError::FileCouldNotBeCreated)?;
        if let Err(_) = writeln!(file, "{}", toml::to_string(&new_config).unwrap()) {
            return Err(ConfigError::FileCouldNotBeCreated);
        }
        Ok(())
    }

    pub fn print_default_config_path() -> String {
        DEFAULT_RELATIVE_CONFIG_PATH.to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config file not found")]
    FileNotFound,
    #[error("config file not able to be created")]
    FileCouldNotBeCreated,
    #[error("user home path can't be determined")]
    HomePathNotFound,
    #[error("binary path can't be created")]
    BinPathNotFound,
    #[error("config file could not be parsed")]
    FileParseError,
    #[error("config file could not be read")]
    FileReadError,
    #[error("config file unknown error")]
    Unknown,
}

impl Telegram {
    pub fn new() -> Self {
        Self {
            token: "".to_string(),
            current_chat_id: "".to_string(),
        }
    }
}

impl Default for Telegram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    #[test]
    fn test_config_file_blank() {}
}
