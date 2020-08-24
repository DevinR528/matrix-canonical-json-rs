use std::{error::Error as StdError, fmt, io};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    IOError(io::Error),
    InvalidInput(String),
    SizeLimit,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::IOError(err) => write!(f, "{}", err),
            Error::InvalidInput(msg) => write!(f, "Found invalid input: {}", msg),
            Error::SizeLimit => write!(f, "canonical JSON larger than 65,535 bytes is not allowed"),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl StdError for Error {}

impl Error {
    pub fn io(err: io::Error) -> Self {
        Self::IOError(err)
    }
}
