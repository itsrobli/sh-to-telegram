use crate::config::{Config, ConfigError, ConfigFileState};
use crate::logger::{LogFile, LogFileError, LogFileState};
use thiserror::Error;

pub struct App {
    config_file_state: ConfigFileState,
    log_file_state: LogFileState,
}

impl App {
    pub fn new() -> Self {
        let config_file_state = match Config::default_path() {
            Ok(path) => {
                match path.exists() {
                    true => ConfigFileState::Exists,
                    false => ConfigFileState::NotExists
                }
            }
            Err(_) => ConfigFileState::NotExists
        };
        let log_file_state = match LogFile::default_path() {
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
                match App::handle_no_log_file() {
                    Ok(_) => {
                        self.log_file_state = LogFileState::Exists;
                        Ok(())
                    },
                    Err(err) => Err(AppError::LogFile(err))
                }
            },
            (ConfigFileState::NotExists, LogFileState::Exists) => {
                match App::handle_no_config_file() {
                    Ok(_) => {
                        self.config_file_state = ConfigFileState::Exists;
                        Err(AppError::NeedsOffAndOn)
                    },
                    Err(err) => Err(AppError::Config(err))
                }
            },
            (ConfigFileState::NotExists, LogFileState::NotExists) => {
                match (App::handle_no_config_file(), App::handle_no_log_file()) {
                    (Ok(_), Ok(_)) => {
                        self.config_file_state = ConfigFileState::Exists;
                        self.log_file_state = LogFileState::Exists;
                        Err(AppError::NeedsOffAndOn)
                    },
                    (Err(err), Ok(_)) => {
                        self.log_file_state = LogFileState::Exists;
                        Err(AppError::Config(err))
                    },
                    (Ok(_), Err(err)) => {
                        self.config_file_state = ConfigFileState::Exists;
                        Err(AppError::LogFile(err)) },
                    // TODO: Need to go into the ConfigError/LogFileError and give user more troubleshooting hints.
                    (Err(_), Err(_)) => Err(AppError::Unknown)
                }
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
    fn handle_no_log_file() -> Result<(), LogFileError> {
        LogFile::create_template_log_file()?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("app needs to shutdown and be re-run")]
    NeedsOffAndOn,
    #[error("config error when initiating app. ConfigError: {0:?}")]
    Config(ConfigError),
    #[error("log file error when initiating app. LogFileError: {0:?}")]
    LogFile(LogFileError),
    #[error("app unknown error")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::app_init::App;
    use crate::config::ConfigFileState;
    use crate::logger::LogFileState;

    #[test]
    fn app_has_all_file() {
        let mut app_state = App {
            config_file_state: ConfigFileState::Exists,
            log_file_state: LogFileState::Exists,
        };
        assert_eq!(app_state.init().unwrap(), ());
    }
}