use std::error::Error;
use std::fmt;

/// Possible HTTP client errors
#[derive(Debug)]
pub enum HttpError {
    ConnectionFailed(std::io::Error),
    Timeout,
    InvalidResponse,
    RequestFailed(u16),
    IoError(std::io::Error),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::ConnectionFailed(e) => write!(f, "connection failed, IO error: {}", e),
            HttpError::Timeout => write!(f, "connection timed out"),
            HttpError::InvalidResponse => write!(f, "invalid HTTP response"),
            HttpError::RequestFailed(code) => write!(f, "HTTP request failed with status {}", code),
            HttpError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for HttpError {}

/// Convert IO errors to our custom error type with specific handling
/// for timeout and connection-related errors
impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::TimedOut {
            HttpError::Timeout
        } else if e.kind() == std::io::ErrorKind::ConnectionRefused
            || e.kind() == std::io::ErrorKind::ConnectionAborted
            || e.kind() == std::io::ErrorKind::ConnectionReset
        {
            HttpError::ConnectionFailed(e)
        } else {
            HttpError::IoError(e)
        }
    }
}
