// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::state::AppState;
use async_trait::async_trait;
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 插件生命周期钩子
#[async_trait]
pub trait PluginLifecycle: Send + Sync {
    /// 插件初始化
    async fn init(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        Ok(())
    }

    /// 插件启动
    async fn start(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        Ok(())
    }

    /// 插件停止
    async fn stop(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        Ok(())
    }

    /// 获取插件名称
    fn name(&self) -> &'static str {
        "unknown"
    }

    /// 获取插件版本
    fn version(&self) -> &'static str {
        "0.0.0"
    }

    /// 获取插件描述
    fn description(&self) -> &'static str {
        ""
    }

    /// 获取插件类型
    fn plugin_type(&self) -> &'static str {
        "custom"
    }
}

/// 迭代接口请求
#[derive(Debug, Deserialize)]
pub struct IterateRequest {
    /// 插件路径
    pub plugin_path: String,
    /// 功能标识
    pub feature_id: String,
    /// 依赖列表
    pub dependencies: Vec<String>,
    /// 插件版本
    pub plugin_version: String,
    /// 核心版本
    pub core_version: String,
}

/// 迭代接口响应
#[derive(Debug, Serialize)]
pub struct IterateResponse {
    /// 成功标志
    pub success: bool,
    /// 消息
    pub message: String,
    /// 数据
    pub data: Option<serde_json::Value>,
    /// 错误码
    pub error_code: Option<u32>,
}

/// 迭代错误
#[derive(Debug, thiserror::Error)]
pub enum IterateError {
    /// 插件路径无效
    #[error("Invalid plugin path: {0}")]
    InvalidPluginPath(String),
    /// 功能标识重复
    #[error("Duplicate feature ID: {0}")]
    DuplicateFeatureId(String),
    /// 依赖缺失
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    /// 版本不兼容
    #[error("Version incompatible: core version {0} does not support plugin version {1}")]
    VersionIncompatible(String, String),
    /// 初始化失败
    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),
    /// 启动失败
    #[error("Plugin start failed: {0}")]
    StartFailed(String),
    /// 停止失败
    #[error("Plugin stop failed: {0}")]
    StopFailed(String),
    /// 内部错误
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<String> for IterateError {
    fn from(msg: String) -> Self {
        IterateError::InternalError(msg)
    }
}

/// 标准化迭代接口
pub struct IterateApi {
    /// 已注册的功能
    registered_features:
        Mutex<std::collections::HashMap<String, Box<dyn PluginLifecycle + Send + Sync>>>,
}

