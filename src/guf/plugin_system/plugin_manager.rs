//! 插件管理器模块
//! 负责插件的生命周期管理（加载、初始化、启动、停止、卸载）

use super::{GufPlugin, GufPluginConfig, PluginStatus};
use log::{info, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;
use chrono;

/// GUF插件管理器
#[derive(Debug, Clone)]
pub struct GufPluginManager {
    /// 插件存储
    plugins: Arc<RwLock<HashMap<String, GufPlugin>>>,
    /// 插件配置存储
    plugin_configs: Arc<RwLock<HashMap<String, GufPluginConfig>>>,
}

impl GufPluginManager {
    /// 创建新的GUF插件管理器
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            plugin_configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 初始化插件管理器
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件管理器");
        Ok(())
    }

    /// 添加插件
    pub async fn add_plugin(&self, plugin: GufPlugin) -> Result<(), String> {
        info!("添加插件: {}", plugin.config.name);

        let mut plugins = self.plugins.write().await;
        let mut plugin_configs = self.plugin_configs.write().await;

        plugins.insert(plugin.config.name.clone(), plugin.clone());
        plugin_configs.insert(plugin.config.name.clone(), plugin.config);

        Ok(())
    }

    /// 启动插件
    pub async fn start_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("启动插件: {}", plugin_name);

        // 检查插件是否存在
        let mut plugins = self.plugins.write().await;
        let plugin = plugins
            .get_mut(plugin_name)
            .ok_or_else(|| format!("插件不存在: {}", plugin_name))?;

        // 检查插件状态
        if plugin.status == PluginStatus::Running {
            return Err(format!("插件已经在运行: {}", plugin_name));
        }

        // 更新插件状态为初始化中
        plugin.status = PluginStatus::Initializing;

        // 尝试启动插件，添加超时处理
        let start_result = tokio::time::timeout(
            Duration::from_secs(30),
            async {
                // 模拟插件启动过程
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                Ok::<(), String>(())
            }
        ).await;

        match start_result {
            Ok(_) => {
                // 更新插件状态为运行中
                plugin.status = PluginStatus::Running;
                plugin.started_at = Some(chrono::Utc::now());
                info!("插件启动完成: {}", plugin_name);
                Ok(())
            }
            Err(_) => {
                plugin.status = PluginStatus::Error("启动超时".to_string());
                error!("插件启动超时: {}", plugin_name);
                Err(format!("插件启动超时: {}", plugin_name))
            }
        }
    }

    /// 停止插件
    pub async fn stop_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("停止插件: {}", plugin_name);

        // 检查插件是否存在
        let mut plugins = self.plugins.write().await;
        let plugin = plugins
            .get_mut(plugin_name)
            .ok_or_else(|| format!("插件不存在: {}", plugin_name))?;

        // 检查插件状态
        if plugin.status == PluginStatus::Stopped {
            return Err(format!("插件已经停止: {}", plugin_name));
        }

        // 更新插件状态为停止中
        plugin.status = PluginStatus::Stopping;

        // 尝试停止插件，添加超时处理
        let stop_result = tokio::time::timeout(
            Duration::from_secs(15),
            async {
                // 模拟插件停止过程
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                Ok::<(), String>(())
            }
        ).await;

        match stop_result {
            Ok(_) => {
                // 更新插件状态为已停止
                plugin.status = PluginStatus::Stopped;
                plugin.started_at = None;
                info!("插件停止完成: {}", plugin_name);
                Ok(())
            }
            Err(_) => {
                plugin.status = PluginStatus::Error("停止超时".to_string());
                error!("插件停止超时: {}", plugin_name);
                Err(format!("插件停止超时: {}", plugin_name))
            }
        }
    }

    /// 卸载插件
    pub async fn unload_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("卸载插件: {}", plugin_name);

        // 先停止插件
        if let Err(e) = self.stop_plugin(plugin_name).await {
            // 如果插件已经停止，继续卸载
            if !e.contains("已经停止") {
                return Err(e);
            }
        }

        // 移除插件
        let mut plugins = self.plugins.write().await;
        let mut plugin_configs = self.plugin_configs.write().await;

        if plugins.remove(plugin_name).is_none() {
            return Err(format!("插件不存在: {}", plugin_name));
        }

        plugin_configs.remove(plugin_name);

        info!("插件卸载完成: {}", plugin_name);
        Ok(())
    }

    /// 获取插件状态
    pub async fn get_plugin_status(&self, plugin_name: &str) -> Result<PluginStatus, String> {
        let plugins = self.plugins.read().await;
        let plugin = plugins
            .get(plugin_name)
            .ok_or_else(|| format!("插件不存在: {}", plugin_name))?;
        Ok(plugin.status.clone())
    }

    /// 列出所有插件
    pub async fn list_plugins(&self) -> Result<Vec<GufPlugin>, String> {
        let plugins = self.plugins.read().await;
        Ok(plugins.values().cloned().collect())
    }

    /// 获取插件
    pub async fn get_plugin(&self, plugin_name: &str) -> Result<GufPlugin, String> {
        let plugins = self.plugins.read().await;
        let plugin = plugins
            .get(plugin_name)
            .ok_or_else(|| format!("插件不存在: {}", plugin_name))?;
        Ok(plugin.clone())
    }

    /// 获取插件配置
    pub async fn get_plugin_config(&self, plugin_name: &str) -> Result<GufPluginConfig, String> {
        let plugin_configs = self.plugin_configs.read().await;
        let config = plugin_configs
            .get(plugin_name)
            .ok_or_else(|| format!("插件配置不存在: {}", plugin_name))?;
        Ok(config.clone())
    }

    /// 更新插件配置
    pub async fn update_plugin_config(
        &self,
        plugin_name: &str,
        config: GufPluginConfig,
    ) -> Result<(), String> {
        info!("更新插件配置: {}", plugin_name);

        // 检查插件是否存在
        let mut plugin_configs = self.plugin_configs.write().await;
        if !plugin_configs.contains_key(plugin_name) {
            return Err(format!("插件不存在: {}", plugin_name));
        }

        // 更新配置
        plugin_configs.insert(plugin_name.to_string(), config);

        // 同时更新插件中的配置
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.config = plugin_configs.get(plugin_name).unwrap().clone();
        }

        info!("插件配置更新完成: {}", plugin_name);
        Ok(())
    }

    /// 检查插件是否存在
    pub async fn plugin_exists(&self, plugin_name: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.contains_key(plugin_name)
    }

    /// 获取插件数量
    pub async fn get_plugin_count(&self) -> usize {
        let plugins = self.plugins.read().await;
        plugins.len()
    }
}
