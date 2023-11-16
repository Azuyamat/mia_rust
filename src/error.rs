// Error Handler
// Author: Derek Blaney

use std::fmt::{Debug, Formatter};

pub enum Error {
    PathNotFound,
    PathNotDir,
    ZipFileFail(Option<std::io::Error>),
    CantReadFile,
    Config(confy::ConfyError),
    ConfigActionError(String)
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
                match error {
                    Some(error) => write!(f, "Zip file couldn't be created {:?}", error),
                    _ => write!(f, "Couldn't execute method on zip file")
                }
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
        }
    }
}