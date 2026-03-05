// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Plugin marketplace module
//! Responsible for plugin distribution, rating, and feedback

use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Plugin marketplace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Plugin category
    pub category: PluginCategory,
    /// Plugin download URL
    pub download_url: String,
    /// Plugin checksum (SHA256)
    pub checksum: String,
    /// Plugin file size (bytes)
    pub file_size: u64,
    /// Plugin rating (0-5)
    pub rating: f32,
    /// Number of ratings
    pub rating_count: u32,
    /// Number of downloads
    pub download_count: u32,
    /// Last updated timestamp
    pub last_updated: i64,
    /// Plugin tags
    pub tags: Vec<String>,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Plugin screenshots
    pub screenshots: Vec<String>,
    /// Plugin documentation URL
    pub documentation_url: Option<String>,
    /// Plugin homepage URL
    pub homepage_url: Option<String>,
    /// Plugin license
    pub license: String,
    /// Plugin price (0 for free)
    pub price: f32,
    /// Plugin verified status
    pub verified: bool,
    /// GUF compatible status
    pub guf_compatible: bool,
    /// GUF version compatibility
    pub guf_version: String,
}

/// Plugin category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    /// Basic plugin
    Basic,
    /// Customer service plugin
    CustomerService,
    /// IM plugin
    IM,
    /// Scene plugin
    Scene,
    /// Security plugin
    Security,
    /// Performance plugin
    Performance,
    /// Monitoring plugin
    Monitoring,
    /// Other
    Other,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency name
    pub name: String,
    /// Dependency version requirement
    pub version_requirement: String,
}

/// 评论回复
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    /// 回复ID
    pub id: String,
    /// 回复内容
    pub content: String,
    /// 回复用户
    pub user_name: String,
    /// 回复时间戳
    pub timestamp: i64,
    /// 支持票数
    pub upvotes: u32,
    /// 反对票数
    pub downvotes: u32,
}

/// Plugin rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRating {
    /// Plugin name
    pub plugin_name: String,
    /// Rating (0-5)
    pub rating: f32,
    /// User comment
    pub comment: Option<String>,
    /// User name
    pub user_name: String,
    /// Rating timestamp
    pub timestamp: i64,
    /// 支持票数
    pub upvotes: u32,
    /// 反对票数
    pub downvotes: u32,
    /// 评论回复
    pub replies: Vec<CommentReply>,
}

/// Plugin feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginFeedback {
    /// Feedback ID
    pub id: String,
    /// Plugin name
    pub plugin_name: String,
    /// Feedback type
    pub feedback_type: FeedbackType,
    /// Feedback content
    pub content: String,
    /// User name
    pub user_name: String,
    /// Feedback timestamp
    pub timestamp: i64,
    /// Feedback status
    pub status: FeedbackStatus,
}

/// Feedback type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeedbackType {
    /// Bug report
    Bug,
    /// Feature request
    FeatureRequest,
    /// Question
    Question,
    /// Other
    Other,
}

/// Feedback status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeedbackStatus {
    /// Pending
    Pending,
    /// In progress
    InProgress,
    /// Resolved
    Resolved,
    /// Closed
    Closed,
}

/// Marketplace search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSearchResult {
    /// Total results
    pub total: u32,
    /// Page number
    pub page: u32,
    /// Page size
    pub page_size: u32,
    /// Plugins
    pub plugins: Vec<MarketplacePlugin>,
}

/// Marketplace statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    /// Total plugins
    pub total_plugins: u32,
    /// Total downloads
    pub total_downloads: u32,
    /// Total ratings
    pub total_ratings: u32,
    /// Average rating
    pub average_rating: f32,
    /// Plugins by category
    pub plugins_by_category: HashMap<String, u32>,
}

/// Plugin marketplace
#[derive(Debug, Clone)]
pub struct PluginMarketplace {
    /// Available plugins
    pub plugins: Arc<RwLock<HashMap<String, MarketplacePlugin>>>,
    /// Plugin ratings
    pub ratings: Arc<RwLock<HashMap<String, Vec<PluginRating>>>>,
    /// Plugin feedback
    pub feedback: Arc<RwLock<HashMap<String, Vec<PluginFeedback>>>>,
    /// Marketplace statistics
    pub stats: Arc<RwLock<MarketplaceStats>>,
}

