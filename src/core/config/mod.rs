// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Flexible configuration management library
//! Provides support for multiple configuration sources and formats

use std::any::Any;

pub mod loader;
pub mod parser;
pub mod watcher;
pub mod error;

/// Configuration source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// File source
    File(String),
    /// Environment variables
    Env,
    /// Command line arguments
    CommandLine,
    /// In-memory source
    Memory,
}

/// Configuration format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigFormat {
    /// TOML format
    Toml,
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// Environment variables
    Env,
}

/// Configuration trait
pub trait Config {
    /// Get configuration value
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, error::ConfigError>;
    
    /// Get configuration value with default
    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T;
    
    /// Set configuration value
    fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), error::ConfigError>;
    
    /// Remove configuration value
    fn remove(&mut self, key: &str) -> Result<(), error::ConfigError>;
    
    /// Check if configuration key exists
    fn contains(&self, key: &str) -> bool;
    
    /// Load configuration from source
    fn load(&mut self, source: ConfigSource) -> Result<(), error::ConfigError>;
    
    /// Save configuration to source
    fn save(&self, source: ConfigSource) -> Result<(), error::ConfigError>;
    
    /// Watch for configuration changes
    fn watch(&mut self, source: ConfigSource, callback: Box<dyn Fn() + Send + Sync>) -> Result<(), error::ConfigError>;
}

/// Load configuration from file
pub fn load_from_file(path: &str) -> Result<Box<dyn Any>, error::ConfigError> {
    let extension = path.split('.').last().unwrap_or("");
    let format = match extension {
        "toml" => {
            let mut config = loader::TomlConfig::new();
            config.load(ConfigSource::File(path.to_string()))?;
            Ok(Box::new(config) as Box<dyn Any>)
        }
        "json" => {
            let mut config = loader::JsonConfig::new();
            config.load(ConfigSource::File(path.to_string()))?;
            Ok(Box::new(config) as Box<dyn Any>)
        }
        "yaml" | "yml" => {
            let mut config = loader::YamlConfig::new();
            config.load(ConfigSource::File(path.to_string()))?;
            Ok(Box::new(config) as Box<dyn Any>)
        }
        _ => Err(error::ConfigError::InvalidFormat("Unknown format".to_string())),
    };
    format
}

/// Load configuration from environment variables
pub fn load_from_env() -> Result<Box<dyn Any>, error::ConfigError> {
    let mut config = loader::EnvConfig::new();
    config.load(ConfigSource::Env)?;
    Ok(Box::new(config) as Box<dyn Any>)
}
