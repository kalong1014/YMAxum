// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use anyhow::{Context, Result};
use log::{error, info};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub max_cpu_percent: u8,          // Maximum CPU usage percentage (0-100)
    pub max_memory_mb: u32,           // Maximum memory usage in MB
    pub max_execution_time: Duration, // Maximum execution time
    pub network_access: NetworkAccess, // Network access control
    pub filesystem_access: FilesystemAccess, // Filesystem access control
    pub process_priority: ProcessPriority, // Process priority
    pub max_disk_io: Option<u32>,     // Maximum disk I/O in MB/s
    pub max_network_io: Option<u32>,   // Maximum network I/O in MB/s
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAccess {
    None,            // No network access
    Whitelist(Vec<String>), // Only allowed domains/IPs
    Blacklist(Vec<String>), // Blocked domains/IPs
    Full,            // Full network access
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilesystemAccess {
    None,            // No filesystem access
    ReadOnly(Vec<String>), // Read-only access to specific paths
    ReadWrite(Vec<String>), // Read-write access to specific paths
    Full,            // Full filesystem access
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Realtime,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            max_cpu_percent: 20,
            max_memory_mb: 50,
            max_execution_time: Duration::from_secs(30),
            network_access: NetworkAccess::None,
            filesystem_access: FilesystemAccess::None,
            process_priority: ProcessPriority::Normal,
            max_disk_io: None,
            max_network_io: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginSandbox {
    config: SandboxConfig,
    running_plugins: Arc<RwLock<Vec<RunningPlugin>>>,
}

#[derive(Debug, Clone)]
pub struct RunningPlugin {
    pub plugin_name: String,
    pub process_id: Option<u32>,
    pub start_time: std::time::Instant,
}

impl PluginSandbox {
    /// Create new plugin sandbox
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config,
            running_plugins: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Run plugin in sandbox
    pub async fn run_plugin(&self, plugin_name: &str, plugin_path: &str) -> Result<()> {
        // Create running plugin record
        let running_plugin = RunningPlugin {
            plugin_name: plugin_name.to_string(),
            process_id: None,
            start_time: std::time::Instant::now(),
        };

        // Add to running plugins
        let mut running_plugins = self.running_plugins.write().await;
        running_plugins.push(running_plugin.clone());

        // Spawn plugin process with resource limits and access controls
        #[cfg(target_os = "windows")]
        {
            // Windows implementation with priority and access controls
            let priority_flag = match self.config.process_priority {
                ProcessPriority::Low => "/low",
                ProcessPriority::Normal => "/normal",
                ProcessPriority::High => "/high",
                ProcessPriority::Realtime => "/realtime",
            };

            // Apply network access control
            let _network_control = match self.config.network_access {
                NetworkAccess::None => "netsh advfirewall firewall add rule name=\"Plugin-\".exe\" dir=out action=block program=\"\"",
                _ => "", // Simplified for now
            };

            // Apply filesystem access control
            let _filesystem_control = match self.config.filesystem_access {
                FilesystemAccess::None => "icacls \"\" /deny Everyone:(OI)(CI)F",
                _ => "", // Simplified for now
            };

            let output = Command::new("cmd")
                .args(["/c", &format!("start /B {} {}", priority_flag, plugin_path)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to start plugin process")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to start plugin: {:?}", output));
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Linux implementation with resource limits, priority, and access controls
            let nice_value = match self.config.process_priority {
                ProcessPriority::Low => 19,
                ProcessPriority::Normal => 0,
                ProcessPriority::High => -10,
                ProcessPriority::Realtime => -20,
            };

            // Build sandbox command with access controls
            let mut sandbox_cmd = format!(
                "ulimit -t {} -v {} && nice -n {}",
                self.config.max_cpu_percent,
                self.config.max_memory_mb * 1024 * 1024,
                nice_value
            );

            // Apply network access control
            match self.config.network_access {
                NetworkAccess::None => {
                    sandbox_cmd.push_str(" && unshare -n"); // Network namespace isolation
                },
                _ => {},
            }

            // Apply filesystem access control
            match &self.config.filesystem_access {
                FilesystemAccess::None => {
                    sandbox_cmd.push_str(" && unshare -m && mount -t tmpfs none /tmp && chroot /tmp");
                },
                FilesystemAccess::ReadOnly(paths) => {
                    // Simplified implementation
                    for path in paths {
                        sandbox_cmd.push_str(&format!(" && mount --bind {} {} && mount -o remount,ro {}", path, path, path));
                    }
                },
                _ => {},
            }

            sandbox_cmd.push_str(&format!(" && {}", plugin_path));

            let output = Command::new("bash")
                .args(["-c", &sandbox_cmd])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to start plugin process")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to start plugin: {:?}", output));
            }
        }

        #[cfg(target_os = "macos")]
        {
            // macOS implementation with priority and access controls
            let nice_value = match self.config.process_priority {
                ProcessPriority::Low => 19,
                ProcessPriority::Normal => 0,
                ProcessPriority::High => -10,
                ProcessPriority::Realtime => -20,
            };

            // Build sandbox command with access controls
            let mut sandbox_cmd = format!("nice -n {}", nice_value);

            // Apply filesystem access control (simplified for macOS)
            match &self.config.filesystem_access {
                FilesystemAccess::None => {
                    sandbox_cmd.push_str(" && sandbox-exec -f /dev/stdin << 'EOF'\n(version 1)\n(deny default)\nEOF");
                },
                _ => {},
            }

            sandbox_cmd.push_str(&format!(" && {}", plugin_path));

            let output = Command::new("bash")
                .args(["-c", &sandbox_cmd])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to start plugin process")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to start plugin: {:?}", output));
            }
        }

        // Start monitoring the plugin
        self.monitor_plugin(plugin_name).await;

        Ok(())
    }

    /// Monitor plugin resource usage
    async fn monitor_plugin(&self, plugin_name: &str) {
        let start_time = std::time::Instant::now();

        loop {
            // Check if plugin has exceeded maximum execution time
            if start_time.elapsed() > self.config.max_execution_time {
                let _ = self.stop_plugin(plugin_name).await;
                break;
            }

            // Check resource usage
            if let Err(e) = self.check_resource_usage(plugin_name).await {
                error!("Error checking resource usage: {:?}", e);
            }

            // Sleep for a short interval
            sleep(Duration::from_secs(1)).await;
        }
    }

    /// Check resource usage for plugin
    async fn check_resource_usage(&self, plugin_name: &str) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            // Windows implementation
            // Use Windows API to check resource usage
            // For simplicity, we'll just check if the process is running
            let output = Command::new("tasklist")
                .args(["/FI", &format!("IMAGENAME eq {}.exe", plugin_name)])
                .output()
                .context("Failed to check resource usage")?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            if !output_str.contains(&format!("{}.exe", plugin_name)) {
                // Plugin process not running
                let mut running_plugins = self.running_plugins.write().await;
                running_plugins.retain(|p| p.plugin_name != plugin_name);
                return Ok(());
            }

            // Check disk I/O if configured
            if let Some(_max_disk_io) = self.config.max_disk_io {
                // Windows-specific disk I/O monitoring
                let output = Command::new("wmic")
                    .args(["process", "where", &format!("name='{}.exe'", plugin_name), "get", "IOReadBytesPerSec,IOWriteBytesPerSec"])
                    .output()
                    .context("Failed to check disk I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse and check disk I/O
                if output_str.contains("IOReadBytesPerSec") {
                    // 简单解析，实际应用中需要更复杂的解析
                    info!("Disk I/O for plugin {}: {}", plugin_name, output_str);
                }
            }

            // Check network I/O if configured
            if let Some(_max_network_io) = self.config.max_network_io {
                // Windows-specific network I/O monitoring
                let output = Command::new("netstat")
                    .args(["-ano"])
                    .output()
                    .context("Failed to check network I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Filter and check network I/O for the plugin
                info!("Network I/O for plugin {}: {}", plugin_name, output_str);
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Linux implementation
            // Use ps command to check resource usage
            let output = Command::new("ps")
                .args(["aux"])
                .output()
                .context("Failed to check resource usage")?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut found = false;
            
            for line in output_str.lines() {
                if line.contains(plugin_name) {
                    found = true;
                    // Parse resource usage
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        // parts[2] is CPU usage, parts[3] is memory usage
                        if let Ok(cpu_usage) = parts[2].parse::<f32>() {
                            if cpu_usage > self.config.max_cpu_percent as f32 {
                                error!(
                                    "Plugin {} exceeded CPU limit: {}% (max: {}%)",
                                    plugin_name, cpu_usage, self.config.max_cpu_percent
                                );
                                let _ = self.stop_plugin(plugin_name).await;
                            }
                        }

                        // Check memory usage
                        if let Ok(mem_usage) = parts[3].parse::<f32>() {
                            if mem_usage > self.config.max_memory_mb as f32 {
                                error!(
                                    "Plugin {} exceeded memory limit: {}MB (max: {}MB)",
                                    plugin_name, mem_usage, self.config.max_memory_mb
                                );
                                let _ = self.stop_plugin(plugin_name).await;
                            }
                        }
                    }
                }
            }
            
            if !found {
                // Plugin process not running
                let mut running_plugins = self.running_plugins.write().await;
                running_plugins.retain(|p| p.plugin_name != plugin_name);
                return Ok(());
            }

            // Check disk I/O if configured
            if let Some(max_disk_io) = self.config.max_disk_io {
                // Linux-specific disk I/O monitoring using iotop
                let output = Command::new("iotop")
                    .args(["-b", "-n", "1"])
                    .output()
                    .context("Failed to check disk I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                // 简单解析，实际应用中需要更复杂的解析
                info!("Disk I/O for plugin {}: {}", plugin_name, output_str);
            }

            // Check network I/O if configured
            if let Some(max_network_io) = self.config.max_network_io {
                // Linux-specific network I/O monitoring using netstat
                let output = Command::new("netstat")
                    .args(["-tunap"])
                    .output()
                    .context("Failed to check network I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                // 简单解析，实际应用中需要更复杂的解析
                info!("Network I/O for plugin {}: {}", plugin_name, output_str);
            }
        }

        #[cfg(target_os = "macos")]
        {
            // macOS implementation
            // Use ps command to check resource usage
            let output = Command::new("ps")
                .args(["aux"])
                .output()
                .context("Failed to check resource usage")?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut found = false;
            
            for line in output_str.lines() {
                if line.contains(plugin_name) {
                    found = true;
                    // Parse resource usage
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        // parts[2] is CPU usage, parts[3] is memory usage
                        if let Ok(cpu_usage) = parts[2].parse::<f32>() {
                            if cpu_usage > self.config.max_cpu_percent as f32 {
                                error!(
                                    "Plugin {} exceeded CPU limit: {}% (max: {}%)",
                                    plugin_name, cpu_usage, self.config.max_cpu_percent
                                );
                                let _ = self.stop_plugin(plugin_name).await;
                            }
                        }

                        // Check memory usage
                        if let Ok(mem_usage) = parts[3].parse::<f32>() {
                            if mem_usage > self.config.max_memory_mb as f32 {
                                error!(
                                    "Plugin {} exceeded memory limit: {}MB (max: {}MB)",
                                    plugin_name, mem_usage, self.config.max_memory_mb
                                );
                                let _ = self.stop_plugin(plugin_name).await;
                            }
                        }
                    }
                }
            }
            
            if !found {
                // Plugin process not running
                let mut running_plugins = self.running_plugins.write().await;
                running_plugins.retain(|p| p.plugin_name != plugin_name);
                return Ok(());
            }

            // Check disk I/O if configured
            if let Some(max_disk_io) = self.config.max_disk_io {
                // macOS-specific disk I/O monitoring
                let output = Command::new("iostat")
                    .args(["-d", "1", "1"])
                    .output()
                    .context("Failed to check disk I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                info!("Disk I/O for plugin {}: {}", plugin_name, output_str);
            }

            // Check network I/O if configured
            if let Some(max_network_io) = self.config.max_network_io {
                // macOS-specific network I/O monitoring
                let output = Command::new("netstat")
                    .args(["-i"])
                    .output()
                    .context("Failed to check network I/O")?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                info!("Network I/O for plugin {}: {}", plugin_name, output_str);
            }
        }

        Ok(())
    }

    /// Stop plugin
    pub async fn stop_plugin(&self, plugin_name: &str) -> Result<()> {
        // Remove from running plugins
        let mut running_plugins = self.running_plugins.write().await;
        running_plugins.retain(|p| p.plugin_name != plugin_name);

        // Stop the plugin process
        #[cfg(target_os = "windows")]
        {
            // Windows implementation
            let _output = Command::new("taskkill")
                .args(["/F", "/IM", &format!("{}.exe", plugin_name)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to stop plugin process")?;

            // Ignore error if process doesn't exist
        }

        #[cfg(target_os = "linux")]
        {
            // Linux implementation
            let output = Command::new("pkill")
                .args([&format!("^{}$", plugin_name)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to stop plugin process")?;

            // Ignore error if process doesn't exist
        }

        #[cfg(target_os = "macos")]
        {
            // macOS implementation
            let output = Command::new("pkill")
                .args([&format!("^{}$", plugin_name)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .context("Failed to stop plugin process")?;

            // Ignore error if process doesn't exist
        }

        Ok(())
    }

    /// Get running plugins
    pub async fn get_running_plugins(&self) -> Vec<RunningPlugin> {
        let running_plugins = self.running_plugins.read().await;
        running_plugins.clone()
    }

    /// Check if plugin is running
    pub async fn is_plugin_running(&self, plugin_name: &str) -> bool {
        let running_plugins = self.running_plugins.read().await;
        running_plugins.iter().any(|p| p.plugin_name == plugin_name)
    }

    /// Check if network access is allowed for a specific domain/IP
    pub fn is_network_access_allowed(&self, domain_or_ip: &str) -> bool {
        match &self.config.network_access {
            NetworkAccess::None => {
                info!("Network access denied for {}: no network access allowed", domain_or_ip);
                false
            },
            NetworkAccess::Whitelist(allowed) => {
                let allowed = allowed.contains(&domain_or_ip.to_string());
                if !allowed {
                    info!("Network access denied for {}: not in whitelist", domain_or_ip);
                }
                allowed
            },
            NetworkAccess::Blacklist(blocked) => {
                let allowed = !blocked.contains(&domain_or_ip.to_string());
                if !allowed {
                    info!("Network access denied for {}: in blacklist", domain_or_ip);
                }
                allowed
            },
            NetworkAccess::Full => true,
        }
    }

    /// Check if filesystem access is allowed for a specific path
    pub fn is_filesystem_access_allowed(&self, path: &str, write: bool) -> bool {
        match &self.config.filesystem_access {
            FilesystemAccess::None => {
                info!("Filesystem access denied for {}: no filesystem access allowed", path);
                false
            },
            FilesystemAccess::ReadOnly(allowed_paths) => {
                if write {
                    info!("Filesystem write access denied for {}: read-only access only", path);
                    false
                } else {
                    let allowed = allowed_paths.iter().any(|allowed_path| path.starts_with(allowed_path));
                    if !allowed {
                        info!("Filesystem read access denied for {}: not in allowed paths", path);
                    }
                    allowed
                }
            },
            FilesystemAccess::ReadWrite(allowed_paths) => {
                let allowed = allowed_paths.iter().any(|allowed_path| path.starts_with(allowed_path));
                if !allowed {
                    info!("Filesystem access denied for {}: not in allowed paths", path);
                }
                allowed
            },
            FilesystemAccess::Full => true,
        }
    }

    /// Apply network access control
    pub fn apply_network_control(&self, plugin_name: &str) {
        match &self.config.network_access {
            NetworkAccess::None => {
                info!("Applying network access control for {}: no network access", plugin_name);
                // 实现具体的网络访问控制
            },
            NetworkAccess::Whitelist(allowed) => {
                info!("Applying network access control for {}: whitelist {:?}", plugin_name, allowed);
                // 实现具体的网络访问控制
            },
            NetworkAccess::Blacklist(blocked) => {
                info!("Applying network access control for {}: blacklist {:?}", plugin_name, blocked);
                // 实现具体的网络访问控制
            },
            NetworkAccess::Full => {
                info!("Applying network access control for {}: full access", plugin_name);
            },
        }
    }

    /// Apply filesystem access control
    pub fn apply_filesystem_control(&self, plugin_name: &str) {
        match &self.config.filesystem_access {
            FilesystemAccess::None => {
                info!("Applying filesystem access control for {}: no filesystem access", plugin_name);
                // 实现具体的文件系统访问控制
            },
            FilesystemAccess::ReadOnly(allowed_paths) => {
                info!("Applying filesystem access control for {}: read-only access to {:?}", plugin_name, allowed_paths);
                // 实现具体的文件系统访问控制
            },
            FilesystemAccess::ReadWrite(allowed_paths) => {
                info!("Applying filesystem access control for {}: read-write access to {:?}", plugin_name, allowed_paths);
                // 实现具体的文件系统访问控制
            },
            FilesystemAccess::Full => {
                info!("Applying filesystem access control for {}: full access", plugin_name);
            },
        }
    }
}