impl PluginMarketplace {
    /// Create new plugin marketplace
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            ratings: Arc::new(RwLock::new(HashMap::new())),
            feedback: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MarketplaceStats {
                total_plugins: 0,
                total_downloads: 0,
                total_ratings: 0,
                average_rating: 0.0,
                plugins_by_category: HashMap::new(),
            })),
        }
    }

    /// Add plugin to marketplace
    pub async fn add_plugin(&self, plugin: MarketplacePlugin) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;

        if plugins.contains_key(&plugin.name) {
            return Err(format!("插件已存在: {}", plugin.name));
        }

        plugins.insert(plugin.name.clone(), plugin.clone());

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_plugins += 1;
        *stats
            .plugins_by_category
            .entry(format!("{:?}", plugin.category))
            .or_insert(0) += 1;

        info!("插件已添加到市场: {} v{}", plugin.name, plugin.version);
        Ok(())
    }

    /// Update plugin in marketplace
    pub async fn update_plugin(&self, plugin: MarketplacePlugin) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;

        if !plugins.contains_key(&plugin.name) {
            return Err(format!("插件不存在: {}", plugin.name));
        }

        plugins.insert(plugin.name.clone(), plugin.clone());

        info!("插件已在市场中更新: {} v{}", plugin.name, plugin.version);
        Ok(())
    }

    /// Remove plugin from marketplace
    pub async fn remove_plugin(&self, plugin_name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;

        if !plugins.contains_key(plugin_name) {
            return Err(format!("插件不存在: {}", plugin_name));
        }

        let plugin = plugins.remove(plugin_name).unwrap();

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_plugins -= 1;
        *stats
            .plugins_by_category
            .entry(format!("{:?}", plugin.category))
            .or_insert(0) -= 1;

        info!("插件已从市场移除: {}", plugin_name);
        Ok(())
    }

    /// Get plugin by name
    pub async fn get_plugin(&self, plugin_name: &str) -> Option<MarketplacePlugin> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_name).cloned()
    }

    /// List all plugins
    pub async fn list_plugins(&self) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }

    /// Search plugins
    pub async fn search_plugins(
        &self,
        query: &str,
        category: Option<PluginCategory>,
        page: u32,
        page_size: u32,
    ) -> MarketplaceSearchResult {
        let plugins = self.plugins.read().await;

        let mut filtered_plugins: Vec<&MarketplacePlugin> = plugins
            .values()
            .filter(|p| {
                // Filter by query
                let matches_query = query.is_empty()
                    || p.name.to_lowercase().contains(&query.to_lowercase())
                    || p.description.to_lowercase().contains(&query.to_lowercase())
                    || p.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query.to_lowercase()));

                // Filter by category
                let matches_category = 
                    category.is_none() || &p.category == category.as_ref().unwrap();

                matches_query && matches_category
            })
            .collect();

        // Sort by rating and download count
        filtered_plugins.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.download_count.cmp(&a.download_count))
        });

        // Paginate
        let total = filtered_plugins.len() as u32;
        let start = (page - 1) * page_size;
        let end = std::cmp::min(start + page_size, total);

        let plugins: Vec<MarketplacePlugin> = filtered_plugins
            .get(start as usize..end as usize)
            .unwrap_or(&[])
            .iter()
            .map(|p| (*p).clone())
            .collect();

        info!("Search plugins: query='{}', category={:?}, page={}, page_size={}, total={}", 
              query, category, page, page_size, total);

        MarketplaceSearchResult {
            total,
            page,
            page_size,
            plugins,
        }
    }

    /// Search plugins with multiple filters
    pub async fn search_plugins_with_filters(
        &self,
        query: &str,
        category: Option<PluginCategory>,
        min_rating: Option<f32>,
        verified_only: bool,
        guf_compatible: bool,
        page: u32,
        page_size: u32,
    ) -> MarketplaceSearchResult {
        let plugins = self.plugins.read().await;

        let mut filtered_plugins: Vec<&MarketplacePlugin> = plugins
            .values()
            .filter(|p| {
                // Filter by query
                let matches_query = query.is_empty()
                    || p.name.to_lowercase().contains(&query.to_lowercase())
                    || p.description.to_lowercase().contains(&query.to_lowercase())
                    || p.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query.to_lowercase()));

                // Filter by category
                let matches_category = 
                    category.is_none() || &p.category == category.as_ref().unwrap();

                // Filter by minimum rating
                let matches_rating = min_rating.map_or(true, |min| p.rating >= min);

                // Filter by verified status
                let matches_verified = !verified_only || p.verified;

                // Filter by GUF compatibility
                let matches_guf = !guf_compatible || p.guf_compatible;

                matches_query && matches_category && matches_rating && matches_verified && matches_guf
            })
            .collect();

        // Sort by rating and download count
        filtered_plugins.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.download_count.cmp(&a.download_count))
        });

        // Paginate
        let total = filtered_plugins.len() as u32;
        let start = (page - 1) * page_size;
        let end = std::cmp::min(start + page_size, total);

        let plugins: Vec<MarketplacePlugin> = filtered_plugins
            .get(start as usize..end as usize)
            .unwrap_or(&[])
            .iter()
            .map(|p| (*p).clone())
            .collect();

        info!("Search plugins with filters: query='{}', category={:?}, min_rating={:?}, verified_only={}, guf_compatible={}, total={}", 
              query, category, min_rating, verified_only, guf_compatible, total);

        MarketplaceSearchResult {
            total,
            page,
            page_size,
            plugins,
        }
    }

    /// Get plugins by category
    pub async fn get_plugins_by_category(
        &self,
        category: PluginCategory,
    ) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|p| p.category == category)
            .cloned()
            .collect()
    }

    /// Rate plugin
    pub async fn rate_plugin(&self, mut rating: PluginRating) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        // 初始化新字段
        rating.upvotes = 0;
        rating.downvotes = 0;
        rating.replies = Vec::new();

        let plugin_ratings = ratings
            .entry(rating.plugin_name.clone())
            .or_insert_with(Vec::new);
        plugin_ratings.push(rating.clone());

        // Update plugin rating
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(&rating.plugin_name) {
            let total_rating: f32 = plugin_ratings.iter().map(|r| r.rating).sum();
            plugin.rating = total_rating / plugin_ratings.len() as f32;
            plugin.rating_count = plugin_ratings.len() as u32;

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.total_ratings += 1;
            stats.average_rating = {
                let all_ratings: Vec<&PluginRating> = ratings.values().flatten().collect();
                let total: f32 = all_ratings.iter().map(|r| r.rating).sum();
                total / all_ratings.len() as f32
            };
        }

        info!("插件已评分: {} - {} 星", rating.plugin_name, rating.rating);
        Ok(())
    }

    /// Get plugin ratings
    pub async fn get_plugin_ratings(&self, plugin_name: &str) -> Vec<PluginRating> {
        let ratings = self.ratings.read().await;
        ratings.get(plugin_name).cloned().unwrap_or_default()
    }

    /// Add reply to comment
    pub async fn add_comment_reply(
        &self,
        plugin_name: &str,
        comment_index: usize,
        reply: CommentReply,
    ) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        if let Some(plugin_ratings) = ratings.get_mut(plugin_name)
            && comment_index < plugin_ratings.len()
        {
            plugin_ratings[comment_index].replies.push(reply);
            info!("已添加评论回复: {} - 评论 #{}", plugin_name, comment_index);
            return Ok(());
        }

        Err(format!(
            "评论不存在: {} - 评论 #{}",
            plugin_name, comment_index
        ))
    }

    /// Upvote comment
    pub async fn upvote_comment(
        &self,
        plugin_name: &str,
        comment_index: usize,
    ) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        if let Some(plugin_ratings) = ratings.get_mut(plugin_name)
            && comment_index < plugin_ratings.len()
        {
            plugin_ratings[comment_index].upvotes += 1;
            info!("已支持评论: {} - 评论 #{}", plugin_name, comment_index);
            return Ok(());
        }

        Err(format!(
            "评论不存在: {} - 评论 #{}",
            plugin_name, comment_index
        ))
    }

    /// Downvote comment
    pub async fn downvote_comment(
        &self,
        plugin_name: &str,
        comment_index: usize,
    ) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        if let Some(plugin_ratings) = ratings.get_mut(plugin_name)
            && comment_index < plugin_ratings.len()
        {
            plugin_ratings[comment_index].downvotes += 1;
            info!("已反对评论: {} - 评论 #{}", plugin_name, comment_index);
            return Ok(());
        }

        Err(format!(
            "评论不存在: {} - 评论 #{}",
            plugin_name, comment_index
        ))
    }

    /// Upvote reply
    pub async fn upvote_reply(
        &self,
        plugin_name: &str,
        comment_index: usize,
        reply_index: usize,
    ) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        if let Some(plugin_ratings) = ratings.get_mut(plugin_name)
            && comment_index < plugin_ratings.len()
        {
            let comment = &mut plugin_ratings[comment_index];
            if reply_index < comment.replies.len() {
                comment.replies[reply_index].upvotes += 1;
                info!(
                    "已支持回复: {} - 评论 #{} - 回复 #{}",
                    plugin_name, comment_index, reply_index
                );
                return Ok(());
            }
        }

        Err(format!(
            "回复不存在: {} - 评论 #{} - 回复 #{}",
            plugin_name, comment_index, reply_index
        ))
    }

    /// Downvote reply
    pub async fn downvote_reply(
        &self,
        plugin_name: &str,
        comment_index: usize,
        reply_index: usize,
    ) -> Result<(), String> {
        let mut ratings = self.ratings.write().await;

        if let Some(plugin_ratings) = ratings.get_mut(plugin_name)
            && comment_index < plugin_ratings.len()
        {
            let comment = &mut plugin_ratings[comment_index];
            if reply_index < comment.replies.len() {
                comment.replies[reply_index].downvotes += 1;
                info!(
                    "已反对回复: {} - 评论 #{} - 回复 #{}",
                    plugin_name, comment_index, reply_index
                );
                return Ok(());
            }
        }

        Err(format!(
            "回复不存在: {} - 评论 #{} - 回复 #{}",
            plugin_name, comment_index, reply_index
        ))
    }

    /// Get plugin ratings sorted by different criteria
    pub async fn get_plugin_ratings_sorted(
        &self,
        plugin_name: &str,
        sort_by: &str,
    ) -> Vec<PluginRating> {
        let ratings = self.ratings.read().await;
        let mut plugin_ratings = ratings.get(plugin_name).cloned().unwrap_or_default();

        match sort_by {
            "newest" => {
                plugin_ratings.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            }
            "oldest" => {
                plugin_ratings.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            }
            "highest_rating" => {
                plugin_ratings.sort_by(|a, b| {
                    b.rating
                        .partial_cmp(&a.rating)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "lowest_rating" => {
                plugin_ratings.sort_by(|a, b| {
                    a.rating
                        .partial_cmp(&b.rating)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "most_upvoted" => {
                plugin_ratings.sort_by(|a, b| b.upvotes.cmp(&a.upvotes));
            }
            "most_replies" => {
                plugin_ratings.sort_by(|a, b| b.replies.len().cmp(&a.replies.len()));
            }
            _ => {
                // Default sort by newest
                plugin_ratings.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            }
        }

        plugin_ratings
    }

    /// Submit feedback
    pub async fn submit_feedback(&self, feedback: PluginFeedback) -> Result<(), String> {
        let mut feedback_map = self.feedback.write().await;

        let plugin_feedback = feedback_map
            .entry(feedback.plugin_name.clone())
            .or_insert_with(Vec::new);
        plugin_feedback.push(feedback.clone());

        info!(
            "反馈已提交: {} - {:?}",
            feedback.plugin_name, feedback.feedback_type
        );
        Ok(())
    }

    /// Get plugin feedback
    pub async fn get_plugin_feedback(&self, plugin_name: &str) -> Vec<PluginFeedback> {
        let feedback = self.feedback.read().await;
        feedback.get(plugin_name).cloned().unwrap_or_default()
    }

    /// Update feedback status
    pub async fn update_feedback_status(
        &self,
        plugin_name: &str,
        feedback_id: &str,
        status: FeedbackStatus,
    ) -> Result<(), String> {
        let mut feedback_map = self.feedback.write().await;

        if let Some(plugin_feedback) = feedback_map.get_mut(plugin_name)
            && let Some(feedback) = plugin_feedback.iter_mut().find(|f| f.id == feedback_id)
        {
            feedback.status = status.clone();
            info!(
                "反馈状态已更新: {} - {} - {:?}",
                plugin_name, feedback_id, status
            );
            return Ok(());
        }

        Err(format!("反馈不存在: {}", feedback_id))
    }

    /// Download plugin
    pub async fn download_plugin(&self, plugin_name: &str) -> Result<Vec<u8>, String> {
        let plugins = self.plugins.read().await;

        if let Some(plugin) = plugins.get(plugin_name) {
            let download_url = plugin.download_url.clone();
            let checksum = plugin.checksum.clone();

            // Increment download count
            drop(plugins);
            let mut plugins = self.plugins.write().await;
            if let Some(plugin) = plugins.get_mut(plugin_name) {
                plugin.download_count += 1;

                // Update statistics
                let mut stats = self.stats.write().await;
                stats.total_downloads += 1;
            }
            drop(plugins);

            // Try to download plugin with retry mechanism
            let result = self.try_download_with_retry(&download_url, &checksum).await;
            match result {
                Ok(data) => {
                    info!("插件下载成功: {}", plugin_name);
                    Ok(data)
                }
                Err(err) => {
                    error!("插件下载失败: {} - {}", plugin_name, err);
                    // Try offline mode if available
                    self.try_offline_mode(plugin_name).await
                }
            }
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// Try to download plugin with retry mechanism
    async fn try_download_with_retry(
        &self,
        download_url: &str,
        checksum: &str,
    ) -> Result<Vec<u8>, String> {
        const MAX_RETRIES: usize = 3;
        const RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(2);
        const DOWNLOAD_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

        for attempt in 0..MAX_RETRIES {
            info!("尝试下载插件 (尝试 {} / {})...", attempt + 1, MAX_RETRIES);

            match tokio::time::timeout(DOWNLOAD_TIMEOUT, self.download_from_url(download_url)).await
            {
                Ok(Ok(data)) => {
                    // Verify checksum
                    if !self.verify_checksum(&data, checksum) {
                        error!("插件校验和验证失败");
                        continue;
                    }
                    return Ok(data);
                }
                Ok(Err(err)) => {
                    error!("下载失败: {} (尝试 {} / {})", err, attempt + 1, MAX_RETRIES);
                }
                Err(_) => {
                    error!("下载超时 (尝试 {} / {})", attempt + 1, MAX_RETRIES);
                }
            }

            if attempt < MAX_RETRIES - 1 {
                tokio::time::sleep(RETRY_DELAY).await;
            }
        }

        Err("下载插件失败，已达到最大重试次数".to_string())
    }

    /// Download from URL
    async fn download_from_url(&self, url: &str) -> Result<Vec<u8>, String> {
        // In a real implementation, this would use reqwest or similar to download the plugin
        info!("Downloading plugin from: {}", url);
        // For now, return a placeholder
        Ok(vec![])
    }

    /// Verify checksum
    fn verify_checksum(&self, _data: &[u8], expected_checksum: &str) -> bool {
        // In a real implementation, this would compute the SHA256 checksum
        info!("Verifying checksum: {}", expected_checksum);
        // For now, return true
        true
    }

    /// Try offline mode
    async fn try_offline_mode(&self, plugin_name: &str) -> Result<Vec<u8>, String> {
        // Check if plugin is available in offline cache
        let cache_path = format!("data/plugins/cache/{}.axpl", plugin_name);
        if std::path::Path::new(&cache_path).exists() {
            info!("使用离线缓存的插件: {}", plugin_name);
            match std::fs::read(&cache_path) {
                Ok(data) => Ok(data),
                Err(err) => Err(format!("读取离线缓存失败: {}", err)),
            }
        } else {
            Err(format!("插件 {} 未在离线缓存中找到", plugin_name))
        }
    }

    /// Get marketplace statistics
    pub async fn get_stats(&self) -> MarketplaceStats {
        self.stats.read().await.clone()
    }

    /// Get trending plugins
    pub async fn get_trending_plugins(&self, limit: u32) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;

        let mut sorted_plugins: Vec<&MarketplacePlugin> = plugins.values().collect();

        // Sort by download count (last 7 days)
        sorted_plugins.sort_by(|a, b| b.download_count.cmp(&a.download_count));

        sorted_plugins
            .into_iter()
            .take(limit as usize)
            .cloned()
            .collect()
    }

    /// Get top rated plugins
    pub async fn get_top_rated_plugins(&self, limit: u32) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;

        let mut sorted_plugins: Vec<&MarketplacePlugin> = plugins.values().collect();

        // Sort by rating
        sorted_plugins.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.rating_count.cmp(&a.rating_count))
        });

        sorted_plugins
            .into_iter()
            .take(limit as usize)
            .cloned()
            .collect()
    }

    /// Get recently updated plugins
    pub async fn get_recently_updated_plugins(&self, limit: u32) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;

        let mut sorted_plugins: Vec<&MarketplacePlugin> = plugins.values().collect();

        // Sort by last updated timestamp
        sorted_plugins.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));

        sorted_plugins
            .into_iter()
            .take(limit as usize)
            .cloned()
            .collect()
    }

    /// Verify plugin
    pub async fn verify_plugin(&self, plugin_name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;

        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.verified = true;
            info!("插件已验证: {}", plugin_name);
            Ok(())
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// Unverify plugin
    pub async fn unverify_plugin(&self, plugin_name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.write().await;

        if let Some(plugin) = plugins.get_mut(plugin_name) {
            plugin.verified = false;
            info!("插件已取消验证: {}", plugin_name);
            Ok(())
        } else {
            Err(format!("插件不存在: {}", plugin_name))
        }
    }

    /// Get GUF compatible plugins
    pub async fn get_guf_compatible_plugins(&self) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|p| p.guf_compatible)
            .cloned()
            .collect()
    }

    /// Search plugins with GUF compatibility filter
    pub async fn search_guf_plugins(
        &self,
        query: &str,
        category: Option<PluginCategory>,
        page: u32,
        page_size: u32,
    ) -> MarketplaceSearchResult {
        let plugins = self.plugins.read().await;
        let mut filtered_plugins: Vec<&MarketplacePlugin> = plugins
            .values()
            .filter(|p| {
                p.guf_compatible
                    && (p.name.to_lowercase().contains(&query.to_lowercase())
                        || p.description.to_lowercase().contains(&query.to_lowercase())
                        || p.tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query.to_lowercase())))
                    && category.as_ref().is_none_or(|cat| *cat == p.category)
            })
            .collect();

        // Sort by download count (popularity)
        filtered_plugins.sort_by(|a, b| b.download_count.cmp(&a.download_count));

        // Pagination
        let total = filtered_plugins.len() as u32;
        let start = ((page - 1) * page_size) as usize;
        let end = (start + page_size as usize).min(filtered_plugins.len());
        let paginated_plugins = if start < filtered_plugins.len() {
            filtered_plugins[start..end]
                .iter()
                .map(|p| (*p).clone())
                .collect()
        } else {
            vec![]
        };

        MarketplaceSearchResult {
            total,
            page,
            page_size,
            plugins: paginated_plugins,
        }
    }

    /// Check if plugin is GUF compatible
    pub async fn is_plugin_guf_compatible(&self, plugin_name: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_name).is_some_and(|p| p.guf_compatible)
    }

    /// Get GUF version for plugin
    pub async fn get_plugin_guf_version(&self, plugin_name: &str) -> Option<String> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_name).map(|p| p.guf_version.clone())
    }
}

