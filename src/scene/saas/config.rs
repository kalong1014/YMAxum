// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Site configuration item type
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ConfigItemType {
    /// String
    String,
    /// Integer
    Integer,
    /// Floating point number
    Float,
    /// Boolean
    Boolean,
    /// JSON object
    Json,
    /// Array
    Array,
}

/// Site configuration item
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    /// Configuration item ID
    pub id: String,
    /// Configuration item display name
    pub name: String,
    /// Configuration item key
    pub key: String,
    /// Configuration item type
    pub item_type: ConfigItemType,
    /// Configuration item value
    pub value: serde_json::Value,
    /// Is required
    pub required: bool,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Description
    pub description: Option<String>,
    /// Is editable
    pub editable: bool,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
}

/// Site configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    /// Configuration ID
    pub id: String,
    /// Associated site ID
    pub site_id: String,
    /// Configuration item list
    pub items: Vec<ConfigItem>,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
}

/// Configuration manager
pub struct ConfigManager {
    /// Site configuration map
    configs: Arc<RwLock<HashMap<String, SiteConfig>>>,
    /// Configuration item map (configuration item ID -> configuration item)
    items: Arc<RwLock<HashMap<String, ConfigItem>>>,
    /// Site ID to configuration ID mapping
    site_to_config: Arc<RwLock<HashMap<String, String>>>,
}

