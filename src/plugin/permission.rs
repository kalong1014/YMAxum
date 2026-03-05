// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件权限控制模块
//! 负责实现细粒度的插件权限管理，确保插件只能访问授权的资源

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 插件权限类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginPermission {
    /// 读取配置权限
    ReadConfig,
    /// 写入配置权限
    WriteConfig,
    /// 读取数据库权限
    ReadDatabase,
    /// 写入数据库权限
    WriteDatabase,
    /// 读取缓存权限
    ReadCache,
    /// 写入缓存权限
    WriteCache,
    /// 发送HTTP请求权限
    SendHttpRequest,
    /// 接收HTTP请求权限
    ReceiveHttpRequest,
    /// 访问文件系统权限
    AccessFileSystem,
    /// 执行系统命令权限
    ExecuteSystemCommand,
    /// 访问网络权限
    AccessNetwork,
    /// 访问系统资源权限
    AccessSystemResources,
    /// 管理其他插件权限
    ManagePlugins,
    /// 访问安全模块权限
    AccessSecurityModule,
    /// 访问性能监控权限
    AccessPerformanceMonitor,
}

/// 插件权限配置
#[derive(Debug, Clone)]
pub struct PluginPermissionConfig {
    /// 插件名称
    pub plugin_name: String,
    /// 授予的权限列表
    pub granted_permissions: HashSet<PluginPermission>,
    /// 拒绝的权限列表
    pub denied_permissions: HashSet<PluginPermission>,
    /// 是否继承默认权限
    pub inherit_default: bool,
}

/// 插件权限管理器
#[derive(Debug)]
pub struct PluginPermissionManager {
    /// 插件权限配置映射
    permissions: Arc<RwLock<HashMap<String, PluginPermissionConfig>>>,
    /// 默认权限列表
    default_permissions: Arc<HashSet<PluginPermission>>,
}

impl PluginPermissionManager {
    /// 创建新的插件权限管理器
    pub fn new() -> Self {
        let default_permissions = Self::get_default_permissions();

        Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
            default_permissions: Arc::new(default_permissions),
        }
    }

    /// 获取默认权限列表
    fn get_default_permissions() -> HashSet<PluginPermission> {
        let mut permissions = HashSet::new();
        permissions.insert(PluginPermission::ReadConfig);
        permissions.insert(PluginPermission::ReadDatabase);
        permissions.insert(PluginPermission::ReadCache);
        permissions.insert(PluginPermission::ReceiveHttpRequest);
        permissions.insert(PluginPermission::AccessPerformanceMonitor);
        permissions
    }

    /// 注册插件权限配置
    pub async fn register_plugin(&self, plugin_name: &str) {
        let config = PluginPermissionConfig {
            plugin_name: plugin_name.to_string(),
            granted_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            inherit_default: true,
        };

        let mut permissions = self.permissions.write().await;
        permissions.insert(plugin_name.to_string(), config);
    }

    /// 授予插件权限
    pub async fn grant_permission(&self, plugin_name: &str, permission: PluginPermission) {
        let mut permissions = self.permissions.write().await;
        if let Some(config) = permissions.get_mut(plugin_name) {
            config.granted_permissions.insert(permission.clone());
            config.denied_permissions.remove(&permission);
        }
    }

    /// 拒绝插件权限
    pub async fn deny_permission(&self, plugin_name: &str, permission: PluginPermission) {
        let mut permissions = self.permissions.write().await;
        if let Some(config) = permissions.get_mut(plugin_name) {
            config.denied_permissions.insert(permission.clone());
            config.granted_permissions.remove(&permission);
        }
    }

    /// 检查插件是否有指定权限
    pub async fn check_permission(&self, plugin_name: &str, permission: &PluginPermission) -> bool {
        let permissions = self.permissions.read().await;

        if let Some(config) = permissions.get(plugin_name) {
            // 优先检查显式授予的权限
            if config.granted_permissions.contains(permission) {
                return true;
            }

            // 检查是否被显式拒绝
            if config.denied_permissions.contains(permission) {
                return false;
            }

            // 检查是否继承默认权限
            if config.inherit_default && self.default_permissions.contains(permission) {
                return true;
            }
        }

        false
    }

    /// 获取插件的所有有效权限
    pub async fn get_effective_permissions(&self, plugin_name: &str) -> HashSet<PluginPermission> {
        let permissions = self.permissions.read().await;
        let mut effective_permissions = HashSet::new();

        if let Some(config) = permissions.get(plugin_name) {
            // 添加显式授予的权限
            effective_permissions.extend(config.granted_permissions.clone());

            // 添加继承的默认权限（排除被拒绝的）
            if config.inherit_default {
                for permission in self.default_permissions.iter() {
                    if !config.denied_permissions.contains(permission) {
                        effective_permissions.insert(permission.clone());
                    }
                }
            }
        }

        effective_permissions
    }

    /// 从文件加载权限配置
    pub async fn load_from_file(&self, _file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 实现从文件加载权限配置的逻辑
        // 这里可以使用serde来解析配置文件
        Ok(())
    }

    /// 保存权限配置到文件
    pub async fn save_to_file(&self, _file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 实现保存权限配置到文件的逻辑
        // 这里可以使用serde来序列化配置
        Ok(())
    }

    /// 重置插件权限
    pub async fn reset_permissions(&self, plugin_name: &str) {
        let mut permissions = self.permissions.write().await;
        if let Some(config) = permissions.get_mut(plugin_name) {
            config.granted_permissions.clear();
            config.denied_permissions.clear();
            config.inherit_default = true;
        }
    }

    /// 移除插件权限配置
    pub async fn remove_plugin(&self, plugin_name: &str) {
        let mut permissions = self.permissions.write().await;
        permissions.remove(plugin_name);
    }

    /// 获取所有插件的权限配置
    pub async fn get_all_permissions(&self) -> HashMap<String, PluginPermissionConfig> {
        let permissions = self.permissions.read().await;
        permissions.clone()
    }
}

