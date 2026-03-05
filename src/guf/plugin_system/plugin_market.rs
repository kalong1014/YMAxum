//! 插件市场模块
//! 负责插件的搜索、安装、卸载和更新

use super::{GufPlugin, GufPluginConfig};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// 插件版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVersion {
    /// 版本号
    pub version: String,
    /// 发布时间
    pub release_date: chrono::DateTime<chrono::Utc>,
    /// 变更日志
    pub changelog: String,
    /// 下载链接
    pub download_url: String,
    /// 兼容性信息
    pub compatibility: Vec<String>,
}

/// 插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// 插件ID
    pub id: String,
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件描述
    pub description: String,
    /// 插件作者
    pub author: String,
    /// 插件类型
    pub r#type: String,
    /// 插件语言
    pub language: String,
    /// 插件平台
    pub platform: Vec<String>,
    /// 插件依赖
    pub dependencies: Vec<PluginDependency>,
    /// 下载次数
    pub downloads: u64,
    /// 评分
    pub rating: f64,
    /// 评分数量
    pub rating_count: u64,
    /// 最后更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
    /// 下载链接
    pub download_url: String,
    /// 分类
    pub categories: Vec<String>,
    /// 标签
    pub tags: Vec<String>,
    /// 版本历史
    pub version_history: Vec<PluginVersion>,
    /// 截图
    pub screenshots: Vec<String>,
    /// 文档链接
    pub documentation_url: Option<String>,
    /// 支持链接
    pub support_url: Option<String>,
    /// 源码链接
    pub source_url: Option<String>,
    /// 许可协议
    pub license: String,
    /// 是否官方认证
    pub is_official: bool,
    /// 是否推荐
    pub is_featured: bool,
}

/// 插件依赖
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// 依赖名称
    pub name: String,
    /// 依赖版本
    pub version: String,
    /// 依赖类型
    pub r#type: String,
}

/// 插件市场
#[derive(Debug, Clone)]
pub struct GufPluginMarket {
    /// 插件存储
    plugins: Arc<tokio::sync::RwLock<std::collections::HashMap<String, PluginInfo>>>,
    /// 已安装的插件
    installed_plugins: Arc<tokio::sync::RwLock<std::collections::HashMap<String, GufPlugin>>>,
    /// 市场URL
    _market_url: String,
    /// 分类管理
    categories: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<String>>>>,
    /// 标签管理
    tags: Arc<tokio::sync::RwLock<std::collections::HashMap<String, u64>>>,
    /// 插件评分
    ratings: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<f64>>>>,
    /// 下载统计
    _download_stats: Arc<Mutex<std::collections::HashMap<String, u64>>>,
}

