//! Unified error types for YMAxum framework
//! Provides comprehensive error handling with detailed error messages and context

use std::fmt::Debug;
use std::vec::Vec;
use thiserror::Error;

/// Error context information, including error source and additional details
#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    /// File where the error occurred
    pub file: Option<String>,
    /// Line number where the error occurred
    pub line: Option<u32>,
    /// Module where the error occurred
    pub module: Option<String>,
    /// Additional key-value metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new() -> Self {
        Self::default()
    }

    /// Add file information
    pub fn with_file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }

    /// Add line number information
    pub fn with_line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Add module information
    pub fn with_module(mut self, module: &str) -> Self {
        self.module = Some(module.to_string());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Format error context as string
    pub fn format(&self) -> String {
        let mut parts = Vec::new();

        if let Some(module) = &self.module {
            parts.push(format!("module: {}", module));
        }

        if let Some(file) = &self.file {
            parts.push(format!("file: {}", file));
        }

        if let Some(line) = self.line {
            parts.push(format!("line: {}", line));
        }

        if !self.metadata.is_empty() {
            let metadata_str: Vec<String> = self
                .metadata
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();
            parts.push(format!("metadata: {{{}}}", metadata_str.join(", ")));
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!("[{}]", parts.join(", "))
        }
    }
}

/// Main error type for YMAxum framework
#[derive(Error, Debug)]
pub enum YMAxumError {
    /// Command execution errors
    #[error("Command execution error: {message}")]
    CommandError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Plugin management errors
    #[error("Plugin error: {message}")]
    PluginError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Dependency management errors
    #[error("Dependency error: {message}")]
    DependencyError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Database errors
    #[error("Database error: {message}")]
    DatabaseError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Network errors
    #[error("Network error: {message}")]
    NetworkError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization/Deserialization errors
    #[error("Serialization error: {message}")]
    SerializationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Validation errors
    #[error("Validation error: {message}")]
    ValidationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Authentication errors
    #[error("Authentication error: {message}")]
    AuthError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Authorization errors
    #[error("Authorization error: {message}")]
    AuthzError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Rate limit errors
    #[error("Rate limit error: {message}")]
    RateLimitError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Cache errors
    #[error("Cache error: {message}")]
    CacheError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Security errors
    #[error("Security error: {message}")]
    SecurityError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Transaction errors
    #[error("Transaction error: {message}")]
    TransactionError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Middleware errors
    #[error("Middleware error: {message}")]
    MiddlewareError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Route errors
    #[error("Route error: {message}")]
    RouteError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Rule errors
    #[error("Rule error: {message}")]
    RuleError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Service errors
    #[error("Service error: {message}")]
    ServiceError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Iterate API errors
    #[error("Iterate API error: {message}")]
    IterateError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Internal errors
    #[error("Internal error: {message}")]
    InternalError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },

    /// Unknown errors
    #[error("Unknown error: {message}")]
    UnknownError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: ErrorContext,
    },
}

impl Clone for YMAxumError {
    fn clone(&self) -> Self {
        match self {
            Self::CommandError {
                message, context, ..
            } => Self::CommandError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::PluginError {
                message, context, ..
            } => Self::PluginError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::DependencyError {
                message, context, ..
            } => Self::DependencyError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::ConfigError {
                message, context, ..
            } => Self::ConfigError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::DatabaseError {
                message, context, ..
            } => Self::DatabaseError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::NetworkError {
                message, context, ..
            } => Self::NetworkError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::IoError(e) => Self::IoError(std::io::Error::new(e.kind(), e.to_string())),
            Self::SerializationError {
                message, context, ..
            } => Self::SerializationError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::ValidationError {
                message, context, ..
            } => Self::ValidationError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::AuthError {
                message, context, ..
            } => Self::AuthError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::AuthzError {
                message, context, ..
            } => Self::AuthzError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::RateLimitError {
                message, context, ..
            } => Self::RateLimitError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::CacheError {
                message, context, ..
            } => Self::CacheError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::SecurityError {
                message, context, ..
            } => Self::SecurityError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::TransactionError {
                message, context, ..
            } => Self::TransactionError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::MiddlewareError {
                message, context, ..
            } => Self::MiddlewareError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::RouteError {
                message, context, ..
            } => Self::RouteError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::RuleError {
                message, context, ..
            } => Self::RuleError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::ServiceError {
                message, context, ..
            } => Self::ServiceError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::IterateError {
                message, context, ..
            } => Self::IterateError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::InternalError {
                message, context, ..
            } => Self::InternalError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
            Self::UnknownError {
                message, context, ..
            } => Self::UnknownError {
                message: message.clone(),
                source: None,
                context: context.clone(),
            },
        }
    }
}

