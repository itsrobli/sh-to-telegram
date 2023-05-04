use std::path::PathBuf;
use crate::config::{Config, ConfigError, ConfigFileState};
use crate::logger::LogFileState;

pub struct App {
    config_file_state: ConfigFileState,
    log_file_state: LogFileState,
}

impl App {
    fn new() -> Self {
        let mut config_file_state = ConfigFileState;
        let mut log_file_state = LogFileState;
        match config_file_state = Config::default_path() {
            Ok(path) => {
                if path.exists() {
                    ConfigFileState::Exists
                }
            }
            Err(_) => ConfigFileState::NotExists
        }
        Self {
            config_file_state,
            log_file_state,
        }
    }
}