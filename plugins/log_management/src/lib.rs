//! YMAxum 日志管理插件
//! 提供基于 GUF 的日志管理功能

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use ymaxum::plugin::{PluginInfo, PluginStatus};
use ymaxum::guf::{GufIntegration, IntegrationStatus};
use chrono::{DateTime, Local};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use log::LevelFilter;
use env_logger::Builder;

/// 插件清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub guf_compatible: bool,
    pub guf_version: String,
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub log_level: LogLevel,
    pub log_file: String,
    pub max_file_size: u64, // 字节
    pub max_files: usize,
    pub enable_console: bool,
    pub enable_file: bool,
}

/// 日志查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogQueryRequest {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub level: Option<LogLevel>,
    pub module: Option<String>,
    pub message: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// 日志查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogQueryResponse {
    pub success: bool,
    pub logs: Vec<LogEntry>,
    pub total: usize,
    pub message: String,
}

/// 日志管理插件
pub struct LogManagementPlugin {
    /// 插件信息
    pub info: PluginInfo,
    /// 插件清单
    pub manifest: PluginManifest,
    /// GUF 集成
    pub guf_integration: Arc<RwLock<GufIntegration>>,
    /// 插件状态
    pub status: PluginStatus,
    /// 日志配置
    pub config: Arc<RwLock<LogConfig>>,
    /// 日志文件写入器
    pub log_writer: Arc<RwLock<Option<BufWriter<File>>>>,
    /// 日志存储
    pub logs: Arc<RwLock<Vec<LogEntry>>>,
}

