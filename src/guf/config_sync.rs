use chrono;
use log::{debug, error};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, watch};

/// GUF 配置同步器
/// 负责实现与 GUF 配置系统的双向同步
pub struct GufConfigSync {
    /// 本地配置存储
    local_config: Arc<RwLock<GufConfig>>,
    /// 配置变更通知
    config_notifier: (watch::Sender<GufConfig>, watch::Receiver<GufConfig>),
    /// 配置项变更通知
    item_notifier: (watch::Sender<(String, serde_json::Value)>, watch::Receiver<(String, serde_json::Value)>),
    /// 同步状态
    sync_status: Arc<RwLock<SyncStatus>>,
    /// GUF 配置服务客户端
    config_client: Arc<GufConfigClient>,
    /// 上次同步的配置版本
    last_sync_version: Arc<RwLock<String>>,
    /// 配置变更缓存
    change_cache: Arc<RwLock<std::collections::HashMap<String, serde_json::Value>>>,
}

/// GUF 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufConfig {
    /// 配置版本
    version: String,
    /// 配置项
    items: std::collections::HashMap<String, serde_json::Value>,
    /// 配置元数据
    metadata: GufConfigMetadata,
}

/// GUF 配置元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufConfigMetadata {
    /// 最后更新时间
    last_updated: u64,
    /// 更新者
    updated_by: String,
    /// 配置环境
    environment: String,
}

/// 同步状态
#[derive(Debug, Clone)]
pub enum SyncStatus {
    /// 已同步
    Synced,
    /// 同步中
    Syncing,
    /// 同步失败
    SyncFailed(String),
    /// 未同步
    NotSynced,
}

/// GUF 配置客户端
pub struct GufConfigClient {
    /// 服务地址
    pub server_address: String,
    /// 认证令牌
    pub auth_token: String,
}

/// GUF 配置同步错误
#[derive(Debug, thiserror::Error)]
pub enum GufConfigSyncError {
    #[error("Sync failed: {0}")]
    SyncFailed(String),

    #[error("Config not found: {0}")]
    ConfigNotFound(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),
}

impl GufConfig {
    /// 创建新的 GUF 配置
    pub fn new(environment: String) -> Self {
        Self {
            version: "1.0.0".to_string(),
            items: std::collections::HashMap::new(),
            metadata: GufConfigMetadata {
                last_updated: chrono::Utc::now().timestamp() as u64,
                updated_by: "system".to_string(),
                environment,
            },
        }
    }

    /// 添加配置项
    pub fn add_item(&mut self, key: String, value: serde_json::Value) {
        self.items.insert(key, value);
        self.metadata.last_updated = chrono::Utc::now().timestamp() as u64;
    }

    /// 获取配置项
    pub fn get_item(&self, key: &str) -> Option<&serde_json::Value> {
        self.items.get(key)
    }

    /// 删除配置项
    pub fn remove_item(&mut self, key: &str) {
        self.items.remove(key);
        self.metadata.last_updated = chrono::Utc::now().timestamp() as u64;
    }

    /// 获取配置版本
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 更新配置版本
    pub fn update_version(&mut self, version: String) {
        self.version = version;
        self.metadata.last_updated = chrono::Utc::now().timestamp() as u64;
    }
}

impl GufConfigClient {
    /// 创建新的 GUF 配置客户端
    pub fn new(server_address: String, auth_token: String) -> Self {
        Self {
            server_address,
            auth_token,
        }
    }

