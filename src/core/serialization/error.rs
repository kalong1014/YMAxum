// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Serialization error definitions

use std::fmt;
use std::error::Error;

/// Serialization error type
#[derive(Debug)]
pub enum SerializationError {
    /// IO error
    Io(std::io::Error),
    /// Invalid data
    InvalidData(String),
    /// Type error
    TypeError(String),
    /// Format error
    FormatError(String),
    /// Other error
    Other(String),
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializationError::Io(err) => write!(f, "I/O error: {}", err),
            SerializationError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            SerializationError::TypeError(msg) => write!(f, "Type error: {}", msg),
            SerializationError::FormatError(msg) => write!(f, "Format error: {}", msg),
            SerializationError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for SerializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerializationError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SerializationError {
    fn from(err: std::io::Error) -> Self {
        SerializationError::Io(err)
    }
}

impl From<serde_json::Error> for SerializationError {
    fn from(error: serde_json::Error) -> Self {
        SerializationError::FormatError(error.to_string())
    }
}

impl From<simd_json::Error> for SerializationError {
    fn from(error: simd_json::Error) -> Self {
        SerializationError::FormatError(error.to_string())
    }
}
