//! 插件加载器模块
//! 负责加载不同语言和平台的GUF插件

use super::{GufPlugin, GufPluginConfig, PluginStatus};
use log::info;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono;

/// 插件运行时
#[derive(Debug, Clone)]
pub struct PluginRuntime {
    /// 运行时类型
    pub runtime_type: String,
    /// 运行时状态
    pub status: String,
    /// 运行时进程ID（如果有）
    pub process_id: Option<u32>,
    /// 运行时配置
    pub config: serde_json::Value,
}

/// 插件加载器
#[derive(Debug, Clone)]
pub struct GufPluginLoader {
    /// 插件管理器引用
    plugin_manager: Arc<super::plugin_manager::GufPluginManager>,
    /// 插件运行时管理
    runtimes: Arc<Mutex<HashMap<String, PluginRuntime>>>,
}

impl GufPluginLoader {
    /// 创建新的插件加载器
    pub fn new() -> Self {
        Self {
            plugin_manager: Arc::new(super::plugin_manager::GufPluginManager::new()),
            runtimes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 初始化插件加载器
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件加载器");
        // 初始化插件管理器
        self.plugin_manager.initialize().await
    }

    /// 加载插件
    pub async fn load_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载插件: {}", plugin_path);

        // 检查插件文件是否存在
        if !std::path::Path::new(plugin_path).exists() {
            return Err(format!("插件文件不存在: {}", plugin_path));
        }

        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;

        // 根据语言类型加载插件
        let plugin = match plugin_config.language.as_str() {
            "rust" => self.load_rust_plugin(plugin_path).await?,
            "python" => self.load_python_plugin(plugin_path).await?,
            "javascript" => self.load_javascript_plugin(plugin_path).await?,
            "java" => self.load_java_plugin(plugin_path).await?,
            "csharp" => self.load_csharp_plugin(plugin_path).await?,
            _ => {
                return Err(format!("不支持的插件语言: {}", plugin_config.language));
            }
        };

        info!("插件加载完成: {}", plugin.config.name);
        Ok(plugin)
    }

    /// 解析插件配置
    async fn parse_plugin_config(&self, plugin_path: &str) -> Result<GufPluginConfig, String> {
        info!("解析插件配置: {}", plugin_path);

        // 这里应该根据插件类型和格式解析配置
        // 为了演示，返回一个示例配置
        Ok(GufPluginConfig {
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
            config: serde_json::json!({
                "key": "value",
                "enabled": true
            }),
        })
    }

    /// 加载Rust插件
    pub async fn load_rust_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载Rust插件: {}", plugin_path);
        
        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;
        
        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: PluginStatus::Initializing,
            path: plugin_path.to_string(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };
        
        // 添加插件到管理器
        self.plugin_manager.add_plugin(plugin.clone()).await?;
        
        // 注册Rust运行时
        let runtime = PluginRuntime {
            runtime_type: "rust".to_string(),
            status: "running".to_string(),
            process_id: None, // Rust插件作为库加载，没有独立进程
            config: serde_json::json!({}),
        };
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        runtimes.insert(plugin.config.name.clone(), runtime);
        
        // 模拟插件加载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        // 更新插件状态
        let mut updated_plugin = plugin;
        updated_plugin.status = PluginStatus::Ready;
        
