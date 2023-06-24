use std::{error, fmt, io};
use zip::result::ZipError;

#[derive(Debug)]
/// Custom Error for program
pub enum MyZipError {
    /// Errors from `zip` lib
    ZipError(ZipError),

    /// I/O related errors
    IoError(io::Error),
}

impl From<ZipError> for MyZipError {
    fn from(err: ZipError) -> Self {
        Self::ZipError(err)
    }
}

impl From<io::Error> for MyZipError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl fmt::Display for MyZipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(io) => write!(f, "IOError: {}", io.kind()),
            Self::ZipError(err) => write!(f, "ZipError: {err}"),
        }
    }
}

impl error::Error for MyZipError {}
