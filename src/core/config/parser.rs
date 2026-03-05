// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Configuration parsers

use std::collections::HashMap;
use crate::core::config::error::ConfigError;

/// Parse configuration value from string
pub fn parse_value<T: std::str::FromStr>(s: &str) -> Result<T, ConfigError> where <T as std::str::FromStr>::Err: std::fmt::Display {
    s.parse().map_err(|e| ConfigError::ParseError(format!("Failed to parse value: {}", e)))
}

/// Parse configuration values from environment variables
pub fn parse_env_vars(prefix: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    for (key, value) in std::env::vars() {
        if key.starts_with(prefix) {
            let normalized_key = key
                .trim_start_matches(prefix)
                .to_lowercase()
                .replace('_', ".");
            result.insert(normalized_key, value);
        }
    }
    
    result
}

/// Parse command line arguments into configuration values
pub fn parse_command_line(args: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    let mut i = 1; // Skip program name
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--").replace('-', ".");
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                result.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                result.insert(key, "true".to_string());
                i += 1;
            }
        } else if arg.starts_with("-") {
            // Short option
            let key = arg.trim_start_matches("-").to_string();
            if i + 1 < args.len() && !args[i + 1].starts_with("-") {
                result.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                result.insert(key, "true".to_string());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    
    result
}

/// Merge multiple configuration sources
pub fn merge_configs(configs: &[HashMap<String, String>]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    for config in configs {
        for (key, value) in config {
            result.insert(key.clone(), value.clone());
        }
    }
    
    result
}
