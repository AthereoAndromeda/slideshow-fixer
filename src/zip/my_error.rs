use std::io;
use zip::result::ZipError;

#[derive(Debug, thiserror::Error, displaydoc::Display)]
/// Custom Error for .zip files
pub enum MyZipError {
    /// Errors from `zip` lib
    ZipError(#[from] ZipError),

    /// I/O related errors
    IoError(#[from] io::Error),
}
