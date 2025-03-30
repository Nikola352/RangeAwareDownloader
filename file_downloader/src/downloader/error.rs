use crate::http::HttpError;
use std::error::Error;
use std::fmt;

/// Possible errors that can occur during download operations.
#[derive(Debug)]
pub enum DownloadError {
    Http(HttpError),
    MissingLength,
    InvalidResponse(String),
    IoError(std::io::Error),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Http(e) => write!(f, "HTTP error: {}", e),
            DownloadError::MissingLength => {
                write!(f, "Server did not convey total length information")
            }
            DownloadError::InvalidResponse(s) => write!(f, "Invalid response: {}", s),
            DownloadError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for DownloadError {}

impl From<HttpError> for DownloadError {
    fn from(error: HttpError) -> Self {
        DownloadError::Http(error)
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(error: std::io::Error) -> Self {
        DownloadError::IoError(error)
    }
}