impl Default for PluginPermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 插件权限检查器
pub struct PluginPermissionChecker {
    manager: Arc<PluginPermissionManager>,
}

impl PluginPermissionChecker {
    /// 创建新的插件权限检查器
    pub fn new(manager: Arc<PluginPermissionManager>) -> Self {
        Self { manager }
    }

    /// 检查插件是否有读取配置权限
    pub async fn can_read_config(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ReadConfig)
            .await
    }

    /// 检查插件是否有写入配置权限
    pub async fn can_write_config(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::WriteConfig)
            .await
    }

    /// 检查插件是否有读取数据库权限
    pub async fn can_read_database(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ReadDatabase)
            .await
    }

    /// 检查插件是否有写入数据库权限
    pub async fn can_write_database(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::WriteDatabase)
            .await
    }

    /// 检查插件是否有读取缓存权限
    pub async fn can_read_cache(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ReadCache)
            .await
    }

    /// 检查插件是否有写入缓存权限
    pub async fn can_write_cache(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::WriteCache)
            .await
    }

    /// 检查插件是否有发送HTTP请求权限
    pub async fn can_send_http_request(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::SendHttpRequest)
            .await
    }

    /// 检查插件是否有接收HTTP请求权限
    pub async fn can_receive_http_request(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ReceiveHttpRequest)
            .await
    }

    /// 检查插件是否有访问文件系统权限
    pub async fn can_access_file_system(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::AccessFileSystem)
            .await
    }

    /// 检查插件是否有执行系统命令权限
    pub async fn can_execute_system_command(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ExecuteSystemCommand)
            .await
    }

    /// 检查插件是否有访问网络权限
    pub async fn can_access_network(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::AccessNetwork)
            .await
    }

    /// 检查插件是否有访问系统资源权限
    pub async fn can_access_system_resources(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::AccessSystemResources)
            .await
    }

    /// 检查插件是否有管理其他插件权限
    pub async fn can_manage_plugins(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::ManagePlugins)
            .await
    }

    /// 检查插件是否有访问安全模块权限
    pub async fn can_access_security_module(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::AccessSecurityModule)
            .await
    }

    /// 检查插件是否有访问性能监控权限
    pub async fn can_access_performance_monitor(&self, plugin_name: &str) -> bool {
        self.manager
            .check_permission(plugin_name, &PluginPermission::AccessPerformanceMonitor)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_permission_manager() {
        let manager = PluginPermissionManager::new();
        let plugin_name = "test_plugin";

        // 注册插件
        manager.register_plugin(plugin_name).await;

        // 检查默认权限
        assert!(
            manager
                .check_permission(plugin_name, &PluginPermission::ReadConfig)
                .await
        );
        assert!(
            !manager
                .check_permission(plugin_name, &PluginPermission::WriteConfig)
                .await
        );

        // 授予权限
        manager
            .grant_permission(plugin_name, PluginPermission::WriteConfig)
            .await;
        assert!(
            manager
                .check_permission(plugin_name, &PluginPermission::WriteConfig)
                .await
        );

        // 拒绝权限
        manager
            .deny_permission(plugin_name, PluginPermission::ReadConfig)
            .await;
        assert!(
            !manager
                .check_permission(plugin_name, &PluginPermission::ReadConfig)
                .await
        );

        // 获取有效权限
        let effective_permissions = manager.get_effective_permissions(plugin_name).await;
        assert!(effective_permissions.contains(&PluginPermission::WriteConfig));
        assert!(!effective_permissions.contains(&PluginPermission::ReadConfig));
    }

    #[tokio::test]
    async fn test_plugin_permission_checker() {
        let manager = Arc::new(PluginPermissionManager::new());
        let checker = PluginPermissionChecker::new(manager.clone());
        let plugin_name = "test_plugin";

        // 注册插件
        manager.register_plugin(plugin_name).await;

        // 检查默认权限
        assert!(checker.can_read_config(plugin_name).await);
        assert!(!checker.can_write_config(plugin_name).await);

        // 授予权限
        manager
            .grant_permission(plugin_name, PluginPermission::WriteConfig)
            .await;
        assert!(checker.can_write_config(plugin_name).await);
    }
}

