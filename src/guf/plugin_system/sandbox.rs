//! 插件沙箱模块
//! 负责插件的安全隔离，限制插件的权限和访问范围

use log::{info, debug};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 插件权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginPermission {
    /// 文件系统访问权限
    FileSystemAccess {
        /// 允许的路径列表
        allowed_paths: HashSet<String>,
        /// 是否允许写入
        allow_write: bool,
    },
    /// 网络访问权限
    NetworkAccess {
        /// 允许的域名列表
        allowed_domains: HashSet<String>,
        /// 是否允许HTTPS
        allow_https: bool,
    },
    /// 系统资源访问权限
    SystemAccess {
        /// 允许的系统调用
        allowed_calls: HashSet<String>,
    },
    /// 插件间通信权限
    PluginCommunication {
        /// 允许通信的插件列表
        allowed_plugins: HashSet<String>,
    },
}

/// 插件沙箱
#[derive(Debug, Clone)]
pub struct GufPluginSandbox {
    /// 插件权限存储
    permissions: Arc<RwLock<HashMap<String, Vec<PluginPermission>>>>,
}

impl GufPluginSandbox {
    /// 创建新的插件沙箱
    pub fn new() -> Self {
        Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 初始化沙箱
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件沙箱");
        Ok(())
    }

    /// 为插件设置权限
    pub async fn set_permissions(
        &self,
        plugin_name: &str,
        permissions: Vec<PluginPermission>,
    ) -> Result<(), String> {
        info!("为插件 {} 设置权限", plugin_name);
        
        let mut permissions_map = self.permissions.write().await;
        permissions_map.insert(plugin_name.to_string(), permissions);
        
        Ok(())
    }

    /// 检查插件是否有特定权限
    pub async fn check_permission(
        &self,
        plugin_name: &str,
        permission: &PluginPermission,
    ) -> Result<bool, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(plugin_permissions) => {
                // 检查权限是否存在
                for p in plugin_permissions {
                    match (p, permission) {
                        (PluginPermission::FileSystemAccess { allowed_paths: _, allow_write: _ }, 
                         PluginPermission::FileSystemAccess { allowed_paths: _, allow_write: _ }) => {
                            return Ok(true);
                        }
                        (PluginPermission::NetworkAccess { allowed_domains: _, allow_https: _ }, 
                         PluginPermission::NetworkAccess { allowed_domains: _, allow_https: _ }) => {
                            return Ok(true);
                        }
                        (PluginPermission::SystemAccess { allowed_calls: _ }, 
                         PluginPermission::SystemAccess { allowed_calls: _ }) => {
                            return Ok(true);
                        }
                        (PluginPermission::PluginCommunication { allowed_plugins: _ }, 
                         PluginPermission::PluginCommunication { allowed_plugins: _ }) => {
                            return Ok(true);
                        }
                        _ => continue,
                    }
                }
                Ok(false)
            }
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }

    /// 获取插件的所有权限
    pub async fn get_permissions(
        &self,
        plugin_name: &str,
    ) -> Result<Vec<PluginPermission>, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(permissions) => Ok(permissions.clone()),
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }

    /// 验证插件操作是否允许
    pub async fn validate_operation(
        &self,
        plugin_name: &str,
        operation: &str,
        details: serde_json::Value,
    ) -> Result<bool, String> {
        debug!("验证插件 {} 的操作: {}", plugin_name, operation);
        
        // 根据操作类型检查权限
        match operation {
            "file_access" => {
                let path = details.get("path").and_then(|v| v.as_str()).unwrap_or("");
                let write = details.get("write").and_then(|v| v.as_bool()).unwrap_or(false);
                
                self.check_file_access(plugin_name, path, write).await
            }
            "network_access" => {
                let domain = details.get("domain").and_then(|v| v.as_str()).unwrap_or("");
                let https = details.get("https").and_then(|v| v.as_bool()).unwrap_or(false);
                
                self.check_network_access(plugin_name, domain, https).await
            }
            "system_call" => {
                let call = details.get("call").and_then(|v| v.as_str()).unwrap_or("");
                
                self.check_system_access(plugin_name, call).await
            }
            "plugin_communication" => {
                let target_plugin = details.get("target_plugin").and_then(|v| v.as_str()).unwrap_or("");
                
                self.check_plugin_communication(plugin_name, target_plugin).await
            }
            _ => Ok(false),
        }
    }

    /// 检查文件系统访问权限
    async fn check_file_access(
        &self,
        plugin_name: &str,
        path: &str,
        write: bool,
    ) -> Result<bool, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(permissions) => {
                for permission in permissions {
                    if let PluginPermission::FileSystemAccess { allowed_paths, allow_write } = permission {
                        // 检查路径是否在允许列表中
                        for allowed_path in allowed_paths {
                            if path.starts_with(allowed_path) {
                                // 检查是否允许写入
                                if write && !allow_write {
                                    return Ok(false);
                                }
                                return Ok(true);
                            }
                        }
                    }
                }
                Ok(false)
            }
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }

    /// 检查网络访问权限
    async fn check_network_access(
        &self,
        plugin_name: &str,
        domain: &str,
        https: bool,
    ) -> Result<bool, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(permissions) => {
                for permission in permissions {
                    if let PluginPermission::NetworkAccess { allowed_domains, allow_https } = permission {
                        // 检查域名是否在允许列表中
                        for allowed_domain in allowed_domains {
                            if domain.ends_with(allowed_domain) {
                                // 检查是否允许HTTPS
                                if https && !allow_https {
                                    return Ok(false);
                                }
                                return Ok(true);
                            }
                        }
                    }
                }
                Ok(false)
            }
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }

    /// 检查系统访问权限
    async fn check_system_access(
        &self,
        plugin_name: &str,
        call: &str,
    ) -> Result<bool, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(permissions) => {
                for permission in permissions {
                    if let PluginPermission::SystemAccess { allowed_calls } = permission {
                        if allowed_calls.contains(call) {
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }

    /// 检查插件间通信权限
    async fn check_plugin_communication(
        &self,
        plugin_name: &str,
        target_plugin: &str,
    ) -> Result<bool, String> {
        let permissions_map = self.permissions.read().await;
        
        match permissions_map.get(plugin_name) {
            Some(permissions) => {
                for permission in permissions {
                    if let PluginPermission::PluginCommunication { allowed_plugins } = permission {
                        if allowed_plugins.contains(target_plugin) {
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            None => Err(format!("插件 {} 不存在", plugin_name)),
        }
    }
}
