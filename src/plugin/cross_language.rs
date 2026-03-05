// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 跨语言插件支持模块
//! 负责处理不同编程语言编写的插件

use super::PluginStatus;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

/// 插件语言类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PluginLanguage {
    Rust,
    JavaScript,
    Python,
    Go,
    Java,
    CSharp,
    TypeScript,
    Unknown,
}

/// 跨语言插件接口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLanguageInterface {
    /// 接口名称
    pub name: String,
    /// 接口版本
    pub version: String,
    /// 接口方法
    pub methods: Vec<InterfaceMethod>,
    /// 事件定义
    pub events: Vec<InterfaceEvent>,
}

/// 接口方法定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMethod {
    /// 方法名称
    pub name: String,
    /// 参数定义
    pub parameters: Vec<MethodParameter>,
    /// 返回类型
    pub return_type: String,
    /// 方法描述
    pub description: Option<String>,
}

/// 方法参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodParameter {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 是否必填
    pub required: bool,
    /// 参数描述
    pub description: Option<String>,
}

/// 跨语言插件运行时
#[derive(Clone)]
pub struct CrossLanguageRuntime {
    /// 语言运行时映射
    runtimes: Arc<RwLock<HashMap<PluginLanguage, Arc<dyn LanguageRuntime>>>>,
}

impl std::fmt::Debug for CrossLanguageRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrossLanguageRuntime")
            .field(
                "runtimes",
                &"HashMap<PluginLanguage, Arc<dyn LanguageRuntime>>",
            )
            .finish()
    }
}

/// 语言运行时接口
#[async_trait::async_trait]
pub trait LanguageRuntime: Send + Sync {
    /// 初始化运行时
    async fn initialize(&self) -> Result<()>;

    /// 启动插件
    async fn start_plugin(&self, plugin_path: &Path) -> Result<()>;

    /// 停止插件
    async fn stop_plugin(&self, plugin_name: &str) -> Result<()>;

    /// 调用插件方法
    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>;

    /// 监听插件事件
    async fn subscribe_event(
        &self,
        plugin_name: &str,
        event_name: &str,
        callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()>;

    /// 获取插件状态
    async fn get_plugin_status(&self, plugin_name: &str) -> Result<PluginStatus>;
}

/// JavaScript 运行时
#[derive(Debug, Clone)]
pub struct JavaScriptRuntime {
    /// 运行时配置
    _config: RuntimeConfig,
}

/// Python 运行时
#[derive(Debug, Clone)]
pub struct PythonRuntime {
    /// 运行时配置
    _config: RuntimeConfig,
}

/// Go 运行时
#[derive(Debug, Clone)]
pub struct GoRuntime {
    /// 运行时配置
    _config: RuntimeConfig,
}

/// Java 运行时
#[derive(Debug, Clone)]
pub struct JavaRuntime {
    /// 运行时配置
    _config: RuntimeConfig,
}

/// C# 运行时
#[derive(Debug, Clone)]
pub struct CSharpRuntime {
    /// 运行时配置
    _config: RuntimeConfig,
}

/// 运行时配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// 运行时路径
    pub runtime_path: Option<String>,
    /// 环境变量
    pub environment: HashMap<String, String>,
    /// 启动参数
    pub arguments: Vec<String>,
    /// 超时设置（毫秒）
    pub timeout: u64,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            runtime_path: None,
            environment: HashMap::new(),
            arguments: Vec::new(),
            timeout: 30000,
        }
    }
}

impl CrossLanguageRuntime {
    /// 创建新的跨语言运行时管理器
    pub fn new() -> Result<Self> {
        // 初始化支持的语言运行时
        let js_runtime: Arc<dyn LanguageRuntime> = Arc::new(JavaScriptRuntime::new()?);
        let py_runtime: Arc<dyn LanguageRuntime> = Arc::new(PythonRuntime::new()?);
        let go_runtime: Arc<dyn LanguageRuntime> = Arc::new(GoRuntime::new()?);
        let java_runtime: Arc<dyn LanguageRuntime> = Arc::new(JavaRuntime::new()?);
        let csharp_runtime: Arc<dyn LanguageRuntime> = Arc::new(CSharpRuntime::new()?);

        let mut runtimes_map = HashMap::new();
        runtimes_map.insert(PluginLanguage::JavaScript, js_runtime);
        runtimes_map.insert(PluginLanguage::Python, py_runtime);
        runtimes_map.insert(PluginLanguage::Go, go_runtime);
        runtimes_map.insert(PluginLanguage::Java, java_runtime);
        runtimes_map.insert(PluginLanguage::CSharp, csharp_runtime);

        Ok(Self {
            runtimes: Arc::new(RwLock::new(runtimes_map)),
        })
    }