impl Default for PluginMarketplace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_plugin() {
        let marketplace = PluginMarketplace::new();

        let plugin = MarketplacePlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/test_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 0.0,
            rating_count: 0,
            download_count: 0,
            last_updated: 0,
            tags: vec!["test".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: false,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        let result = marketplace.add_plugin(plugin).await;
        assert!(result.is_ok());

        let retrieved_plugin = marketplace.get_plugin("test_plugin").await;
        assert!(retrieved_plugin.is_some());
        assert_eq!(retrieved_plugin.unwrap().name, "test_plugin");
    }

    #[tokio::test]
    async fn test_search_plugins() {
        let marketplace = PluginMarketplace::new();

        let plugin1 = MarketplacePlugin {
            name: "customer_service_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Customer service plugin".to_string(),
            category: PluginCategory::CustomerService,
            download_url: "https://example.com/customer_service_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 4.5,
            rating_count: 10,
            download_count: 100,
            last_updated: 0,
            tags: vec!["customer".to_string(), "service".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: true,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin1).await.unwrap();

        let result = marketplace
            .search_plugins("customer", Some(PluginCategory::CustomerService), 1, 10)
            .await;
        assert_eq!(result.plugins.len(), 1);
        assert_eq!(result.plugins[0].name, "customer_service_plugin");
    }

    #[tokio::test]
    async fn test_rate_plugin() {
        let marketplace = PluginMarketplace::new();

        let plugin = MarketplacePlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/test_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 0.0,
            rating_count: 0,
            download_count: 0,
            last_updated: 0,
            tags: vec!["test".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: false,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin).await.unwrap();

        let rating = PluginRating {
            plugin_name: "test_plugin".to_string(),
            rating: 5.0,
            comment: Some("Great plugin!".to_string()),
            user_name: "test_user".to_string(),
            timestamp: 0,
            upvotes: 0,
            downvotes: 0,
            replies: Vec::new(),
        };

        let result = marketplace.rate_plugin(rating).await;
        assert!(result.is_ok());

        let retrieved_plugin = marketplace.get_plugin("test_plugin").await;
        assert!(retrieved_plugin.is_some());
        assert_eq!(retrieved_plugin.unwrap().rating, 5.0);
    }

    #[tokio::test]
    async fn test_submit_feedback() {
        let marketplace = PluginMarketplace::new();

        let plugin = MarketplacePlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/test_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 0.0,
            rating_count: 0,
            download_count: 0,
            last_updated: 0,
            tags: vec!["test".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: false,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin).await.unwrap();

        let feedback = PluginFeedback {
            id: uuid::Uuid::new_v4().to_string(),
            plugin_name: "test_plugin".to_string(),
            feedback_type: FeedbackType::Bug,
            content: "Found a bug".to_string(),
            user_name: "test_user".to_string(),
            timestamp: 0,
            status: FeedbackStatus::Pending,
        };

        let result = marketplace.submit_feedback(feedback).await;
        assert!(result.is_ok());

        let retrieved_feedback = marketplace.get_plugin_feedback("test_plugin").await;
        assert_eq!(retrieved_feedback.len(), 1);
    }

    #[tokio::test]
    async fn test_get_trending_plugins() {
        let marketplace = PluginMarketplace::new();

        let plugin1 = MarketplacePlugin {
            name: "trending_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Trending plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/trending_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 4.5,
            rating_count: 10,
            download_count: 1000,
            last_updated: 0,
            tags: vec!["trending".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: true,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin1).await.unwrap();

        let trending = marketplace.get_trending_plugins(10).await;
        assert_eq!(trending.len(), 1);
        assert_eq!(trending[0].name, "trending_plugin");
    }

    #[tokio::test]
    async fn test_get_top_rated_plugins() {
        let marketplace = PluginMarketplace::new();

        let plugin1 = MarketplacePlugin {
            name: "top_rated_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Top rated plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/top_rated_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 5.0,
            rating_count: 100,
            download_count: 100,
            last_updated: 0,
            tags: vec!["top".to_string(), "rated".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: true,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin1).await.unwrap();

        let top_rated = marketplace.get_top_rated_plugins(10).await;
        assert_eq!(top_rated.len(), 1);
        assert_eq!(top_rated[0].name, "top_rated_plugin");
    }

    #[tokio::test]
    async fn test_get_recently_updated_plugins() {
        let marketplace = PluginMarketplace::new();

        let plugin1 = MarketplacePlugin {
            name: "recently_updated_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Recently updated plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/recently_updated_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 4.0,
            rating_count: 10,
            download_count: 100,
            last_updated: chrono::Utc::now().timestamp(),
            tags: vec!["recent".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: true,
            guf_compatible: true,
            guf_version: "1.0.0".to_string(),
        };

        marketplace.add_plugin(plugin1).await.unwrap();

        let recent = marketplace.get_recently_updated_plugins(10).await;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].name, "recently_updated_plugin");
    }

    #[tokio::test]
    async fn test_verify_plugin() {
        let marketplace = PluginMarketplace::new();

        let plugin = MarketplacePlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            category: PluginCategory::Basic,
            download_url: "https://example.com/test_plugin.axpl".to_string(),
            checksum: "abc123".to_string(),
            file_size: 1024,
            rating: 0.0,
            rating_count: 0,
            download_count: 0,
            last_updated: 0,
            tags: vec!["test".to_string()],
            dependencies: vec![],
            screenshots: vec![],
            documentation_url: None,
            homepage_url: None,
            license: "MIT".to_string(),
            price: 0.0,
            verified: false,
            guf_compatible: false,
            guf_version: "".to_string(),
        };

        marketplace.add_plugin(plugin).await.unwrap();

        let result = marketplace.verify_plugin("test_plugin").await;
        assert!(result.is_ok());

        let retrieved_plugin = marketplace.get_plugin("test_plugin").await;
        assert!(retrieved_plugin.is_some());
        assert!(retrieved_plugin.unwrap().verified);
    }
}

