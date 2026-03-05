use log::error;

pub mod adapter;
pub mod component_manager;
pub mod config_sync;
pub mod event_bus;
pub mod mock_guf_server;
pub mod plugin_system;
pub mod templates;
pub mod version_compatibility;

#[cfg(test)]
pub mod guf_test;

/// GUF 集成模块
/// 负责实现与 GUF 生态系统的集成
///
/// GUF (Godot UI Framework) 集成架构：
/// - 适配器层：处理与 GUF 框架的核心通信
/// - 组件管理层：管理 GUF 组件的生命周期
/// - 配置同步层：实现配置的双向同步
/// - 事件总线层：处理事件的发布和订阅
///
/// 主要功能：
/// - Godot UI Framework v4.4 集成
/// - 组件生命周期管理（创建、初始化、启动、停止、销毁）
/// - 配置管理和同步
/// - 事件驱动架构
/// - 依赖管理和循环检测
/// - 性能优化和并发处理
pub struct GufIntegration {
    /// GUF 适配器 - 负责与 GUF 框架的核心通信
    adapter: adapter::GufAdapter,
    /// GUF 组件管理器 - 负责组件的生命周期管理
    component_manager: component_manager::GufComponentManager,
    /// GUF 配置同步器 - 负责配置的双向同步
    config_sync: config_sync::GufConfigSync,
    /// GUF 事件总线 - 负责事件的发布和订阅
    event_bus: event_bus::GufEventBus,
    /// GUF 版本兼容层 - 负责版本兼容性管理
    version_compatibility: version_compatibility::GufVersionCompatibilityLayer,
    /// 集成状态 - 记录当前集成的运行状态
    status: IntegrationStatus,
    /// 服务器地址
    server_address: String,
    /// 认证令牌
    auth_token: String,
}

/// 集成状态
#[derive(Debug, Clone)]
pub enum IntegrationStatus {
    /// 已初始化
    Initialized,
    /// 运行中
    Running,
    /// 停止中
    Stopping,
    /// 已停止
    Stopped,
    /// 错误
    Error(String),
}

impl GufIntegration {
    /// 创建新的 GUF 集成
    pub fn new() -> Self {
        let adapter = adapter::GufAdapter::new();
        let component_manager = component_manager::GufComponentManager::new();
        let config_sync = config_sync::GufConfigSync::new();
        let event_bus = event_bus::GufEventBus::new();
        let version_compatibility = version_compatibility::GufVersionCompatibilityLayer::new();

        Self {
            adapter,
            component_manager,
            config_sync,
            event_bus,
            version_compatibility,
            status: IntegrationStatus::Stopped,
            server_address: "http://localhost:8080".to_string(),
            auth_token: "default".to_string(),
        }
    }

    /// 使用指定的适配器创建新的 GUF 集成
    pub fn new_with_adapter(adapter: adapter::GufAdapter) -> Self {
        let component_manager = component_manager::GufComponentManager::new();
        let config_sync = config_sync::GufConfigSync::new();
        let event_bus = event_bus::GufEventBus::new();
        let version_compatibility = version_compatibility::GufVersionCompatibilityLayer::new();

        Self {
            adapter,
            component_manager,
            config_sync,
            event_bus,
            version_compatibility,
            status: IntegrationStatus::Stopped,
            server_address: "http://localhost:8080".to_string(),
            auth_token: "default".to_string(),
        }
    }

    /// 使用指定的服务器地址和认证令牌创建新的 GUF 集成
    pub fn new_with_config(server_address: String, auth_token: String) -> Self {
        let adapter = adapter::GufAdapter::new();
        let component_manager = component_manager::GufComponentManager::new();
        let config_sync =
            config_sync::GufConfigSync::new_with_config(server_address.clone(), auth_token.clone());
        let event_bus = event_bus::GufEventBus::new();
        let version_compatibility = version_compatibility::GufVersionCompatibilityLayer::new();

        Self {
            adapter,
            component_manager,
            config_sync,
            event_bus,
            version_compatibility,
            status: IntegrationStatus::Stopped,
            server_address,
            auth_token,
        }
    }