impl IterateApi {
    /// 创建新的迭代接口
    pub fn new() -> Self {
        Self {
            registered_features: Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// 注册插件
    pub async fn register_plugin(
        &self,
        request: IterateRequest,
    ) -> Result<IterateResponse, IterateError> {
        // 验证版本兼容性
        self.validate_version(&request.core_version, &request.plugin_version)?;
        // 验证依赖
        self.validate_dependencies(&request.dependencies)?;
        // 验证功能标识唯一性
        {
            let features = self.registered_features.lock().await;
            if features.contains_key(&request.feature_id) {
                return Err(IterateError::DuplicateFeatureId(request.feature_id));
            }
        }
        // 动态加载插件（简化实现，实际需要动态加载）
        let plugin = self.load_plugin(&request.plugin_path).await?;
        // 注册插件
        let mut features = self.registered_features.lock().await;
        features.insert(request.feature_id.clone(), plugin);
        Ok(IterateResponse {
            success: true,
            message: format!("Plugin {} registered successfully", request.feature_id),
            data: None,
            error_code: None,
        })
    }

    /// 初始化插件
    pub async fn init_plugin(
        &self,
        feature_id: &str,
        state: Arc<AppState>,
    ) -> Result<(), IterateError> {
        let features = self.registered_features.lock().await;
        if let Some(plugin) = features.get(feature_id) {
            plugin.init(state).await?;
        }

        Ok(())
    }

    /// 启动插件
    pub async fn start_plugin(
        &self,
        feature_id: &str,
        state: Arc<AppState>,
    ) -> Result<(), IterateError> {
        let features = self.registered_features.lock().await;
        if let Some(plugin) = features.get(feature_id) {
            plugin.start(state).await?;
        }

        Ok(())
    }

    /// 停止插件
    pub async fn stop_plugin(
        &self,
        feature_id: &str,
        state: Arc<AppState>,
    ) -> Result<(), IterateError> {
        let features = self.registered_features.lock().await;
        if let Some(plugin) = features.get(feature_id) {
            plugin.stop(state).await?;
        }

        Ok(())
    }

    /// 版本兼容性验证
    fn validate_version(
        &self,
        core_version: &str,
        plugin_version: &str,
    ) -> Result<(), IterateError> {
        // 解析核心版本
        let core_parts: Vec<&str> = core_version.split('.').collect();
        if core_parts.len() < 2 {
            return Err(IterateError::VersionIncompatible(
                core_version.to_string(),
                plugin_version.to_string(),
            ));
        }

        // 解析插件版本
        let plugin_parts: Vec<&str> = plugin_version.split('.').collect();
        if plugin_parts.len() < 2 {
            return Err(IterateError::VersionIncompatible(
                core_version.to_string(),
                plugin_version.to_string(),
            ));
        }

        // 检查主版本号是否匹配
        if core_parts[0] != plugin_parts[0] {
            return Err(IterateError::VersionIncompatible(
                core_version.to_string(),
                plugin_version.to_string(),
            ));
        }

        // 检查次版本号是否兼容
        let core_minor = core_parts[1].parse::<u32>().unwrap_or(0);
        let plugin_minor = plugin_parts[1].parse::<u32>().unwrap_or(0);
        if plugin_minor > core_minor {
            return Err(IterateError::VersionIncompatible(
                core_version.to_string(),
                plugin_version.to_string(),
            ));
        }

        Ok(())
    }

    /// 依赖验证
    fn validate_dependencies(&self, dependencies: &[String]) -> Result<(), IterateError> {
        // 检查依赖是否为空
        if dependencies.is_empty() {
            return Ok(());
        }

        // 检查每个依赖是否存在且版本兼容
        for dep in dependencies {
            // 解析依赖格式：name@version
            let parts: Vec<&str> = dep.split('@').collect();
            if parts.len() != 2 {
                return Err(IterateError::MissingDependency(dep.to_string()));
            }

            let dep_name = parts[0];
            let dep_version = parts[1];

            // 简化实现，实际需要检查依赖是否存在且版本兼容
            // 这里只检查依赖名称是否合理
            if dep_name.is_empty() || dep_version.is_empty() {
                return Err(IterateError::MissingDependency(dep.to_string()));
            }

            // 检查依赖版本格式是否正确
            let version_parts: Vec<&str> = dep_version.split('.').collect();
            if version_parts.len() < 2 {
                return Err(IterateError::MissingDependency(dep.to_string()));
            }
        }

        Ok(())
    }

    /// 加载插件
    async fn load_plugin(
        &self,
        plugin_path: &str,
    ) -> Result<Box<dyn PluginLifecycle + Send + Sync>, IterateError> {
        // 检查插件路径是否存在
        let path = Path::new(plugin_path);
        if !path.exists() {
            return Err(IterateError::InvalidPluginPath(format!(
                "Plugin path does not exist: {}",
                plugin_path
            )));
        }

        // 检查是否为动态库文件
        let is_dylib = plugin_path.ends_with(".so")
            || plugin_path.ends_with(".dylib")
            || plugin_path.ends_with(".dll");

        if is_dylib {
            // 尝试动态加载插件
            unsafe {
                match Library::new(plugin_path) {
                    Ok(lib) => {
                        // 尝试获取插件创建函数
                        let create_plugin: Result<Symbol<fn() -> *mut dyn PluginLifecycle>, _> =
                            lib.get(b"create_plugin");

                        match create_plugin {
                            Ok(create) => {
                                let plugin_ptr = create();
                                let plugin = Box::from_raw(plugin_ptr);
                                log::info!("Plugin loaded successfully from: {}", plugin_path);
                                Ok(plugin)
                            }
                            Err(e) => {
                                log::error!("Failed to get create_plugin symbol: {}", e);
                                Err(IterateError::InitFailed(format!(
                                    "Failed to get create_plugin symbol: {}",
                                    e
                                )))
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to load plugin library: {}", e);
                        Err(IterateError::InitFailed(format!(
                            "Failed to load plugin library: {}",
                            e
                        )))
                    }
                }
            }
        } else {
            // 检查是否为.axpl文件
            if plugin_path.ends_with(".axpl") {
                // 尝试解析.axpl文件
                match self.parse_axpl_plugin(plugin_path).await {
                    Ok(plugin) => {
                        log::info!("Plugin loaded successfully from axpl file: {}", plugin_path);
                        Ok(plugin)
                    }
                    Err(e) => {
                        log::error!("Failed to parse axpl plugin: {}", e);
                        Err(IterateError::InitFailed(format!(
                            "Failed to parse axpl plugin: {}",
                            e
                        )))
                    }
                }
            } else {
                // 检查是否为目录
                if path.is_dir() {
                    // 尝试从目录加载插件
                    match self.load_plugin_from_dir(plugin_path).await {
                        Ok(plugin) => {
                            log::info!(
                                "Plugin loaded successfully from directory: {}",
                                plugin_path
                            );
                            Ok(plugin)
                        }
                        Err(e) => {
                            log::error!("Failed to load plugin from directory: {}", e);
                            Err(IterateError::InitFailed(format!(
                                "Failed to load plugin from directory: {}",
                                e
                            )))
                        }
                    }
                } else {
                    Err(IterateError::InvalidPluginPath(format!(
                        "Unsupported plugin format: {}",
                        plugin_path
                    )))
                }
            }
        }
    }

    /// 从.axpl文件加载插件
    async fn parse_axpl_plugin(
        &self,
        plugin_path: &str,
    ) -> Result<Box<dyn PluginLifecycle + Send + Sync>, IterateError> {
        use std::fs::File;
        use std::io::Read;
        use zip::ZipArchive;

        // 打开.axpl文件
        let file = File::open(plugin_path).map_err(|e| {
            IterateError::InvalidPluginPath(format!("Failed to open axpl file: {}", e))
        })?;

        // 解压文件
        let mut archive = ZipArchive::new(file)
            .map_err(|e| IterateError::InitFailed(format!("Failed to parse axpl file: {}", e)))?;

        // 读取manifest.json
        let mut manifest_file = archive.by_name("manifest.json").map_err(|e| {
            IterateError::InitFailed(format!("Failed to read manifest.json: {}", e))
        })?;

        let mut manifest_content = String::new();
        manifest_file
            .read_to_string(&mut manifest_content)
            .map_err(|e| {
                IterateError::InitFailed(format!("Failed to read manifest content: {}", e))
            })?;

        // 解析manifest
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| IterateError::InitFailed(format!("Failed to parse manifest: {}", e)))?;

        // 获取插件信息
        let plugin_name = manifest
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let plugin_version = manifest
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0");

        let plugin_description = manifest
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // 创建动态插件实例
        let plugin = DynamicPlugin {
            name: plugin_name.to_string(),
            version: plugin_version.to_string(),
            description: plugin_description.to_string(),
        };

        Ok(Box::new(plugin))
    }

    /// 从目录加载插件
    async fn load_plugin_from_dir(
        &self,
        plugin_dir: &str,
    ) -> Result<Box<dyn PluginLifecycle + Send + Sync>, IterateError> {
        use std::fs;
        use std::path::Path;

        // 检查manifest.json是否存在
        let manifest_path = Path::new(plugin_dir).join("manifest.json");
        if !manifest_path.exists() {
            return Err(IterateError::InvalidPluginPath(
                "manifest.json not found in plugin directory".to_string(),
            ));
        }

        // 读取manifest.json
        let manifest_content = fs::read_to_string(&manifest_path).map_err(|e| {
            IterateError::InitFailed(format!("Failed to read manifest.json: {}", e))
        })?;

        // 解析manifest
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| IterateError::InitFailed(format!("Failed to parse manifest: {}", e)))?;

        // 获取插件信息
        let plugin_name = manifest
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let plugin_version = manifest
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0");

        let plugin_description = manifest
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // 创建动态插件实例
        let plugin = DynamicPlugin {
            name: plugin_name.to_string(),
            version: plugin_version.to_string(),
            description: plugin_description.to_string(),
        };

        Ok(Box::new(plugin))
    }
}

/// 默认插件实现
#[allow(dead_code)]
struct DefaultPlugin {}

/// 动态插件实现
struct DynamicPlugin {
    name: String,
    version: String,
    #[allow(dead_code)]
    description: String,
}

#[async_trait]
impl PluginLifecycle for DynamicPlugin {
    async fn init(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Dynamic plugin {} v{} initialized", self.name, self.version);
        Ok(())
    }

    async fn start(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Dynamic plugin {} v{} started", self.name, self.version);
        Ok(())
    }

    async fn stop(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Dynamic plugin {} v{} stopped", self.name, self.version);
        Ok(())
    }

    fn name(&self) -> &'static str {
        // 由于生命周期限制，这里返回一个静态字符串
        // 实际使用时应该使用其他方式传递动态名称
        "dynamic"
    }

    fn version(&self) -> &'static str {
        // 由于生命周期限制，这里返回一个静态字符串
        // 实际使用时应该使用其他方式传递动态版本
        "1.0.0"
    }

