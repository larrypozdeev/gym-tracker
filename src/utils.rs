use crate::user_profile::{UserProfile, Users};
use std::fs::File;
use std::io::prelude::*;
use crate::errors::{Result, FileError, ResultError};
use crate::errors::ResultError::OtherError;

use std::io::ErrorKind;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileContents {
    Users(Users),
    UserProfile(UserProfile),
}

pub fn read_file(path_str: &str) -> Result<FileContents> {
    let path = Path::new(path_str);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            return match e.kind() {
                ErrorKind::NotFound => Ok(FileContents::Users(Users::new())),
                _ => Err(OtherError(e.to_string())),
            }
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ResultError::FileError(FileError::IoError(e)))?;

    serde_json::from_str(&contents).map_err(|e| ResultError::FileError(FileError::SerdeError(e)))
}

pub fn update_file(path: &str, contents: &FileContents) -> Result<()> {
    let path = Path::new(path);

    let contents = serde_json::to_string(&contents).map_err(FileError::SerdeError)?;

    let mut file = File::create(&path).map_err(FileError::IoError)?;

    file.write_all(contents.as_bytes())
        .map_err(|e| ResultError::FileError(FileError::IoError(e)))
}
