//! GUF插件系统模块
//! 负责管理跨语言、跨平台的GUF插件

pub mod plugin_communicator;
pub mod plugin_dependency;
pub mod plugin_interface;
pub mod plugin_loader;
pub mod plugin_manager;
pub mod plugin_market;
pub mod sandbox;
pub mod signature;

use serde::{Deserialize, Serialize};

/// GUF插件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufPluginConfig {
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件描述
    pub description: String,
    /// 插件作者
    pub author: String,
    /// 插件类型
    pub r#type: String,
    /// 插件语言
    pub language: String,
    /// 插件平台
    pub platform: Vec<String>,
    /// 插件依赖
    pub dependencies: Vec<PluginDependency>,
    /// 插件配置
    pub config: serde_json::Value,
}

/// 插件依赖
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// 依赖名称
    pub name: String,
    /// 依赖版本
    pub version: String,
    /// 依赖类型
    pub r#type: String,
}

/// 插件状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginStatus {
    /// 初始化中
    Initializing,
    /// 就绪
    Ready,
    /// 运行中
    Running,
    /// 停止中
    Stopping,
    /// 已停止
    Stopped,
    /// 错误
    Error(String),
}

/// GUF插件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufPlugin {
    /// 插件配置
    pub config: GufPluginConfig,
    /// 插件状态
    pub status: PluginStatus,
    /// 插件路径
    pub path: String,
    /// 插件加载时间
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// 插件启动时间
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// GUF插件系统
#[derive(Clone)]
pub struct GufPluginSystem {
    /// 插件管理器
    plugin_manager: plugin_manager::GufPluginManager,
    /// 插件接口
    plugin_interface: plugin_interface::GufPluginInterface,
    /// 插件通信器
    plugin_communicator: plugin_communicator::GufPluginCommunicator,
    /// 插件加载器
    plugin_loader: plugin_loader::GufPluginLoader,
    /// 插件市场
    plugin_market: plugin_market::GufPluginMarket,
    /// 插件依赖管理器
    plugin_dependency: plugin_dependency::GufPluginDependencyManager,
    /// 插件沙箱
    plugin_sandbox: sandbox::GufPluginSandbox,
    /// 插件签名管理器
    plugin_signature: signature::GufPluginSignatureManager,
}

impl GufPluginSystem {
    /// 创建新的GUF插件系统
    pub fn new() -> Self {
        Self {
            plugin_manager: plugin_manager::GufPluginManager::new(),
            plugin_interface: plugin_interface::GufPluginInterface::new(),
            plugin_communicator: plugin_communicator::GufPluginCommunicator::new(),
            plugin_loader: plugin_loader::GufPluginLoader::new(),
            plugin_market: plugin_market::GufPluginMarket::new(),
            plugin_dependency: plugin_dependency::GufPluginDependencyManager::new(),
            plugin_sandbox: sandbox::GufPluginSandbox::new(),
            plugin_signature: signature::GufPluginSignatureManager::new(),
        }
    }

    /// 初始化GUF插件系统
    pub async fn initialize(&self) -> Result<(), String> {
        // 初始化插件管理器
        if let Err(e) = self.plugin_manager.initialize().await {
            return Err(format!("Failed to initialize plugin manager: {}", e));
        }

        // 初始化插件接口
        if let Err(e) = self.plugin_interface.initialize().await {
            return Err(format!("Failed to initialize plugin interface: {}", e));
        }

        // 初始化插件通信器
        if let Err(e) = self.plugin_communicator.initialize().await {
            return Err(format!("Failed to initialize plugin communicator: {}", e));
        }

        // 初始化插件加载器
        if let Err(e) = self.plugin_loader.initialize().await {
            return Err(format!("Failed to initialize plugin loader: {}", e));
        }

        // 初始化插件市场
        if let Err(e) = self.plugin_market.initialize().await {
            return Err(format!("Failed to initialize plugin market: {}", e));
        }

        // 初始化插件依赖管理器
        if let Err(e) = self.plugin_dependency.initialize().await {
            return Err(format!(
                "Failed to initialize plugin dependency manager: {}",
                e
            ));
        }

        // 初始化插件沙箱
        if let Err(e) = self.plugin_sandbox.initialize().await {
            return Err(format!("Failed to initialize plugin sandbox: {}", e));
        }

        // 初始化插件签名管理器
        if let Err(e) = self.plugin_signature.initialize().await {
            return Err(format!("Failed to initialize plugin signature manager: {}", e));
        }

        Ok(())
    }