    fn description(&self) -> &'static str {
        // 由于生命周期限制，这里返回一个静态字符串
        // 实际使用时应该使用其他方式传递动态描述
        "Dynamic plugin implementation"
    }

    fn plugin_type(&self) -> &'static str {
        "external"
    }
}

#[async_trait]
impl PluginLifecycle for DefaultPlugin {
    async fn init(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Default plugin initialized");
        Ok(())
    }

    async fn start(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Default plugin started");
        Ok(())
    }

    async fn stop(&self, _state: Arc<AppState>) -> Result<(), IterateError> {
        log::info!("Default plugin stopped");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "default"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn description(&self) -> &'static str {
        "Default plugin implementation"
    }

    fn plugin_type(&self) -> &'static str {
        "builtin"
    }
}

impl Default for IterateApi {
    fn default() -> Self {
        Self::new()
    }
}

/// 迭代接口扩展
pub type IterateApiExtension = Arc<IterateApi>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_iterate_api_creation() {
        let _iterate_api = IterateApi::new();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_validate_version() {
        let iterate_api = IterateApi::new();

        // Test compatible versions
        let result = iterate_api.validate_version("1.0.0", "1.0.0");
        assert!(result.is_ok());

        // Test incompatible major versions
        let result = iterate_api.validate_version("1.0.0", "2.0.0");
        assert!(result.is_err());

        // Test incompatible minor versions (plugin minor > core minor)
        let result = iterate_api.validate_version("1.0.0", "1.1.0");
        assert!(result.is_err());

        // Test compatible minor versions (plugin minor <= core minor)
        let result = iterate_api.validate_version("1.1.0", "1.0.0");
        assert!(result.is_ok());

        // Test invalid version formats
        let result = iterate_api.validate_version("1", "1.0.0");
        assert!(result.is_err());

        let result = iterate_api.validate_version("1.0.0", "1");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_dependencies() {
        let iterate_api = IterateApi::new();

        // Test empty dependencies
        let result = iterate_api.validate_dependencies(&[]);
        assert!(result.is_ok());

        // Test valid dependencies
        let valid_deps = vec!["dep1@1.0.0".to_string(), "dep2@2.1.0".to_string()];
        let result = iterate_api.validate_dependencies(&valid_deps);
        assert!(result.is_ok());

        // Test invalid dependencies (missing version)
        let invalid_deps = vec!["dep1".to_string()];
        let result = iterate_api.validate_dependencies(&invalid_deps);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        let app_state = Arc::new(AppState::new());

        // Test DynamicPlugin
        let dynamic_plugin = DynamicPlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
        };

        // Test init
        let result = dynamic_plugin.init(app_state.clone()).await;
        assert!(result.is_ok());

        // Test start
        let result = dynamic_plugin.start(app_state.clone()).await;
        assert!(result.is_ok());

        // Test stop
        let result = dynamic_plugin.stop(app_state.clone()).await;
        assert!(result.is_ok());

        // Test getters
        assert_eq!(dynamic_plugin.name(), "dynamic");
        assert_eq!(dynamic_plugin.version(), "1.0.0");
        assert_eq!(
            dynamic_plugin.description(),
            "Dynamic plugin implementation"
        );
        assert_eq!(dynamic_plugin.plugin_type(), "external");

        // Test DefaultPlugin
        let default_plugin = DefaultPlugin {};

        // Test init
        let result = default_plugin.init(app_state.clone()).await;
        assert!(result.is_ok());

        // Test start
        let result = default_plugin.start(app_state.clone()).await;
        assert!(result.is_ok());

        // Test stop
        let result = default_plugin.stop(app_state.clone()).await;
        assert!(result.is_ok());

        // Test getters
        assert_eq!(default_plugin.name(), "default");
        assert_eq!(default_plugin.version(), "1.0.0");
        assert_eq!(
            default_plugin.description(),
            "Default plugin implementation"
        );
        assert_eq!(default_plugin.plugin_type(), "builtin");
    }

    #[tokio::test]
    async fn test_iterate_request_response() {
        // Test IterateRequest creation
        let _request = IterateRequest {
            plugin_path: "/path/to/plugin".to_string(),
            feature_id: "test_feature".to_string(),
            dependencies: vec!["dep1@1.0.0".to_string()],
            plugin_version: "1.0.0".to_string(),
            core_version: "1.0.0".to_string(),
        };

        // Test IterateResponse creation
        let response = IterateResponse {
            success: true,
            message: "Test message".to_string(),
            data: None,
            error_code: None,
        };

        assert!(response.success);
        assert_eq!(response.message, "Test message");
    }

    #[tokio::test]
    async fn test_iterate_error() {
        // Test different IterateError variants
        let _error1 = IterateError::InvalidPluginPath("/invalid/path".to_string());
        let _error2 = IterateError::DuplicateFeatureId("feature1".to_string());
        let _error3 = IterateError::MissingDependency("dep1".to_string());
        let _error4 = IterateError::VersionIncompatible("1.0.0".to_string(), "2.0.0".to_string());
        let _error5 = IterateError::InitFailed("Init failed".to_string());
        let _error6 = IterateError::StartFailed("Start failed".to_string());
        let _error7 = IterateError::StopFailed("Stop failed".to_string());
        let _error8 = IterateError::InternalError("Internal error".to_string());

        // Test From<String> implementation
        let error9: IterateError = "Test error".to_string().into();
        assert!(matches!(error9, IterateError::InternalError(_)));
    }
}