    /// 检测插件语言
    pub fn detect_language(&self, plugin_path: &Path) -> PluginLanguage {
        // 检查package.json（JavaScript/TypeScript）
        if plugin_path.join("package.json").exists() {
            return PluginLanguage::JavaScript;
        }

        // 检查setup.py或pyproject.toml（Python）
        if plugin_path.join("setup.py").exists() || plugin_path.join("pyproject.toml").exists() {
            return PluginLanguage::Python;
        }

        // 检查go.mod（Go）
        if plugin_path.join("go.mod").exists() {
            return PluginLanguage::Go;
        }

        // 检查Cargo.toml（Rust）
        if plugin_path.join("Cargo.toml").exists() {
            return PluginLanguage::Rust;
        }

        // 检查pom.xml（Java）
        if plugin_path.join("pom.xml").exists() {
            return PluginLanguage::Java;
        }

        // 检查.csproj（C#）
        if plugin_path.read_dir().map_or(false, |dir| {
            dir.filter_map(|entry| entry.ok())
                .any(|entry| entry.path().extension().map_or(false, |ext| ext == "csproj"))
        }) {
            return PluginLanguage::CSharp;
        }

        PluginLanguage::Unknown
    }

    /// 初始化插件运行时
    pub async fn initialize_runtime(&self, language: PluginLanguage) -> Result<()> {
        let runtimes = self.runtimes.read().await;
        if let Some(runtime) = runtimes.get(&language) {
            runtime.initialize().await
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }

    /// 启动跨语言插件
    pub async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        let language = self.detect_language(plugin_path);
        let runtimes = self.runtimes.read().await;

        if let Some(runtime) = runtimes.get(&language) {
            runtime.start_plugin(plugin_path).await
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }

    /// 停止跨语言插件
    pub async fn stop_plugin(&self, plugin_name: &str, language: PluginLanguage) -> Result<()> {
        let runtimes = self.runtimes.read().await;

        if let Some(runtime) = runtimes.get(&language) {
            runtime.stop_plugin(plugin_name).await
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }

    /// 调用插件方法
    pub async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
        language: PluginLanguage,
    ) -> Result<serde_json::Value> {
        let runtimes = self.runtimes.read().await;

        if let Some(runtime) = runtimes.get(&language) {
            info!("调用插件 {} 的方法 {}，语言: {:?}", plugin_name, method_name, language);
            let result = runtime.call_method(plugin_name, method_name, params).await;
            info!("插件方法调用结果: {:?}", result);
            result
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }

    /// 监听插件事件
    pub async fn subscribe_event(
        &self,
        plugin_name: &str,
        event_name: &str,
        callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
        language: PluginLanguage,
    ) -> Result<()> {
        let runtimes = self.runtimes.read().await;

        if let Some(runtime) = runtimes.get(&language) {
            info!("订阅插件 {} 的事件 {}，语言: {:?}", plugin_name, event_name, language);
            let result = runtime
                .subscribe_event(plugin_name, event_name, callback)
                .await;
            info!("事件订阅结果: {:?}", result);
            result
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }



    /// 获取插件状态
    pub async fn get_plugin_status(
        &self,
        plugin_name: &str,
        language: PluginLanguage,
    ) -> Result<PluginStatus> {
        let runtimes = self.runtimes.read().await;

        if let Some(runtime) = runtimes.get(&language) {
            runtime.get_plugin_status(plugin_name).await
        } else {
            Err(anyhow::anyhow!("不支持的插件语言: {:?}", language))
        }
    }
}

impl JavaScriptRuntime {
    /// 创建新的JavaScript运行时
    pub fn new() -> Result<Self> {
        Ok(Self {
            _config: RuntimeConfig::default(),
        })
    }
}

#[async_trait::async_trait]
impl LanguageRuntime for JavaScriptRuntime {
    async fn initialize(&self) -> Result<()> {
        // 检查Node.js是否安装
        let output = std::process::Command::new("node")
            .arg("--version")
            .output()
            .context("检查Node.js版本失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Node.js未安装或不可用"));
        }

        Ok(())
    }

    async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        // 检查package.json
        let package_json = plugin_path.join("package.json");
        if !package_json.exists() {
            return Err(anyhow::anyhow!("缺少package.json文件"));
        }

        // 验证插件路径，防止路径遍历攻击
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("无效的插件路径"))?;
        if plugin_path_str.contains("..") {
            return Err(anyhow::anyhow!("无效的插件路径: 包含路径遍历"));
        }

