use crate::core::state::AppState;
use crate::ui::core::adapter::GufVersion;
use chrono;
use log::{debug, error, info, warn};
use reqwest;
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GUF 版本范围
#[derive(Debug, Clone)]
pub struct GufVersionRange {
    /// 最低版本
    pub min: GufVersion,
    /// 最高版本
    pub max: GufVersion,
}

/// 版本兼容性结果
#[derive(Debug, Clone)]
pub enum VersionCompatibility {
    /// 完全兼容
    Compatible,
    /// 部分兼容
    PartiallyCompatible(Vec<String>),
    /// 不兼容
    Incompatible(Vec<String>),
}

impl GufVersionRange {
    /// 检查版本是否在范围内
    pub fn contains(&self, version: &GufVersion) -> bool {
        (version.major > self.min.major || 
         (version.major == self.min.major && version.minor >= self.min.minor)) &&
        (version.major < self.max.major || 
         (version.major == self.max.major && version.minor <= self.max.minor))
    }
}

/// GUF 核心适配器
/// 负责实现与 GUF 框架的核心连接和通信
pub struct GufAdapter {
    /// GUF 运行时实例
    runtime: Option<Arc<GufRuntime>>,
    /// 连接状态
    connected: bool,
    /// 应用状态
    _app_state: Arc<RwLock<AppState>>,
    /// Godot UI Framework 版本
    godot_version: GufVersion,
    /// 兼容的 GUF 版本范围
    compatible_versions: Vec<GufVersionRange>,
    /// 版本检测结果
    version_compatibility: Option<VersionCompatibility>,
}

/// GUF 运行时
pub struct GufRuntime {
    /// 运行时配置
    config: GufRuntimeConfig,
    /// 连接管理器
    connection_manager: Arc<GufConnectionManager>,
    /// 组件注册表
    component_registry: Arc<RwLock<GufComponentRegistry>>,
}

/// GUF 运行时配置
#[derive(Clone)]
pub struct GufRuntimeConfig {
    /// GUF 服务器地址
    pub server_address: String,
    /// 认证信息
    pub auth_info: GufAuthInfo,
    /// 连接超时
    pub connection_timeout: u64,
    /// 心跳间隔
    pub heartbeat_interval: u64,
    /// 连接池大小
    pub pool_size: usize,
}

/// GUF 认证信息
#[derive(Clone)]
pub struct GufAuthInfo {
    /// 客户端 ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 认证令牌
    pub auth_token: Option<String>,
}

/// GUF 连接管理器
#[derive(Clone)]
pub struct GufConnectionManager {
    /// 活跃连接
    active_connections: Arc<RwLock<Vec<GufConnection>>>,
    /// 连接池大小
    pool_size: usize,
}

/// GUF 连接
pub struct GufConnection {
    /// 连接 ID
    _id: String,
    /// 连接状态
    state: GufConnectionState,
    /// 最后活动时间
    last_activity: u64,
}

/// GUF 连接状态
pub enum GufConnectionState {
    /// 已连接
    Connected,
    /// 连接中
    Connecting,
    /// 已断开
    Disconnected,
    /// 错误
    Error(String),
}

/// GUF 组件注册表
pub struct GufComponentRegistry {
    /// 已注册组件
    components: Vec<GufComponentInfo>,
}

/// GUF 组件信息
#[derive(Debug, Clone)]
pub struct GufComponentInfo {
    /// 组件 ID
    pub id: String,
    /// 组件名称
    pub name: String,
    /// 组件版本
    pub version: String,
    /// 组件状态
    pub status: GufComponentStatus,
}

/// GUF 组件状态
#[derive(Debug, Clone, PartialEq)]
pub enum GufComponentStatus {
    /// 已注册
    Registered,
    /// 初始化中
    Initializing,
    /// 已启动
    Started,
    /// 已停止
    Stopped,
    /// 错误
    Error(String),
}

