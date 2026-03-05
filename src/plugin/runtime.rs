// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件运行时
//! 负责插件的生命周期管理，包括注册、初始化、启动、停止和热重载等运行时管理

use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::iterate_api::PluginLifecycle;
use crate::core::state::AppState;

/// 插件运行时状态
#[derive(Debug, PartialEq, Clone)]
pub enum PluginRuntimeStatus {
    /// 已注册
    Registered,
    /// 已初始化
    Initialized,
    /// 运行中
    Running,
    /// 已停止
    Stopped,
    /// 初始化失败
    InitFailed,
    /// 启动失败
    StartFailed,
}

/// 插件运行时信息
#[derive(Debug, Clone)]
pub struct PluginRuntimeInfo {
    /// 插件名称
    pub name: String,
    /// 运行时状态
    pub status: PluginRuntimeStatus,
}

/// 插件运行时
pub struct PluginRuntime {
    /// 插件生命周期映射
    plugins: Arc<RwLock<HashMap<String, Arc<dyn PluginLifecycle>>>>,
    /// 插件运行时信息映射
    runtime_info: Arc<RwLock<HashMap<String, PluginRuntimeInfo>>>,
}

impl PluginRuntime {
    /// 创建新的插件运行时实例
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            runtime_info: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册插件
    pub async fn register_plugin(&self, plugin: Arc<dyn PluginLifecycle>) -> Result<(), String> {
        let plugin_name = plugin.name().to_string();
        info!("注册插件: {}", plugin_name);

        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin_name.clone(), plugin.clone());

        let mut runtime_info = self.runtime_info.write().await;
        runtime_info.insert(
            plugin_name.clone(),
            PluginRuntimeInfo {
                name: plugin_name.clone(),
                status: PluginRuntimeStatus::Registered,
            },
        );

        info!("插件注册成功: {}", plugin_name);
        Ok(())
    }

    /// 初始化插件
    pub async fn init_plugin(
        &self,
        plugin_name: &str,
        app_state: Arc<AppState>,
    ) -> Result<(), String> {
        info!("初始化插件: {}", plugin_name);

        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            match plugin.init(app_state).await {
                Ok(_) => {
                    let mut runtime_info = self.runtime_info.write().await;
                    if let Some(info) = runtime_info.get_mut(plugin_name) {
                        info.status = PluginRuntimeStatus::Initialized;
                    }

                    info!("插件初始化成功: {}", plugin_name);
                    Ok(())
                }
                Err(e) => Err(format!("插件初始化失败: {}", e)),
            }
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// 启动插件
    pub async fn start_plugin(
        &self,
        plugin_name: &str,
        app_state: Arc<AppState>,
    ) -> Result<(), String> {
        info!("启动插件: {}", plugin_name);

        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            match plugin.start(app_state).await {
                Ok(_) => {
                    let mut runtime_info = self.runtime_info.write().await;
                    if let Some(info) = runtime_info.get_mut(plugin_name) {
                        info.status = PluginRuntimeStatus::Running;
                    }

                    info!("插件启动成功: {}", plugin_name);
                    Ok(())
                }
                Err(e) => Err(format!("插件启动失败: {}", e)),
            }
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// 停止插件
    pub async fn stop_plugin(
        &self,
        plugin_name: &str,
        app_state: Arc<AppState>,
    ) -> Result<(), String> {
        info!("停止插件: {}", plugin_name);

        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            match plugin.stop(app_state).await {
                Ok(_) => {
                    let mut runtime_info = self.runtime_info.write().await;
                    if let Some(info) = runtime_info.get_mut(plugin_name) {
                        info.status = PluginRuntimeStatus::Stopped;
                    }

                    info!("插件停止成功: {}", plugin_name);
                    Ok(())
                }
                Err(e) => Err(format!("插件停止失败: {}", e)),
            }
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// 获取插件运行时信息
    pub async fn get_plugin_info(&self, plugin_name: &str) -> Option<PluginRuntimeInfo> {
        let runtime_info = self.runtime_info.read().await;
        runtime_info.get(plugin_name).cloned()
    }

    /// 获取所有插件运行时信息
    pub async fn get_all_plugins(&self) -> Vec<PluginRuntimeInfo> {
        let runtime_info = self.runtime_info.read().await;
        runtime_info.values().cloned().collect()
    }
}

impl Default for PluginRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::iterate_api::PluginLifecycle;
    use async_trait::async_trait;

    struct TestPlugin {
        name: &'static str,
    }

    impl TestPlugin {
        fn new(name: &'static str) -> Self {
            Self { name }
        }
    }

    #[async_trait]
    impl PluginLifecycle for TestPlugin {
        async fn init(
            &self,
            _state: Arc<AppState>,
        ) -> Result<(), crate::core::iterate_api::IterateError> {
            log::info!("TestPlugin {} initialized", self.name);
            Ok(())
        }

        async fn start(
            &self,
            _state: Arc<AppState>,
        ) -> Result<(), crate::core::iterate_api::IterateError> {
            log::info!("TestPlugin {} started", self.name);
            Ok(())
        }

        async fn stop(
            &self,
            _state: Arc<AppState>,
        ) -> Result<(), crate::core::iterate_api::IterateError> {
            log::info!("TestPlugin {} stopped", self.name);
            Ok(())
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn description(&self) -> &'static str {
            "Test plugin"
        }

        fn plugin_type(&self) -> &'static str {
            "test"
        }
    }

    #[tokio::test]
    async fn test_plugin_runtime() {
        let runtime = PluginRuntime::new();
        let plugin = Arc::new(TestPlugin::new("test_plugin"));
        let app_state = Arc::new(AppState::new());

        // 注册插件
        runtime.register_plugin(plugin.clone()).await.unwrap();

        // 获取插件信息
        let info = runtime.get_plugin_info("test_plugin").await;
        assert!(info.is_some());
        assert_eq!(info.unwrap().status, PluginRuntimeStatus::Registered);

        // 初始化插件
        runtime
            .init_plugin("test_plugin", app_state.clone())
            .await
            .unwrap();

        // 获取插件信息
        let info = runtime.get_plugin_info("test_plugin").await;
        assert_eq!(info.unwrap().status, PluginRuntimeStatus::Initialized);

        // 启动插件
        runtime
            .start_plugin("test_plugin", app_state.clone())
            .await
            .unwrap();

        // 获取插件信息
        let info = runtime.get_plugin_info("test_plugin").await;
        assert_eq!(info.unwrap().status, PluginRuntimeStatus::Running);

        // 停止插件
        runtime.stop_plugin("test_plugin", app_state).await.unwrap();

        // 获取插件信息
        let info = runtime.get_plugin_info("test_plugin").await;
        assert_eq!(info.unwrap().status, PluginRuntimeStatus::Stopped);
    }
}