        Ok(updated_plugin)
    }

    /// 加载Python插件
    pub async fn load_python_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载Python插件: {}", plugin_path);
        
        // 检查Python是否安装
        if !self.check_python_installed().await {
            return Err("Python未安装，请先安装Python 3.7+".to_string());
        }
        
        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;
        
        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: PluginStatus::Initializing,
            path: plugin_path.to_string(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };
        
        // 添加插件到管理器
        self.plugin_manager.add_plugin(plugin.clone()).await?;
        
        // 启动Python运行时
        let runtime = self.start_python_runtime(&plugin.config.name).await?;
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        runtimes.insert(plugin.config.name.clone(), runtime);
        
        // 模拟插件加载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // 更新插件状态
        let mut updated_plugin = plugin;
        updated_plugin.status = PluginStatus::Ready;
        
        Ok(updated_plugin)
    }

    /// 加载JavaScript插件
    pub async fn load_javascript_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载JavaScript插件: {}", plugin_path);
        
        // 检查Node.js是否安装
        if !self.check_nodejs_installed().await {
            return Err("Node.js未安装，请先安装Node.js 14+".to_string());
        }
        
        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;
        
        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: PluginStatus::Initializing,
            path: plugin_path.to_string(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };
        
        // 添加插件到管理器
        self.plugin_manager.add_plugin(plugin.clone()).await?;
        
        // 启动Node.js运行时
        let runtime = self.start_nodejs_runtime(&plugin.config.name).await?;
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        runtimes.insert(plugin.config.name.clone(), runtime);
        
        // 模拟插件加载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // 更新插件状态
        let mut updated_plugin = plugin;
        updated_plugin.status = PluginStatus::Ready;
        
        Ok(updated_plugin)
    }

    /// 加载Java插件
    pub async fn load_java_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载Java插件: {}", plugin_path);
        
        // 检查Java是否安装
        if !self.check_java_installed().await {
            return Err("Java未安装，请先安装Java 8+".to_string());
        }
        
        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;
        
        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: PluginStatus::Initializing,
            path: plugin_path.to_string(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };
        
        // 添加插件到管理器
        self.plugin_manager.add_plugin(plugin.clone()).await?;
        
        // 启动Java运行时
        let runtime = self.start_java_runtime(&plugin.config.name).await?;
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        runtimes.insert(plugin.config.name.clone(), runtime);
        
        // 模拟插件加载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // 更新插件状态
        let mut updated_plugin = plugin;
        updated_plugin.status = PluginStatus::Ready;
        
        Ok(updated_plugin)
    }

    /// 加载C#插件
    pub async fn load_csharp_plugin(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("加载C#插件: {}", plugin_path);
        
        // 检查.NET是否安装
        if !self.check_dotnet_installed().await {
            return Err(".NET未安装，请先安装.NET 6.0+".to_string());
        }
        
        // 解析插件配置
        let plugin_config = self.parse_plugin_config(plugin_path).await?;
        
        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: PluginStatus::Initializing,
            path: plugin_path.to_string(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };
        
        // 添加插件到管理器
        self.plugin_manager.add_plugin(plugin.clone()).await?;
        
        // 启动.NET运行时
        let runtime = self.start_dotnet_runtime(&plugin.config.name).await?;
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        runtimes.insert(plugin.config.name.clone(), runtime);
        
        // 模拟插件加载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // 更新插件状态
        let mut updated_plugin = plugin;
        updated_plugin.status = PluginStatus::Ready;
        
        Ok(updated_plugin)
    }

    /// 根据文件扩展名加载插件
    pub async fn load_plugin_by_extension(&self, plugin_path: &str) -> Result<GufPlugin, String> {
        info!("根据文件扩展名加载插件: {}", plugin_path);

        let extension = std::path::Path::new(plugin_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| format!("无法获取文件扩展名: {}", plugin_path))?;

        match extension.to_lowercase().as_str() {
            "rs" => self.load_rust_plugin(plugin_path).await,
            "py" => self.load_python_plugin(plugin_path).await,
            "js" | "jsx" | "ts" | "tsx" => self.load_javascript_plugin(plugin_path).await,
            "java" => self.load_java_plugin(plugin_path).await,
            "cs" => self.load_csharp_plugin(plugin_path).await,
            _ => Err(format!("不支持的插件文件类型: {}", extension)),
        }
    }

    /// 卸载插件
    pub async fn unload_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("卸载插件: {}", plugin_name);
        self.plugin_manager.unload_plugin(plugin_name).await
    }

    /// 列出已加载的插件
    pub async fn list_loaded_plugins(&self) -> Result<Vec<GufPlugin>, String> {
        self.plugin_manager.list_plugins().await
    }

    /// 检查插件是否已加载
    pub async fn is_plugin_loaded(&self, plugin_name: &str) -> bool {
        self.plugin_manager.plugin_exists(plugin_name).await
    }

    /// 获取已加载插件的数量
    pub async fn get_loaded_plugin_count(&self) -> usize {
        self.plugin_manager.get_plugin_count().await
    }

    /// 检查Python是否安装
    async fn check_python_installed(&self) -> bool {
        let output = std::process::Command::new("python3")
            .arg("--version")
            .output()
            .ok();
        
        output.map(|o| o.status.success()).unwrap_or(false)
    }

    /// 检查Node.js是否安装
    async fn check_nodejs_installed(&self) -> bool {
        let output = std::process::Command::new("node")
            .arg("--version")
            .output()
            .ok();
        
        output.map(|o| o.status.success()).unwrap_or(false)
    }

    /// 检查Java是否安装
    async fn check_java_installed(&self) -> bool {
        let output = std::process::Command::new("java")
            .arg("-version")
            .output()
            .ok();
        
        output.map(|o| o.status.success()).unwrap_or(false)
    }

    /// 检查.NET是否安装
    async fn check_dotnet_installed(&self) -> bool {
        let output = std::process::Command::new("dotnet")
            .arg("--version")
            .output()
            .ok();
        
        output.map(|o| o.status.success()).unwrap_or(false)
    }

    /// 启动Python运行时
    async fn start_python_runtime(&self, plugin_name: &str) -> Result<PluginRuntime, String> {
        info!("启动Python运行时: {}", plugin_name);
        
        // 模拟启动Python运行时
        Ok(PluginRuntime {
            runtime_type: "python".to_string(),
            status: "running".to_string(),
            process_id: Some(1234), // 模拟进程ID
            config: serde_json::json!({
                "python_version": "3.10",
                "virtual_env": "venv"
            }),
        })
    }

    /// 启动Node.js运行时
    async fn start_nodejs_runtime(&self, plugin_name: &str) -> Result<PluginRuntime, String> {
        info!("启动Node.js运行时: {}", plugin_name);
        
        // 模拟启动Node.js运行时
        Ok(PluginRuntime {
            runtime_type: "nodejs".to_string(),
            status: "running".to_string(),
            process_id: Some(5678), // 模拟进程ID
            config: serde_json::json!({
                "node_version": "18.0",
                "npm_version": "9.0"
            }),
        })
    }

    /// 启动Java运行时
    async fn start_java_runtime(&self, plugin_name: &str) -> Result<PluginRuntime, String> {
        info!("启动Java运行时: {}", plugin_name);
        
        // 模拟启动Java运行时
        Ok(PluginRuntime {
            runtime_type: "java".to_string(),
            status: "running".to_string(),
            process_id: Some(91011), // 模拟进程ID
            config: serde_json::json!({
                "java_version": "11",
                "jvm_args": "-Xmx512m"
            }),
        })
    }

    /// 启动.NET运行时
    async fn start_dotnet_runtime(&self, plugin_name: &str) -> Result<PluginRuntime, String> {
        info!("启动.NET运行时: {}", plugin_name);
        
        // 模拟启动.NET运行时
        Ok(PluginRuntime {
            runtime_type: "dotnet".to_string(),
            status: "running".to_string(),
            process_id: Some(121314), // 模拟进程ID
            config: serde_json::json!({
                "dotnet_version": "6.0",
                "runtime": "core"
            }),
        })
    }

    /// 获取插件运行时
    pub async fn get_plugin_runtime(&self, plugin_name: &str) -> Result<Option<PluginRuntime>, String> {
        let runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        Ok(runtimes.get(plugin_name).cloned())
    }

    /// 停止插件运行时
    pub async fn stop_plugin_runtime(&self, plugin_name: &str) -> Result<(), String> {
        info!("停止插件运行时: {}", plugin_name);
        
        let mut runtimes = self.runtimes.lock().map_err(|e| format!("无法获取运行时锁: {}", e))?;
        
        if let Some(runtime) = runtimes.get_mut(plugin_name) {
            // 模拟停止运行时
            runtime.status = "stopped".to_string();
            runtime.process_id = None;
            info!("插件运行时已停止: {}", plugin_name);
        }
        
        Ok(())
    }
}
