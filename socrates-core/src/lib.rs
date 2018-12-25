pub mod common;
pub mod component;
pub mod module;
pub mod service;

#[derive(Debug)]
pub enum Error {
    StrError(String),
    IoError(std::io::Error),
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::StrError(error.into())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::StrError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
