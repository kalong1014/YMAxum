// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Configuration error definitions

use std::fmt;
use std::error::Error;

/// Configuration error type
#[derive(Debug)]
pub enum ConfigError {
    /// IO error
    Io(std::io::Error),
    /// Parse error
    ParseError(String),
    /// Invalid format
    InvalidFormat(String),
    /// Key not found
    KeyNotFound(String),
    /// Type error
    TypeError(String),
    /// Watch error
    WatchError(String),
    /// Other error
    Other(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error: {}", err),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ConfigError::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            ConfigError::TypeError(msg) => write!(f, "Type error: {}", msg),
            ConfigError::WatchError(msg) => write!(f, "Watch error: {}", msg),
            ConfigError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}
