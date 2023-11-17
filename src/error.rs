// Error Handler
// Author: Derek Blaney

use confy::ConfyError;
use std::fmt::{Debug, Formatter};
use zip::result::ZipError;

pub enum Error {
    PathNotFound,
    PathNotDir,
    ZipFileFail(ZipError),
    IO(std::io::Error),
    CantReadFile,
    Config(ConfyError),
    ConfigActionError(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathNotFound => {
                write!(f, "The input path is not found")
            }
            Error::PathNotDir => {
                write!(f, "The input path is not a directory")
            }
            Error::ZipFileFail(error) => {
                write!(f, "Zip file failed {error}")
            }
            Error::CantReadFile => {
                write!(f, "File couldn't be read")
            }
            Error::Config(error) => {
                write!(f, "Config error {error}")
            }
            Error::ConfigActionError(error) => {
                write!(f, "Config action error: {error}")
            }
            Error::IO(error) => {
                write!(f, "I/O error {error}")
            }
        }
    }
}

impl From<ConfyError> for Error {
    fn from(error: ConfyError) -> Self {
        Error::Config(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(_error: std::io::Error) -> Self {
        Error::ConfigActionError("Failed to execute I/O action".to_string())
    }
}

impl From<ZipError> for Error {
    fn from(error: ZipError) -> Self {
        Error::ZipFileFail(error)
    }
}
