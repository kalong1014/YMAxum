//! Plugin market core functionality
//! Provides core functionality for the plugin market, including plugin installation, uninstallation, and updates

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use crate::plugin_market::models::{
    Compatibility, InstallationStatus, Maintenance, Performance, PluginInfo, PluginManifest,
    PluginVersion, Security,
};
use crate::plugin_market::repository::{PluginRepository, RepositoryError};

/// Plugin market
pub struct PluginMarket {
    /// Local plugin repository
    local_repo: Box<dyn PluginRepository>,
    /// Remote plugin repositories
    remote_repos: Vec<Box<dyn PluginRepository>>,
    /// Installation statuses
    installation_statuses: HashMap<String, InstallationStatus>,
    /// Plugins directory
    plugins_dir: PathBuf,
}

impl PluginMarket {
    /// Create a new plugin market
    pub fn new(plugins_dir: &Path) -> Result<Self, Box<dyn Error>> {
        // Create plugins directory if it doesn't exist
        fs::create_dir_all(plugins_dir)?;

        // Create local repository
        let local_repo_dir = plugins_dir.join("repo");
        let mut local_repo = Box::new(
            crate::plugin_market::repository::LocalPluginRepository::new(
                &local_repo_dir,
                plugins_dir,
            )?,
        );

        // Add example plugins to local repository
        Self::add_example_plugins(local_repo.as_mut())?;

        Ok(Self {
            local_repo,
            remote_repos: Vec::new(),
            installation_statuses: HashMap::new(),
            plugins_dir: plugins_dir.to_path_buf(),
        })
    }

