//! Plugin repository module
//! Provides functionality for storing, retrieving, and managing plugin data

use std::collections::HashMap;

use std::fs;
use std::path::{Path, PathBuf};

use crate::plugin_market::models::{PluginInfo, PluginVersion};

/// Repository error types
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Plugin not found
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    /// Version not found
    #[error("Version not found: {0} for plugin {1}")]
    VersionNotFound(String, String),
    /// Invalid plugin manifest
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),
    /// Repository error
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

/// Plugin repository trait
pub trait PluginRepository {
    /// Get all plugins
    fn get_all_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError>;

    /// Get plugin by name
    fn get_plugin(&self, name: &str) -> Result<PluginInfo, RepositoryError>;

    /// Search plugins
    fn search_plugins(&self, query: &str) -> Result<Vec<PluginInfo>, RepositoryError>;

    /// Get plugin versions
    fn get_plugin_versions(&self, name: &str) -> Result<Vec<PluginVersion>, RepositoryError>;

    /// Get latest plugin version
    fn get_latest_version(&self, name: &str) -> Result<PluginVersion, RepositoryError>;

    /// Add plugin to repository
    fn add_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError>;

    /// Update plugin in repository
    fn update_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError>;

    /// Remove plugin from repository
    fn remove_plugin(&mut self, name: &str) -> Result<(), RepositoryError>;

    /// Get installed plugins
    fn get_installed_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError>;

    /// Check for plugin updates
    fn check_for_updates(&self) -> Result<Vec<(PluginInfo, PluginVersion)>, RepositoryError>;
}

/// Local file system plugin repository
pub struct LocalPluginRepository {
    /// Repository directory
    pub repo_dir: PathBuf,
    /// Installed plugins directory
    pub installed_dir: PathBuf,
    /// Plugin cache
    pub plugin_cache: HashMap<String, PluginInfo>,
}

impl LocalPluginRepository {
    /// Create a new local plugin repository
    pub fn new(repo_dir: &Path, installed_dir: &Path) -> Result<Self, RepositoryError> {
        // Create directories if they don't exist
        fs::create_dir_all(repo_dir)?;
        fs::create_dir_all(installed_dir)?;

        Ok(Self {
            repo_dir: repo_dir.to_path_buf(),
            installed_dir: installed_dir.to_path_buf(),
            plugin_cache: HashMap::new(),
        })
    }
}

impl PluginRepository for LocalPluginRepository {
    fn get_all_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError> {
        // In a real implementation, this would fetch plugins from both local and remote sources
        // For now, we'll just return the cached plugins
        Ok(self.plugin_cache.values().cloned().collect())
    }