    /// 初始化 GUF 集成
    pub async fn initialize(&mut self) -> Result<(), String> {
        // 启动事件总线
        self.event_bus.start().await;

        // 初始化版本兼容层
        if let Err(e) = self.version_compatibility.initialize().await {
            return Err(format!("Failed to initialize version compatibility layer: {}", e));
        }

        // 初始化适配器
        let config = adapter::GufRuntimeConfig {
            server_address: self.server_address.clone(),
            auth_info: adapter::GufAuthInfo {
                client_id: "default".to_string(),
                client_secret: "default".to_string(),
                auth_token: Some(self.auth_token.clone()),
            },
            connection_timeout: 30,
            heartbeat_interval: 60,
            pool_size: 10,
        };
        if let Err(e) = self.adapter.initialize(config).await {
            return Err(format!("Failed to initialize adapter: {}", e));
        }

        // 初始化配置同步
        if let Err(e) = self.config_sync.initialize("default").await {
            return Err(format!("Failed to initialize config sync: {}", e));
        }

        // 更新状态
        self.status = IntegrationStatus::Initialized;

        Ok(())
    }

    /// 简化的初始化方法
    pub async fn init(&mut self) -> Result<(), String> {
        // 初始化版本兼容层
        if let Err(e) = self.version_compatibility.initialize().await {
            return Err(format!("Failed to initialize version compatibility layer: {}", e));
        }

        // 初始化适配器
        if let Err(e) = self.adapter.init().await {
            return Err(format!("Failed to initialize adapter: {}", e));
        }

        // 初始化组件管理器
        if let Err(e) = self.component_manager.init().await {
            return Err(format!("Failed to initialize component manager: {}", e));
        }

        // 初始化配置同步
        if let Err(e) = self.config_sync.init().await {
            return Err(format!("Failed to initialize config sync: {}", e));
        }

        // 初始化事件总线
        if let Err(e) = self.event_bus.init().await {
            return Err(format!("Failed to initialize event bus: {}", e));
        }

        // 启动事件总线
        self.event_bus.start().await;

        // 更新状态
        self.status = IntegrationStatus::Initialized;

        Ok(())
    }

    /// 启动 GUF 集成
    pub async fn start(&mut self) -> Result<(), String> {
        // 连接到 GUF 生态系统
        if let Err(e) = self.adapter.connect().await {
            return Err(format!("Failed to connect to GUF ecosystem: {}", e));
        }

        // 启动配置同步服务
        let config_sync = self.config_sync.clone();
        tokio::spawn(async move {
            config_sync.start_sync_service("default", 30).await;
        });

        // 更新状态
        self.status = IntegrationStatus::Running;

        Ok(())
    }

    /// 停止 GUF 集成
    pub async fn stop(&mut self) -> Result<(), String> {
        // 断开与 GUF 生态系统的连接
        if let Err(e) = self.adapter.disconnect().await {
            error!("Failed to disconnect from GUF ecosystem: {}", e);
        }

        // 停止事件总线
        self.event_bus.stop().await;

        // 更新状态
        self.status = IntegrationStatus::Stopped;

        Ok(())
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        matches!(
            self.status,
            IntegrationStatus::Initialized | IntegrationStatus::Running
        )
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        matches!(self.status, IntegrationStatus::Running)
    }

    /// 获取 GUF 适配器
    pub fn get_adapter(&self) -> &adapter::GufAdapter {
        &self.adapter
    }

    /// 获取 GUF 组件管理器
    pub fn get_component_manager(&self) -> &component_manager::GufComponentManager {
        &self.component_manager
    }

    /// 获取 GUF 配置同步器
    pub fn get_config_sync(&self) -> &config_sync::GufConfigSync {
        &self.config_sync
    }

    /// 获取 GUF 事件总线
    pub fn get_event_bus(&self) -> &event_bus::GufEventBus {
        &self.event_bus
    }

    /// 获取 GUF 版本兼容层
    pub fn get_version_compatibility(&self) -> &version_compatibility::GufVersionCompatibilityLayer {
        &self.version_compatibility
    }

    /// 获取集成状态
    pub fn get_status(&self) -> IntegrationStatus {
        self.status.clone()
    }
}
