//! 插件接口模块
//! 定义GUF插件的标准接口和生命周期方法

use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 插件接口
#[async_trait::async_trait]
pub trait GufPluginInterfaceTrait: Send + Sync {
    /// 初始化插件
    async fn initialize(&self, config: serde_json::Value) -> Result<(), String>;

    /// 启动插件
    async fn start(&self) -> Result<(), String>;

    /// 停止插件
    async fn stop(&self) -> Result<(), String>;

    /// 销毁插件
    async fn destroy(&self) -> Result<(), String>;

    /// 获取插件信息
    fn get_info(&self) -> PluginInfo;

    /// 处理请求
    async fn handle_request(&self, request: serde_json::Value)
    -> Result<serde_json::Value, String>;

    /// 处理事件
    async fn handle_event(&self, event: &str, data: serde_json::Value) -> Result<(), String>;

    /// 获取插件状态
    fn get_status(&self) -> String;
}

/// 插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
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

/// 插件接口管理器
#[derive(Clone)]
pub struct GufPluginInterface {
    /// 已注册的插件接口
    registered_interfaces: Arc<
        tokio::sync::RwLock<std::collections::HashMap<String, Box<dyn GufPluginInterfaceTrait>>>,
    >,
}

impl GufPluginInterface {
    /// 创建新的插件接口管理器
    pub fn new() -> Self {
        Self {
            registered_interfaces: Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// 初始化插件接口管理器
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件接口管理器");
        Ok(())
    }

    /// 注册插件接口
    pub async fn register_interface(
        &self,
        plugin_name: &str,
        interface: Box<dyn GufPluginInterfaceTrait>,
    ) -> Result<(), String> {
        info!("注册插件接口: {}", plugin_name);

        let mut interfaces = self.registered_interfaces.write().await;
        interfaces.insert(plugin_name.to_string(), interface);

        Ok(())
    }

    /// 获取插件接口
    pub async fn get_interface(
        &self,
        plugin_name: &str,
    ) -> Result<Option<Box<dyn GufPluginInterfaceTrait>>, String> {
        let interfaces = self.registered_interfaces.read().await;
        match interfaces.get(plugin_name) {
            Some(_) => {
                // 这里简化处理，返回错误，实际实现需要根据具体的插件接口类型进行克隆
                Err("暂不支持获取插件接口".to_string())
            }
            None => Err(format!("插件接口不存在: {}", plugin_name)),
        }
    }

    /// 获取插件接口克隆（需要实现具体类型的克隆）
    pub async fn get_interface_clone(
        &self,
        _plugin_name: &str,
    ) -> Result<Box<dyn GufPluginInterfaceTrait>, String> {
        // 注意：实际实现需要根据具体的插件接口类型进行克隆
        // 这里简化处理，返回错误
        Err("暂不支持克隆插件接口".to_string())
    }

    /// 初始化插件
    pub async fn initialize_plugin(
        &self,
        plugin_name: &str,
        config: serde_json::Value,
    ) -> Result<(), String> {
        info!("初始化插件: {}", plugin_name);

        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.initialize(config).await
    }

    /// 启动插件
    pub async fn start_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("启动插件: {}", plugin_name);

        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.start().await
    }

    /// 停止插件
    pub async fn stop_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("停止插件: {}", plugin_name);

        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.stop().await
    }

    /// 销毁插件
    pub async fn destroy_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("销毁插件: {}", plugin_name);

        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.destroy().await
    }

    /// 获取插件信息
    pub async fn get_plugin_info(&self, plugin_name: &str) -> Result<PluginInfo, String> {
        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        Ok(interface.get_info())
    }

    /// 处理插件请求
    pub async fn handle_plugin_request(
        &self,
        plugin_name: &str,
        request: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.handle_request(request).await
    }

    /// 处理插件事件
    pub async fn handle_plugin_event(
        &self,
        plugin_name: &str,
        event: &str,
        data: serde_json::Value,
    ) -> Result<(), String> {
        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        interface.handle_event(event, data).await
    }

    /// 获取插件状态
    pub async fn get_plugin_status(&self, plugin_name: &str) -> Result<String, String> {
        let interfaces = self.registered_interfaces.read().await;
        let interface = interfaces
            .get(plugin_name)
            .ok_or_else(|| format!("插件接口不存在: {}", plugin_name))?;

        Ok(interface.get_status())
    }

    /// 注销插件接口
    pub async fn unregister_interface(&self, plugin_name: &str) -> Result<(), String> {
        info!("注销插件接口: {}", plugin_name);

        let mut interfaces = self.registered_interfaces.write().await;
        if interfaces.remove(plugin_name).is_none() {
            return Err(format!("插件接口不存在: {}", plugin_name));
        }

        Ok(())
    }

    /// 列出所有已注册的插件接口
    pub async fn list_interfaces(&self) -> Result<Vec<String>, String> {
        let interfaces = self.registered_interfaces.read().await;
        Ok(interfaces.keys().cloned().collect())
    }

    /// 检查插件接口是否存在
    pub async fn interface_exists(&self, plugin_name: &str) -> bool {
        let interfaces = self.registered_interfaces.read().await;
        interfaces.contains_key(plugin_name)
    }
}

/// 示例插件接口实现
#[derive(Debug, Clone)]
pub struct ExamplePluginInterface {
    info: PluginInfo,
}

impl ExamplePluginInterface {
    /// 创建新的示例插件接口
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "example-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "示例插件".to_string(),
                author: "GUF Team".to_string(),
                r#type: "example".to_string(),
                language: "rust".to_string(),
                platform: vec![
                    "windows".to_string(),
                    "linux".to_string(),
                    "macos".to_string(),
                ],
                dependencies: vec![],
            },
        }
    }
}

#[async_trait::async_trait]
impl GufPluginInterfaceTrait for ExamplePluginInterface {
    async fn initialize(&self, config: serde_json::Value) -> Result<(), String> {
        info!("初始化示例插件，配置: {:?}", config);
        Ok(())
    }

    async fn start(&self) -> Result<(), String> {
        info!("启动示例插件");
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        info!("停止示例插件");
        Ok(())
    }

    async fn destroy(&self) -> Result<(), String> {
        info!("销毁示例插件");
        Ok(())
    }

    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn handle_request(
        &self,
        request: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        info!("处理示例插件请求: {:?}", request);
        Ok(serde_json::json!({
            "message": "示例插件请求处理成功",
            "request": request
        }))
    }

    async fn handle_event(&self, event: &str, data: serde_json::Value) -> Result<(), String> {
        info!("处理示例插件事件: {}, 数据: {:?}", event, data);
        Ok(())
    }

    fn get_status(&self) -> String {
        "running".to_string()
    }
}