impl GufAdapter {
    /// 创建新的 GUF 适配器
    pub fn new() -> Self {
        // 创建默认的 app_state
        let app_state = Arc::new(RwLock::new(AppState::new()));
        // 初始化 Godot UI Framework v4.4 版本
        let godot_version = GufVersion {
            major: 4,
            minor: 4,
            patch: 0,
        };
        // 定义兼容的版本范围
        let compatible_versions = vec![
            GufVersionRange {
                min: GufVersion { major: 4, minor: 3, patch: 0 },
                max: GufVersion { major: 4, minor: 4, patch: 0 },
            },
        ];
        Self {
            runtime: None,
            connected: false,
            _app_state: app_state,
            godot_version,
            compatible_versions,
            version_compatibility: None,
        }
    }

    /// 使用指定的 app_state 创建新的 GUF 适配器
    pub fn new_with_app_state(app_state: Arc<RwLock<AppState>>) -> Self {
        // 初始化 Godot UI Framework v4.4 版本
        let godot_version = GufVersion {
            major: 4,
            minor: 4,
            patch: 0,
        };
        // 定义兼容的版本范围
        let compatible_versions = vec![
            GufVersionRange {
                min: GufVersion { major: 4, minor: 3, patch: 0 },
                max: GufVersion { major: 4, minor: 4, patch: 0 },
            },
        ];
        Self {
            runtime: None,
            connected: false,
            _app_state: app_state,
            godot_version,
            compatible_versions,
            version_compatibility: None,
        }
    }

    /// 检查版本兼容性
    pub async fn check_version_compatibility(&mut self, server_version: &GufVersion) -> VersionCompatibility {
        let mut issues = Vec::new();
        
        // 检查是否在兼容范围内
        let is_in_range = self.compatible_versions.iter().any(|range| range.contains(server_version));
        
        if !is_in_range {
            issues.push(format!("GUF version {} is not in compatible range", 
                format!("{}.{}.{}", server_version.major, server_version.minor, server_version.patch)));
        }
        
        // 检查主要版本兼容性
        if server_version.major != self.godot_version.major {
            issues.push("Major version mismatch detected".to_string());
        }
        
        // 检查次要版本兼容性
        if server_version.minor < self.godot_version.minor {
            issues.push("Server version is older than client version".to_string());
        }
        
        // 确定兼容性结果
        let compatibility = if issues.is_empty() {
            VersionCompatibility::Compatible
        } else if server_version.major == self.godot_version.major {
            VersionCompatibility::PartiallyCompatible(issues)
        } else {
            VersionCompatibility::Incompatible(issues)
        };
        
        // 保存兼容性结果
        self.version_compatibility = Some(compatibility.clone());
        
        compatibility
    }

    /// 获取版本兼容性结果
    pub fn get_version_compatibility(&self) -> Option<&VersionCompatibility> {
        self.version_compatibility.as_ref()
    }

    /// 获取 Godot UI Framework 版本
    pub fn godot_version(&self) -> &GufVersion {
        &self.godot_version
    }

    /// 初始化 GUF 适配器
    pub async fn initialize(&mut self, config: GufRuntimeConfig) -> Result<(), String> {
        // 初始化 UI 适配器系统
        if let Err(e) = crate::ui::core::adapter::initialize().await {
            return Err(format!("Failed to initialize UI adapter: {:?}", e));
        }

        // 创建 GUF 运行时
        let runtime = self.create_runtime(config).await?;
        self.runtime = Some(Arc::new(runtime));

        // 连接到 GUF 生态系统
        self.connect().await?;

        // 检查版本兼容性
        let server_version = self.godot_version.clone();
        let compatibility = self.check_version_compatibility(&server_version).await;
        
        match compatibility {
            VersionCompatibility::Incompatible(issues) => {
                return Err(format!("Version compatibility issues: {}", issues.join(", ")));
            }
            VersionCompatibility::PartiallyCompatible(issues) => {
                // 记录部分兼容的警告
                warn!("Version compatibility issues: {}", issues.join(", "));
            }
            _ => {}
        }

        // 注册 Godot UI Framework 组件
        self.register_godot_components().await?;

        Ok(())
    }

