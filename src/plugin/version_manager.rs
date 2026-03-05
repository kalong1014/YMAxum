// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件版本管理器
//! 用于管理插件的版本信息和自动更新

use crate::plugin::format::{PluginFormatHandler, PluginManifest};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// 插件版本信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginVersionInfo {
    /// 插件名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 下载URL
    pub download_url: String,
    /// 发布日期
    pub release_date: String,
    /// 发布说明
    pub release_notes: String,
    /// 兼容的核心版本
    pub compatible_core_versions: Vec<String>,
    /// 插件大小（字节）
    pub size: u64,
    /// 哈希值
    pub hash: String,
}

/// 插件版本管理器
#[derive(Debug, Clone)]
pub struct PluginVersionManager {
    /// 插件版本缓存
    version_cache: Arc<tokio::sync::RwLock<HashMap<String, Vec<PluginVersionInfo>>>>,
    /// 插件格式处理器
    format_handler: PluginFormatHandler,
    /// 插件存储路径
    plugin_store_path: PathBuf,
}

impl PluginVersionManager {
    /// 创建新的插件版本管理器
    pub fn new(plugin_store_path: &Path) -> Self {
        Self {
            version_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            format_handler: PluginFormatHandler::new(),
            plugin_store_path: plugin_store_path.to_path_buf(),
        }
    }

    /// 检查插件更新
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    /// * `current_version` - 当前版本
    ///
    /// # 返回
    /// * `Option<PluginVersionInfo>` - 可用的更新版本信息
    pub async fn check_update(
        &self,
        plugin_name: &str,
        current_version: &str,
    ) -> Option<PluginVersionInfo> {
        // 获取插件版本列表
        let versions = self.get_plugin_versions(plugin_name).await?;

        // 找到比当前版本新的版本
        for version_info in versions {
            if self.is_newer_version(&version_info.version, current_version) {
                return Some(version_info);
            }
        }

        None
    }

    /// 获取插件版本列表
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    ///
    /// # 返回
    /// * `Option<Vec<PluginVersionInfo>>` - 版本信息列表
    pub async fn get_plugin_versions(&self, plugin_name: &str) -> Option<Vec<PluginVersionInfo>> {
        // 检查缓存
        let cache = self.version_cache.read().await;
        if let Some(versions) = cache.get(plugin_name) {
            return Some(versions.clone());
        }
        drop(cache);

        // 从插件市场获取版本信息
        // 这里可以实现从插件市场API获取版本信息的逻辑
        // 现在返回模拟数据
        let versions = self.get_mock_versions(plugin_name);

        // 更新缓存
        let mut cache = self.version_cache.write().await;
        cache.insert(plugin_name.to_string(), versions.clone());

        Some(versions)
    }

    /// 更新插件
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    /// * `target_version` - 目标版本
    ///
    /// # 返回
    /// * `Result<(), String>` - 更新结果
    pub async fn update_plugin(
        &self,
        plugin_name: &str,
        target_version: &str,
    ) -> Result<(), String> {
        // 获取目标版本信息
        let versions = self
            .get_plugin_versions(plugin_name)
            .await
            .ok_or_else(|| format!("无法获取插件 {} 的版本信息", plugin_name))?;

        let target_version_info = versions
            .into_iter()
            .find(|v| v.version == target_version)
            .ok_or_else(|| format!("版本 {} 不存在", target_version))?;

        // 下载插件
        let download_path = self
            .download_plugin(&target_version_info)
            .await
            .map_err(|e| format!("下载插件失败: {}", e))?;

        // 安装插件
        self.install_plugin(&download_path, plugin_name)
            .await
            .map_err(|e| format!("安装插件失败: {}", e))?;

        info!("插件 {} 已成功更新到版本 {}", plugin_name, target_version);
        Ok(())
    }

    /// 下载插件
    ///
    /// # 参数
    /// * `version_info` - 版本信息
    ///
    /// # 返回
    /// * `Result<PathBuf, String>` - 下载路径
    async fn download_plugin(&self, version_info: &PluginVersionInfo) -> Result<PathBuf, String> {
        // 这里可以实现从下载URL获取插件的逻辑
        // 现在返回模拟路径
        let download_path = self.plugin_store_path.join(format!(
            "{}_{}.axpl",
            version_info.name, version_info.version
        ));
        info!(
            "模拟下载插件: {} 版本 {} 到 {:?}",
            version_info.name, version_info.version, download_path
        );
        Ok(download_path)
    }

