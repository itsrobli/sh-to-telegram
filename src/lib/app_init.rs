use std::path::PathBuf;
use crate::config::{Config, ConfigError, ConfigFileState};
use crate::logger::{LogFile, LogFileError, LogFileState};

pub struct App {
    config_file_state: ConfigFileState,
    log_file_state: LogFileState,
}

impl App {
    fn new() -> Self {
        let mut config_file_state = ConfigFileState::NotExists;
        let mut log_file_state = LogFileState::NotExists;
        config_file_state = match Config::default_path() {
            Ok(path) => {
                match path.exists() {
                    true => ConfigFileState::Exists,
                    false => ConfigFileState::NotExists
                }
            }
            Err(_) => ConfigFileState::NotExists
        };
        log_file_state = match LogFile::default_path() {
            Ok(path) => {
                match path.exists() {
                    true => LogFileState::Exists,
                    false => LogFileState::NotExists
                }
            }
            Err(_) => LogFileState::NotExists
        };
        Self {
            config_file_state,
            log_file_state,
        }
    }
}