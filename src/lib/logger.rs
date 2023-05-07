use chrono::prelude::*;
use std::fs;
use std::io::prelude::*;
use dirs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use thiserror::Error;

const DEFAULT_RELATIVE_LOG_PATH: &str = "bin/sh-to-telegram-log.txt";

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct LogFile;

impl LogFile {
    pub fn default_path() -> Result<PathBuf, LogFileError> {
        let mut default_path = dirs::home_dir().ok_or(LogFileError::HomePathNotFound)?;
        default_path.push(PathBuf::from(DEFAULT_RELATIVE_LOG_PATH));
        Ok(default_path)
    }
    pub fn create_template_log_file() -> Result<(), LogFileError> {
        let mut file = fs::File::create(LogFile::default_path()?)
            .map_err(|_| LogFileError::FileCouldNotBeCreated)?;
        if let Err(_) = writeln!(file) {
            return Err(LogFileError::FileCouldNotBeCreated);
        }
        Ok(())
    }
    pub fn log_this(msg: String) -> Result<(), LogFileError> {
        let log_path = LogFile::default_path()?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_path)
            .map_err(|_| LogFileError::FileCouldNotBeOpened)?;

        if let Err(_e) = writeln!(file, "{}", log_formatter(msg)) {
            return Err(LogFileError::FileCouldNotBeWritten);
        }
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum LogFileState {
    Exists,
    NotExists,
}

#[derive(Error, Debug, PartialEq)]
pub enum LogFileError {
    #[error("log file not found")]
    FileNotFound,
    #[error("log file not able to be created")]
    FileCouldNotBeCreated,
    #[error("log file not able to be opened")]
    FileCouldNotBeOpened,
    #[error("log file not able to be written to")]
    FileCouldNotBeWritten,
    #[error("user home path can't be determined")]
    HomePathNotFound,
    #[error("binary path can't be created")]
    BinPathNotFound,
    #[error("log file could not be parsed")]
    FileParseError,
    #[error("log file could not be read")]
    FileReadError,
    #[error("log file unknown error")]
    Unknown,
}

fn log_formatter(msg: String) -> String {
    let now = Local::now().format("%Y/%m/%d").to_string();
    "[".to_owned() + &*now + "] " + &*msg
}