    /// 安装插件
    ///
    /// # 参数
    /// * `plugin_path` - 插件路径
    /// * `plugin_name` - 插件名称
    ///
    /// # 返回
    /// * `Result<(), String>` - 安装结果
    async fn install_plugin(&self, plugin_path: &Path, plugin_name: &str) -> Result<(), String> {
        // 解析插件清单
        let manifest = self
            .format_handler
            .parse_manifest(plugin_path)
            .map_err(|e| format!("解析插件清单失败: {}", e))?;

        // 验证插件兼容性
        self.validate_plugin_compatibility(&manifest)?;

        // 安装插件到存储路径
        let plugin_dir = self.plugin_store_path.join(plugin_name);
        if plugin_dir.exists() {
            // 备份旧插件
            let backup_dir = self
                .plugin_store_path
                .join(format!("{}.backup", plugin_name));
            if backup_dir.exists() {
                std::fs::remove_dir_all(&backup_dir)
                    .map_err(|e| format!("删除旧备份失败: {}", e))?;
            }
            std::fs::rename(&plugin_dir, &backup_dir)
                .map_err(|e| format!("备份旧插件失败: {}", e))?;
        }

        // 提取新插件
        std::fs::create_dir_all(&plugin_dir).map_err(|e| format!("创建插件目录失败: {}", e))?;

        self.format_handler
            .extract_plugin(plugin_path, &plugin_dir)
            .map_err(|e| format!("提取插件失败: {}", e))?;

        Ok(())
    }

    /// 验证插件兼容性
    ///
    /// # 参数
    /// * `manifest` - 插件清单
    ///
    /// # 返回
    /// * `Result<(), String>` - 验证结果
    fn validate_plugin_compatibility(&self, _manifest: &PluginManifest) -> Result<(), String> {
        // 这里可以实现插件兼容性验证逻辑
        // 例如，检查核心版本兼容性
        Ok(())
    }

    /// 比较版本号
    ///
    /// # 参数
    /// * `version1` - 版本1
    /// * `version2` - 版本2
    ///
    /// # 返回
    /// * `bool` - version1是否比version2新
    fn is_newer_version(&self, version1: &str, version2: &str) -> bool {
        // 简单的版本比较逻辑
        // 实际应用中应该使用更复杂的版本比较库
        let v1_parts: Vec<&str> = version1.split('.').collect();
        let v2_parts: Vec<&str> = version2.split('.').collect();

        for (i, v1_part) in v1_parts.iter().enumerate() {
            if i >= v2_parts.len() {
                return true;
            }

            let v1_num: u32 = v1_part.parse().unwrap_or(0);
            let v2_num: u32 = v2_parts[i].parse().unwrap_or(0);

            if v1_num > v2_num {
                return true;
            } else if v1_num < v2_num {
                return false;
            }
        }

        v1_parts.len() > v2_parts.len()
    }

    /// 获取模拟版本数据
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    ///
    /// # 返回
    /// * `Vec<PluginVersionInfo>` - 模拟版本信息
    fn get_mock_versions(&self, plugin_name: &str) -> Vec<PluginVersionInfo> {
        vec![
            PluginVersionInfo {
                name: plugin_name.to_string(),
                version: "1.0.2".to_string(),
                download_url: format!(
                    "https://plugin-market.example.com/{}/1.0.2.axpl",
                    plugin_name
                ),
                release_date: "2026-01-20".to_string(),
                release_notes: "Bug fixes and performance improvements".to_string(),
                compatible_core_versions: vec!["1.0.0".to_string(), "1.0.1".to_string()],
                size: 1024000,
                hash: "abcdef1234567890".to_string(),
            },
            PluginVersionInfo {
                name: plugin_name.to_string(),
                version: "1.0.1".to_string(),
                download_url: format!(
                    "https://plugin-market.example.com/{}/1.0.1.axpl",
                    plugin_name
                ),
                release_date: "2026-01-15".to_string(),
                release_notes: "Initial release".to_string(),
                compatible_core_versions: vec!["1.0.0".to_string()],
                size: 1024000,
                hash: "0987654321fedcba".to_string(),
            },
        ]
    }

    /// 清理版本缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.version_cache.write().await;
        cache.clear();
        info!("插件版本缓存已清理");
    }

    /// 获取所有已安装插件的更新状态
    ///
    /// # 返回
    /// * `HashMap<String, Option<PluginVersionInfo>>` - 插件更新状态
    pub async fn get_all_update_status(&self) -> HashMap<String, Option<PluginVersionInfo>> {
        let mut update_status = HashMap::new();

        // 遍历已安装的插件
        if let Ok(entries) = std::fs::read_dir(&self.plugin_store_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let plugin_name = entry.file_name().to_string_lossy().to_string();

                        // 尝试读取插件清单
                        let manifest_path = path.join("manifest.json");
                        if manifest_path.exists()
                            && let Ok(manifest) = self.format_handler.parse_manifest(&manifest_path)
                        {
                            // 检查更新
                            if let Some(update) =
                                self.check_update(&plugin_name, &manifest.version).await
                            {
                                update_status.insert(plugin_name, Some(update));
                            } else {
                                update_status.insert(plugin_name, None);
                            }
                        }
                    }
                }
            }
        }

        update_status
    }
}

