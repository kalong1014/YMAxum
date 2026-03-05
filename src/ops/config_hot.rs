use log::{debug, error, info};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{Notify, RwLock};
use tokio::time::{self, Duration};

/// Configuration item
#[derive(Debug, Clone)]
pub struct ConfigItem {
    /// Configuration key
    pub key: String,
    /// Configuration value
    pub value: String,
    /// Configuration item type
    pub item_type: ConfigItemType,
    /// Configuration item description
    pub description: Option<String>,
}

/// Configuration item type
#[derive(Debug, Clone, PartialEq)]
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

/// Hot update configuration
#[derive(Debug, Clone, Default)]
pub struct ConfigHotUpdateConfig {
    /// Configuration file path
    pub config_file_path: String,
    /// Check interval (milliseconds)
    pub check_interval: u64,
    /// Enable hot update
    pub enabled: bool,
}

/// Hot update service
#[derive(Debug, Clone)]
pub struct ConfigHotUpdateService {
    /// Configuration
    pub config: ConfigHotUpdateConfig,
    /// Current configuration
    pub current_config: Arc<RwLock<HashMap<String, ConfigItem>>>,
    /// Is running
    pub is_running: Arc<RwLock<bool>>,
    /// Shutdown notification channel
    pub shutdown_notify: Arc<Notify>,
}

impl ConfigHotUpdateService {
    /// Create new hot update service
    pub fn new(config: ConfigHotUpdateConfig) -> Self {
        Self {
            config,
            current_config: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(RwLock::new(false)),
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    /// Start hot update service
    pub async fn start(&self) -> io::Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            info!("Hot update service is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!(
            "Starting hot update service, check interval: {}ms",
            self.config.check_interval
        );

        // Start monitoring task
        let service_clone = self.clone();
        tokio::spawn(async move {
            service_clone.monitor_config().await;
        });

        // Load initial configuration
        self.load_config().await?;

        Ok(())
    }

    /// Stop hot update service
    pub async fn stop(&self) -> io::Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        // Send shutdown notification
        self.shutdown_notify.notify_one();

        info!("Hot update service stopped");
        Ok(())
    }

    /// Monitor configuration file
    async fn monitor_config(&self) {
        let mut interval = time::interval(Duration::from_millis(self.config.check_interval));
        loop {
            tokio::select! {
                // Check configuration file changes periodically
                _ = interval.tick() => {
                    let is_running = *self.is_running.read().await;
                    if !is_running {
                        break;
                    }

                    // Reload configuration file
                    if let Err(e) = self.load_config().await {
                        error!("Failed to load configuration file: {:?}", e);
                    }
                },
                // Wait for shutdown notification
                _ = self.shutdown_notify.notified() => {
                    break;
                }
            }
        }
    }

    /// Load configuration file
    pub async fn load_config(&self) -> io::Result<()> {
        let path = Path::new(&self.config.config_file_path);
        if !path.exists() {
            debug!("Configuration file does not exist: {:?}", path);
            return Ok(());
        }

        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut new_config = HashMap::new();

        while let Some(line) = lines.next_line().await? {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse configuration line: KEY=VALUE or KEY=VALUE # comment
            if let Some((key_part, value_part)) = line.split_once('=') {
                let key = key_part.trim().to_string();
                let mut value_parts = value_part.splitn(2, '#');
                let value = value_parts.next().unwrap_or("").trim().to_string();

                // Infer configuration type
                let item_type = self.infer_config_type(&value);

                let config_item = ConfigItem {
                    key: key.clone(),
                    value,
                    item_type,
                    description: None,
                };

                new_config.insert(key, config_item);
            }
        }

        // Save new configuration
        let mut current_config = self.current_config.write().await;
        let old_config = std::mem::replace(&mut *current_config, new_config);
        drop(current_config);

        // Check configuration changes
        self.check_config_changes(&old_config).await;

        info!("Configuration file loaded successfully: {:?}", path);
        Ok(())
    }

    /// Infer configuration type
    fn infer_config_type(&self, value: &str) -> ConfigItemType {
        // Check if it's an integer
        if value.parse::<i64>().is_ok() {
            return ConfigItemType::Integer;
        }

        // Check if it's a float
        if value.parse::<f64>().is_ok() {
            return ConfigItemType::Float;
        }

        // Check if it's a boolean
        if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
            return ConfigItemType::Boolean;
        }

        // Check if it's an array
        if value.starts_with('[') && value.ends_with(']') {
            return ConfigItemType::Array;
        }

        // Check if it's an object
        if value.starts_with('{') && value.ends_with('}') {
            return ConfigItemType::Object;
        }

