use std::io::Error as IOError;
use std::path::StripPrefixError;

#[derive(Debug)]
pub enum SmartShredsError {
    FileIOError(String),
    InvalidDirectoryPath(String),
}

impl From<IOError> for SmartShredsError {
    fn from(error: IOError) -> Self {
        SmartShredsError::FileIOError(error.to_string())
    }
}

impl From<StripPrefixError> for SmartShredsError {
    fn from(error: StripPrefixError) -> Self {
        SmartShredsError::InvalidDirectoryPath(error.to_string())
    }
}

impl std::fmt::Display for SmartShredsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartShredsError::FileIOError(error) => write!(f, "File IO Error: {}", error),
            SmartShredsError::InvalidDirectoryPath(error) => write!(f, "Invalid Directory Path: {}", error),
        }
    }
}