    /// 简化的初始化方法
    pub async fn init(&mut self) -> Result<(), String> {
        // 创建默认配置
        let config = GufRuntimeConfig {
            server_address: "http://localhost:8080".to_string(),
            auth_info: GufAuthInfo {
                client_id: "default".to_string(),
                client_secret: "default".to_string(),
                auth_token: None,
            },
            connection_timeout: 30,
            heartbeat_interval: 60,
            pool_size: 10,
        };

        self.initialize(config).await
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.runtime.is_some()
    }

    /// 创建 GUF 运行时
    async fn create_runtime(&self, config: GufRuntimeConfig) -> Result<GufRuntime, String> {
        // 初始化连接管理器
        let connection_manager = Arc::new(GufConnectionManager::new(config.pool_size));

        // 初始化组件注册表
        let component_registry = Arc::new(RwLock::new(GufComponentRegistry::new()));

        Ok(GufRuntime {
            config,
            connection_manager,
            component_registry,
        })
    }

    /// 连接到 GUF 生态系统
    pub async fn connect(&mut self) -> Result<(), String> {
        if let Some(runtime) = &self.runtime {
            // 建立连接
            let result = runtime.connection_manager.connect(&runtime.config).await;
            match result {
                Ok(_) => {
                    self.connected = true;
                    Ok(())
                }
                Err(e) => Err(format!("Failed to connect to GUF ecosystem: {}", e)),
            }
        } else {
            Err("GUF runtime not initialized".to_string())
        }
    }

    /// 断开与 GUF 生态系统的连接
    pub async fn disconnect(&mut self) -> Result<(), String> {
        if let Some(runtime) = &self.runtime {
            // 断开连接
            runtime.connection_manager.disconnect().await;
            self.connected = false;
            Ok(())
        } else {
            Err("GUF runtime not initialized".to_string())
        }
    }

    /// 注册组件到 GUF 生态系统
    pub async fn register_component(&self, component_info: GufComponentInfo) -> Result<(), String> {
        if let Some(runtime) = &self.runtime {
            // 注册组件
            let mut registry = runtime.component_registry.write().await;
            registry.register(component_info);
            Ok(())
        } else {
            Err("GUF runtime not initialized".to_string())
        }
    }

    /// 获取 GUF 生态系统中的组件
    pub async fn get_components(&self) -> Result<Vec<GufComponentInfo>, String> {
        if let Some(runtime) = &self.runtime {
            // 获取组件列表
            let registry = runtime.component_registry.read().await;
            Ok(registry.components.clone())
        } else {
            Err("GUF runtime not initialized".to_string())
        }
    }

    /// 检查连接状态
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// 获取 GUF 运行时
    pub fn get_runtime(&self) -> Option<Arc<GufRuntime>> {
        self.runtime.clone()
    }

    /// 注册 Godot UI Framework v4.4 组件
    async fn register_godot_components(&self) -> Result<(), String> {
        if let Some(runtime) = &self.runtime {
            let mut registry = runtime.component_registry.write().await;

            // 注册常用 Godot 组件
            let godot_components = vec![
                GufComponentInfo {
                    id: "godot_button".to_string(),
                    name: "Button".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_label".to_string(),
                    name: "Label".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_texture_button".to_string(),
                    name: "TextureButton".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_color_rect".to_string(),
                    name: "ColorRect".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_panel".to_string(),
                    name: "Panel".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_v_box_container".to_string(),
                    name: "VBoxContainer".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_h_box_container".to_string(),
                    name: "HBoxContainer".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
                GufComponentInfo {
                    id: "godot_grid_container".to_string(),
                    name: "GridContainer".to_string(),
                    version: "4.4.0".to_string(),
                    status: GufComponentStatus::Registered,
                },
            ];

            for component in godot_components {
                registry.register(component);
            }

            Ok(())
        } else {
            Err("GUF runtime not initialized".to_string())
        }
    }
}

impl GufConnectionManager {
    /// 创建新的连接管理器
    pub fn new(pool_size: usize) -> Self {
        Self {
            active_connections: Arc::new(RwLock::new(Vec::new())),
            pool_size,
        }
    }

