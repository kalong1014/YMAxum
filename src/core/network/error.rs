// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Network error definitions

use std::fmt;
use std::io::Error as IoError;
use std::error::Error;

/// Network error type
#[derive(Debug)]
pub enum NetworkError {
    /// I/O error
    Io(IoError),
    /// Invalid URL
    InvalidUrl(String),
    /// Invalid request
    InvalidRequest(String),
    /// Invalid response
    InvalidResponse(String),
    /// Connection error
    ConnectionError(String),
    /// Timeout error
    Timeout,
    /// SSL/TLS error
    SslError(String),
    /// HTTP error
    HttpError(u16, String),
    /// Unsupported feature
    UnsupportedFeature(String),
    /// Other error
    Other(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::Io(err) => write!(f, "I/O error: {}", err),
            NetworkError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            NetworkError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            NetworkError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            NetworkError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            NetworkError::Timeout => write!(f, "Timeout"),
            NetworkError::SslError(msg) => write!(f, "SSL/TLS error: {}", msg),
            NetworkError::HttpError(status, msg) => write!(f, "HTTP error {}: {}", status, msg),
            NetworkError::UnsupportedFeature(msg) => write!(f, "Unsupported feature: {}", msg),
            NetworkError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetworkError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<IoError> for NetworkError {
    fn from(err: IoError) -> Self {
        NetworkError::Io(err)
    }
}