impl LogManagementPlugin {
    /// 创建新的日志管理插件实例
    pub fn new() -> Self {
        let manifest = PluginManifest {
            name: "log_management_plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "YMAxum 日志管理插件，提供基于 GUF 的日志管理功能".to_string(),
            author: "YMAxum Team <team@ymaxum.com>".to_string(),
            license: "MIT".to_string(),
            dependencies: vec!["ymaxum".to_string(), "guf-core".to_string(), "chrono".to_string(), "log".to_string()],
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        let info = PluginInfo {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            status: PluginStatus::Installed,
            manifest: Some(manifest.clone()),
        };

        let guf_integration = Arc::new(RwLock::new(GufIntegration::new()));

        let default_config = LogConfig {
            log_level: LogLevel::Info,
            log_file: "./logs/ymaxum.log".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 3,
            enable_console: true,
            enable_file: true,
        };

        Self {
            info,
            manifest,
            guf_integration,
            status: PluginStatus::Installed,
            config: Arc::new(RwLock::new(default_config)),
            log_writer: Arc::new(RwLock::new(None)),
            logs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 初始化插件
    pub async fn initialize(&mut self) -> Result<()> {
        println!("Initializing log management plugin...");

        // 初始化 GUF 集成
        let mut guf_integration = self.guf_integration.write().await;
        guf_integration.init().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize GUF integration: {}", e))?;

        // 启动 GUF 集成
        guf_integration.start().await
            .map_err(|e| anyhow::anyhow!("Failed to start GUF integration: {}", e))?;

        // 初始化日志配置
        self.init_log_config().await?;

        // 初始化日志文件
        self.init_log_file().await?;

        // 更新插件状态
        self.status = PluginStatus::Enabled;
        self.info.status = PluginStatus::Enabled;

        println!("Log management plugin initialized successfully!");
        Ok(())
    }

    /// 启动插件
    pub async fn start(&mut self) -> Result<()> {
        println!("Starting log management plugin...");

        // 检查 GUF 集成状态
        let guf_integration = self.guf_integration.read().await;
        if !guf_integration.is_running() {
            drop(guf_integration);
            let mut guf_integration = self.guf_integration.write().await;
            guf_integration.start().await
                .map_err(|e| anyhow::anyhow!("Failed to start GUF integration: {}", e))?;
        }

        // 重新初始化日志文件
        self.init_log_file().await?;

        // 更新插件状态
        self.status = PluginStatus::Enabled;
        self.info.status = PluginStatus::Enabled;

        println!("Log management plugin started successfully!");
        Ok(())
    }

    /// 停止插件
    pub async fn stop(&mut self) -> Result<()> {
        println!("Stopping log management plugin...");

        // 停止 GUF 集成
        let mut guf_integration = self.guf_integration.write().await;
        guf_integration.stop().await
            .map_err(|e| anyhow::anyhow!("Failed to stop GUF integration: {}", e))?;

        // 关闭日志文件
        let mut log_writer = self.log_writer.write().await;
        *log_writer = None;

        // 更新插件状态
        self.status = PluginStatus::Disabled;
        self.info.status = PluginStatus::Disabled;

        println!("Log management plugin stopped successfully!");
        Ok(())
    }

    /// 卸载插件
    pub async fn uninstall(&mut self) -> Result<()> {
        println!("Uninstalling log management plugin...");

        // 停止 GUF 集成
        let mut guf_integration = self.guf_integration.write().await;
        if guf_integration.is_running() {
            guf_integration.stop().await
                .map_err(|e| anyhow::anyhow!("Failed to stop GUF integration: {}", e))?;
        }

        // 关闭日志文件
        let mut log_writer = self.log_writer.write().await;
        *log_writer = None;

        // 清理日志数据
        let mut logs = self.logs.write().await;
        logs.clear();

        // 更新插件状态
        self.status = PluginStatus::Uninstalled;
        self.info.status = PluginStatus::Uninstalled;

        println!("Log management plugin uninstalled successfully!");
        Ok(())
    }

    /// 获取插件信息
    pub fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }

    /// 获取插件清单
    pub fn get_manifest(&self) -> PluginManifest {
        self.manifest.clone()
    }

    /// 检查 GUF 集成状态
    pub async fn check_guf_status(&self) -> IntegrationStatus {
        let guf_integration = self.guf_integration.read().await;
        guf_integration.get_status()
    }

    /// 初始化日志配置
    async fn init_log_config(&self) -> Result<()> {
        let config = self.config.read().await;

        // 配置 env_logger
        let mut builder = Builder::new();
        builder.filter_level(match config.log_level {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Fatal => LevelFilter::Error,
        });

        if config.enable_console {
            builder.init();
        }

        Ok(())
    }

    /// 初始化日志文件
    async fn init_log_file(&self) -> Result<()> {
        let config = self.config.read().await;

        if config.enable_file {
            // 确保日志目录存在
            let log_path = Path::new(&config.log_file);
            if let Some(parent) = log_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // 打开或创建日志文件
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&config.log_file)?;

            let writer = BufWriter::new(file);
            let mut log_writer = self.log_writer.write().await;
            *log_writer = Some(writer);
        }

        Ok(())
    }

    /// 记录日志
    pub async fn log(&self, level: LogLevel, module: &str, message: &str, details: Option<serde_json::Value>) -> Result<()> {
        let timestamp = Local::now().to_string();
        let log_entry = LogEntry {
            timestamp: timestamp.clone(),
            level: level.clone(),
            module: module.to_string(),
            message: message.to_string(),
            details,
        };

        // 存储日志
        let mut logs = self.logs.write().await;
        logs.push(log_entry.clone());
        // 限制内存中存储的日志数量
        if logs.len() > 10000 {
            logs.drain(0..logs.len() - 10000);
        }
        drop(logs);

        // 写入日志文件
        let log_writer = self.log_writer.read().await;
        if let Some(writer) = &*log_writer {
            let log_str = format!("[{}] [{}] [{}] {} {}\n",
                timestamp,
                self.log_level_to_string(&level),
                module,
                message,
                details.map(|d| serde_json::to_string(&d).unwrap()).unwrap_or("".to_string())
            );
            let mut writer = writer.clone();
            writer.write_all(log_str.as_bytes())?;
            writer.flush()?;
        }

        Ok(())
    }

    /// 查询日志
    pub async fn query_logs(&self, request: LogQueryRequest) -> Result<LogQueryResponse> {
        let logs = self.logs.read().await;

        // 过滤日志
        let filtered_logs: Vec<LogEntry> = logs.iter()
            .filter(|log| {
                // 时间过滤
                if let Some(start_time) = &request.start_time {
                    if log.timestamp < start_time {
                        return false;
                    }
                }
                if let Some(end_time) = &request.end_time {
                    if log.timestamp > end_time {
                        return false;
                    }
                }
                // 级别过滤
                if let Some(level) = &request.level {
                    if &log.level != level {
                        return false;
                    }
                }
                // 模块过滤
                if let Some(module) = &request.module {
                    if !log.module.contains(module) {
                        return false;
                    }
                }
                // 消息过滤
                if let Some(message) = &request.message {
                    if !log.message.contains(message) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // 分页
        let total = filtered_logs.len();
        let offset = request.offset.unwrap_or(0);
        let limit = request.limit.unwrap_or(100);
        let paginated_logs = filtered_logs
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(LogQueryResponse {
            success: true,
            logs: paginated_logs,
            total,
            message: "Logs queried successfully".to_string(),
        })
    }

    /// 更新日志配置
    pub async fn update_config(&self, new_config: LogConfig) -> Result<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        drop(config);

        // 重新初始化日志配置和文件
        self.init_log_config().await?;
        self.init_log_file().await?;

        Ok(())
    }

    /// 获取当前日志配置
    pub async fn get_config(&self) -> LogConfig {
        self.config.read().await.clone()
    }

    /// 清理旧日志
    pub async fn clean_old_logs(&self) -> Result<()> {
        let config = self.config.read().await;
        let log_path = Path::new(&config.log_file);

        // 检查日志文件大小
        if let Ok(metadata) = log_path.metadata() {
            if metadata.len() > config.max_file_size {
                // 备份当前日志文件
                let backup_path = format!("{}.{}", config.log_file, Local::now().format("%Y%m%d%H%M%S"));
                std::fs::copy(&config.log_file, backup_path)?;

                // 清空当前日志文件
                std::fs::write(&config.log_file, "")?;

                // 重新初始化日志文件
                drop(config);
                self.init_log_file().await?;
            }
        }

        Ok(())
    }

    /// 将日志级别转换为字符串
    fn log_level_to_string(&self, level: &LogLevel) -> String {
        match level {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
        .to_string()
    }

    /// 处理 GUF 事件
    pub async fn handle_guf_event(&self, event_type: String, event_data: serde_json::Value) -> Result<()> {
        println!("Handling GUF event: {} with data: {:?}", event_type, event_data);
        // 记录 GUF 事件
        self.log(LogLevel::Info, "guf_event", &format!("Received event: {}", event_type), Some(event_data)).await?;
        Ok(())
    }

    /// 调用 GUF 服务
    pub async fn call_guf_service(&self, service_name: String, service_params: serde_json::Value) -> Result<serde_json::Value> {
        println!("Calling GUF service: {} with params: {:?}", service_name, service_params);
        // 记录 GUF 服务调用
        self.log(LogLevel::Info, "guf_service", &format!("Calling service: {}", service_name), Some(service_params.clone())).await?;
        Ok(serde_json::json!({
            "status": "success",
            "message": format!("Service {} called successfully", service_name),
            "data": service_params
        }))
    }
}

/// 插件入口点
#[no_mangle]
pub extern "C" fn plugin_create() -> *mut LogManagementPlugin {
    let plugin = Box::new(LogManagementPlugin::new());
    Box::into_raw(plugin)
}

/// 插件初始化
#[no_mangle]
pub extern "C" fn plugin_initialize(plugin: *mut LogManagementPlugin) -> bool {
    if plugin.is_null() {
        return false;
    }

    let plugin = unsafe { &mut *plugin };
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            plugin.initialize().await.is_ok()
        })
}

/// 插件启动
#[no_mangle]
pub extern "C" fn plugin_start(plugin: *mut LogManagementPlugin) -> bool {
    if plugin.is_null() {
        return false;
    }

    let plugin = unsafe { &mut *plugin };
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            plugin.start().await.is_ok()
        })
}

/// 插件停止
#[no_mangle]
pub extern "C" fn plugin_stop(plugin: *mut LogManagementPlugin) -> bool {
    if plugin.is_null() {
        return false;
    }

    let plugin = unsafe { &mut *plugin };
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            plugin.stop().await.is_ok()
        })
}

/// 插件卸载
#[no_mangle]
pub extern "C" fn plugin_uninstall(plugin: *mut LogManagementPlugin) -> bool {
    if plugin.is_null() {
        return false;
    }

    let plugin = unsafe { &mut *plugin };
    let result = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            plugin.uninstall().await.is_ok()
        });

    if result {
        unsafe {
            Box::from_raw(plugin);
        }
    }

    result
}

/// 插件获取信息
#[no_mangle]
pub extern "C" fn plugin_get_info(plugin: *mut LogManagementPlugin) -> *const PluginInfo {
    if plugin.is_null() {
        return std::ptr::null();
    }

    let plugin = unsafe { &*plugin };
    let info = plugin.get_info();
    let boxed_info = Box::new(info);
    Box::into_raw(boxed_info)
}