    /// 从 GUF 配置服务获取配置
    pub async fn get_config(&self, environment: &str) -> Result<GufConfig, GufConfigSyncError> {
        // 构建请求URL
        let url = format!("{}/api/v1/config/{}", self.server_address, environment);

        // 创建HTTP客户端
        let client = reqwest::Client::new();

        // 发送GET请求
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Content-Type", "application/json")
            .timeout(tokio::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| GufConfigSyncError::NetworkError(e.to_string()))?;

        // 检查响应状态
        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(GufConfigSyncError::ConfigNotFound(environment.to_string()));
            }
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(GufConfigSyncError::AuthenticationFailed(
                    "Invalid token".to_string(),
                ));
            }
            return Err(GufConfigSyncError::SyncFailed(format!(
                "Server error: {}",
                response.status()
            )));
        }

        // 解析响应
        let config_data: serde_json::Value = response.json().await.map_err(|e| {
            GufConfigSyncError::SyncFailed(format!("Failed to parse response: {}", e))
        })?;

        // 构建GUF配置
        let config = self.parse_config(config_data, environment)?;
        Ok(config)
    }

    /// 将配置推送到 GUF 配置服务
    pub async fn push_config(&self, config: &GufConfig) -> Result<(), GufConfigSyncError> {
        // 构建请求URL
        let url = format!(
            "{}/api/v1/config/{}",
            self.server_address, config.metadata.environment
        );

        // 构建请求体
        let request_body = serde_json::json!({
            "version": config.version,
            "items": config.items,
            "metadata": config.metadata
        });

        // 创建HTTP客户端
        let client = reqwest::Client::new();

        // 发送PUT请求
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .timeout(tokio::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| GufConfigSyncError::NetworkError(e.to_string()))?;

        // 检查响应状态
        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(GufConfigSyncError::AuthenticationFailed(
                    "Invalid token".to_string(),
                ));
            }
            return Err(GufConfigSyncError::SyncFailed(format!(
                "Server error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// 监听配置变更
    pub async fn watch_config(
        &self,
        environment: &str,
    ) -> Result<watch::Receiver<GufConfig>, GufConfigSyncError> {
        // 创建通道
        let (tx, rx) = watch::channel(GufConfig::new(environment.to_string()));

        // 启动监听任务
        let server_address = self.server_address.clone();
        let auth_token = self.auth_token.clone();
        let env = environment.to_string();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            let client = GufConfigClient::new(server_address, auth_token);

            loop {
                interval.tick().await;

                // 定期获取配置
                if let Ok(config) = client.get_config(&env).await {
                    // 发送配置变更通知
                    let _ = tx.send(config);
                }
            }
        });

        Ok(rx)
    }

    /// 解析配置响应
    fn parse_config(
        &self,
        data: serde_json::Value,
        environment: &str,
    ) -> Result<GufConfig, GufConfigSyncError> {
        // 提取版本
        let version = data
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string();

        // 提取配置项
        let items = data
            .get("items")
            .and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();

        // 提取元数据
        let metadata = GufConfigMetadata {
            last_updated: data
                .get("metadata")
                .and_then(|m| m.get("last_updated"))
                .and_then(|v| v.as_u64())
                .unwrap_or(chrono::Utc::now().timestamp() as u64),
            updated_by: data
                .get("metadata")
                .and_then(|m| m.get("updated_by"))
                .and_then(|v| v.as_str())
                .unwrap_or("system")
                .to_string(),
            environment: environment.to_string(),
        };

        Ok(GufConfig {
            version,
            items,
            metadata,
        })
    }
}

impl GufConfigSync {
    /// 创建新的 GUF 配置同步器
    pub fn new() -> Self {
        let local_config = Arc::new(RwLock::new(GufConfig::new("default".to_string())));
        let config_notifier = watch::channel(GufConfig::new("default".to_string()));
        let item_notifier = watch::channel(("initial".to_string(), serde_json::Value::Null));
        let sync_status = Arc::new(RwLock::new(SyncStatus::NotSynced));
        let config_client = Arc::new(GufConfigClient::new(
            "http://localhost:8080".to_string(),
            "default".to_string(),
        ));
        let last_sync_version = Arc::new(RwLock::new("0.0.0".to_string()));
        let change_cache = Arc::new(RwLock::new(std::collections::HashMap::new()));

        Self {
            local_config,
            config_notifier,
            item_notifier,
            sync_status,
            config_client,
            last_sync_version,
            change_cache,
        }
    }

