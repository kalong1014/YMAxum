// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::bus::{PluginBus, PluginMessage};
use super::cross_language::{CrossLanguageRuntime, PluginLanguage};
use super::format::PluginManifest;
use super::market::PluginMarketplace;
use super::sign::{PluginSigner, SignatureInfo};
use super::version_manager::PluginVersionManager;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use tokio::task;
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub status: PluginStatus,
    pub manifest: Option<PluginManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginStatus {
    Installed,
    Enabled,
    Disabled,
    Uninstalled,
}

#[derive(Debug, Clone)]
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, PluginInfo>>>,
    permission_manager: Arc<PluginPermissionManager>,
    version_manager: Arc<PluginVersionManager>,
    cross_language_runtime: Arc<CrossLanguageRuntime>,
    plugin_cache: Arc<RwLock<HashMap<String, CachedPlugin>>>,
    plugin_bus: Arc<PluginBus>,
    marketplace: Arc<PluginMarketplace>,
}

#[derive(Debug, Clone)]
pub struct CachedPlugin {
    pub info: PluginInfo,
    pub loaded_at: Instant,
    pub last_used: Instant,
    pub manifest: Option<PluginManifest>,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        let plugins_dir = Path::new("..").join("plugins");

        std::fs::create_dir_all(&plugins_dir).context(format!(
            "Failed to create plugins directory: {:?}",
            plugins_dir
        ))?;

        let version_manager = Arc::new(PluginVersionManager::new(&plugins_dir));
        let cross_language_runtime = Arc::new(CrossLanguageRuntime::new()?);
        let plugin_bus = Arc::new(PluginBus::new());
        let marketplace = Arc::new(PluginMarketplace::new());

