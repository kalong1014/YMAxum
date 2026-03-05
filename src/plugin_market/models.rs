//! Plugin market models
//! Defines data structures for plugin information, manifests, versions, etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Plugin manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Plugin license
    pub license: String,
    /// Plugin repository URL
    pub repository: String,
    /// Plugin homepage URL
    pub homepage: String,
    /// Plugin keywords
    pub keywords: Vec<String>,
    /// Plugin capabilities
    pub capabilities: Vec<String>,
    /// Plugin dependencies
    pub dependencies: Vec<String>,
    /// Plugin configuration
    pub configuration: HashMap<String, serde_json::Value>,
    /// Plugin compatibility
    pub compatibility: Compatibility,
    /// Plugin security information
    pub security: Security,
    /// Plugin performance information
    pub performance: Performance,
    /// Plugin maintenance information
    pub maintenance: Maintenance,
}

/// Compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compatibility {
    /// Minimum GUF version required
    pub guf_version: String,
    /// Minimum Rust version required
    pub rust_version: String,
    /// Supported operating systems
    pub os: Vec<String>,
}

/// Security information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    /// Required permissions
    pub permissions: Vec<String>,
    /// Whether the plugin runs in a sandbox
    pub sandboxed: bool,
}

/// Performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Performance {
    /// Memory usage level
    pub memory_usage: String,
    /// CPU usage level
    pub cpu_usage: String,
    /// Disk usage level
    pub disk_usage: String,
}

/// Maintenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maintenance {
    /// Maintenance status
    pub status: String,
    /// Last updated timestamp
    pub last_updated: String,
    /// Support email
    pub support_email: String,
}

/// Plugin version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVersion {
    /// Version string
    pub version: String,
    /// Release date
    pub release_date: SystemTime,
    /// Release notes
    pub release_notes: String,
    /// Download URL
    pub download_url: String,
    /// SHA256 hash of the plugin package
    pub sha256: String,
    /// Size in bytes
    pub size: u64,
}

/// Plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Available versions
    pub versions: Vec<PluginVersion>,
    /// Latest version
    pub latest_version: PluginVersion,
    /// Installation status
    pub installed: bool,
    /// Installed version
    pub installed_version: Option<String>,
    /// Rating
    pub rating: f32,
    /// Download count
    pub download_count: u64,
    /// Last updated timestamp
    pub last_updated: SystemTime,
}

/// Search options for plugins
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Search query
    pub query: Option<String>,
    /// Filter by categories/keywords
    pub keywords: Vec<String>,
    /// Filter by compatibility
    pub compatibility: Option<CompatibilityFilter>,
    /// Sort by
    pub sort_by: SortBy,
    /// Sort order
    pub sort_order: SortOrder,
    /// Page number
    pub page: u32,
    /// Items per page
    pub per_page: u32,
}

/// Compatibility filter
#[derive(Debug, Clone)]
pub struct CompatibilityFilter {
    /// GUF version
    pub guf_version: Option<String>,
    /// Operating system
    pub os: Option<String>,
}

/// Sort by options
#[derive(Debug, Clone, Default)]
pub enum SortBy {
    /// Sort by name
    #[default]
    Name,
    /// Sort by download count
    Downloads,
    /// Sort by rating
    Rating,
    /// Sort by last updated
    LastUpdated,
    /// Sort by version
    Version,
}

/// Sort order options
#[derive(Debug, Clone, Default)]
pub enum SortOrder {
    /// Ascending order
    #[default]
    Ascending,
    /// Descending order
    Descending,
}

/// Plugin installation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallationStatus {
    /// Not installed
    NotInstalled,
    /// Installed
    Installed {
        /// Installed version
        version: String,
        /// Installation date
        installed_date: SystemTime,
    },
    /// Installation in progress
    Installing {
        /// Progress percentage
        progress: u8,
    },
    /// Updating
    Updating {
        /// Current version
        current_version: String,
        /// New version
        new_version: String,
        /// Progress percentage
        progress: u8,
    },
    /// Uninstalling
    Uninstalling {
        /// Progress percentage
        progress: u8,
    },
    /// Error
    Error {
        /// Error message
        message: String,
    },
}

/// Plugin market statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatistics {
    /// Total number of plugins
    pub total_plugins: u64,
    /// Number of installed plugins
    pub installed_plugins: u64,
    /// Number of plugins with updates available
    pub updates_available: u64,
    /// Top rated plugins
    pub top_rated: Vec<PluginInfo>,
    /// Most downloaded plugins
    pub most_downloaded: Vec<PluginInfo>,
    /// Recently updated plugins
    pub recently_updated: Vec<PluginInfo>,
}
