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
        let msg: String = match self {
            Error::PathNotFound => "The input path is not found".to_owned(),
            Error::PathNotDir => "The input path is not a directory".to_owned(),
            Error::ZipFileFail(error) => format!("Zip file failed {}", error),
            Error::CantReadFile => "File couldn't be read".to_owned(),
            Error::Config(error) => format!("Config error {}", error),
            Error::IO(error) => format!("IO error {}", error),
            Error::ConfigActionError(error) => format!("Config action error: {}", error),
        };
        write!(f, "{}", msg)
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