    /// Add example plugins to repository
    fn add_example_plugins(repo: &mut dyn PluginRepository) -> Result<(), Box<dyn Error>> {
        use std::time::{Duration, SystemTime};

        // Create example plugins
        let example_plugins = vec![
            // Auth plugin
            PluginInfo {
                manifest: PluginManifest {
                    name: "auth-plugin".to_string(),
                    version: "1.2.0".to_string(),
                    author: "GUF Team".to_string(),
                    description: "Authentication plugin with JWT support".to_string(),
                    license: "MIT".to_string(),
                    repository: "https://github.com/guf-team/auth-plugin".to_string(),
                    homepage: "https://guf-plugin-market.example.com/plugins/auth-plugin".to_string(),
                    keywords: ["auth", "jwt", "security"].iter().map(|s| s.to_string()).collect(),
                    capabilities: ["authentication", "authorization"].iter().map(|s| s.to_string()).collect(),
                    dependencies: ["serde", "jsonwebtoken"].iter().map(|s| s.to_string()).collect(),
                    configuration: HashMap::new(),
                    compatibility: Compatibility {
                        guf_version: "1.0.0".to_string(),
                        rust_version: "1.93.0".to_string(),
                        os: ["windows", "linux", "macos"].iter().map(|s| s.to_string()).collect(),
                    },
                    security: Security {
                        permissions: ["read", "write"].iter().map(|s| s.to_string()).collect(),
                        sandboxed: true,
                    },
                    performance: Performance {
                        memory_usage: "low".to_string(),
                        cpu_usage: "low".to_string(),
                        disk_usage: "low".to_string(),
                    },
                    maintenance: Maintenance {
                        status: "active".to_string(),
                        last_updated: "2024-01-01".to_string(),
                        support_email: "support@guf-team.com".to_string(),
                    },
                },
                versions: vec![
                    PluginVersion {
                        version: "1.0.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 90),
                        release_notes: "Initial release".to_string(),
                        download_url: "https://example.com/plugins/auth-plugin-1.0.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024,
                    },
                    PluginVersion {
                        version: "1.1.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 30),
                        release_notes: "Added OAuth support".to_string(),
                        download_url: "https://example.com/plugins/auth-plugin-1.1.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 2,
                    },
                    PluginVersion {
                        version: "1.2.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 7),
                        release_notes: "Fixed security issues".to_string(),
                        download_url: "https://example.com/plugins/auth-plugin-1.2.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 2,
                    },
                ],
                latest_version: PluginVersion {
                    version: "1.2.0".to_string(),
                    release_date: SystemTime::now() - Duration::from_secs(86400 * 7),
                    release_notes: "Fixed security issues".to_string(),
                    download_url: "https://example.com/plugins/auth-plugin-1.2.0.zip".to_string(),
                    sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                    size: 1024 * 1024 * 2,
                },
                installed: false,
                installed_version: None,
                rating: 4.8,
                download_count: 1234,
                last_updated: SystemTime::now() - Duration::from_secs(86400 * 7),
            },
            // Analytics plugin
            PluginInfo {
                manifest: PluginManifest {
                    name: "analytics-plugin".to_string(),
                    version: "2.0.1".to_string(),
                    author: "Analytics Team".to_string(),
                    description: "Analytics plugin for user behavior tracking".to_string(),
                    license: "MIT".to_string(),
                    repository: "https://github.com/analytics-team/analytics-plugin".to_string(),
                    homepage: "https://guf-plugin-market.example.com/plugins/analytics-plugin".to_string(),
                    keywords: ["analytics", "tracking", "metrics"].iter().map(|s| s.to_string()).collect(),
                    capabilities: ["data_collection", "reporting"].iter().map(|s| s.to_string()).collect(),
                    dependencies: ["serde", "tokio"].iter().map(|s| s.to_string()).collect(),
                    configuration: HashMap::new(),
                    compatibility: Compatibility {
                        guf_version: "1.0.0".to_string(),
                        rust_version: "1.93.0".to_string(),
                        os: ["windows", "linux", "macos"].iter().map(|s| s.to_string()).collect(),
                    },
                    security: Security {
                        permissions: ["read"].iter().map(|s| s.to_string()).collect(),
                        sandboxed: true,
                    },
                    performance: Performance {
                        memory_usage: "medium".to_string(),
                        cpu_usage: "medium".to_string(),
                        disk_usage: "high".to_string(),
                    },
                    maintenance: Maintenance {
                        status: "active".to_string(),
                        last_updated: "2024-01-01".to_string(),
                        support_email: "support@analytics-team.com".to_string(),
                    },
                },
                versions: vec![
                    PluginVersion {
                        version: "1.0.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 180),
                        release_notes: "Initial release".to_string(),
                        download_url: "https://example.com/plugins/analytics-plugin-1.0.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 5,
                    },
                    PluginVersion {
                        version: "2.0.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 30),
                        release_notes: "Complete rewrite with better performance".to_string(),
                        download_url: "https://example.com/plugins/analytics-plugin-2.0.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 3,
                    },
                    PluginVersion {
                        version: "2.0.1".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 3),
                        release_notes: "Fixed minor bugs".to_string(),
                        download_url: "https://example.com/plugins/analytics-plugin-2.0.1.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 3,
                    },
                ],
                latest_version: PluginVersion {
                    version: "2.0.1".to_string(),
                    release_date: SystemTime::now() - Duration::from_secs(86400 * 3),
                    release_notes: "Fixed minor bugs".to_string(),
                    download_url: "https://example.com/plugins/analytics-plugin-2.0.1.zip".to_string(),
                    sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                    size: 1024 * 1024 * 3,
                },
                installed: false,
                installed_version: None,
                rating: 4.5,
                download_count: 892,
                last_updated: SystemTime::now() - Duration::from_secs(86400 * 3),
            },
            // Payment plugin
            PluginInfo {
                manifest: PluginManifest {
                    name: "payment-plugin".to_string(),
                    version: "1.5.3".to_string(),
                    author: "Payment Team".to_string(),
                    description: "Payment processing plugin with multiple payment gateways".to_string(),
                    license: "MIT".to_string(),
                    repository: "https://github.com/payment-team/payment-plugin".to_string(),
                    homepage: "https://guf-plugin-market.example.com/plugins/payment-plugin".to_string(),
                    keywords: ["payment", "gateway", "processing"].iter().map(|s| s.to_string()).collect(),
                    capabilities: ["payment_processing", "transaction_management"].iter().map(|s| s.to_string()).collect(),
                    dependencies: ["serde", "reqwest"].iter().map(|s| s.to_string()).collect(),
                    configuration: HashMap::new(),
                    compatibility: Compatibility {
                        guf_version: "1.0.0".to_string(),
                        rust_version: "1.93.0".to_string(),
                        os: ["windows", "linux"].iter().map(|s| s.to_string()).collect(),
                    },
                    security: Security {
                        permissions: ["read", "write"].iter().map(|s| s.to_string()).collect(),
                        sandboxed: true,
                    },
                    performance: Performance {
                        memory_usage: "medium".to_string(),
                        cpu_usage: "low".to_string(),
                        disk_usage: "low".to_string(),
                    },
                    maintenance: Maintenance {
                        status: "active".to_string(),
                        last_updated: "2024-01-01".to_string(),
                        support_email: "support@payment-team.com".to_string(),
                    },
                },
                versions: vec![
                    PluginVersion {
                        version: "1.0.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 270),
                        release_notes: "Initial release".to_string(),
                        download_url: "https://example.com/plugins/payment-plugin-1.0.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 4,
                    },
                    PluginVersion {
                        version: "1.5.0".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 90),
                        release_notes: "Added new payment gateways".to_string(),
                        download_url: "https://example.com/plugins/payment-plugin-1.5.0.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 6,
                    },
                    PluginVersion {
                        version: "1.5.3".to_string(),
                        release_date: SystemTime::now() - Duration::from_secs(86400 * 14),
                        release_notes: "Fixed security vulnerabilities".to_string(),
                        download_url: "https://example.com/plugins/payment-plugin-1.5.3.zip".to_string(),
                        sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                        size: 1024 * 1024 * 6,
                    },
                ],
                latest_version: PluginVersion {
                    version: "1.5.3".to_string(),
                    release_date: SystemTime::now() - Duration::from_secs(86400 * 14),
                    release_notes: "Fixed security vulnerabilities".to_string(),
                    download_url: "https://example.com/plugins/payment-plugin-1.5.3.zip".to_string(),
                    sha256: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
                    size: 1024 * 1024 * 6,
                },
                installed: false,
                installed_version: None,
                rating: 4.7,
                download_count: 567,
                last_updated: SystemTime::now() - Duration::from_secs(86400 * 14),
            },
        ];

        // Add example plugins to local repository
        for plugin in example_plugins {
            repo.add_plugin(plugin)?;
        }

        Ok(())
    }

    /// Add remote repository
    pub fn add_remote_repo(&mut self, repo_url: &str) -> Result<(), Box<dyn Error>> {
        let cache_dir = self
            .plugins_dir
            .join("cache")
            .join(format!("remote_{}", repo_url.replace(['/', ':'], "_")));
        let remote_repo = Box::new(
            crate::plugin_market::repository::RemotePluginRepository::new(repo_url, &cache_dir)?,
        );

        self.remote_repos.push(remote_repo);
        Ok(())
    }

    /// Get all plugins
    pub fn get_all_plugins(&self) -> Result<Vec<PluginInfo>, Box<dyn Error>> {
        let mut all_plugins = self.local_repo.get_all_plugins()?;

        // Add plugins from remote repositories
        for remote_repo in &self.remote_repos {
            let remote_plugins = remote_repo.get_all_plugins()?;
            all_plugins.extend(remote_plugins);
        }

        Ok(all_plugins)
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Result<PluginInfo, Box<dyn Error>> {
        // First check local repository
        match self.local_repo.get_plugin(name) {
            Ok(plugin) => Ok(plugin),
            Err(_) => {
                // Check remote repositories
                for remote_repo in &self.remote_repos {
                    match remote_repo.get_plugin(name) {
                        Ok(plugin) => return Ok(plugin),
                        Err(_) => continue,
                    }
                }

                Err(Box::new(RepositoryError::PluginNotFound(name.to_string())))
            }
        }
    }

    /// Search plugins
    pub fn search_plugins(&self, query: &str) -> Result<Vec<PluginInfo>, Box<dyn Error>> {
        let mut results = self.local_repo.search_plugins(query)?;

        // Search remote repositories
        for remote_repo in &self.remote_repos {
            let remote_results = remote_repo.search_plugins(query)?;
            results.extend(remote_results);
        }

        Ok(results)
    }

    /// Get installed plugins
    pub fn get_installed_plugins(&self) -> Result<Vec<PluginInfo>, Box<dyn Error>> {
        self.local_repo
            .get_installed_plugins()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    /// Check for plugin updates
    pub fn check_for_updates(&self) -> Result<Vec<(PluginInfo, PluginVersion)>, Box<dyn Error>> {
        let mut updates = self.local_repo.check_for_updates()?;

        // Check remote repositories for updates
        for remote_repo in &self.remote_repos {
            let remote_updates = remote_repo.check_for_updates()?;
            updates.extend(remote_updates);
        }

        Ok(updates)
    }

    /// Install plugin
    pub fn install_plugin(
        &mut self,
        plugin_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        // Set installation status to installing
        self.installation_statuses.insert(
            plugin_name.to_string(),
            InstallationStatus::Installing { progress: 0 },
        );

        // Spawn a thread to perform the installation
        let plugin_name_clone = plugin_name.to_string();
        let _versions_clone = version.map(|v| v.to_string());
        let _plugins_dir_clone = self.plugins_dir.clone();

        thread::spawn(move || {
            // Simulate installation process
            for _progress in 0..=100 {
                thread::sleep(std::time::Duration::from_millis(50));
                // In a real implementation, we would update the installation status here
            }

            // In a real implementation, we would download and install the plugin here
            // For now, we'll just simulate a successful installation
            println!("Plugin {} installed successfully", plugin_name_clone);
        });

        Ok(())
    }

    /// Uninstall plugin
    pub fn uninstall_plugin(&mut self, plugin_name: &str) -> Result<(), Box<dyn Error>> {
        // Set installation status to uninstalling
        self.installation_statuses.insert(
            plugin_name.to_string(),
            InstallationStatus::Uninstalling { progress: 0 },
        );

        // Spawn a thread to perform the uninstallation
        let plugin_name_clone = plugin_name.to_string();
        let _plugins_dir_clone = self.plugins_dir.clone();

        thread::spawn(move || {
            // Simulate uninstallation process
            for _progress in 0..=100 {
                thread::sleep(std::time::Duration::from_millis(30));
                // In a real implementation, we would update the uninstallation status here
            }

            // In a real implementation, we would uninstall the plugin here
            // For now, we'll just simulate a successful uninstallation
            println!("Plugin {} uninstalled successfully", plugin_name_clone);
        });

        Ok(())
    }

    /// Update plugin
    pub fn update_plugin(&mut self, plugin_name: &str) -> Result<(), Box<dyn Error>> {
        // Get plugin info
        let plugin = self.get_plugin(plugin_name)?;
        let current_version = plugin
            .installed_version
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let new_version = plugin.latest_version.version.clone();

        // Set installation status to updating
        self.installation_statuses.insert(
            plugin_name.to_string(),
            InstallationStatus::Updating {
                current_version: current_version.clone(),
                new_version: new_version.clone(),
                progress: 0,
            },
        );

        // Spawn a thread to perform the update
        let plugin_name_clone = plugin_name.to_string();
        let current_version_clone = current_version;
        let new_version_clone = new_version;
        let _plugins_dir_clone = self.plugins_dir.clone();

        thread::spawn(move || {
            // Simulate update process
            for _progress in 0..=100 {
                thread::sleep(std::time::Duration::from_millis(40));
                // In a real implementation, we would update the update status here
            }

            // In a real implementation, we would download and update the plugin here
            // For now, we'll just simulate a successful update
            println!(
                "Plugin {} updated from {} to {} successfully",
                plugin_name_clone, current_version_clone, new_version_clone
            );
        });

        Ok(())
    }

    /// Get installation status
    pub fn get_installation_status(&self, plugin_name: &str) -> Option<InstallationStatus> {
        self.installation_statuses.get(plugin_name).cloned()
    }

    /// Refresh plugin list
    pub fn refresh_plugins(&mut self) -> Result<(), Box<dyn Error>> {
        // In a real implementation, this would refresh the plugin list from remote repositories
        // For now, we'll just simulate a refresh
        Ok(())
    }

    /// Get market statistics
    pub fn get_statistics(
        &self,
    ) -> Result<crate::plugin_market::models::MarketStatistics, Box<dyn Error>> {
        let all_plugins = self.get_all_plugins()?;
        let installed_plugins = self.get_installed_plugins()?;
        let updates_available = self.check_for_updates()?;

        // Get top rated plugins
        let mut top_rated = all_plugins.clone();
        top_rated.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let top_rated = top_rated.into_iter().take(5).collect();

        // Get most downloaded plugins
        let mut most_downloaded = all_plugins.clone();
        most_downloaded.sort_by(|a, b| b.download_count.cmp(&a.download_count));
        let most_downloaded = most_downloaded.into_iter().take(5).collect();

        // Get recently updated plugins
        let mut recently_updated = all_plugins.clone();
        recently_updated.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));
        let recently_updated = recently_updated.into_iter().take(5).collect();

        Ok(crate::plugin_market::models::MarketStatistics {
            total_plugins: all_plugins.len() as u64,
            installed_plugins: installed_plugins.len() as u64,
            updates_available: updates_available.len() as u64,
            top_rated,
            most_downloaded,
            recently_updated,
        })
    }
}