    fn get_plugin(&self, name: &str) -> Result<PluginInfo, RepositoryError> {
        self.plugin_cache
            .get(name)
            .cloned()
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn search_plugins(&self, query: &str) -> Result<Vec<PluginInfo>, RepositoryError> {
        let query_lower = query.to_lowercase();
        let results: Vec<PluginInfo> = self
            .plugin_cache
            .values()
            .filter(|plugin| {
                plugin.manifest.name.to_lowercase().contains(&query_lower)
                    || plugin
                        .manifest
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || plugin
                        .manifest
                        .keywords
                        .iter()
                        .any(|kw| kw.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect();

        Ok(results)
    }

    fn get_plugin_versions(&self, name: &str) -> Result<Vec<PluginVersion>, RepositoryError> {
        self.plugin_cache
            .get(name)
            .map(|plugin| plugin.versions.clone())
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn get_latest_version(&self, name: &str) -> Result<PluginVersion, RepositoryError> {
        self.plugin_cache
            .get(name)
            .map(|plugin| plugin.latest_version.clone())
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn add_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError> {
        self.plugin_cache
            .insert(plugin.manifest.name.clone(), plugin);
        Ok(())
    }

    fn update_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError> {
        if self.plugin_cache.contains_key(&plugin.manifest.name) {
            self.plugin_cache
                .insert(plugin.manifest.name.clone(), plugin);
            Ok(())
        } else {
            Err(RepositoryError::PluginNotFound(
                plugin.manifest.name.clone(),
            ))
        }
    }

    fn remove_plugin(&mut self, name: &str) -> Result<(), RepositoryError> {
        if self.plugin_cache.remove(name).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::PluginNotFound(name.to_string()))
        }
    }

    fn get_installed_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError> {
        let installed: Vec<PluginInfo> = self
            .plugin_cache
            .values()
            .filter(|plugin| plugin.installed)
            .cloned()
            .collect();

        Ok(installed)
    }

    fn check_for_updates(&self) -> Result<Vec<(PluginInfo, PluginVersion)>, RepositoryError> {
        // In a real implementation, this would check remote repositories for updates
        // For now, we'll just return an empty vector
        Ok(Vec::new())
    }
}

/// Remote plugin repository
pub struct RemotePluginRepository {
    /// Repository URL
    pub repo_url: String,
    /// Local cache directory
    pub cache_dir: PathBuf,
    /// Plugin cache
    pub plugin_cache: HashMap<String, PluginInfo>,
}

impl RemotePluginRepository {
    /// Create a new remote plugin repository
    pub fn new(repo_url: &str, cache_dir: &Path) -> Result<Self, RepositoryError> {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(cache_dir)?;

        Ok(Self {
            repo_url: repo_url.to_string(),
            cache_dir: cache_dir.to_path_buf(),
            plugin_cache: HashMap::new(),
        })
    }
}

impl PluginRepository for RemotePluginRepository {
    fn get_all_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError> {
        // In a real implementation, this would fetch plugins from the remote repository
        // For now, we'll just return the cached plugins
        Ok(self.plugin_cache.values().cloned().collect())
    }

    fn get_plugin(&self, name: &str) -> Result<PluginInfo, RepositoryError> {
        self.plugin_cache
            .get(name)
            .cloned()
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn search_plugins(&self, query: &str) -> Result<Vec<PluginInfo>, RepositoryError> {
        let query_lower = query.to_lowercase();
        let results: Vec<PluginInfo> = self
            .plugin_cache
            .values()
            .filter(|plugin| {
                plugin.manifest.name.to_lowercase().contains(&query_lower)
                    || plugin
                        .manifest
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || plugin
                        .manifest
                        .keywords
                        .iter()
                        .any(|kw| kw.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect();

        Ok(results)
    }

    fn get_plugin_versions(&self, name: &str) -> Result<Vec<PluginVersion>, RepositoryError> {
        self.plugin_cache
            .get(name)
            .map(|plugin| plugin.versions.clone())
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn get_latest_version(&self, name: &str) -> Result<PluginVersion, RepositoryError> {
        self.plugin_cache
            .get(name)
            .map(|plugin| plugin.latest_version.clone())
            .ok_or_else(|| RepositoryError::PluginNotFound(name.to_string()))
    }

    fn add_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError> {
        self.plugin_cache
            .insert(plugin.manifest.name.clone(), plugin);
        Ok(())
    }

    fn update_plugin(&mut self, plugin: PluginInfo) -> Result<(), RepositoryError> {
        if self.plugin_cache.contains_key(&plugin.manifest.name) {
            self.plugin_cache
                .insert(plugin.manifest.name.clone(), plugin);
            Ok(())
        } else {
            Err(RepositoryError::PluginNotFound(
                plugin.manifest.name.clone(),
            ))
        }
    }

    fn remove_plugin(&mut self, name: &str) -> Result<(), RepositoryError> {
        if self.plugin_cache.remove(name).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::PluginNotFound(name.to_string()))
        }
    }

    fn get_installed_plugins(&self) -> Result<Vec<PluginInfo>, RepositoryError> {
        // Remote repository doesn't track installed plugins
        Ok(Vec::new())
    }

    fn check_for_updates(&self) -> Result<Vec<(PluginInfo, PluginVersion)>, RepositoryError> {
        // In a real implementation, this would check the remote repository for updates
        // For now, we'll just return an empty vector
        Ok(Vec::new())
    }
}
