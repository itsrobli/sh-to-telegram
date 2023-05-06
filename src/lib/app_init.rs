use std::path::PathBuf;
use crate::config::{Config, ConfigError, ConfigFileState};
use crate::logger::{LogFile, LogFileError, LogFileState};
use thiserror::Error;

pub struct App {
    config_file_state: ConfigFileState,
    log_file_state: LogFileState,
}

impl App {
    pub fn new() -> Self {
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
    pub fn init(&mut self) -> Result<(), AppError> {
        match (&self.config_file_state, &self.log_file_state) {
            (ConfigFileState::Exists, LogFileState::Exists) => Ok(()),
            (ConfigFileState::Exists, LogFileState::NotExists) => {
                todo!()
            },
            (ConfigFileState::NotExists, LogFileState::Exists) => {
                match App::handle_no_config_file() {
                    Ok(_) => Err(AppError::NeedsOffAndOn),
                    Err(err) => Err(AppError::Config(err))
                }
            },
            (ConfigFileState::NotExists, LogFileState::NotExists) => {
                todo!()
            },
        }
    }
    fn handle_no_config_file() -> Result<(), ConfigError> {
        match Config::create_template_config_file() {
            Ok(_) => {
                println!(
                    "New user. Please setup configs in you user dir {}",
                    Config::print_default_config_path()
                );
                println!("Then run this program again. \n Goodbye!");
                Ok(())
            }
            Err(err) => {
                println!("{}", err);
                Err(err)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("app needs to shutdown and be re-run")]
    NeedsOffAndOn,
    #[error("config error when initiating app")]
    Config(ConfigError),
    #[error("app unknown error")]
    Unknown,
}