// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件热重载功能
//! 提供插件的动态加载、卸载和重载功能

use log::{info, error, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, mpsc};

/// 插件状态
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    /// 未加载
    Unloaded,
    /// 已加载
    Loaded,
    /// 已启动
    Started,
    /// 已停止
    Stopped,
}

/// 插件热重载信息
#[derive(Debug, Clone)]
pub struct HotReloadPluginInfo {
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件路径
    pub path: PathBuf,
    /// 插件状态
    pub state: PluginState,
    /// 加载时间
    pub loaded_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 最后修改时间
    pub last_modified: Option<std::time::SystemTime>,
    /// 插件状态数据
    pub state_data: Option<serde_json::Value>,
    /// 状态保存时间
    pub state_saved_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 热重载事件
#[derive(Debug, Clone)]
pub enum HotReloadEvent {
    /// 插件已加载
    PluginLoaded(String),
    /// 插件已卸载
    PluginUnloaded(String),
    /// 插件已重载
    PluginReloaded(String),
    /// 插件加载失败
    PluginLoadFailed(String, String),
    /// 插件卸载失败
    PluginUnloadFailed(String, String),
}

/// 热重载配置
#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    /// 插件目录
    pub plugin_dir: String,
    /// 监控间隔（秒）
    pub watch_interval: u64,
    /// 是否启用自动重载
    pub auto_reload_enabled: bool,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            plugin_dir: "plugins".to_string(),
            watch_interval: 5,
            auto_reload_enabled: true,
        }
    }
}

/// 插件热重载器
pub struct PluginHotReloader {
    /// 配置信息
    config: HotReloadConfig,
    /// 插件热重载信息
    plugins: Arc<RwLock<HashMap<String, HotReloadPluginInfo>>>,
    /// 事件发送器
    event_sender: mpsc::UnboundedSender<HotReloadEvent>,
    /// 事件接收器
    event_receiver: Option<mpsc::UnboundedReceiver<HotReloadEvent>>,
    /// 是否正在运行
    running: Arc<RwLock<bool>>,
}

impl PluginHotReloader {
    /// 创建新的插件热重载器实例
    pub fn new(config: HotReloadConfig) -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Self {
            config,
            plugins: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Some(event_receiver),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// 启动热重载器
    pub async fn start(&self) -> Result<(), String> {
        info!("启动插件热重载器...");

        let mut running = self.running.write().await;
        if *running {
            return Err("热重载器已在运行".to_string());
        }

        *running = true;

        let plugin_dir = Path::new(&self.config.plugin_dir);
        if !plugin_dir.exists() {
            return Err(format!("插件目录不存在：{}", self.config.plugin_dir));
        }

        info!("扫描插件目录：{}", self.config.plugin_dir);
        self.scan_plugins().await;

        if self.config.auto_reload_enabled {
            let plugins = self.plugins.clone();
            let event_sender = self.event_sender.clone();
            let plugin_dir_clone = plugin_dir.to_path_buf();
            let watch_interval = self.config.watch_interval;

            tokio::spawn(async move {
                Self::watch_plugins(plugins, event_sender, plugin_dir_clone, watch_interval).await;
            });
        }

        info!("插件热重载器已启动");
        Ok(())
    }

    /// 停止热重载器
    pub async fn stop(&self) -> Result<(), String> {
        info!("停止插件热重载器...");

        let mut running = self.running.write().await;
        if !*running {
            return Err("热重载器未在运行".to_string());
        }

        *running = false;

        info!("插件热重载器已停止");
        Ok(())
    }

    /// 扫描插件目录
    async fn scan_plugins(&self) {
        let plugin_dir = Path::new(&self.config.plugin_dir);
        let mut plugins = self.plugins.write().await;

        if let Ok(entries) = std::fs::read_dir(plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("axpl") {
                    let plugin_name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let metadata = std::fs::metadata(&path).ok();
                    let last_modified = metadata.and_then(|m| m.modified().ok());

                    let plugin_info = HotReloadPluginInfo {
                        name: plugin_name.clone(),
                        version: "1.0.0".to_string(),
                        path: path.clone(),
                        state: PluginState::Unloaded,
                        loaded_at: None,
                        last_modified,
                        state_data: None,
                        state_saved_at: None,
                    };

                    plugins.insert(plugin_name.clone(), plugin_info);

                    info!("发现新插件：{}", plugin_name);
                }
            }
        }
    }