    /// 使用指定的服务器地址和认证令牌创建新的 GUF 配置同步器
    pub fn new_with_config(server_address: String, auth_token: String) -> Self {
        let local_config = Arc::new(RwLock::new(GufConfig::new("default".to_string())));
        let config_notifier = watch::channel(GufConfig::new("default".to_string()));
        let item_notifier = watch::channel(("initial".to_string(), serde_json::Value::Null));
        let sync_status = Arc::new(RwLock::new(SyncStatus::NotSynced));
        let config_client = Arc::new(GufConfigClient::new(server_address, auth_token));
        let last_sync_version = Arc::new(RwLock::new("0.0.0".to_string()));
        let change_cache = Arc::new(RwLock::new(std::collections::HashMap::new()));

        Self {
            local_config,
            config_notifier,
            item_notifier,
            sync_status,
            config_client,
            last_sync_version,
            change_cache,
        }
    }

    /// 简化的初始化方法
    pub async fn init(&self) -> Result<(), String> {
        // 初始化配置同步器
        match self.initialize("default").await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        // 简单检查，实际应用中可能需要更复杂的逻辑
        true
    }

    /// 初始化配置同步器
    pub async fn initialize(&self, environment: &str) -> Result<(), GufConfigSyncError> {
        // 从 GUF 配置服务获取初始配置
        let config = self.config_client.get_config(environment).await?;

        // 更新本地配置
        let mut local_config = self.local_config.write().await;
        *local_config = config.clone();
        drop(local_config);

        // 通知配置变更
        self.config_notifier
            .0
            .send(config)
            .map_err(|e| GufConfigSyncError::SyncFailed(e.to_string()))?;

        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Synced;

        Ok(())
    }

    /// 初始化 Godot UI Framework v4.4 配置
    pub async fn initialize_godot_ui_config(&self) -> Result<(), GufConfigSyncError> {
        log::info!("Initializing Godot UI Framework v4.4 config");

        // 添加 Godot UI Framework v4.4 配置
        self.add_config_item(
            "godot.ui.version".to_string(),
            serde_json::Value::String("4.4.0".to_string()),
        )
        .await?;

        // 添加 Godot UI 组件配置
        self.add_config_item(
            "godot.ui.components".to_string(),
            serde_json::json!({
                "Button": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "Label": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "TextureButton": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "ColorRect": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "Panel": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "VBoxContainer": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "HBoxContainer": {
                    "version": "4.4.0",
                    "enabled": true
                },
                "GridContainer": {
                    "version": "4.4.0",
                    "enabled": true
                }
            }),
        )
        .await?;

        // 添加 Godot UI 主题配置
        self.add_config_item(
            "godot.ui.theme".to_string(),
            serde_json::json!({
                "name": "default",
                "version": "4.4.0",
                "primary_color": "#4CAF50",
                "secondary_color": "#2196F3",
                "background_color": "#FFFFFF",
                "text_color": "#333333"
            }),
        )
        .await?;

        log::info!("Godot UI Framework v4.4 config initialized");
        Ok(())
    }

    /// 同步配置到 GUF 配置服务
    pub async fn sync_to_guf(&self) -> Result<(), GufConfigSyncError> {
        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Syncing;
        drop(sync_status);

        // 获取本地配置
        let local_config = self.local_config.read().await;
        let config = local_config.clone();
        drop(local_config);

        // 推送配置到 GUF 配置服务
        self.config_client.push_config(&config).await?;

        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Synced;

        Ok(())
    }

    /// 从 GUF 配置服务同步配置
    pub async fn sync_from_guf(&self, environment: &str) -> Result<(), GufConfigSyncError> {
        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Syncing;
        drop(sync_status);

        // 从 GUF 配置服务获取配置
        let config = self.config_client.get_config(environment).await?;

        // 更新本地配置
        let mut local_config = self.local_config.write().await;
        *local_config = config.clone();
        drop(local_config);

        // 通知配置变更
        self.config_notifier
            .0
            .send(config)
            .map_err(|e| GufConfigSyncError::SyncFailed(e.to_string()))?;

        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Synced;

        Ok(())
    }