        // 安装依赖
        let output = std::process::Command::new("npm")
            .arg("install")
            .current_dir(plugin_path)
            .output()
            .context("安装JavaScript依赖失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "安装JavaScript依赖失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn stop_plugin(&self, _plugin_name: &str) -> Result<()> {
        // 停止JavaScript插件的逻辑
        Ok(())
    }

    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // 调用JavaScript插件方法的逻辑
        Ok(serde_json::json!({
            "result": "success",
            "method": method_name,
            "plugin": plugin_name,
            "params": params
        }))
    }

    async fn subscribe_event(
        &self,
        _plugin_name: &str,
        _event_name: &str,
        _callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()> {
        // 订阅JavaScript插件事件的逻辑
        Ok(())
    }

    async fn get_plugin_status(&self, _plugin_name: &str) -> Result<PluginStatus> {
        // 获取JavaScript插件状态的逻辑
        Ok(PluginStatus::Enabled)
    }
}

impl PythonRuntime {
    /// 创建新的Python运行时
    pub fn new() -> Result<Self> {
        Ok(Self {
            _config: RuntimeConfig::default(),
        })
    }
}

#[async_trait::async_trait]
impl LanguageRuntime for PythonRuntime {
    async fn initialize(&self) -> Result<()> {
        // 检查Python是否安装
        let output = std::process::Command::new("python")
            .arg("--version")
            .output()
            .context("检查Python版本失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Python未安装或不可用"));
        }

        Ok(())
    }

    async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        // 检查requirements.txt或pyproject.toml
        let requirements_txt = plugin_path.join("requirements.txt");
        let pyproject_toml = plugin_path.join("pyproject.toml");

        // 验证插件路径，防止路径遍历攻击
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("无效的插件路径"))?;
        if plugin_path_str.contains("..") {
            return Err(anyhow::anyhow!("无效的插件路径: 包含路径遍历"));
        }

        // 安装依赖
        if requirements_txt.exists() {
            let output = std::process::Command::new("pip")
                .arg("install")
                .arg("-r")
                .arg("requirements.txt")
                .current_dir(plugin_path)
                .output()
                .context("安装Python依赖失败")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "安装Python依赖失败: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        } else if pyproject_toml.exists() {
            let output = std::process::Command::new("pip")
                .arg("install")
                .arg(".")
                .current_dir(plugin_path)
                .output()
                .context("安装Python依赖失败")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "安装Python依赖失败: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        Ok(())
    }

    async fn stop_plugin(&self, _plugin_name: &str) -> Result<()> {
        // 停止Python插件的逻辑
        Ok(())
    }

    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // 调用Python插件方法的逻辑
        Ok(serde_json::json!({
            "result": "success",
            "method": method_name,
            "plugin": plugin_name,
            "params": params
        }))
    }

    async fn subscribe_event(
        &self,
        _plugin_name: &str,
        _event_name: &str,
        _callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()> {
        // 订阅Python插件事件的逻辑
        Ok(())
    }

    async fn get_plugin_status(&self, _plugin_name: &str) -> Result<PluginStatus> {
        // 获取Python插件状态的逻辑
        Ok(PluginStatus::Enabled)
    }
}

impl GoRuntime {
    /// 创建新的Go运行时
    pub fn new() -> Result<Self> {
        Ok(Self {
            _config: RuntimeConfig::default(),
        })
    }
}

#[async_trait::async_trait]
impl LanguageRuntime for GoRuntime {
    async fn initialize(&self) -> Result<()> {
        // 检查Go是否安装
        let output = std::process::Command::new("go")
            .arg("version")
            .output()
            .context("检查Go版本失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Go未安装或不可用"));
        }

        Ok(())
    }

    async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        // 检查go.mod
        let go_mod = plugin_path.join("go.mod");
        if !go_mod.exists() {
            return Err(anyhow::anyhow!("缺少go.mod文件"));
        }

        // 验证插件路径，防止路径遍历攻击
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("无效的插件路径"))?;
        if plugin_path_str.contains("..") {
            return Err(anyhow::anyhow!("无效的插件路径: 包含路径遍历"));
        }

        // 构建Go插件
        let output = std::process::Command::new("go")
            .arg("build")
            .arg(".")
            .current_dir(plugin_path)
            .output()
            .context("构建Go插件失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "构建Go插件失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn stop_plugin(&self, _plugin_name: &str) -> Result<()> {
        // 停止Go插件的逻辑
        Ok(())
    }

    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // 调用Go插件方法的逻辑
        Ok(serde_json::json!({
            "result": "success",
            "method": method_name,
            "plugin": plugin_name,
            "params": params
        }))
    }

    async fn subscribe_event(
        &self,
        _plugin_name: &str,
        _event_name: &str,
        _callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()> {
        // 订阅Go插件事件的逻辑
        Ok(())
    }

    async fn get_plugin_status(&self, _plugin_name: &str) -> Result<PluginStatus> {
        // 获取Go插件状态的逻辑
        Ok(PluginStatus::Enabled)
    }
}