        // Default to string type
        ConfigItemType::String
    }

    /// Check configuration changes
    async fn check_config_changes(&self, old_config: &HashMap<String, ConfigItem>) {
        let current_config = self.current_config.read().await;

        // Check for added configuration items
        for (key, item) in current_config.iter() {
            if !old_config.contains_key(key) {
                info!(
                    "Configuration item added: {} = {:?} (type: {:?})",
                    key, item.value, item.item_type
                );
                self.on_config_added(key, item).await;
            }
        }

        // Check for modified configuration items
        for (key, old_item) in old_config.iter() {
            if let Some(new_item) = current_config.get(key)
                && old_item.value != new_item.value
            {
                info!(
                    "Configuration item modified: {} {:?} -> {:?}",
                    key, old_item.value, new_item.value
                );
                self.on_config_changed(key, old_item, new_item).await;
            }
        }

        // Check for removed configuration items
        for (key, old_item) in old_config.iter() {
            if !current_config.contains_key(key) {
                info!(
                    "Configuration item removed: {} (value: {:?})",
                    key, old_item.value
                );
                self.on_config_removed(key, old_item).await;
            }
        }
    }

    /// Configuration item added callback
    async fn on_config_added(&self, key: &str, item: &ConfigItem) {
        // Integration with other modules via callback
        debug!(
            "Configuration item added callback: {} = {:?}",
            key, item.value
        );
    }

    /// Configuration item modified callback
    async fn on_config_changed(&self, key: &str, old_item: &ConfigItem, new_item: &ConfigItem) {
        // Integration with other modules via callback
        debug!(
            "Configuration item modified callback: {} {:?} -> {:?}",
            key, old_item.value, new_item.value
        );
    }

    /// Configuration item removed callback
    async fn on_config_removed(&self, key: &str, old_item: &ConfigItem) {
        // Integration with other modules via callback
        debug!(
            "Configuration item removed callback: {} (value: {:?})",
            key, old_item.value
        );
    }

    /// Get configuration value
    pub async fn get_config(&self, key: &str) -> Option<String> {
        let current_config = self.current_config.read().await;
        current_config.get(key).map(|item| item.value.clone())
    }

    /// Get configuration item
    pub async fn get_config_item(&self, key: &str) -> Option<ConfigItem> {
        let current_config = self.current_config.read().await;
        current_config.get(key).cloned()
    }

    /// Update configuration item (write to file)
    pub async fn set_config(&self, key: &str, value: &str) -> io::Result<()> {
        // Read existing configuration file
        let current_config = self.current_config.read().await;
        let mut config_lines = Vec::new();

        // Read existing configuration file content
        let path = Path::new(&self.config.config_file_path);
        if path.exists() {
            let file = File::open(path).await?;
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await? {
                let line = line.trim();

                // Skip comments and empty lines
                if line.is_empty() || line.starts_with('#') {
                    config_lines.push(line.to_string());
                    continue;
                }

                // Check if we need to update this configuration item
                if let Some((key_part, _)) = line.split_once('=') {
                    let line_key = key_part.trim();
                    if line_key == key {
                        // Update this configuration item
                        config_lines.push(format!("{} = {}", key, value));
                        continue;
                    }
                }

                // Keep original line
                config_lines.push(line.to_string());
            }
        }

        // Add new configuration item if it doesn't exist
        if !current_config.contains_key(key) {
            config_lines.push(format!("{} = {}", key, value));
        }

        // Write configuration file
        let file = File::create(path).await?;
        let mut writer = io::BufWriter::new(file);

        for line in config_lines {
            writer.write_all(format!("{}\n", line).as_bytes()).await?;
        }
        writer.flush().await?;

        // Reload configuration
        self.load_config().await?;

        info!(
            "Configuration item updated successfully: {} = {}",
            key, value
        );
        Ok(())
    }
}

/// Hot update service tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_hot_update() {
        // Create test configuration file to avoid file I/O errors
        let config = ConfigHotUpdateConfig {
            config_file_path: "test_config.txt".to_string(),
            check_interval: 100,
            enabled: false, // Disable hot update for testing
        };

        let service = ConfigHotUpdateService::new(config);

        // Test configuration type inference
        let string_type = service.infer_config_type("test_value");
        assert_eq!(string_type, ConfigItemType::String);

        let int_type = service.infer_config_type("123");
        assert_eq!(int_type, ConfigItemType::Integer);

        let float_type = service.infer_config_type("123.456");
        assert_eq!(float_type, ConfigItemType::Float);

        let bool_true_type = service.infer_config_type("true");
        assert_eq!(bool_true_type, ConfigItemType::Boolean);

        let bool_false_type = service.infer_config_type("false");
        assert_eq!(bool_false_type, ConfigItemType::Boolean);

        let array_type = service.infer_config_type("[1, 2, 3]");
        assert_eq!(array_type, ConfigItemType::Array);

        let object_type = service.infer_config_type("{\"key\": \"value\"}");
        assert_eq!(object_type, ConfigItemType::Object);

        // Test configuration item
        let config_item = ConfigItem {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            item_type: ConfigItemType::String,
            description: Some("test description".to_string()),
        };

        assert_eq!(config_item.key, "test_key");
        assert_eq!(config_item.value, "test_value");
        assert_eq!(config_item.item_type, ConfigItemType::String);
        assert_eq!(
            config_item.description,
            Some("test description".to_string())
        );
    }
}