    /// 加载插件
    pub async fn load_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        self.plugin_loader.load_plugin(plugin_path).await
    }

    /// 启动插件
    pub async fn start_plugin(&self, plugin_name: &str) -> Result<(), String> {
        self.plugin_manager.start_plugin(plugin_name).await
    }

    /// 停止插件
    pub async fn stop_plugin(&self, plugin_name: &str) -> Result<(), String> {
        self.plugin_manager.stop_plugin(plugin_name).await
    }

    /// 卸载插件
    pub async fn unload_plugin(&self, plugin_name: &str) -> Result<(), String> {
        self.plugin_manager.unload_plugin(plugin_name).await
    }

    /// 获取插件状态
    pub async fn get_plugin_status(&self, plugin_name: &str) -> Result<PluginStatus, String> {
        self.plugin_manager.get_plugin_status(plugin_name).await
    }

    /// 列出所有插件
    pub async fn list_plugins(&self) -> Result<Vec<GufPlugin>, String> {
        self.plugin_manager.list_plugins().await
    }

    /// 搜索插件市场
    pub async fn search_plugins(
        &self,
        query: &str,
    ) -> Result<Vec<plugin_market::PluginInfo>, String> {
        self.plugin_market.search_plugins(query).await
    }

    /// 安装插件
    pub async fn install_plugin(&self, plugin_id: &str) -> Result<GufPlugin, String> {
        self.plugin_market.install_plugin(plugin_id).await
    }

    /// 卸载插件
    pub async fn uninstall_plugin(&self, plugin_name: &str) -> Result<(), String> {
        self.plugin_market.uninstall_plugin(plugin_name).await
    }

    /// 更新插件
    pub async fn update_plugin(&self, plugin_name: &str) -> Result<GufPlugin, String> {
        self.plugin_market.update_plugin(plugin_name).await
    }

    /// 检查插件依赖
    pub async fn check_dependencies(
        &self,
        plugin_name: &str,
    ) -> Result<Vec<PluginDependency>, String> {
        self.plugin_dependency.check_dependencies(plugin_name).await
    }

    /// 解析插件依赖
    pub async fn resolve_dependencies(&self, plugin_name: &str) -> Result<(), String> {
        self.plugin_dependency
            .resolve_dependencies(plugin_name)
            .await
    }

    /// 插件间通信
    pub async fn send_message(
        &self,
        from_plugin: &str,
        to_plugin: &str,
        message: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        self.plugin_communicator
            .send_message(from_plugin, to_plugin, message)
            .await
    }

    /// 发布插件事件
    pub async fn publish_event(
        &self,
        plugin_name: &str,
        event: &str,
        data: serde_json::Value,
    ) -> Result<(), String> {
        self.plugin_communicator
            .publish_event(plugin_name, event, data)
            .await
    }

    /// 订阅插件事件
    pub async fn subscribe_event(
        &self,
        plugin_name: &str,
        event: &str,
        callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<(), String> {
        self.plugin_communicator
            .subscribe_event(plugin_name, event, callback.into())
            .await
    }

    /// 为插件设置权限
    pub async fn set_plugin_permissions(
        &self,
        plugin_name: &str,
        permissions: Vec<sandbox::PluginPermission>,
    ) -> Result<(), String> {
        self.plugin_sandbox.set_permissions(plugin_name, permissions).await
    }

    /// 检查插件是否有特定权限
    pub async fn check_plugin_permission(
        &self,
        plugin_name: &str,
        permission: &sandbox::PluginPermission,
    ) -> Result<bool, String> {
        self.plugin_sandbox.check_permission(plugin_name, permission).await
    }

    /// 验证插件操作是否允许
    pub async fn validate_plugin_operation(
        &self,
        plugin_name: &str,
        operation: &str,
        details: serde_json::Value,
    ) -> Result<bool, String> {
        self.plugin_sandbox.validate_operation(plugin_name, operation, details).await
    }

    /// 生成插件签名
    pub async fn sign_plugin(
        &self,
        plugin_path: &str,
        private_key_path: &str,
        signer: &str,
    ) -> Result<signature::PluginSignature, String> {
        self.plugin_signature.sign_plugin(plugin_path, private_key_path, signer).await
    }

    /// 验证插件签名
    pub async fn verify_plugin(&self, plugin_path: &str) -> Result<signature::PluginSignature, String> {
        self.plugin_signature.verify_plugin(plugin_path).await
    }

    /// 注册签名者公钥
    pub async fn register_signer_public_key(&self, signer: &str, public_key: &str) -> Result<(), String> {
        self.plugin_signature.register_public_key(signer, public_key).await
    }

    /// 验证签名者
    pub async fn verify_signer(&self, signer: &str, public_key: &str) -> Result<bool, String> {
        self.plugin_signature.verify_signer(signer, public_key).await
    }
}
