//! Configuration validation module
//! Provides configuration validation, type checking, version control, and migration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Configuration validation error
#[derive(Error, Debug)]
pub enum ConfigValidationError {
    #[error("Configuration key '{key}' is invalid: {reason}")]
    InvalidKey { key: String, reason: String },

    #[error("Configuration value '{key}' is invalid: {reason}")]
    InvalidValue { key: String, reason: String },

    #[error("Configuration type mismatch for key '{key}': expected {expected}, found {found}")]
    TypeMismatch {
        key: String,
        expected: String,
        found: String,
    },

    #[error("Configuration version '{version}' is not supported")]
    UnsupportedVersion { version: String },

    #[error("Configuration migration failed: {reason}")]
    MigrationFailed { reason: String },

    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Configuration file read error: {reason}")]
    ReadError { reason: String },

    #[error("Configuration file write error: {reason}")]
    WriteError { reason: String },
}

/// Configuration item type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigItemType {
    /// String type
    String,
    /// Integer type
    Integer,
    /// Boolean type
    Boolean,
    /// Float type
    Float,
    /// Array type
    Array,
    /// Object type
    Object,
}

/// Configuration item validation rule
#[derive(Debug, Clone)]
pub struct ConfigValidationRule {
    /// Configuration key
    pub key: String,
    /// Configuration item type
    pub item_type: ConfigItemType,
    /// Is required
    pub required: bool,
    /// Default value
    pub default_value: Option<String>,
    /// Minimum value (for Integer, Float)
    pub min_value: Option<f64>,
    /// Maximum value (for Integer, Float)
    pub max_value: Option<f64>,
    /// Allowed values (for String, Integer)
    pub allowed_values: Option<Vec<String>>,
    /// Regex pattern (for String)
    pub pattern: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// Configuration version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigVersion {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
}

impl ConfigVersion {
    /// Create new configuration version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse version from string
    pub fn parse(version_str: &str) -> Result<Self, ConfigValidationError> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err(ConfigValidationError::UnsupportedVersion {
                version: version_str.to_string(),
            });
        }

        let major =
            parts[0]
                .parse::<u32>()
                .map_err(|_| ConfigValidationError::UnsupportedVersion {
                    version: version_str.to_string(),
                })?;
        let minor =
            parts[1]
                .parse::<u32>()
                .map_err(|_| ConfigValidationError::UnsupportedVersion {
                    version: version_str.to_string(),
                })?;
        let patch =
            parts[2]
                .parse::<u32>()
                .map_err(|_| ConfigValidationError::UnsupportedVersion {
                    version: version_str.to_string(),
                })?;

        Ok(Self::new(major, minor, patch))
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Compare versions
    pub fn compare(&self, other: &ConfigVersion) -> std::cmp::Ordering {
        if self.major != other.major {
            return self.major.cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.cmp(&other.minor);
        }
        self.patch.cmp(&other.patch)
    }

    /// Check if versions are equal
    pub fn is_equal(&self, other: &ConfigVersion) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

/// Configuration migration
#[derive(Debug, Clone)]
pub struct ConfigMigration {
    /// Source version
    pub from_version: ConfigVersion,
    /// Target version
    pub to_version: ConfigVersion,
    /// Migration function
    pub migrate_fn:
        fn(&HashMap<String, String>) -> Result<HashMap<String, String>, ConfigValidationError>,
}

/// Configuration validator
pub struct ConfigValidator {
    /// Validation rules
    pub rules: HashMap<String, ConfigValidationRule>,
    /// Current configuration version
    pub current_version: ConfigVersion,
    /// Supported versions
    pub supported_versions: Vec<ConfigVersion>,
    /// Migrations
    pub migrations: Vec<ConfigMigration>,
}

impl ConfigValidator {
    /// Create new configuration validator
    pub fn new(current_version: ConfigVersion) -> Self {
        let version_clone = current_version.clone();
        Self {
            rules: HashMap::new(),
            current_version,
            supported_versions: vec![version_clone],
            migrations: Vec::new(),
        }
    }