        Ok(Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            permission_manager: Arc::new(PluginPermissionManager::new()),
            version_manager,
            cross_language_runtime,
            plugin_cache: Arc::new(RwLock::new(HashMap::new())),
            plugin_bus,
            marketplace,
        })
    }

    pub async fn install_plugin(&self, plugin_path: &str) -> Result<PluginInfo> {
        let path = Path::new(plugin_path);

        // 验证插件签名
        self.verify_plugin_signature(path)?;

        // 检测插件语言
        let language = self.cross_language_runtime.detect_language(path);

        // 初始化语言运行时
        self.cross_language_runtime
            .initialize_runtime(language.clone())
            .await?;

        // 启动跨语言插件
        if language != PluginLanguage::Rust {
            self.cross_language_runtime.start_plugin(path).await?;
        }

        // 读取插件清单
        let manifest = self.load_plugin_manifest(path).await?;

        // 提取插件名称和版本
        let plugin_name = manifest.name.clone();
        let plugin_version = manifest.version.clone();

        let plugin_info = PluginInfo {
            name: plugin_name.clone(),
            version: plugin_version,
            status: PluginStatus::Installed,
            manifest: Some(manifest),
        };

        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin_name.clone(), plugin_info.clone());

        // 注册插件权限配置
        self.permission_manager.register_plugin(&plugin_name).await;

        // 添加到插件缓存
        let cached_plugin = CachedPlugin {
            info: plugin_info.clone(),
            loaded_at: Instant::now(),
            last_used: Instant::now(),
            manifest: Some(plugin_info.manifest.clone().unwrap()),
        };
        let mut cache = self.plugin_cache.write().await;
        cache.insert(plugin_name.clone(), cached_plugin);

        // 注册插件到通信总线
        self.plugin_bus.register_plugin(&plugin_name).await;

        Ok(plugin_info)
    }

    /// 加载插件清单
    async fn load_plugin_manifest(&self, plugin_path: &Path) -> Result<PluginManifest> {
        // 尝试从插件目录读取manifest.json
        let manifest_path = plugin_path.join("manifest.json");
        if manifest_path.exists() {
            let manifest_content = fs::read_to_string(&manifest_path)
                .context(format!("读取插件清单失败: {:?}", manifest_path))?;
            let manifest: PluginManifest = serde_json::from_str(&manifest_content)
                .context("解析插件清单失败")?;
            Ok(manifest)
        } else {
            // 如果没有manifest.json，创建默认清单
            Ok(PluginManifest {
                name: plugin_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string(),
                version: "1.0.0".to_string(),
                author: "Unknown".to_string(),
                description: "Default plugin".to_string(),
                plugin_type: "basic".to_string(),
                dependencies: Vec::new(),
                core_version: "1.0.0".to_string(),
                entry_file: "main.rs".to_string(),
                config_file: None,
                signature_file: "signature.json".to_string(),
                license: "MIT".to_string(),
                routes: None,
            })
        }
    }

    /// 验证插件签名
    fn verify_plugin_signature(&self, plugin_path: &Path) -> Result<()> {
        // 检查签名文件是否存在
        let signature_path = plugin_path.join("signature.json");
        if !signature_path.exists() {
            return Err(anyhow::anyhow!("签名文件不存在: {:?}", signature_path));
        }

        // 加载签名信息
        let signature_content = fs::read_to_string(&signature_path)
            .context(format!("读取签名文件失败: {:?}", signature_path))?;
        let signature_info: SignatureInfo =
            serde_json::from_str(&signature_content).context("解析签名信息失败")?;

        // 验证签名
        let mut signer = PluginSigner::new();
        
        // 加载默认公钥（实际项目中应该从配置或内置存储中加载）
        let public_key_path = Path::new("data").join("plugins").join("public_key.pem");
        if !public_key_path.exists() {
            // 如果公钥文件不存在，创建默认公钥目录
            std::fs::create_dir_all(public_key_path.parent().unwrap())
                .context("创建公钥目录失败")?;
            // 这里应该生成或复制默认公钥，现在暂时返回错误
            return Err(anyhow::anyhow!("公钥文件不存在，请配置公钥"));
        }
        
        signer
            .load_public_key(public_key_path)
            .map_err(|e| anyhow::anyhow!("加载公钥失败: {}", e))?;

        // 加载可信白名单
        signer.get_whitelist().load()
            .map_err(|e| anyhow::anyhow!("加载可信白名单失败: {}", e))?;

        let verified = signer
            .verify_signature(plugin_path, &signature_info)
            .map_err(|e| anyhow::anyhow!("验证签名失败: {}", e))?;

        if !verified {
            return Err(anyhow::anyhow!("签名验证失败，插件可能被篡改"));
        }

        info!("插件签名验证成功: {:?}", plugin_path);
        Ok(())
    }

    pub async fn enable_plugin(&self, plugin_name: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.status = PluginStatus::Enabled;
            // 启动插件
            self.start_plugin(plugin_name).await?;
        }

        // 更新缓存中的插件状态
        let mut cache = self.plugin_cache.write().await;
        if let Some(cached_plugin) = cache.get_mut(plugin_name) {
            cached_plugin.info.status = PluginStatus::Enabled;
            cached_plugin.last_used = Instant::now();
        }

        Ok(())
    }

    pub async fn disable_plugin(&self, plugin_name: &str) -> Result<()> {
        // 停止插件
        self.stop_plugin(plugin_name).await?;
        
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.status = PluginStatus::Disabled;
        }

        // 更新缓存中的插件状态
        let mut cache = self.plugin_cache.write().await;
        if let Some(cached_plugin) = cache.get_mut(plugin_name) {
            cached_plugin.info.status = PluginStatus::Disabled;
            cached_plugin.last_used = Instant::now();
        }

        Ok(())
    }

    /// 启动插件
    pub async fn start_plugin(&self, plugin_name: &str) -> Result<()> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            if plugin.status != PluginStatus::Installed && plugin.status != PluginStatus::Enabled {
                return Err(anyhow::anyhow!("插件 {} 状态不正确，无法启动", plugin_name));
            }
        } else {
            return Err(anyhow::anyhow!("插件 {} 不存在", plugin_name));
        }
        drop(plugins);

        // 启动插件逻辑
        // 1. 加载插件代码
        // 2. 初始化插件
        // 3. 注册插件路由
        // 4. 启动插件进程
        
        // 模拟插件启动
        info!("启动插件: {}", plugin_name);
        
        // 更新插件状态
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.status = PluginStatus::Enabled;
        }

        // 更新缓存
        let mut cache = self.plugin_cache.write().await;
        if let Some(cached_plugin) = cache.get_mut(plugin_name) {
            cached_plugin.info.status = PluginStatus::Enabled;
            cached_plugin.last_used = Instant::now();
        }

        Ok(())
    }

    /// 停止插件
    pub async fn stop_plugin(&self, plugin_name: &str) -> Result<()> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            if plugin.status != PluginStatus::Enabled {
                return Err(anyhow::anyhow!("插件 {} 未启用，无法停止", plugin_name));
            }
        } else {
            return Err(anyhow::anyhow!("插件 {} 不存在", plugin_name));
        }
        drop(plugins);

        // 停止插件逻辑
        // 1. 停止插件进程
        // 2. 清理插件资源
        // 3. 注销插件路由
        
        // 模拟插件停止
        info!("停止插件: {}", plugin_name);
        
        // 更新插件状态
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.status = PluginStatus::Installed;
        }

        // 更新缓存
        let mut cache = self.plugin_cache.write().await;
        if let Some(cached_plugin) = cache.get_mut(plugin_name) {
            cached_plugin.info.status = PluginStatus::Installed;
            cached_plugin.last_used = Instant::now();
        }

        Ok(())
    }

    pub async fn uninstall_plugin(&self, plugin_name: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        plugins.remove(plugin_name);

        // 移除插件权限配置
        self.permission_manager.remove_plugin(plugin_name).await;

        // 从缓存中移除插件
        let mut cache = self.plugin_cache.write().await;
        cache.remove(plugin_name);

        // 从通信总线中注销插件
        self.plugin_bus.unregister_plugin(plugin_name).await;

        Ok(())
    }

    pub async fn update_plugin(&self, plugin_name: &str) -> Result<()> {
        let plugins = self.plugins.read().await;
        let current_version = if let Some(plugin) = plugins.get(plugin_name) {
            plugin.version.clone()
        } else {
            return Err(anyhow::anyhow!("插件 {} 不存在", plugin_name));
        };
        drop(plugins);

        // 检查更新
        if let Some(update_info) = self
            .version_manager
            .check_update(plugin_name, &current_version)
            .await
        {
            info!("发现插件 {} 的更新版本: {}", plugin_name, update_info.version);
            
            // 执行更新
            self.version_manager
                .update_plugin(plugin_name, &update_info.version)
                .await
                .map_err(|e| anyhow::anyhow!("更新插件失败: {}", e))?;

            // 克隆版本信息以避免移动
            let version = update_info.version.clone();
            
            // 更新插件信息
            let mut plugins = self.plugins.write().await;
            if let Some(plugin) = plugins.get_mut(plugin_name) {
                plugin.version = version.clone();
                info!("插件 {} 已更新到版本: {}", plugin_name, version);
            }

            // 更新缓存中的插件信息
            let mut cache = self.plugin_cache.write().await;
            if let Some(cached_plugin) = cache.get_mut(plugin_name) {
                cached_plugin.info.version = version;
                cached_plugin.last_used = Instant::now();
            }

            Ok(())
        } else {
            Err(anyhow::anyhow!("没有可用的更新版本"))
        }
    }

    /// 检查插件更新
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    ///
    /// # 返回
    /// * `Result<Option<super::version_manager::PluginVersionInfo>, anyhow::Error>` - 可用的更新版本信息
    pub async fn check_plugin_update(
        &self,
        plugin_name: &str,
    ) -> Result<Option<super::version_manager::PluginVersionInfo>> {
        let plugins = self.plugins.read().await;
        let current_version = if let Some(plugin) = plugins.get(plugin_name) {
            plugin.version.clone()
        } else {
            return Err(anyhow::anyhow!("插件 {} 不存在", plugin_name));
        };
        drop(plugins);

        Ok(self
            .version_manager
            .check_update(plugin_name, &current_version)
            .await)
    }

    /// 获取所有插件的更新状态
    ///
    /// # 返回
    /// * `std::collections::HashMap<String, Option<super::version_manager::PluginVersionInfo>>` - 插件更新状态
    pub async fn get_all_update_status(
        &self,
    ) -> std::collections::HashMap<String, Option<super::version_manager::PluginVersionInfo>> {
        self.version_manager.get_all_update_status().await
    }

    /// 获取版本管理器
    ///
    /// # 返回
    /// * `Arc<PluginVersionManager>` - 版本管理器
    pub fn get_version_manager(&self) -> Arc<PluginVersionManager> {
        self.version_manager.clone()
    }

    pub async fn get_all_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }

    pub async fn get_plugins_by_status(&self, status: PluginStatus) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|p| p.status == status)
            .cloned()
            .collect()
    }

    /// 授予插件权限
    pub async fn grant_permission(
        &self,
        plugin_name: &str,
        permission: super::permission::PluginPermission,
    ) {
        self.permission_manager
            .grant_permission(plugin_name, permission)
            .await;
    }

    /// 拒绝插件权限
    pub async fn deny_permission(
        &self,
        plugin_name: &str,
        permission: super::permission::PluginPermission,
    ) {
        self.permission_manager
            .deny_permission(plugin_name, permission)
            .await;
    }

    /// 检查插件是否有指定权限
    pub async fn check_permission(
        &self,
        plugin_name: &str,
        permission: &super::permission::PluginPermission,
    ) -> bool {
        self.permission_manager
            .check_permission(plugin_name, permission)
            .await
    }

    /// 获取插件的所有有效权限
    pub async fn get_effective_permissions(
        &self,
        plugin_name: &str,
    ) -> std::collections::HashSet<super::permission::PluginPermission> {
        self.permission_manager
            .get_effective_permissions(plugin_name)
            .await
    }

    /// 重置插件权限
    pub async fn reset_permissions(&self, plugin_name: &str) {
        self.permission_manager.reset_permissions(plugin_name).await;
    }

    /// 获取权限管理器
    pub fn get_permission_manager(&self) -> Arc<super::permission::PluginPermissionManager> {
        self.permission_manager.clone()
    }

    /// 调用跨语言插件方法
    pub async fn call_cross_language_method(
        &self,
        plugin_name: &str,
        method_name: &str,
        params: serde_json::Value,
        language: PluginLanguage,
    ) -> Result<serde_json::Value> {
        self.cross_language_runtime
            .call_method(plugin_name, method_name, params, language)
            .await
    }

    /// 订阅跨语言插件事件
    pub async fn subscribe_cross_language_event(
        &self,
        plugin_name: &str,
        event_name: &str,
        callback: Box<dyn Fn(serde_json::Value) + Send + Sync>,
        language: PluginLanguage,
    ) -> Result<()> {
        self.cross_language_runtime
            .subscribe_event(plugin_name, event_name, callback, language)
            .await
    }

    /// 获取跨语言运行时
    pub fn get_cross_language_runtime(&self) -> Arc<CrossLanguageRuntime> {
        self.cross_language_runtime.clone()
    }

    /// 获取插件的语言类型
    pub fn get_plugin_language(&self, plugin_path: &Path) -> PluginLanguage {
        self.cross_language_runtime.detect_language(plugin_path)
    }

    /// 从缓存中获取插件信息
    pub async fn get_cached_plugin(&self, plugin_name: &str) -> Option<PluginInfo> {
        let mut cache = self.plugin_cache.write().await;
        if let Some(cached_plugin) = cache.get_mut(plugin_name) {
            // 更新最后使用时间
            cached_plugin.last_used = Instant::now();
            Some(cached_plugin.info.clone())
        } else {
            None
        }
    }

    /// 清理过期的插件缓存
    pub async fn cleanup_cache(&self, max_age: Duration) {
        let mut cache = self.plugin_cache.write().await;
        let now = Instant::now();
        let before_count = cache.len();
        
        cache.retain(|_, cached_plugin| {
            now.duration_since(cached_plugin.last_used) < max_age
        });
        
        let after_count = cache.len();
        if before_count > after_count {
            info!("清理了 {} 个过期的插件缓存", before_count - after_count);
        }
    }

    /// 预加载常用插件
    pub async fn preload_plugins(&self, plugin_names: &[&str]) {
        for plugin_name in plugin_names {
            if let Some(plugin) = self.get_cached_plugin(plugin_name).await {
                info!("预加载插件: {} (版本: {})", plugin_name, plugin.version);
            }
        }
    }

    /// 批量获取插件信息
    pub async fn get_plugins_batch(&self, plugin_names: &[&str]) -> Vec<Option<PluginInfo>> {
        let plugins = self.plugins.read().await;
        plugin_names
            .iter()
            .map(|name| plugins.get(*name).cloned())
            .collect()
    }

    /// 并行安装多个插件
    pub async fn install_plugins_parallel(&self, plugin_paths: &[&str]) -> Result<Vec<Result<PluginInfo>>> {
        let tasks: Vec<_> = plugin_paths.iter().map(|path| {
            let manager = self.clone();
            let path = path.to_string(); // 复制路径到新的字符串，避免生命周期问题
            task::spawn(async move {
                manager.install_plugin(&path).await
            })
        }).collect();

        let results = futures::future::join_all(tasks).await;
        Ok(results.into_iter().map(|result| result.unwrap_or_else(|e| Err(anyhow::anyhow!("Task failed: {:?}", e)))).collect())
    }

    /// 获取插件通信总线
    pub fn get_plugin_bus(&self) -> Arc<PluginBus> {
        self.plugin_bus.clone()
    }

    /// 发送消息给指定插件
    pub async fn send_message(&self, message: PluginMessage) -> Result<(), String> {
        self.plugin_bus.send_message(message).await
    }

    /// 广播消息给所有插件
    pub async fn broadcast_message(&self, message_type: &str, data: serde_json::Value, sender: &str) -> Result<(), String> {
        self.plugin_bus.broadcast(message_type, data, sender).await
    }

    /// 发送消息给特定插件
    pub async fn send_message_to(&self, message_type: &str, data: serde_json::Value, sender: &str, target: &str) -> Result<(), String> {
        self.plugin_bus.send_to(message_type, data, sender, target).await
    }

    /// 订阅指定类型的消息
    pub async fn subscribe_to_messages(&self, plugin_name: &str, message_type: &str) -> Result<tokio::sync::broadcast::Receiver<PluginMessage>, String> {
        self.plugin_bus.subscribe(plugin_name, message_type).await
    }

    /// 从插件市场搜索插件
    pub async fn search_marketplace(&self, query: &str, category: Option<super::market::PluginCategory>, page: u32, page_size: u32) -> Result<super::market::MarketplaceSearchResult> {
        Ok(self.marketplace.search_plugins(query, category, page, page_size).await)
    }

    /// 从插件市场下载插件
    pub async fn download_from_marketplace(&self, plugin_name: &str) -> Result<Vec<u8>> {
        let result: Result<Vec<u8>, String> = self.marketplace.download_plugin(plugin_name).await;
        result.map_err(|e| anyhow::anyhow!("下载插件失败: {}", e))
    }

    /// 从插件市场安装插件
    pub async fn install_from_marketplace(&self, plugin_name: &str) -> Result<PluginInfo> {
        // 从市场下载插件
        let plugin_data = self.download_from_marketplace(plugin_name).await?;
        
        // 保存插件文件
        let plugins_dir = Path::new("..").join("plugins");
        let plugin_path = plugins_dir.join(format!("{}.axpl", plugin_name));
        
        std::fs::write(&plugin_path, plugin_data)
            .context(format!("保存插件文件失败: {:?}", plugin_path))?;
        
        // 安装插件
        self.install_plugin(plugin_path.to_str().unwrap_or(""))
            .await
    }

    /// 检查插件更新
    pub async fn check_marketplace_updates(&self) -> Result<std::collections::HashMap<String, Option<super::version_manager::PluginVersionInfo>>> {
        Ok(self.version_manager.get_all_update_status().await)
    }

    /// 从市场更新插件
    pub async fn update_from_marketplace(&self, plugin_name: &str) -> Result<()> {
        self.update_plugin(plugin_name).await
    }

    /// 对插件进行评分
    pub async fn rate_plugin(&self, plugin_name: &str, rating: f32, comment: Option<String>, user_name: &str) -> Result<()> {
        let plugin_rating = super::market::PluginRating {
            plugin_name: plugin_name.to_string(),
            rating,
            comment,
            user_name: user_name.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            upvotes: 0,
            downvotes: 0,
            replies: Vec::new(),
        };
        
        let result: Result<(), String> = self.marketplace.rate_plugin(plugin_rating).await;
        result.map_err(|e| anyhow::anyhow!("评分失败: {}", e))
    }

    /// 获取插件市场统计信息
    pub async fn get_marketplace_stats(&self) -> super::market::MarketplaceStats {
        self.marketplace.get_stats().await
    }

    /// 获取推荐插件
    pub async fn get_featured_plugins(&self, limit: u32) -> Vec<super::market::MarketplacePlugin> {
        self.marketplace.get_top_rated_plugins(limit).await
    }

    /// 获取热门插件
    pub async fn get_trending_plugins(&self, limit: u32) -> Vec<super::market::MarketplacePlugin> {
        self.marketplace.get_trending_plugins(limit).await
    }

    /// 获取最近更新的插件
    pub async fn get_recently_updated_plugins(&self, limit: u32) -> Vec<super::market::MarketplacePlugin> {
        self.marketplace.get_recently_updated_plugins(limit).await
    }
}

// 为了方便使用，重新导出权限相关的类型
pub use super::permission::{
    PluginPermission, PluginPermissionChecker, PluginPermissionConfig, PluginPermissionManager,
};