    /// 添加配置项
    pub async fn add_config_item(
        &self,
        key: String,
        value: serde_json::Value,
    ) -> Result<(), GufConfigSyncError> {
        // 更新本地配置
        let mut local_config = self.local_config.write().await;
        local_config.add_item(key.clone(), value.clone());
        let config = local_config.clone();
        drop(local_config);

        // 缓存变更
        let mut change_cache = self.change_cache.write().await;
        change_cache.insert(key.clone(), value.clone());
        drop(change_cache);

        // 通知配置变更
        self.config_notifier
            .0
            .send(config)
            .map_err(|e| GufConfigSyncError::SyncFailed(e.to_string()))?;

        // 通知配置项变更
        self.item_notifier
            .0
            .send((key, value))
            .map_err(|e| GufConfigSyncError::SyncFailed(e.to_string()))?;

        Ok(())
    }

    /// 增量同步配置到 GUF 配置服务
    pub async fn incremental_sync_to_guf(&self) -> Result<(), GufConfigSyncError> {
        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Syncing;
        drop(sync_status);

        // 获取变更缓存
        let change_cache = self.change_cache.write().await;
        if change_cache.is_empty() {
            // 没有变更，直接返回
            let mut sync_status = self.sync_status.write().await;
            *sync_status = SyncStatus::Synced;
            return Ok(());
        }

        // 构建增量同步请求
        let current_version = {
            let local_config = self.local_config.read().await;
            local_config.version().to_string()
        };
        let last_sync_version = {
            let last_sync = self.last_sync_version.read().await;
            last_sync.clone()
        };
        // 获取实际的变更缓存数据
        let changes = change_cache.clone();
        drop(change_cache);
        let request_body = serde_json::json!({
            "version": {
                "current": current_version,
                "last_sync": last_sync_version
            },
            "changes": changes
        });

        // 发送增量同步请求
        let url = format!("{}/api/v1/config/incremental", self.config_client.server_address);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config_client.auth_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .timeout(tokio::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| GufConfigSyncError::NetworkError(e.to_string()))?;

        // 检查响应状态
        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(GufConfigSyncError::AuthenticationFailed(
                    "Invalid token".to_string(),
                ));
            }
            return Err(GufConfigSyncError::SyncFailed(format!(
                "Server error: {}",
                response.status()
            )));
        }

        // 解析响应
        let response_data: serde_json::Value = response.json().await.map_err(|e| {
            GufConfigSyncError::SyncFailed(format!("Failed to parse response: {}", e))
        })?;

        // 更新上次同步版本
        if let Some(version) = response_data.get("version").and_then(|v| v.as_str()) {
            let mut last_sync = self.last_sync_version.write().await;
            *last_sync = version.to_string();
        }

        // 清空变更缓存
        let mut change_cache = self.change_cache.write().await;
        change_cache.clear();

        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Synced;

        Ok(())
    }

    /// 获取配置项变更通知接收器
    pub fn get_item_notifier(&self) -> watch::Receiver<(String, serde_json::Value)> {
        self.item_notifier.1.clone()
    }

    /// 批量同步配置到 GUF 配置服务
    pub async fn batch_sync_to_guf(&self) -> Result<(), GufConfigSyncError> {
        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Syncing;
        drop(sync_status);

        // 获取本地配置
        let local_config = self.local_config.read().await;
        let config = local_config.clone();
        drop(local_config);

        // 推送配置到 GUF 配置服务
        self.config_client.push_config(&config).await?;

        // 更新同步状态
        let mut sync_status = self.sync_status.write().await;
        *sync_status = SyncStatus::Synced;

        Ok(())
    }

    /// 获取配置项
    pub async fn get_config_item(&self, key: &str) -> Option<serde_json::Value> {
        let local_config = self.local_config.read().await;
        local_config.get_item(key).cloned()
    }

    /// 删除配置项
    pub async fn remove_config_item(&self, key: &str) -> Result<(), GufConfigSyncError> {
        // 更新本地配置
        let mut local_config = self.local_config.write().await;
        local_config.remove_item(key);
        let config = local_config.clone();
        drop(local_config);

        // 通知配置变更
        self.config_notifier
            .0
            .send(config)
            .map_err(|e| GufConfigSyncError::SyncFailed(e.to_string()))?;

        // 同步到 GUF 配置服务
        self.sync_to_guf().await?;

        Ok(())
    }

    /// 获取配置变更通知接收器
    pub fn get_config_notifier(&self) -> watch::Receiver<GufConfig> {
        self.config_notifier.1.clone()
    }

    /// 获取 Godot UI Framework v4.4 配置
    pub async fn get_godot_ui_config(&self) -> Result<serde_json::Value, GufConfigSyncError> {
        let local_config = self.local_config.read().await;

        // 检查是否存在 Godot UI 配置
        if let Some(godot_ui_config) = local_config.get_item("godot.ui") {
            Ok(godot_ui_config.clone())
        } else {
            // 构建默认的 Godot UI 配置
            let default_config = serde_json::json!({
                "version": "4.4.0",
                "components": {
                    "Button": { "version": "4.4.0", "enabled": true },
                    "Label": { "version": "4.4.0", "enabled": true },
                    "TextureButton": { "version": "4.4.0", "enabled": true },
                    "ColorRect": { "version": "4.4.0", "enabled": true },
                    "Panel": { "version": "4.4.0", "enabled": true },
                    "VBoxContainer": { "version": "4.4.0", "enabled": true },
                    "HBoxContainer": { "version": "4.4.0", "enabled": true },
                    "GridContainer": { "version": "4.4.0", "enabled": true }
                },
                "theme": {
                    "name": "default",
                    "version": "4.4.0",
                    "primary_color": "#4CAF50",
                    "secondary_color": "#2196F3",
                    "background_color": "#FFFFFF",
                    "text_color": "#333333"
                }
            });

            Ok(default_config)
        }
    }

    /// 获取同步状态
    pub async fn get_sync_status(&self) -> SyncStatus {
        let sync_status = self.sync_status.read().await;
        sync_status.clone()
    }

    /// 启动配置同步服务
    pub async fn start_sync_service(&self, environment: &str, interval: u64) {
        tokio::spawn({
            let self_clone = self.clone();
            let env = environment.to_string();
            async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
                    // 从 GUF 配置服务同步
                    if let Err(e) = self_clone.sync_from_guf(&env).await {
                        let mut sync_status = self_clone.sync_status.write().await;
                        *sync_status = SyncStatus::SyncFailed(e.to_string());
                    }
                    // 使用增量同步到 GUF 配置服务
                    if let Err(e) = self_clone.incremental_sync_to_guf().await {
                        let mut sync_status = self_clone.sync_status.write().await;
                        *sync_status = SyncStatus::SyncFailed(e.to_string());
                    }
                }
            }
        });
    }
}