impl JavaRuntime {
    /// 创建新的Java运行时
    pub fn new() -> Result<Self> {
        Ok(Self {
            _config: RuntimeConfig::default(),
        })
    }
}

#[async_trait::async_trait]
impl LanguageRuntime for JavaRuntime {
    async fn initialize(&self) -> Result<()> {
        // 检查Java是否安装
        let output = std::process::Command::new("java")
            .arg("-version")
            .output()
            .context("检查Java版本失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Java未安装或不可用"));
        }

        Ok(())
    }

    async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        // 检查pom.xml
        let pom_xml = plugin_path.join("pom.xml");
        if !pom_xml.exists() {
            return Err(anyhow::anyhow!("缺少pom.xml文件"));
        }

        // 验证插件路径，防止路径遍历攻击
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("无效的插件路径"))?;
        if plugin_path_str.contains("..") {
            return Err(anyhow::anyhow!("无效的插件路径: 包含路径遍历"));
        }

        // 构建Java插件
        let output = std::process::Command::new("mvn")
            .arg("clean")
            .arg("package")
            .current_dir(plugin_path)
            .output()
            .context("构建Java插件失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "构建Java插件失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn stop_plugin(&self, _plugin_name: &str) -> Result<()> {
        // 停止Java插件的逻辑
        Ok(())
    }

    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // 调用Java插件方法的逻辑
        Ok(serde_json::json!({
            "result": "success",
            "method": method_name,
            "plugin": plugin_name,
            "params": params
        }))
    }

    async fn subscribe_event(
        &self,
        _plugin_name: &str,
        _event_name: &str,
        _callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()> {
        // 订阅Java插件事件的逻辑
        Ok(())
    }

    async fn get_plugin_status(&self, _plugin_name: &str) -> Result<PluginStatus> {
        // 获取Java插件状态的逻辑
        Ok(PluginStatus::Enabled)
    }
}

impl CSharpRuntime {
    /// 创建新的C#运行时
    pub fn new() -> Result<Self> {
        Ok(Self {
            _config: RuntimeConfig::default(),
        })
    }
}

#[async_trait::async_trait]
impl LanguageRuntime for CSharpRuntime {
    async fn initialize(&self) -> Result<()> {
        // 检查dotnet是否安装
        let output = std::process::Command::new("dotnet")
            .arg("--version")
            .output()
            .context("检查dotnet版本失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("dotnet未安装或不可用"));
        }

        Ok(())
    }

    async fn start_plugin(&self, plugin_path: &Path) -> Result<()> {
        // 检查.csproj文件
        let csproj_files = plugin_path.read_dir()
            .context("读取插件目录失败")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "csproj"))
            .collect::<Vec<_>>();

        if csproj_files.is_empty() {
            return Err(anyhow::anyhow!("缺少.csproj文件"));
        }

        // 验证插件路径，防止路径遍历攻击
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("无效的插件路径"))?;
        if plugin_path_str.contains("..") {
            return Err(anyhow::anyhow!("无效的插件路径: 包含路径遍历"));
        }

        // 构建C#插件
        let output = std::process::Command::new("dotnet")
            .arg("build")
            .current_dir(plugin_path)
            .output()
            .context("构建C#插件失败")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "构建C#插件失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn stop_plugin(&self, _plugin_name: &str) -> Result<()> {
        // 停止C#插件的逻辑
        Ok(())
    }

    async fn call_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // 调用C#插件方法的逻辑
        Ok(serde_json::json!({
            "result": "success",
            "method": method_name,
            "plugin": plugin_name,
            "params": params
        }))
    }

    async fn subscribe_event(
        &self,
        _plugin_name: &str,
        _event_name: &str,
        _callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
    ) -> Result<()> {
        // 订阅C#插件事件的逻辑
        Ok(())
    }

    async fn get_plugin_status(&self, _plugin_name: &str) -> Result<PluginStatus> {
        // 获取C#插件状态的逻辑
        Ok(PluginStatus::Enabled)
    }
}

/// 接口事件定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceEvent {
    /// 事件名称
    pub name: String,
    /// 事件参数
    pub parameters: Vec<EventParameter>,
    /// 事件描述
    pub description: Option<String>,
}

/// 事件参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParameter {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 参数描述
    pub description: Option<String>,
}

