// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Plugin dependency management module
//! Responsible for automatic dependency identification, version conflict detection, and automatic download

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginDependency {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Dependency type (plugin, crate, system)
    pub dep_type: DependencyType,
    /// Whether to dependency is optional
    pub optional: bool,
}

/// Dependency type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyType {
    /// Plugin dependency
    Plugin,
    /// Crate dependency
    Crate,
    /// System dependency
    System,
}

/// Dependency resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyResolution {
    /// Plugin name
    pub plugin_name: String,
    /// All dependencies (including transitive)
    pub all_dependencies: Vec<PluginDependency>,
    /// Missing dependencies
    pub missing_dependencies: Vec<PluginDependency>,
    /// Version conflicts
    pub version_conflicts: Vec<VersionConflict>,
    /// Download URLs for missing dependencies
    pub download_urls: Vec<DownloadUrl>,
}

/// Version conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConflict {
    /// Dependency name
    pub dependency_name: String,
    /// Required version
    pub required_version: String,
    /// Installed version
    pub installed_version: String,
    /// Conflict type
    pub conflict_type: ConflictType,
}

/// Conflict type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    /// Version too low
    VersionTooLow,
    /// Version too high
    VersionTooHigh,
    /// Incompatible version
    Incompatible,
}

/// Download URL information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrl {
    /// Dependency name
    pub dependency_name: String,
    /// Dependency version
    pub version: String,
    /// Download URL
    pub url: String,
    /// Checksum (SHA256)
    pub checksum: String,
}

/// Dependency manager
#[derive(Debug)]
pub struct DependencyManager {
    /// Available plugins
    pub available_plugins: HashMap<String, PluginInfo>,
    /// Installed plugins
    pub installed_plugins: HashMap<String, PluginInfo>,
    /// Dependency registry
    pub dependency_registry: HashMap<String, Vec<String>>,
}

/// Plugin information (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}

impl DependencyManager {
    /// Create new dependency manager
    pub fn new() -> Self {
        Self {
            available_plugins: HashMap::new(),
            installed_plugins: HashMap::new(),
            dependency_registry: HashMap::new(),
        }
    }

    /// Register available plugin
    pub fn register_plugin(&mut self, plugin: PluginInfo) {
        info!("Registering plugin: {} v{}", plugin.name, plugin.version);
        self.available_plugins.insert(plugin.name.clone(), plugin);
    }

    /// Register installed plugin
    pub fn register_installed_plugin(&mut self, plugin: PluginInfo) {
        info!(
            "Registering installed plugin: {} v{}",
            plugin.name, plugin.version
        );
        self.installed_plugins.insert(plugin.name.clone(), plugin);
    }

    /// Resolve dependencies for a plugin
    pub fn resolve_dependencies(&self, plugin_name: &str) -> Result<DependencyResolution, String> {
        info!("Resolving dependencies for plugin: {}", plugin_name);

        let _plugin = self
            .available_plugins
            .get(plugin_name)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

        let dependencies = self.parse_dependencies(plugin_name)?;

        let mut all_dependencies = Vec::new();
        let mut missing_dependencies = Vec::new();
        let mut version_conflicts = Vec::new();
        let mut download_urls = Vec::new();

        for dep in &dependencies {
            all_dependencies.push(dep.clone());

            if !dep.optional {
                if let Some(installed) = self.installed_plugins.get(&dep.name) {
                    if !self.is_version_compatible(&dep.version, &installed.version) {
                        version_conflicts.push(VersionConflict {
                            dependency_name: dep.name.clone(),
                            required_version: dep.version.clone(),
                            installed_version: installed.version.clone(),
                            conflict_type: ConflictType::Incompatible,
                        });
                    }
                } else {
                    missing_dependencies.push(dep.clone());
                    let url = self.generate_download_url(dep)?;
                    download_urls.push(url);
                }
            }
        }

        let resolution = DependencyResolution {
            plugin_name: plugin_name.to_string(),
            all_dependencies,
            missing_dependencies,
            version_conflicts,
            download_urls,
        };

        debug!("Dependency resolution: {:?}", resolution);
        Ok(resolution)
    }

    /// Parse dependencies from plugin manifest
    fn parse_dependencies(&self, plugin_name: &str) -> Result<Vec<PluginDependency>, String> {
        let _plugin = self
            .available_plugins
            .get(plugin_name)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

        let deps = self
            .dependency_registry
            .get(plugin_name)
            .cloned()
            .unwrap_or_default();

        let mut dependencies = Vec::new();
        for dep_str in deps {
            let parts: Vec<&str> = dep_str.split(':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid dependency format: {}", dep_str));
            }

            let name = parts[0].to_string();
            let version = parts[1].to_string();
            let dep_type = self.detect_dependency_type(&name);

            dependencies.push(PluginDependency {
                name: name.clone(),
                version: version.clone(),
                dep_type,
                optional: false,
            });
        }

        Ok(dependencies)
    }

    /// Detect dependency type
    fn detect_dependency_type(&self, name: &str) -> DependencyType {
        if self.available_plugins.contains_key(name) {
            DependencyType::Plugin
        } else if name.starts_with("crate:") {
            DependencyType::Crate
        } else {
            DependencyType::System
        }
    }