    /// 监控插件目录
    async fn watch_plugins(
        plugins: Arc<RwLock<HashMap<String, HotReloadPluginInfo>>>,
        event_sender: mpsc::UnboundedSender<HotReloadEvent>,
        plugin_dir: PathBuf,
        watch_interval: u64,
    ) {
        loop {
            tokio::time::sleep(Duration::from_secs(watch_interval)).await;

            let mut plugins_guard = plugins.write().await;
            let mut has_changes = false;

            if let Ok(entries) = std::fs::read_dir(&plugin_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("axpl") {
                        let plugin_name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let metadata = std::fs::metadata(&path).ok();
                        let last_modified = metadata.and_then(|m| m.modified().ok());

                        if let Some(plugin_info) = plugins_guard.get_mut(&plugin_name)
                            && plugin_info.last_modified != last_modified
                        {
                            info!("检测到插件变化：{}", plugin_name);

                            plugin_info.last_modified = last_modified;
                            has_changes = true;

                            let _ = event_sender
                                .send(HotReloadEvent::PluginReloaded(plugin_name.clone()));
                        }
                    }
                }
            }

            if has_changes {
                info!("插件目录有变化，开始重载插件...");
            }
        }
    }

    /// 加载插件
    pub async fn load_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("加载插件：{}", plugin_name);

        let mut plugins = self.plugins.write().await;

        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            if plugin_info.state == PluginState::Loaded || plugin_info.state == PluginState::Started
            {
                let error_msg = format!("插件已加载或启动：{}", plugin_name);
                warn!("{}", error_msg);
                return Err(error_msg);
            }

            // 尝试加载插件
            match self.try_load_plugin(plugin_info).await {
                Ok(()) => {
                    plugin_info.state = PluginState::Loaded;
                    plugin_info.loaded_at = Some(chrono::Utc::now());

                    let _ = self
                        .event_sender
                        .send(HotReloadEvent::PluginLoaded(plugin_name.to_string()));

                    info!("插件加载成功：{}", plugin_name);
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("加载插件失败：{}: {}", plugin_name, e);
                    error!("{}", error_msg);
                    let _ = self.event_sender.send(HotReloadEvent::PluginLoadFailed(plugin_name.to_string(), error_msg.clone()));
                    Err(error_msg)
                }
            }
        } else {
            let error_msg = format!("插件不存在：{}", plugin_name);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }

    /// 尝试加载插件的内部方法
    async fn try_load_plugin(&self, _plugin_info: &mut HotReloadPluginInfo) -> Result<(), String> {
        // 这里可以添加具体的插件加载逻辑，如验证插件完整性、初始化插件等
        // 目前只是一个占位符，实际实现需要根据插件类型和加载方式进行调整
        Ok(())
    }

    /// 卸载插件
    pub async fn unload_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("卸载插件：{}", plugin_name);

        let mut plugins = self.plugins.write().await;

        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            if plugin_info.state == PluginState::Unloaded {
                let error_msg = format!("插件未加载：{}", plugin_name);
                warn!("{}", error_msg);
                return Err(error_msg);
            }

            // 尝试卸载插件
            match self.try_unload_plugin(plugin_info).await {
                Ok(()) => {
                    plugin_info.state = PluginState::Unloaded;
                    plugin_info.loaded_at = None;

                    let _ = self
                        .event_sender
                        .send(HotReloadEvent::PluginUnloaded(plugin_name.to_string()));

                    info!("插件卸载成功：{}", plugin_name);
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("卸载插件失败：{}: {}", plugin_name, e);
                    error!("{}", error_msg);
                    let _ = self.event_sender.send(HotReloadEvent::PluginUnloadFailed(plugin_name.to_string(), error_msg.clone()));
                    Err(error_msg)
                }
            }
        } else {
            let error_msg = format!("插件不存在：{}", plugin_name);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }

    /// 尝试卸载插件的内部方法
    async fn try_unload_plugin(&self, _plugin_info: &mut HotReloadPluginInfo) -> Result<(), String> {
        // 这里可以添加具体的插件卸载逻辑，如清理资源、停止插件进程等
        // 目前只是一个占位符，实际实现需要根据插件类型和加载方式进行调整
        Ok(())
    }

    /// 重载插件
    pub async fn reload_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("重载插件：{}", plugin_name);

        // 保存插件状态
        let saved_state = self.load_plugin_state(plugin_name).await;
        let original_state = self.get_plugin_info(plugin_name).await;

        // 卸载插件
        if let Err(e) = self.unload_plugin(plugin_name).await {
            let error_msg = format!("卸载插件失败: {}", e);
            error!("{}", error_msg);
            let _ = self.event_sender.send(HotReloadEvent::PluginUnloadFailed(plugin_name.to_string(), error_msg.clone()));
            return Err(error_msg);
        }

        // 加载插件
        if let Err(e) = self.load_plugin(plugin_name).await {
            let error_msg = format!("加载插件失败: {}", e);
            error!("{}", error_msg);
            let _ = self.event_sender.send(HotReloadEvent::PluginLoadFailed(plugin_name.to_string(), error_msg.clone()));

            // 尝试回滚到原始状态
            if let Some(original) = original_state {
                warn!("尝试回滚插件状态");
                let mut plugins = self.plugins.write().await;
                plugins.insert(plugin_name.to_string(), original);
            }

            return Err(error_msg);
        }

        // 恢复插件状态
        if let Some(_state) = saved_state {
            // 这里可以通过插件通信机制将状态发送给重新加载的插件
            info!("恢复插件状态：{}", plugin_name);
        }

        let _ = self
            .event_sender
            .send(HotReloadEvent::PluginReloaded(plugin_name.to_string()));

        info!("插件重载成功：{}", plugin_name);
        Ok(())
    }

    /// 获取插件信息
    pub async fn get_plugin_info(&self, plugin_name: &str) -> Option<HotReloadPluginInfo> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_name).cloned()
    }

    /// 获取所有插件信息
    pub async fn get_all_plugins(&self) -> Vec<HotReloadPluginInfo> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }

    /// 获取事件接收器
    pub fn take_event_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<HotReloadEvent>> {
        self.event_receiver.take()
    }

    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// 保存插件状态
    pub async fn save_plugin_state(&self, plugin_name: &str, state: serde_json::Value) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;
        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            plugin_info.state_data = Some(state);
            plugin_info.state_saved_at = Some(chrono::Utc::now());
            info!("保存插件状态: {}", plugin_name);
            Ok(())
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// 加载插件状态
    pub async fn load_plugin_state(&self, plugin_name: &str) -> Option<serde_json::Value> {
        let plugins = self.plugins.read().await;
        if let Some(plugin_info) = plugins.get(plugin_name) {
            plugin_info.state_data.clone()
        } else {
            None
        }
    }

    /// 清除插件状态
    pub async fn clear_plugin_state(&self, plugin_name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;
        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            plugin_info.state_data = None;
            plugin_info.state_saved_at = None;
            Ok(())
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }
}

impl Default for PluginHotReloader {
    fn default() -> Self {
        Self::new(HotReloadConfig::default())
    }
}