    /// Add validation rule
    pub fn add_rule(&mut self, rule: ConfigValidationRule) {
        self.rules.insert(rule.key.clone(), rule);
    }

    /// Add supported version
    pub fn add_supported_version(&mut self, version: ConfigVersion) {
        if !self.supported_versions.contains(&version) {
            self.supported_versions.push(version);
        }
    }

    /// Add migration
    pub fn add_migration(&mut self, migration: ConfigMigration) {
        self.migrations.push(migration);
    }

    /// Validate configuration
    pub fn validate(&self, config: &HashMap<String, String>) -> Result<(), ConfigValidationError> {
        // Check required keys
        for (key, rule) in &self.rules {
            if rule.required && !config.contains_key(key) {
                return Err(ConfigValidationError::InvalidKey {
                    key: key.clone(),
                    reason: format!("required configuration key '{}' is missing", key),
                });
            }
        }

        // Validate each configuration item
        for (key, value) in config {
            if let Some(rule) = self.rules.get(key) {
                self.validate_value(key, value, rule)?;
            }
        }

        Ok(())
    }

    /// Validate configuration value
    fn validate_value(
        &self,
        key: &str,
        value: &str,
        rule: &ConfigValidationRule,
    ) -> Result<(), ConfigValidationError> {
        // Type validation
        match rule.item_type {
            ConfigItemType::Integer => {
                if value.parse::<i64>().is_err() {
                    return Err(ConfigValidationError::TypeMismatch {
                        key: key.to_string(),
                        expected: "Integer".to_string(),
                        found: "String".to_string(),
                    });
                }
            }
            ConfigItemType::Float => {
                if value.parse::<f64>().is_err() {
                    return Err(ConfigValidationError::TypeMismatch {
                        key: key.to_string(),
                        expected: "Float".to_string(),
                        found: "String".to_string(),
                    });
                }
            }
            ConfigItemType::Boolean => {
                if value != "true" && value != "false" {
                    return Err(ConfigValidationError::TypeMismatch {
                        key: key.to_string(),
                        expected: "Boolean".to_string(),
                        found: "String".to_string(),
                    });
                }
            }
            ConfigItemType::String => {
                // Check regex pattern
                if let Some(pattern) = &rule.pattern {
                    let re = regex::Regex::new(pattern).map_err(|e| {
                        ConfigValidationError::InvalidValue {
                            key: key.to_string(),
                            reason: format!("invalid regex pattern: {}", e),
                        }
                    })?;
                    if !re.is_match(value) {
                        return Err(ConfigValidationError::InvalidValue {
                            key: key.to_string(),
                            reason: format!("value does not match pattern '{}'", pattern),
                        });
                    }
                }
            }
            _ => {}
        }

        // Check allowed values
        if let Some(allowed_values) = &rule.allowed_values
            && !allowed_values.contains(&value.to_string())
        {
            return Err(ConfigValidationError::InvalidValue {
                key: key.to_string(),
                reason: format!(
                    "value '{}' is not in allowed values: {:?}",
                    value, allowed_values
                ),
            });
        }

        // Check min/max values for numeric types
        if rule.item_type == ConfigItemType::Integer || rule.item_type == ConfigItemType::Float {
            let num_value =
                value
                    .parse::<f64>()
                    .map_err(|_| ConfigValidationError::InvalidValue {
                        key: key.to_string(),
                        reason: "failed to parse as number".to_string(),
                    })?;

            if let Some(min_value) = rule.min_value
                && num_value < min_value
            {
                return Err(ConfigValidationError::InvalidValue {
                    key: key.to_string(),
                    reason: format!("value {} is less than minimum {}", num_value, min_value),
                });
            }

            if let Some(max_value) = rule.max_value
                && num_value > max_value
            {
                return Err(ConfigValidationError::InvalidValue {
                    key: key.to_string(),
                    reason: format!("value {} is greater than maximum {}", num_value, max_value),
                });
            }
        }

        Ok(())
    }