impl GufPluginMarket {
    /// 创建新的插件市场
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            installed_plugins: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            _market_url: "https://guf-plugin-market.example.com".to_string(),
            categories: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            tags: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            ratings: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            _download_stats: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// 初始化插件市场
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件市场");
        // 模拟加载市场插件
        self.load_market_plugins().await
    }

    /// 加载市场插件
    async fn load_market_plugins(&self) -> Result<(), String> {
        info!("加载市场插件");

        // 模拟市场插件数据
        let market_plugins = vec![
            PluginInfo {
                id: "1".to_string(),
                name: "auth-plugin".to_string(),
                version: "1.2.0".to_string(),
                description: "认证插件，提供用户登录和权限管理功能".to_string(),
                author: "GUF Team".to_string(),
                r#type: "auth".to_string(),
                language: "rust".to_string(),
                platform: vec![
                    "windows".to_string(),
                    "linux".to_string(),
                    "macos".to_string(),
                ],
                dependencies: vec![],
                downloads: 1234,
                rating: 4.8,
                rating_count: 120,
                last_updated: chrono::Utc::now() - chrono::Duration::days(7),
                download_url: "https://example.com/plugins/auth-plugin-1.2.0.zip".to_string(),
                categories: vec!["authentication".to_string(), "security".to_string()],
                tags: vec!["login".to_string(), "permissions".to_string(), "oauth".to_string()],
                version_history: vec![
                    PluginVersion {
                        version: "1.2.0".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(7),
                        changelog: "添加OAuth 2.0支持，修复登录问题".to_string(),
                        download_url: "https://example.com/plugins/auth-plugin-1.2.0.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                    PluginVersion {
                        version: "1.1.0".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(30),
                        changelog: "添加多因素认证，优化性能".to_string(),
                        download_url: "https://example.com/plugins/auth-plugin-1.1.0.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                ],
                screenshots: vec![
                    "https://example.com/plugins/auth-plugin/screenshot1.png".to_string(),
                    "https://example.com/plugins/auth-plugin/screenshot2.png".to_string(),
                ],
                documentation_url: Some("https://example.com/plugins/auth-plugin/docs".to_string()),
                support_url: Some("https://example.com/plugins/auth-plugin/support".to_string()),
                source_url: Some("https://github.com/gufteam/auth-plugin".to_string()),
                license: "MIT".to_string(),
                is_official: true,
                is_featured: true,
            },
            PluginInfo {
                id: "2".to_string(),
                name: "analytics-plugin".to_string(),
                version: "2.0.1".to_string(),
                description: "分析插件，提供用户行为分析和数据统计功能".to_string(),
                author: "Analytics Team".to_string(),
                r#type: "analytics".to_string(),
                language: "javascript".to_string(),
                platform: vec![
                    "windows".to_string(),
                    "linux".to_string(),
                    "macos".to_string(),
                ],
                dependencies: vec![],
                downloads: 892,
                rating: 4.5,
                rating_count: 85,
                last_updated: chrono::Utc::now() - chrono::Duration::days(3),
                download_url: "https://example.com/plugins/analytics-plugin-2.0.1.zip".to_string(),
                categories: vec!["analytics".to_string(), "tracking".to_string()],
                tags: vec!["analytics".to_string(), "statistics".to_string(), "dashboard".to_string()],
                version_history: vec![
                    PluginVersion {
                        version: "2.0.1".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(3),
                        changelog: "修复数据统计错误，优化图表显示".to_string(),
                        download_url: "https://example.com/plugins/analytics-plugin-2.0.1.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                    PluginVersion {
                        version: "2.0.0".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(14),
                        changelog: "全新UI设计，添加实时数据监控".to_string(),
                        download_url: "https://example.com/plugins/analytics-plugin-2.0.0.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                ],
                screenshots: vec![
                    "https://example.com/plugins/analytics-plugin/screenshot1.png".to_string(),
                    "https://example.com/plugins/analytics-plugin/screenshot2.png".to_string(),
                ],
                documentation_url: Some("https://example.com/plugins/analytics-plugin/docs".to_string()),
                support_url: Some("https://example.com/plugins/analytics-plugin/support".to_string()),
                source_url: Some("https://github.com/analyticsteam/analytics-plugin".to_string()),
                license: "Apache 2.0".to_string(),
                is_official: false,
                is_featured: true,
            },
            PluginInfo {
                id: "3".to_string(),
                name: "payment-plugin".to_string(),
                version: "1.5.3".to_string(),
                description: "支付插件，提供多种支付方式集成".to_string(),
                author: "Payment Team".to_string(),
                r#type: "payment".to_string(),
                language: "python".to_string(),
                platform: vec!["windows".to_string(), "linux".to_string()],
                dependencies: vec![],
                downloads: 567,
                rating: 4.7,
                rating_count: 60,
                last_updated: chrono::Utc::now() - chrono::Duration::days(14),
                download_url: "https://example.com/plugins/payment-plugin-1.5.3.zip".to_string(),
                categories: vec!["payment".to_string(), "e-commerce".to_string()],
                tags: vec!["payment".to_string(), "stripe".to_string(), "paypal".to_string()],
                version_history: vec![
                    PluginVersion {
                        version: "1.5.3".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(14),
                        changelog: "添加PayPal支持，修复支付流程问题".to_string(),
                        download_url: "https://example.com/plugins/payment-plugin-1.5.3.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                    PluginVersion {
                        version: "1.5.2".to_string(),
                        release_date: chrono::Utc::now() - chrono::Duration::days(30),
                        changelog: "添加Stripe支持，优化支付体验".to_string(),
                        download_url: "https://example.com/plugins/payment-plugin-1.5.2.zip".to_string(),
                        compatibility: vec!["ymaxum>=1.0.0".to_string()],
                    },
                ],
                screenshots: vec![
                    "https://example.com/plugins/payment-plugin/screenshot1.png".to_string(),
                    "https://example.com/plugins/payment-plugin/screenshot2.png".to_string(),
                ],
                documentation_url: Some("https://example.com/plugins/payment-plugin/docs".to_string()),
                support_url: Some("https://example.com/plugins/payment-plugin/support".to_string()),
                source_url: Some("https://github.com/paymentteam/payment-plugin".to_string()),
                license: "MIT".to_string(),
                is_official: false,
                is_featured: false,
            },
        ];

        // 添加插件到市场
        let mut plugins = self.plugins.write().await;
        let mut categories = self.categories.write().await;
        let mut tags = self.tags.write().await;
        
        for plugin in market_plugins {
            plugins.insert(plugin.id.clone(), plugin.clone());
            
            // 更新分类
            for category in &plugin.categories {
                let plugins_in_category = categories.entry(category.clone()).or_insert_with(Vec::new);
                if !plugins_in_category.contains(&plugin.id) {
                    plugins_in_category.push(plugin.id.clone());
                }
            }
            
            // 更新标签
            for tag in &plugin.tags {
                *tags.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        info!("市场插件加载完成，共 {} 个插件", plugins.len());
        Ok(())
    }

    /// 搜索插件
    pub async fn search_plugins(&self, query: &str) -> Result<Vec<PluginInfo>, String> {
        info!("搜索插件: {}", query);

        let plugins = self.plugins.read().await;
        let mut results = vec![];

        for plugin in plugins.values() {
            if plugin.name.contains(query)
                || plugin.description.contains(query)
                || plugin.author.contains(query)
            {
                results.push(plugin.clone());
            }
        }

        info!("搜索完成，找到 {} 个插件", results.len());
        Ok(results)
    }

    /// 安装插件
    pub async fn install_plugin(&self, plugin_id: &str) -> Result<GufPlugin, String> {
        info!("安装插件: {}", plugin_id);

        // 检查插件是否存在
        let plugins = self.plugins.read().await;
        let plugin_info = plugins
            .get(plugin_id)
            .ok_or_else(|| format!("插件不存在: {}", plugin_id))?;

        // 检查插件是否已安装
        let installed_plugins = self.installed_plugins.read().await;
        if installed_plugins.contains_key(&plugin_info.name) {
            return Err(format!("插件已经安装: {}", plugin_info.name));
        }
        drop(installed_plugins);

        // 模拟插件下载和安装过程
        info!("下载插件: {}", plugin_info.download_url);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // 创建插件配置
        let plugin_config = GufPluginConfig {
            name: plugin_info.name.clone(),
            version: plugin_info.version.clone(),
            description: plugin_info.description.clone(),
            author: plugin_info.author.clone(),
            r#type: plugin_info.r#type.clone(),
            language: plugin_info.language.clone(),
            platform: plugin_info.platform.clone(),
            dependencies: plugin_info
                .dependencies
                .iter()
                .map(|d| super::PluginDependency {
                    name: d.name.clone(),
                    version: d.version.clone(),
                    r#type: d.r#type.clone(),
                })
                .collect(),
            config: serde_json::json!({}),
        };

        // 创建插件
        let plugin = GufPlugin {
            config: plugin_config,
            status: super::PluginStatus::Ready,
            path: format!("plugins/{}", plugin_info.name),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };

        // 添加到已安装插件
        let mut installed_plugins = self.installed_plugins.write().await;
        installed_plugins.insert(plugin.config.name.clone(), plugin.clone());

        info!("插件安装完成: {}", plugin.config.name);
        Ok(plugin)
    }

    /// 卸载插件
    pub async fn uninstall_plugin(&self, plugin_name: &str) -> Result<(), String> {
        info!("卸载插件: {}", plugin_name);

        // 检查插件是否已安装
        let mut installed_plugins = self.installed_plugins.write().await;
        if installed_plugins.remove(plugin_name).is_none() {
            return Err(format!("插件未安装: {}", plugin_name));
        }

        // 模拟插件卸载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        info!("插件卸载完成: {}", plugin_name);
        Ok(())
    }

    /// 更新插件
    pub async fn update_plugin(&self, plugin_name: &str) -> Result<GufPlugin, String> {
        info!("更新插件: {}", plugin_name);

        // 检查插件是否已安装
        let installed_plugin = {
            let installed_plugins = self.installed_plugins.read().await;
            installed_plugins
                .get(plugin_name)
                .ok_or_else(|| format!("插件未安装: {}", plugin_name))?
                .clone()
        };

        // 查找插件在市场中的信息
        let market_plugin = {
            let plugins = self.plugins.read().await;
            plugins
                .values()
                .find(|p| p.name == plugin_name)
                .ok_or_else(|| format!("插件在市场中不存在: {}", plugin_name))?
                .clone()
        };

        // 检查是否有新版本
        if market_plugin.version == installed_plugin.config.version {
            return Err(format!("插件已经是最新版本: {}", plugin_name));
        }

        // 模拟插件更新过程
        info!("下载插件更新: {}", market_plugin.download_url);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // 创建更新后的插件配置
        let plugin_config = GufPluginConfig {
            name: market_plugin.name.clone(),
            version: market_plugin.version.clone(),
            description: market_plugin.description.clone(),
            author: market_plugin.author.clone(),
            r#type: market_plugin.r#type.clone(),
            language: market_plugin.language.clone(),
            platform: market_plugin.platform.clone(),
            dependencies: market_plugin
                .dependencies
                .iter()
                .map(|d| super::PluginDependency {
                    name: d.name.clone(),
                    version: d.version.clone(),
                    r#type: d.r#type.clone(),
                })
                .collect(),
            config: serde_json::json!({}),
        };

        // 创建更新后的插件
        let updated_plugin = GufPlugin {
            config: plugin_config,
            status: super::PluginStatus::Ready,
            path: installed_plugin.path.clone(),
            loaded_at: chrono::Utc::now(),
            started_at: None,
        };

        // 更新已安装插件
        let mut installed_plugins = self.installed_plugins.write().await;
        installed_plugins.insert(plugin_name.to_string(), updated_plugin.clone());

        info!(
            "插件更新完成: {} 版本: {}",
            plugin_name, market_plugin.version
        );
        Ok(updated_plugin)
    }

    /// 列出已安装的插件
    pub async fn list_installed_plugins(&self) -> Result<Vec<GufPlugin>, String> {
        let installed_plugins = self.installed_plugins.read().await;
        Ok(installed_plugins.values().cloned().collect())
    }

    /// 检查插件更新
    pub async fn check_for_updates(&self, plugin_name: &str) -> Result<Option<PluginInfo>, String> {
        info!("检查插件更新: {}", plugin_name);

        // 检查插件是否已安装
        let installed_plugin = {
            let installed_plugins = self.installed_plugins.read().await;
            installed_plugins
                .get(plugin_name)
                .ok_or_else(|| format!("插件未安装: {}", plugin_name))?
                .clone()
        };

        // 查找插件在市场中的信息
        let market_plugin = {
            let plugins = self.plugins.read().await;
            plugins.values().find(|p| p.name == plugin_name).cloned()
        };

        if let Some(market_plugin) = market_plugin
            && market_plugin.version != installed_plugin.config.version
        {
            return Ok(Some(market_plugin.clone()));
        }

        Ok(None)
    }

    /// 检查所有插件更新
    pub async fn check_all_updates(&self) -> Result<Vec<(String, PluginInfo)>, String> {
        info!("检查所有插件更新");

        let installed_plugins = self.installed_plugins.read().await;
        let plugins = self.plugins.read().await;
        let mut updates = vec![];

        for installed_plugin in installed_plugins.values() {
            if let Some(market_plugin) = plugins
                .values()
                .find(|p| p.name == installed_plugin.config.name)
                && market_plugin.version != installed_plugin.config.version
            {
                updates.push((installed_plugin.config.name.clone(), market_plugin.clone()));
            }
        }

        info!("检查完成，找到 {} 个插件有更新", updates.len());
        Ok(updates)
    }

    /// 按分类搜索插件
    pub async fn search_plugins_by_category(&self, category: &str) -> Result<Vec<PluginInfo>, String> {
        info!("按分类搜索插件: {}", category);
        
        let categories = self.categories.read().await;
        let plugins = self.plugins.read().await;
        
        let mut results = vec![];
        
        if let Some(plugin_ids) = categories.get(category) {
            for plugin_id in plugin_ids {
                if let Some(plugin) = plugins.get(plugin_id) {
                    results.push(plugin.clone());
                }
            }
        }
        
        info!("搜索完成，找到 {} 个插件", results.len());
        Ok(results)
    }

    /// 按标签搜索插件
    pub async fn search_plugins_by_tag(&self, tag: &str) -> Result<Vec<PluginInfo>, String> {
        info!("按标签搜索插件: {}", tag);
        
        let plugins = self.plugins.read().await;
        let mut results = vec![];
        
        for plugin in plugins.values() {
            if plugin.tags.contains(&tag.to_string()) {
                results.push(plugin.clone());
            }
        }
        
        info!("搜索完成，找到 {} 个插件", results.len());
        Ok(results)
    }

    /// 评分插件
    pub async fn rate_plugin(&self, plugin_id: &str, rating: f64) -> Result<f64, String> {
        info!("评分插件: {} 评分: {}", plugin_id, rating);
        
        if rating < 1.0 || rating > 5.0 {
            return Err("评分必须在1.0到5.0之间".to_string());
        }
        
        let mut ratings = self.ratings.write().await;
        let plugin_ratings = ratings.entry(plugin_id.to_string()).or_insert_with(Vec::new);
        plugin_ratings.push(rating);
        
        // 计算平均评分
        let average_rating: f64 = plugin_ratings.iter().sum::<f64>() / plugin_ratings.len() as f64;
        
        // 更新插件评分
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.rating = average_rating;
            plugin.rating_count = plugin_ratings.len() as u64;
        }
        
        info!("插件评分更新成功: {} 平均评分: {:.2}", plugin_id, average_rating);
        Ok(average_rating)
    }

    /// 发布插件
    pub async fn publish_plugin(&self, plugin_info: PluginInfo) -> Result<PluginInfo, String> {
        info!("发布插件: {}", plugin_info.name);
        
        let mut plugins = self.plugins.write().await;
        let mut categories = self.categories.write().await;
        let mut tags = self.tags.write().await;
        
        // 生成插件ID
        let plugin_id = uuid::Uuid::new_v4().to_string();
        let mut new_plugin = plugin_info;
        new_plugin.id = plugin_id;
        new_plugin.last_updated = chrono::Utc::now();
        new_plugin.downloads = 0;
        new_plugin.rating = 0.0;
        new_plugin.rating_count = 0;
        
        // 添加插件到市场
        plugins.insert(new_plugin.id.clone(), new_plugin.clone());
        
        // 更新分类
        for category in &new_plugin.categories {
            let plugins_in_category = categories.entry(category.clone()).or_insert_with(Vec::new);
            if !plugins_in_category.contains(&new_plugin.id) {
                plugins_in_category.push(new_plugin.id.clone());
            }
        }
        
        // 更新标签
        for tag in &new_plugin.tags {
            *tags.entry(tag.clone()).or_insert(0) += 1;
        }
        
        info!("插件发布成功: {}", new_plugin.name);
        Ok(new_plugin)
    }

    /// 获取推荐插件
    pub async fn get_featured_plugins(&self) -> Result<Vec<PluginInfo>, String> {
        info!("获取推荐插件");
        
        let plugins = self.plugins.read().await;
        let mut featured_plugins = vec![];
        
        for plugin in plugins.values() {
            if plugin.is_featured {
                featured_plugins.push(plugin.clone());
            }
        }
        
        info!("找到 {} 个推荐插件", featured_plugins.len());
        Ok(featured_plugins)
    }

    /// 获取热门插件
    pub async fn get_popular_plugins(&self, limit: usize) -> Result<Vec<PluginInfo>, String> {
        info!("获取热门插件，限制: {}", limit);
        
        let plugins = self.plugins.read().await;
        let mut popular_plugins: Vec<PluginInfo> = plugins.values().cloned().collect();
        
        // 按下载次数排序
        popular_plugins.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        
        // 限制数量
        let popular_plugins: Vec<PluginInfo> = popular_plugins.into_iter().take(limit).collect();
        
        info!("找到 {} 个热门插件", popular_plugins.len());
        Ok(popular_plugins)
    }

    /// 获取最新插件
    pub async fn get_latest_plugins(&self, limit: usize) -> Result<Vec<PluginInfo>, String> {
        info!("获取最新插件，限制: {}", limit);
        
        let plugins = self.plugins.read().await;
        let mut latest_plugins: Vec<PluginInfo> = plugins.values().cloned().collect();
        
        // 按最后更新时间排序
        latest_plugins.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));
        
        // 限制数量
        let latest_plugins: Vec<PluginInfo> = latest_plugins.into_iter().take(limit).collect();
        
        info!("找到 {} 个最新插件", latest_plugins.len());
        Ok(latest_plugins)
    }

    /// 获取插件分类列表
    pub async fn get_categories(&self) -> Result<Vec<String>, String> {
        let categories = self.categories.read().await;
        Ok(categories.keys().cloned().collect())
    }

    /// 获取热门标签
    pub async fn get_popular_tags(&self, limit: usize) -> Result<Vec<(String, u64)>, String> {
        let tags = self.tags.read().await;
        let mut tag_list: Vec<(String, u64)> = tags.iter().map(|(k, v)| (k.clone(), *v)).collect();
        
        // 按使用次数排序
        tag_list.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 限制数量
        let popular_tags = tag_list.into_iter().take(limit).collect();
        
        Ok(popular_tags)
    }

    /// 获取插件统计信息
    pub async fn get_market_stats(&self) -> Result<serde_json::Value, String> {
        let plugins = self.plugins.read().await;
        let categories = self.categories.read().await;
        let tags = self.tags.read().await;
        
        let stats = serde_json::json!(
            {
                "total_plugins": plugins.len(),
                "total_categories": categories.len(),
                "total_tags": tags.len(),
                "total_downloads": plugins.values().map(|p| p.downloads).sum::<u64>(),
                "average_rating": plugins.values().map(|p| p.rating).sum::<f64>() / plugins.len() as f64,
                "plugins_by_language": {
                    "rust": plugins.values().filter(|p| p.language == "rust").count(),
                    "javascript": plugins.values().filter(|p| p.language == "javascript").count(),
                    "python": plugins.values().filter(|p| p.language == "python").count(),
                    "java": plugins.values().filter(|p| p.language == "java").count(),
                    "csharp": plugins.values().filter(|p| p.language == "csharp").count(),
                },
                "plugins_by_category": categories.iter().map(|(c, ps)| (c, ps.len())).collect::<serde_json::Value>()
            }
        );
        
        Ok(stats)
    }
}
