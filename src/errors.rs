use std::io::Error;
use std::fmt;

#[derive(Debug)]
pub enum FileError {
    IoError(Error),
    SerdeError(serde_json::Error),
}
impl PartialEq for FileError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileError::IoError(e1), FileError::IoError(e2)) => e1.kind() == e2.kind(),
            (FileError::SerdeError(e1), FileError::SerdeError(e2)) => e1.io_error_kind() == e2.io_error_kind(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResultError {
    FileError(FileError),
    OtherError(String),
}

impl From<Error> for FileError {
    fn from(error: Error) -> Self {
        FileError::IoError(error)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(error: serde_json::Error) -> Self {
        FileError::SerdeError(error)
    }
}

impl From<FileError> for ResultError {
    fn from(error: FileError) -> Self {
        ResultError::FileError(error)
    }
}

impl From<String> for ResultError {
    fn from(error: String) -> Self {
        ResultError::OtherError(error)
    }
}


impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::IoError(e) => write!(f, "IO error: {}", e),
            FileError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            // Handle other variants...
        }
    }
}

impl fmt::Display for ResultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultError::FileError(e) => write!(f, "File error: {}", e),
            ResultError::OtherError(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, ResultError>;