    /// Validate configuration version
    pub fn validate_version(&self, version: &ConfigVersion) -> Result<(), ConfigValidationError> {
        if !self.supported_versions.contains(version) {
            return Err(ConfigValidationError::UnsupportedVersion {
                version: version.to_string(),
            });
        }
        Ok(())
    }

    /// Migrate configuration
    pub fn migrate(
        &self,
        config: &HashMap<String, String>,
        from_version: &ConfigVersion,
        to_version: &ConfigVersion,
    ) -> Result<HashMap<String, String>, ConfigValidationError> {
        // Check if migration is needed
        if from_version.is_equal(to_version) {
            return Ok(config.clone());
        }

        // Find migration path
        let mut current_version = from_version.clone();
        let mut migrated_config = config.clone();

        while !current_version.is_equal(to_version) {
            let mut found_migration = false;

            for migration in &self.migrations {
                if migration.from_version.is_equal(&current_version) {
                    migrated_config = (migration.migrate_fn)(&migrated_config)?;
                    current_version = migration.to_version.clone();
                    found_migration = true;
                    break;
                }
            }

            if !found_migration {
                return Err(ConfigValidationError::MigrationFailed {
                    reason: format!(
                        "no migration found from version {} to {}",
                        current_version.to_string(),
                        to_version.to_string()
                    ),
                });
            }
        }

        Ok(migrated_config)
    }

    /// Get default configuration
    pub fn get_default_config(&self) -> HashMap<String, String> {
        let mut default_config = HashMap::new();

        for (key, rule) in &self.rules {
            if let Some(default_value) = &rule.default_value {
                default_config.insert(key.clone(), default_value.clone());
            }
        }

        default_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_version_parse() {
        let version = ConfigVersion::parse("1.0.0").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn test_config_version_compare() {
        let v1 = ConfigVersion::parse("1.0.0").unwrap();
        let v2 = ConfigVersion::parse("2.0.0").unwrap();
        assert_eq!(v1.compare(&v2), std::cmp::Ordering::Less);

        let v1 = ConfigVersion::parse("1.0.0").unwrap();
        let v2 = ConfigVersion::parse("1.1.0").unwrap();
        assert_eq!(v1.compare(&v2), std::cmp::Ordering::Less);

        let v1 = ConfigVersion::parse("1.0.0").unwrap();
        let v2 = ConfigVersion::parse("1.0.1").unwrap();
        assert_eq!(v1.compare(&v2), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_config_validator() {
        let mut validator = ConfigValidator::new(ConfigVersion::new(1, 0, 0));

        validator.add_rule(ConfigValidationRule {
            key: "port".to_string(),
            item_type: ConfigItemType::Integer,
            required: true,
            default_value: Some("8080".to_string()),
            min_value: Some(1024.0),
            max_value: Some(65535.0),
            allowed_values: None,
            pattern: None,
            description: Some("Server port".to_string()),
        });

        let mut config = HashMap::new();
        config.insert("port".to_string(), "8080".to_string());

        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_config_validator_invalid_type() {
        let mut validator = ConfigValidator::new(ConfigVersion::new(1, 0, 0));

        validator.add_rule(ConfigValidationRule {
            key: "port".to_string(),
            item_type: ConfigItemType::Integer,
            required: true,
            default_value: Some("8080".to_string()),
            min_value: Some(1024.0),
            max_value: Some(65535.0),
            allowed_values: None,
            pattern: None,
            description: Some("Server port".to_string()),
        });

        let mut config = HashMap::new();
        config.insert("port".to_string(), "invalid".to_string());

        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_config_validator_out_of_range() {
        let mut validator = ConfigValidator::new(ConfigVersion::new(1, 0, 0));

        validator.add_rule(ConfigValidationRule {
            key: "port".to_string(),
            item_type: ConfigItemType::Integer,
            required: true,
            default_value: Some("8080".to_string()),
            min_value: Some(1024.0),
            max_value: Some(65535.0),
            allowed_values: None,
            pattern: None,
            description: Some("Server port".to_string()),
        });

        let mut config = HashMap::new();
        config.insert("port".to_string(), "100".to_string());

        assert!(validator.validate(&config).is_err());
    }
}
