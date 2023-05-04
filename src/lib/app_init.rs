use crate::config::ConfigFileState;
use crate::logger::LogFileState;

pub struct App {
    config_file_state: ConfigFileState,
    log_file_state: LogFileState,
}