    /// 连接到 GUF 生态系统
    pub async fn connect(&self, config: &GufRuntimeConfig) -> Result<(), String> {
        // 清理现有的连接
        let mut connections = self.active_connections.write().await;
        connections.clear();
        drop(connections);

        // 并行建立新的连接池，提高性能
        // 使用批量处理，每批处理的连接数
        let batch_size = 10; // 增加批处理大小以提高性能
        let mut new_connections = Vec::with_capacity(self.pool_size);
        let mut _failed_attempts = 0;

        // 分批建立连接，避免一次性创建过多任务
        for batch_start in (0..self.pool_size).step_by(batch_size) {
            let batch_end = std::cmp::min(batch_start + batch_size, self.pool_size);
            let mut batch_tasks = Vec::with_capacity(batch_end - batch_start);

            for i in batch_start..batch_end {
                let config_clone = config.clone();
                let self_clone = self.clone();
                batch_tasks.push(tokio::spawn(async move {
                    self_clone.establish_connection(&config_clone, i).await
                }));
            }

            // 收集批次连接结果
            for task in batch_tasks {
                match task.await {
                    Ok(Ok(connection)) => new_connections.push(connection),
                    Ok(Err(e)) => {
                        _failed_attempts += 1;
                        error!("Connection failed: {}", e);
                    }
                    Err(e) => {
                        _failed_attempts += 1;
                        error!("Connection task failed: {:?}", e);
                    }
                }
            }

            // 批次之间添加短暂延迟，避免对服务器造成过大压力
            if batch_end < self.pool_size {
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await; // 减少延迟以提高性能
            }
        }

        // 检查是否有足够的连接建立成功
        if new_connections.is_empty() {
            return Err("Failed to establish any connections to GUF ecosystem".to_string());
        }

        // 添加到活跃连接
        let mut connections = self.active_connections.write().await;
        connections.extend(new_connections);

        // 启动心跳检测
        self.start_heartbeat(config.heartbeat_interval).await;

        // 启动连接恢复任务
        self.start_connection_recovery(config.clone()).await;

        info!("Connected to GUF ecosystem with {} connections", connections.len());
        Ok(())
    }

    /// 建立单个连接
    async fn establish_connection(
        &self,
        config: &GufRuntimeConfig,
        index: usize,
    ) -> Result<GufConnection, String> {
        // 创建连接ID
        let connection_id = format!("conn_{}_{}", index, chrono::Utc::now().timestamp());

        // 构建请求URL
        let url = format!("{}/api/v1/connect", config.server_address);

        // 构建请求体
        let request_body = serde_json::json!({
            "client_id": config.auth_info.client_id,
            "client_secret": config.auth_info.client_secret,
            "version": "1.0.0",
            "timestamp": chrono::Utc::now().timestamp()
        });

        // 连接重试机制 - 实现指数退避策略
        let max_retries = 5;
        let base_retry_delay = tokio::time::Duration::from_secs(1);

        for attempt in 1..=max_retries {
            // 发送连接请求
            let client = reqwest::Client::new();
            match client
                .post(&url)
                .json(&request_body)
                .timeout(tokio::time::Duration::from_secs(config.connection_timeout))
                .send()
                .await
            {
                Ok(response) => {
                    // 检查响应
                    if response.status().is_success() {
                        // 解析响应
                        match response.json::<serde_json::Value>().await {
                            Ok(data) => {
                                // 提取认证令牌（如果有）
                                if let Some(_token) = data.get("auth_token").and_then(|v| v.as_str()) {
                                    // 这里可以存储认证令牌
                                }
                                
                                // 创建连接对象
                                return Ok(GufConnection {
                                    _id: connection_id,
                                    state: GufConnectionState::Connected,
                                    last_activity: chrono::Utc::now().timestamp() as u64,
                                });
                            }
                            Err(e) => {
                                if attempt < max_retries {
                                    let delay = base_retry_delay.mul_f64(2.0_f64.powi(attempt as i32 - 1));
                                    tokio::time::sleep(delay).await;
                                    continue;
                                }
                                return Err(format!("Failed to parse response: {}", e));
                            }
                        }
                    } else {
                        if attempt < max_retries {
                            let delay = base_retry_delay.mul_f64(2.0_f64.powi(attempt as i32 - 1));
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        return Err(format!("Connection failed: {}", response.status()));
                    }
                }
                Err(e) => {
                    if attempt < max_retries {
                        let delay = base_retry_delay.mul_f64(2.0_f64.powi(attempt as i32 - 1));
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err(format!("Network error: {}", e));
                }
            }
        }

        Err("Max connection retries exceeded".to_string())
    }

    /// 断开与 GUF 生态系统的连接
    pub async fn disconnect(&self) {
        let mut connections = self.active_connections.write().await;
        for connection in connections.iter_mut() {
            connection.state = GufConnectionState::Disconnected;
        }
        connections.clear();
    }

    /// 获取活跃连接数
    pub async fn get_active_connections(&self) -> usize {
        let connections = self.active_connections.read().await;
        connections.len()
    }

    /// 启动心跳检测
    async fn start_heartbeat(&self, interval: u64) {
        let active_connections = self.active_connections.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

                let mut connections = active_connections.write().await;
                let mut to_remove = Vec::new();

                for (i, connection) in connections.iter_mut().enumerate() {
                    if !Self::send_heartbeat(connection).await {
                        to_remove.push(i);
                    } else {
                        connection.last_activity = chrono::Utc::now().timestamp() as u64;
                    }
                }

                // 移除失败的连接
                for i in to_remove.iter().rev() {
                    connections.remove(*i);
                }
            }
        });
    }