impl ConfigManager {
    /// Create new configuration manager
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            items: Arc::new(RwLock::new(HashMap::new())),
            site_to_config: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create site configuration
    pub async fn create_config(&self, site_id: &str) -> Result<SiteConfig, String> {
        let mut configs = self.configs.write().await;
        let mut site_to_config = self.site_to_config.write().await;

        // Check if site already has configuration
        if site_to_config.contains_key(site_id) {
            return Err(format!("Site {} already has configuration", site_id));
        }

        // Generate configuration ID
        let config_id = format!("config_{}", uuid::Uuid::new_v4());
        let now = chrono::Utc::now().timestamp() as u64;

        // Create new configuration
        let new_config = SiteConfig {
            id: config_id.clone(),
            site_id: site_id.to_string(),
            items: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        // Save configuration
        configs.insert(config_id.clone(), new_config.clone());
        site_to_config.insert(site_id.to_string(), config_id.clone());

        info!("Site configuration created: {} -> {}", site_id, config_id);
        Ok(new_config)
    }

    /// Get site configuration
    pub async fn get_config(&self, site_id: &str) -> Option<SiteConfig> {
        let site_to_config = self.site_to_config.read().await;
        if let Some(config_id) = site_to_config.get(site_id) {
            let configs = self.configs.read().await;
            configs.get(config_id).cloned()
        } else {
            None
        }
    }

    /// Add configuration item
    pub async fn add_config_item(
        &self,
        site_id: &str,
        item: ConfigItem,
    ) -> Result<SiteConfig, String> {
        let mut configs = self.configs.write().await;
        let site_to_config = self.site_to_config.read().await;
        let mut items = self.items.write().await;

        // Get configuration
        let config_id = site_to_config
            .get(site_id)
            .ok_or(format!("Site {} does not have configuration", site_id))?;
        let config = configs
            .get_mut(config_id)
            .ok_or("Configuration does not exist".to_string())?;

        // Check if configuration item key already exists
        if config.items.iter().any(|i| i.key == item.key) {
            return Err(format!(
                "Configuration item key {} already exists",
                item.key
            ));
        }

        // Check if configuration item ID already exists
        if items.contains_key(&item.id) {
            return Err(format!("Configuration item ID {} already exists", item.id));
        }

        // Generate new configuration item
        let mut new_item = item.clone();
        new_item.created_at = chrono::Utc::now().timestamp() as u64;
        new_item.updated_at = new_item.created_at;

        // Add configuration item
        config.items.push(new_item.clone());
        items.insert(new_item.id.clone(), new_item.clone());
        config.updated_at = new_item.created_at;

        info!(
            "Configuration item added: {} -> {} -> {}",
            site_id, item.key, item.value
        );
        Ok(config.clone())
    }

    /// Update configuration item
    pub async fn update_config_item(
        &self,
        site_id: &str,
        item: ConfigItem,
    ) -> Result<SiteConfig, String> {
        let mut configs = self.configs.write().await;
        let site_to_config = self.site_to_config.read().await;
        let mut items = self.items.write().await;

        // Get configuration
        let config_id = site_to_config
            .get(site_id)
            .ok_or(format!("Site {} does not have configuration", site_id))?;
        let config = configs
            .get_mut(config_id)
            .ok_or("Configuration does not exist".to_string())?;

        // Find configuration item
        let item_index = config
            .items
            .iter()
            .position(|i| i.id == item.id)
            .ok_or(format!("Configuration item ID {} does not exist", item.id))?;

        // Update configuration item
        let mut updated_item = item.clone();
        updated_item.updated_at = chrono::Utc::now().timestamp() as u64;

        config.items[item_index] = updated_item.clone();
        items.insert(updated_item.id.clone(), updated_item.clone());
        config.updated_at = updated_item.updated_at;

        info!(
            "Configuration item updated: {} -> {} -> {}",
            site_id, item.key, item.value
        );
        Ok(config.clone())
    }

    /// Delete configuration item
    pub async fn delete_config_item(
        &self,
        site_id: &str,
        item_id: &str,
    ) -> Result<SiteConfig, String> {
        let mut configs = self.configs.write().await;
        let site_to_config = self.site_to_config.read().await;
        let mut items = self.items.write().await;

        // Get configuration
        let config_id = site_to_config
            .get(site_id)
            .ok_or(format!("Site {} does not have configuration", site_id))?;
        let config = configs
            .get_mut(config_id)
            .ok_or("Configuration does not exist".to_string())?;

        // Find and delete configuration item
        let item_index = config
            .items
            .iter()
            .position(|i| i.id == item_id)
            .ok_or(format!("Configuration item ID {} does not exist", item_id))?;
        let item = config.items.remove(item_index);
        items.remove(&item.id);
        config.updated_at = chrono::Utc::now().timestamp() as u64;

        info!("Configuration item deleted: {} -> {}", site_id, item.key);
        Ok(config.clone())
    }

    /// Get configuration item
    pub async fn get_config_item(&self, item_id: &str) -> Option<ConfigItem> {
        let items = self.items.read().await;
        items.get(item_id).cloned()
    }

    /// Get configuration item by key
    pub async fn get_config_item_by_key(&self, site_id: &str, key: &str) -> Option<ConfigItem> {
        let config = self.get_config(site_id).await?;
        config.items.iter().find(|i| i.key == key).cloned()
    }

    /// Get configuration item value
    pub async fn get_config_value<T: serde::de::DeserializeOwned>(
        &self,
        site_id: &str,
        key: &str,
    ) -> Result<T, String> {
        let item = self
            .get_config_item_by_key(site_id, key)
            .await
            .ok_or(format!("Configuration item key {} does not exist", key))?;

        serde_json::from_value(item.value.clone())
            .map_err(|e| format!("Configuration item value deserialization failed: {}", e))
    }

    /// Set configuration item value
    pub async fn set_config_value<T: serde::Serialize>(
        &self,
        site_id: &str,
        key: &str,
        value: T,
    ) -> Result<SiteConfig, String> {
        let config = self
            .get_config(site_id)
            .await
            .ok_or(format!("Site {} does not have configuration", site_id))?;

        // Find configuration item
        let item = config
            .items
            .iter()
            .find(|i| i.key == key)
            .ok_or(format!("Configuration item key {} does not exist", key))?;

        // Check if item is editable
        if !item.editable {
            return Err(format!("Configuration item key {} is not editable", key));
        }

        // Serialize value
        let json_value = serde_json::to_value(value)
            .map_err(|e| format!("Value serialization failed: {}", e))?;

        // Update configuration item
        let updated_item = ConfigItem {
            id: item.id.clone(),
            name: item.name.clone(),
            key: item.key.clone(),
            item_type: item.item_type.clone(),
            value: json_value.clone(),
            required: item.required,
            default_value: item.default_value.clone(),
            description: item.description.clone(),
            editable: item.editable,
            created_at: item.created_at,
            updated_at: chrono::Utc::now().timestamp() as u64,
        };

        self.update_config_item(site_id, updated_item).await
    }

    /// Delete site configuration
    pub async fn delete_config(&self, site_id: &str) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        let mut site_to_config = self.site_to_config.write().await;
        let mut items = self.items.write().await;

        // Get configuration ID
        let config_id = site_to_config
            .remove(site_id)
            .ok_or(format!("Site {} does not have configuration", site_id))?;

        // Get configuration
        let config = configs
            .remove(&config_id)
            .ok_or("Configuration does not exist".to_string())?;

        // Delete all configuration items
        for item in config.items {
            items.remove(&item.id);
        }

        info!("Site configuration deleted: {} -> {}", site_id, config_id);
        Ok(())
    }

    /// Validate site configuration
    pub async fn validate_config(&self, site_id: &str) -> Result<(), String> {
        let config = self
            .get_config(site_id)
            .await
            .ok_or(format!("Site {} does not have configuration", site_id))?;

        // Check required items
        for item in &config.items {
            if item.required
                && (item.value.is_null() || item.value == serde_json::Value::String("".to_string()))
            {
                return Err(format!("Configuration item {} is required", item.key));
            }
        }

        Ok(())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