    /// Check version compatibility
    fn is_version_compatible(&self, required: &str, installed: &str) -> bool {
        let req_parts: Vec<&str> = required.split('.').collect();
        let inst_parts: Vec<&str> = installed.split('.').collect();

        if req_parts.len() != inst_parts.len() {
            return false;
        }

        // Check major version compatibility (must be same)
        if let (Some(req_major), Some(inst_major)) = (req_parts.first(), inst_parts.first()) {
            let req_major_ver = req_major.parse::<u32>().unwrap_or(0);
            let inst_major_ver = inst_major.parse::<u32>().unwrap_or(0);

            if req_major_ver != inst_major_ver {
                return false;
            }
        }

        // Check minor and patch versions (installed must be >= required)
        for (req, inst) in req_parts.iter().zip(inst_parts.iter()) {
            let req_ver = req.parse::<u32>().unwrap_or(0);
            let inst_ver = inst.parse::<u32>().unwrap_or(0);

            if inst_ver < req_ver {
                return false;
            }
        }

        true
    }

    /// Generate download URL for dependency
    fn generate_download_url(&self, dep: &PluginDependency) -> Result<DownloadUrl, String> {
        let url = match &dep.dep_type {
            DependencyType::Plugin => {
                format!(
                    "https://plugins.ymaxum.com/download/{}{}.axpl",
                    dep.name, dep.version
                )
            }
            DependencyType::Crate => {
                format!("https://crates.io/api/v1/crates/{}/download", dep.name)
            }
            DependencyType::System => {
                return Err(format!(
                    "System dependencies cannot be downloaded automatically: {}",
                    dep.name
                ));
            }
        };

        let checksum = format!("{:x}", 0);

        Ok(DownloadUrl {
            dependency_name: dep.name.clone(),
            version: dep.version.clone(),
            url,
            checksum,
        })
    }

    /// Download missing dependencies
    pub async fn download_dependencies(
        &self,
        resolution: &DependencyResolution,
    ) -> Result<(), String> {
        info!(
            "Downloading {} missing dependencies",
            resolution.download_urls.len()
        );

        for url in &resolution.download_urls {
            info!("Downloading: {} v{}", url.dependency_name, url.version);

            match self.download_dependency(url).await {
                Ok(_) => {
                    info!(
                        "Successfully downloaded: {} v{}",
                        url.dependency_name, url.version
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to download: {} v{} - {}",
                        url.dependency_name, url.version, e
                    );
                    return Err(format!("Failed to download dependency: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Download a single dependency
    async fn download_dependency(&self, url: &DownloadUrl) -> Result<(), String> {
        use reqwest::Client;
        use tokio::io::AsyncWriteExt;

        let client = Client::new();
        let response = client
            .get(&url.url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch URL: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed with status: {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read bytes: {}", e))?;

        let file_path = format!("deps/{}.axpl", url.dependency_name);
        let mut file = tokio::fs::File::create(&file_path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        file.write_all(&bytes)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        info!("Downloaded {} bytes to {}", bytes.len(), file_path);
        Ok(())
    }

    /// Get all available plugins
    pub fn get_available_plugins(&self) -> Vec<PluginInfo> {
        self.available_plugins.values().cloned().collect()
    }

    /// Get all installed plugins
    pub fn get_installed_plugins(&self) -> Vec<PluginInfo> {
        self.installed_plugins.values().cloned().collect()
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_manager_creation() {
        let manager = DependencyManager::new();

        assert_eq!(manager.get_available_plugins().len(), 0);
        assert_eq!(manager.get_installed_plugins().len(), 0);
    }

    #[test]
    fn test_plugin_registration() {
        let mut manager = DependencyManager::new();

        let plugin = PluginInfo {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
        };

        manager.register_plugin(plugin.clone());
        assert_eq!(manager.get_available_plugins().len(), 1);

        manager.register_installed_plugin(plugin);
        assert_eq!(manager.get_installed_plugins().len(), 1);
    }

    #[test]
    fn test_version_compatibility() {
        let manager = DependencyManager::new();

        assert!(manager.is_version_compatible("1.0.0", "1.0.0"));
        assert!(manager.is_version_compatible("1.0.0", "1.1.0"));
        assert!(!manager.is_version_compatible("1.1.0", "1.0.0"));
        assert!(!manager.is_version_compatible("1.0.0", "2.0.0"));
    }

    #[test]
    fn test_dependency_type_detection() {
        let mut manager = DependencyManager::new();

        let plugin = PluginInfo {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
        };

        manager.register_plugin(plugin);

        assert!(matches!(
            manager.detect_dependency_type("test_plugin"),
            DependencyType::Plugin
        ));
        assert!(matches!(
            manager.detect_dependency_type("crate:test"),
            DependencyType::Crate
        ));
        assert!(matches!(
            manager.detect_dependency_type("system:lib"),
            DependencyType::System
        ));
    }

    #[test]
    fn test_download_url_generation() {
        let manager = DependencyManager::new();

        let plugin_dep = PluginDependency {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            dep_type: DependencyType::Plugin,
            optional: false,
        };

        let url = manager.generate_download_url(&plugin_dep).unwrap();
        assert!(url.url.contains("test_plugin"));
        assert!(url.url.contains("1.0.0"));
        assert!(url.url.contains(".axpl"));
    }
}