impl Clone for GufConfigSync {
    fn clone(&self) -> Self {
        Self {
            local_config: self.local_config.clone(),
            config_notifier: (
                self.config_notifier.0.clone(),
                self.config_notifier.1.clone(),
            ),
            item_notifier: (
                self.item_notifier.0.clone(),
                self.item_notifier.1.clone(),
            ),
            sync_status: self.sync_status.clone(),
            config_client: self.config_client.clone(),
            last_sync_version: self.last_sync_version.clone(),
            change_cache: self.change_cache.clone(),
        }
    }
}

/// 示例配置同步使用
pub async fn example_usage() {
    // 创建配置同步器
    let config_sync = GufConfigSync::new_with_config(
        "http://guf-config-server:8080".to_string(),
        "auth-token".to_string(),
    );

    // 初始化配置同步器
    if let Err(e) = config_sync.initialize("production").await {
        error!("Failed to initialize config sync: {}", e);
        return;
    }

    // 启动配置同步服务
    let config_sync_clone = config_sync.clone();
    tokio::spawn(async move {
        config_sync_clone.start_sync_service("production", 30).await;
    });

    // 添加配置项
    if let Err(e) = config_sync
        .add_config_item(
            "database.url".to_string(),
            serde_json::Value::String("postgres://localhost:5432/mydb".to_string()),
        )
        .await
    {
        error!("Failed to add config item: {}", e);
    }

    // 获取配置项
    if let Some(value) = config_sync.get_config_item("database.url").await {
        debug!("Database URL: {:?}", value);
    }

    // 监听配置变更
    let mut notifier = config_sync.get_config_notifier();
    tokio::spawn(async move {
        while notifier.changed().await.is_ok() {
            let config = notifier.borrow();
            debug!("Config changed: {:?}", config);
        }
    });
}
