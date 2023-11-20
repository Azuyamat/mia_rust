// Error Handler
// Author: Derek Blaney

use confy::ConfyError;
use std::fmt::{Debug, Formatter};
use zip::result::ZipError;

#[allow(clippy::upper_case_acronyms)]
pub enum Error {
    PathNotFound,
    PathNotDir,
    ZipFileFail(ZipError),
    IO(std::io::Error),
    CantReadFile,
    Config(ConfyError),
    ConfigActionError(String),
    Request(reqwest::Error),
    JSON(serde_json::Error),
    Custom(String)
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            // Directory or file path not found.
            Error::PathNotFound => "The input path is not found".to_owned(),
            // Initial path given isn't a directory. Therefore, it can't be zipped.
            Error::PathNotDir => "The input path is not a directory".to_owned(),
            // Zip failed to create. Usually from ZipError.
            Error::ZipFileFail(error) => format!("Zip file failed: {}", error),
            // Target file couldn't be read.
            Error::CantReadFile => "File couldn't be read".to_owned(),
            // Reading or writing error to config data.
            Error::Config(error) => format!("Config error: {}", error),
            // I/O error.
            Error::IO(error) => format!("IO error: {}", error),
            // Error with config action. Usually from ConfyError.
            Error::ConfigActionError(error) => format!("Config action error: {}", error),
            // Occurs when trying to request a resource fails
            Error::Request(error) => format!("Request error: {:?}", error),
            // JSON
            Error::JSON(error) => format!("JSON error: error {error}"),
            // Custom
            Error::Custom(error) => format!("Error: {error}")
        };
        write!(f, "{}", msg)
    }
}

// Implementing the Error trait allows us to use the ? operator instead of mapping the error.

impl From<ConfyError> for Error {
    fn from(error: ConfyError) -> Self {
        Error::Config(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<ZipError> for Error {
    fn from(error: ZipError) -> Self {
        Error::ZipFileFail(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JSON(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Custom(error.to_string())
    }
}