impl YMAxumError {
    /// Create a new command error
    pub fn command_error(message: impl Into<String>) -> Self {
        Self::CommandError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new command error with source
    pub fn command_error_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::CommandError {
            message: message.into(),
            source: Some(Box::new(source)),
            context: ErrorContext::new(),
        }
    }

    /// Create a new command error with context
    pub fn command_error_with_context(message: impl Into<String>, context: ErrorContext) -> Self {
        Self::CommandError {
            message: message.into(),
            source: None,
            context,
        }
    }

    /// Create a new plugin error
    pub fn plugin_error(message: impl Into<String>) -> Self {
        Self::PluginError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new plugin error with source
    pub fn plugin_error_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::PluginError {
            message: message.into(),
            source: Some(Box::new(source)),
            context: ErrorContext::new(),
        }
    }

    /// Create a new plugin error with context
    pub fn plugin_error_with_context(message: impl Into<String>, context: ErrorContext) -> Self {
        Self::PluginError {
            message: message.into(),
            source: None,
            context,
        }
    }

    /// Create a new dependency error
    pub fn dependency_error(message: impl Into<String>) -> Self {
        Self::DependencyError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new database error
    pub fn database_error(message: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new network error
    pub fn network_error(message: impl Into<String>) -> Self {
        Self::NetworkError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new serialization error
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new authentication error
    pub fn auth_error(message: impl Into<String>) -> Self {
        Self::AuthError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new authorization error
    pub fn authz_error(message: impl Into<String>) -> Self {
        Self::AuthzError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new rate limit error
    pub fn rate_limit_error(message: impl Into<String>) -> Self {
        Self::RateLimitError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new cache error
    pub fn cache_error(message: impl Into<String>) -> Self {
        Self::CacheError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new security error
    pub fn security_error(message: impl Into<String>) -> Self {
        Self::SecurityError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new transaction error
    pub fn transaction_error(message: impl Into<String>) -> Self {
        Self::TransactionError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new middleware error
    pub fn middleware_error(message: impl Into<String>) -> Self {
        Self::MiddlewareError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new route error
    pub fn route_error(message: impl Into<String>) -> Self {
        Self::RouteError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new rule error
    pub fn rule_error(message: impl Into<String>) -> Self {
        Self::RuleError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new service error
    pub fn service_error(message: impl Into<String>) -> Self {
        Self::ServiceError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new iterate API error
    pub fn iterate_error(message: impl Into<String>) -> Self {
        Self::IterateError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Create a new unknown error
    pub fn unknown_error(message: impl Into<String>) -> Self {
        Self::UnknownError {
            message: message.into(),
            source: None,
            context: ErrorContext::new(),
        }
    }

    /// Get the error code
    pub fn error_code(&self) -> u16 {
        match self {
            Self::CommandError { .. } => 400,
            Self::PluginError { .. } => 500,
            Self::DependencyError { .. } => 500,
            Self::ConfigError { .. } => 500,
            Self::DatabaseError { .. } => 500,
            Self::NetworkError { .. } => 503,
            Self::IoError { .. } => 500,
            Self::SerializationError { .. } => 500,
            Self::ValidationError { .. } => 400,
            Self::AuthError { .. } => 401,
            Self::AuthzError { .. } => 403,
            Self::RateLimitError { .. } => 429,
            Self::CacheError { .. } => 500,
            Self::SecurityError { .. } => 500,
            Self::TransactionError { .. } => 500,
            Self::MiddlewareError { .. } => 500,
            Self::RouteError { .. } => 500,
            Self::RuleError { .. } => 500,
            Self::ServiceError { .. } => 500,
            Self::IterateError { .. } => 500,
            Self::InternalError { .. } => 500,
            Self::UnknownError { .. } => 500,
        }
    }

    /// Get the error category
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::CommandError { .. } => "command",
            Self::PluginError { .. } => "plugin",
            Self::DependencyError { .. } => "dependency",
            Self::ConfigError { .. } => "config",
            Self::DatabaseError { .. } => "database",
            Self::NetworkError { .. } => "network",
            Self::IoError { .. } => "io",
            Self::SerializationError { .. } => "serialization",
            Self::ValidationError { .. } => "validation",
            Self::AuthError { .. } => "auth",
            Self::AuthzError { .. } => "authz",
            Self::RateLimitError { .. } => "rate_limit",
            Self::CacheError { .. } => "cache",
            Self::SecurityError { .. } => "security",
            Self::TransactionError { .. } => "transaction",
            Self::MiddlewareError { .. } => "middleware",
            Self::RouteError { .. } => "route",
            Self::RuleError { .. } => "rule",
            Self::ServiceError { .. } => "service",
            Self::IterateError { .. } => "iterate",
            Self::InternalError { .. } => "internal",
            Self::UnknownError { .. } => "unknown",
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError { .. }
                | Self::DatabaseError { .. }
                | Self::CacheError { .. }
                | Self::RateLimitError { .. }
        )
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError { .. }
                | Self::DatabaseError { .. }
                | Self::CacheError { .. }
                | Self::RateLimitError { .. }
                | Self::ValidationError { .. }
        )
    }

    /// Get detailed error message with context
    pub fn detailed_message(&self) -> String {
        let base_message = format!("{}", self);
        match self {
            Self::CommandError { context, .. }
            | Self::PluginError { context, .. }
            | Self::DependencyError { context, .. }
            | Self::ConfigError { context, .. }
            | Self::DatabaseError { context, .. }
            | Self::NetworkError { context, .. }
            | Self::SerializationError { context, .. }
            | Self::ValidationError { context, .. }
            | Self::AuthError { context, .. }
            | Self::AuthzError { context, .. }
            | Self::RateLimitError { context, .. }
            | Self::CacheError { context, .. }
            | Self::SecurityError { context, .. }
            | Self::TransactionError { context, .. }
            | Self::MiddlewareError { context, .. }
            | Self::RouteError { context, .. }
            | Self::RuleError { context, .. }
            | Self::ServiceError { context, .. }
            | Self::IterateError { context, .. }
            | Self::InternalError { context, .. }
            | Self::UnknownError { context, .. } => {
                let context_str = context.format();
                if context_str.is_empty() {
                    base_message
                } else {
                    format!("{} {}", base_message, context_str)
                }
            }
            Self::IoError(_) => base_message,
        }
    }

    /// Get error suggestion for recovery
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            Self::NetworkError { .. } => Some("Check network connection and try again later"),
            Self::DatabaseError { .. } => {
                Some("Check database connection and ensure database service is running")
            }
            Self::CacheError { .. } => Some("Check cache service and try clearing cache"),
            Self::RateLimitError { .. } => Some("Request too frequent, please try again later"),
            Self::ValidationError { .. } => {
                Some("Check input parameters to ensure they meet requirements")
            }
            Self::AuthError { .. } => Some("Check username and password"),
            Self::AuthzError { .. } => Some("Check user permissions to ensure access rights"),
            Self::PluginError { .. } => {
                Some("Check plugin configuration and ensure proper installation")
            }
            Self::DependencyError { .. } => {
                Some("Check dependency configuration and ensure proper installation")
            }
            Self::ConfigError { .. } => Some("Check configuration files to ensure correctness"),
            Self::IoError { .. } => Some("Check file path and permissions, ensure file exists"),
            Self::SerializationError { .. } => {
                Some("Check data format to ensure it meets requirements")
            }
            Self::SecurityError { .. } => {
                Some("Check security configuration to ensure no policy violations")
            }
            Self::TransactionError { .. } => {
                Some("Check transaction configuration and ensure proper commit")
            }
            Self::MiddlewareError { .. } => {
                Some("Check middleware configuration and ensure proper loading")
            }
            Self::RouteError { .. } => {
                Some("Check route configuration and ensure proper registration")
            }
            Self::RuleError { .. } => Some("Check rule configuration and ensure proper setup"),
            Self::ServiceError { .. } => {
                Some("Check service configuration and ensure service is running")
            }
            Self::IterateError { .. } => {
                Some("Check iterate API configuration and ensure proper implementation")
            }
            Self::InternalError { .. } => {
                Some("Contact system administrator and check system logs")
            }
            Self::UnknownError { .. } => Some("Contact system administrator and check system logs"),
            Self::CommandError { .. } => Some("Check command parameters to ensure correctness"),
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> &'static str {
        match self {
            Self::ValidationError { .. } => "low",
            Self::RateLimitError { .. } => "medium",
            Self::AuthError { .. } => "medium",
            Self::AuthzError { .. } => "medium",
            Self::NetworkError { .. } => "medium",
            Self::CacheError { .. } => "low",
            Self::IoError { .. } => "medium",
            Self::SerializationError { .. } => "medium",
            Self::PluginError { .. } => "high",
            Self::DependencyError { .. } => "high",
            Self::ConfigError { .. } => "high",
            Self::DatabaseError { .. } => "high",
            Self::SecurityError { .. } => "critical",
            Self::TransactionError { .. } => "high",
            Self::MiddlewareError { .. } => "high",
            Self::RouteError { .. } => "high",
            Self::RuleError { .. } => "high",
            Self::ServiceError { .. } => "high",
            Self::IterateError { .. } => "high",
            Self::InternalError { .. } => "critical",
            Self::UnknownError { .. } => "critical",
            Self::CommandError { .. } => "medium",
        }
    }

    /// Convert error to HTTP response
    pub fn to_http_response(&self) -> (u16, String) {
        let code = self.error_code();
        let message = self.detailed_message();
        let suggestion = self
            .suggestion()
            .unwrap_or("Please contact system administrator");
        let severity = self.severity();
        let category = self.error_category();

        let error_json = serde_json::json!({
            "code": code,
            "message": message,
            "suggestion": suggestion,
            "severity": severity,
            "category": category
        });

        (code, error_json.to_string())
    }
}

/// Result type alias for YMAxum
pub type Result<T> = std::result::Result<T, YMAxumError>;

/// Error type alias for convenience
pub type Error = YMAxumError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = YMAxumError::command_error("Test command error");
        assert!(matches!(error, YMAxumError::CommandError { .. }));
        assert_eq!(error.error_code(), 400);
        assert_eq!(error.error_category(), "command");
    }

    #[test]
    fn test_error_code() {
        assert_eq!(YMAxumError::command_error("").error_code(), 400);
        assert_eq!(YMAxumError::plugin_error("").error_code(), 500);
        assert_eq!(YMAxumError::auth_error("").error_code(), 401);
        assert_eq!(YMAxumError::authz_error("").error_code(), 403);
        assert_eq!(YMAxumError::rate_limit_error("").error_code(), 429);
    }

    #[test]
    fn test_error_category() {
        assert_eq!(YMAxumError::command_error("").error_category(), "command");
        assert_eq!(YMAxumError::plugin_error("").error_category(), "plugin");
        assert_eq!(YMAxumError::database_error("").error_category(), "database");
    }

    #[test]
    fn test_is_retryable() {
        assert!(YMAxumError::network_error("").is_retryable());
        assert!(YMAxumError::database_error("").is_retryable());
        assert!(YMAxumError::cache_error("").is_retryable());
        assert!(YMAxumError::rate_limit_error("").is_retryable());
        assert!(!YMAxumError::command_error("").is_retryable());
        assert!(!YMAxumError::validation_error("").is_retryable());
    }

    #[test]
    fn test_is_recoverable() {
        assert!(YMAxumError::network_error("").is_recoverable());
        assert!(YMAxumError::database_error("").is_recoverable());
        assert!(YMAxumError::cache_error("").is_recoverable());
        assert!(YMAxumError::rate_limit_error("").is_recoverable());
        assert!(YMAxumError::validation_error("").is_recoverable());
        assert!(!YMAxumError::auth_error("").is_recoverable());
        assert!(!YMAxumError::authz_error("").is_recoverable());
    }

    #[test]
    fn test_error_display() {
        let error = YMAxumError::command_error("Test error");
        let error_message = format!("{}", error);
        assert!(error_message.contains("Command execution error"));
        assert!(error_message.contains("Test error"));
    }

    #[test]
    fn test_error_debug() {
        let error = YMAxumError::command_error("Test error");
        let error_debug = format!("{:?}", error);
        assert!(error_debug.contains("CommandError"));
    }
}
