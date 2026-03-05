// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use thiserror::Error;

/// Plugin error types
#[derive(Error, Debug)]
pub enum PluginError {
    /// Plugin file not found
    #[error("Plugin file not found: {0}")]
    FileNotFound(String),
    
    /// Plugin manifest parsing error
    #[error("Failed to parse plugin manifest: {0}")]
    ManifestParseError(String),
    
    /// Plugin signature verification error
    #[error("Plugin signature verification failed: {0}")]
    SignatureError(String),
    
    /// Plugin version compatibility error
    #[error("Plugin version incompatible: {0}")]
    VersionError(String),
    
    /// Plugin dependency error
    #[error("Plugin dependency error: {0}")]
    DependencyError(String),
    
    /// Plugin activation error
    #[error("Failed to activate plugin: {0}")]
    ActivationError(String),
    
    /// Plugin execution error
    #[error("Plugin execution failed: {0}")]
    ExecutionError(String),
    
    /// Plugin sandbox error
    #[error("Plugin sandbox error: {0}")]
    SandboxError(String),
    
    /// Plugin route error
    #[error("Plugin route error: {0}")]
    RouteError(String),
    
    /// Plugin not found
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    
    /// Plugin already exists
    #[error("Plugin already exists: {0}")]
    PluginExists(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

impl PluginError {
    /// Get error category
    pub fn category(&self) -> &str {
        match self {
            PluginError::FileNotFound(_) => "File",
            PluginError::ManifestParseError(_) => "Manifest",
            PluginError::SignatureError(_) => "Security",
            PluginError::VersionError(_) => "Compatibility",
            PluginError::DependencyError(_) => "Dependency",
            PluginError::ActivationError(_) => "Activation",
            PluginError::ExecutionError(_) => "Execution",
            PluginError::SandboxError(_) => "Sandbox",
            PluginError::RouteError(_) => "Route",
            PluginError::PluginNotFound(_) => "NotFound",
            PluginError::PluginExists(_) => "Exists",
            PluginError::IoError(_) => "IO",
            PluginError::JsonError(_) => "JSON",
            PluginError::Other(_) => "Other",
        }
    }

    /// Get recovery suggestion
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            PluginError::FileNotFound(_) => Some("Check if the plugin file exists and is accessible"),
            PluginError::ManifestParseError(_) => Some("Verify the plugin manifest format is correct"),
            PluginError::SignatureError(_) => Some("Ensure the plugin is properly signed with a valid key"),
            PluginError::VersionError(_) => Some("Update the plugin to a version compatible with the core"),
            PluginError::DependencyError(_) => Some("Install missing dependencies"),
            PluginError::ActivationError(_) => Some("Check plugin permissions and dependencies"),
            PluginError::ExecutionError(_) => Some("Review plugin code for errors"),
            PluginError::SandboxError(_) => Some("Check sandbox configuration"),
            PluginError::RouteError(_) => Some("Verify plugin routes are correctly defined"),
            PluginError::PluginNotFound(_) => Some("Install the plugin first"),
            PluginError::PluginExists(_) => Some("Uninstall the existing plugin first"),
            _ => None,
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            PluginError::FileNotFound(_) => true,
            PluginError::ManifestParseError(_) => false,
            PluginError::SignatureError(_) => false,
            PluginError::VersionError(_) => true,
            PluginError::DependencyError(_) => true,
            PluginError::ActivationError(_) => true,
            PluginError::ExecutionError(_) => true,
            PluginError::SandboxError(_) => true,
            PluginError::RouteError(_) => true,
            PluginError::PluginNotFound(_) => true,
            PluginError::PluginExists(_) => true,
            PluginError::IoError(_) => true,
            PluginError::JsonError(_) => false,
            PluginError::Other(_) => true,
        }
    }
}

/// Plugin error context
#[derive(Debug, Clone, serde::Serialize)]
pub struct PluginErrorContext {
    /// Plugin name
    pub plugin_name: Option<String>,
    /// Plugin version
    pub plugin_version: Option<String>,
    /// Error details
    pub details: serde_json::Value,
}

impl PluginErrorContext {
    /// Create new error context
    pub fn new() -> Self {
        Self {
            plugin_name: None,
            plugin_version: None,
            details: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    /// Set plugin name
    pub fn with_plugin_name(mut self, name: &str) -> Self {
        self.plugin_name = Some(name.to_string());
        self
    }

    /// Set plugin version
    pub fn with_plugin_version(mut self, version: &str) -> Self {
        self.plugin_version = Some(version.to_string());
        self
    }

    /// Add detail
    pub fn with_detail<K: Into<String>, V: serde::Serialize>(mut self, key: K, value: V) -> Self {
        self.details[key.into()] = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
        self
    }
}

/// Result type for plugin operations
pub type PluginResult<T> = Result<T, PluginError>;