    /// 启动连接恢复任务
    async fn start_connection_recovery(&self, config: GufRuntimeConfig) {
        let active_connections = self.active_connections.clone();
        let pool_size = self.pool_size;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

                let current_connections = active_connections.read().await.len();
                if current_connections < pool_size {
                    warn!("Connection pool below threshold: {} out of {}", current_connections, pool_size);
                    
                    // 尝试恢复连接
                    let needed_connections = pool_size - current_connections;
                    let mut new_connections = Vec::with_capacity(needed_connections);

                    for i in 0..needed_connections {
                        let config_clone = config.clone();
                        // 创建临时连接管理器进行恢复
                        let temp_manager = GufConnectionManager::new(1);
                        match temp_manager.establish_connection(&config_clone, current_connections + i).await {
                            Ok(connection) => {
                                new_connections.push(connection);
                                debug!("Recovered connection {}", i + 1);
                            }
                            Err(e) => {
                                error!("Failed to recover connection: {}", e);
                            }
                        }
                    }

                    // 添加恢复的连接
                    if !new_connections.is_empty() {
                        let count = new_connections.len();
                        let mut connections = active_connections.write().await;
                        connections.extend(new_connections);
                        info!("Added {} recovered connections", count);
                    }
                }
            }
        });
    }

    /// 发送心跳
    async fn send_heartbeat(connection: &mut GufConnection) -> bool {
        // 模拟心跳逻辑 - 实际实现应该发送真实的心跳请求
        // 这里添加一些随机失败来测试恢复机制
        if rand::random::<f64>() < 0.01 { // 1% 失败率
            connection.state = GufConnectionState::Error("Heartbeat failed".to_string());
            error!("Heartbeat failed for connection");
            false
        } else {
            connection.state = GufConnectionState::Connected;
            true
        }
    }
}

impl GufComponentRegistry {
    /// 创建新的组件注册表
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// 注册组件
    pub fn register(&mut self, component_info: GufComponentInfo) {
        self.components.push(component_info);
    }

    /// 注销组件
    pub fn unregister(&mut self, component_id: &str) {
        self.components.retain(|c| c.id != component_id);
    }

    /// 获取组件
    pub fn get(&self, component_id: &str) -> Option<&GufComponentInfo> {
        self.components.iter().find(|c| c.id == component_id)
    }

    /// 更新组件状态
    pub fn update_status(&mut self, component_id: &str, status: GufComponentStatus) {
        if let Some(component) = self.components.iter_mut().find(|c| c.id == component_id) {
            component.status = status;
        }
    }
}

/// GUF 适配器错误
#[derive(Debug, thiserror::Error)]
pub enum GufAdapterError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Initialization error: {0}")]
    InitializationError(String),
    #[error("Component error: {0}")]
    ComponentError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Version compatibility error: {0}")]
    VersionCompatibilityError(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Response error: {0}")]
    ResponseError(String),
}
