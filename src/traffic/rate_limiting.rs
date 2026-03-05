//! 限流模块
//!
//! 提供API请求速率限制、防止系统过载等功能

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 限流策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RateLimitStrategy {
    /// 固定窗口
    FixedWindow,
    /// 滑动窗口
    SlidingWindow,
    /// 令牌桶
    TokenBucket,
    /// 漏桶
    LeakyBucket,
}

/// 限流配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterConfig {
    /// 启用限流
    pub enabled: bool,
    /// 限流策略
    pub strategy: RateLimitStrategy,
    /// 限制数量
    pub limit: u32,
    /// 时间窗口（毫秒）
    pub window_ms: u64,
    /// 令牌桶填充速率（令牌/秒）
    pub token_fill_rate: f64,
    /// 令牌桶容量
    pub token_bucket_capacity: u32,
    /// 漏桶流出速率（请求/秒）
    pub leak_rate: f64,
    /// 漏桶容量
    pub leak_bucket_capacity: u32,
    /// 限流规则
    pub rules: Vec<RateLimitRule>,
}

/// 限流规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRule {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 匹配路径
    pub path_pattern: String,
    /// 匹配方法
    pub method_pattern: String,
    /// 匹配头部
    pub header_patterns: Vec<(String, String)>,
    /// 限制数量
    pub limit: u32,
    /// 时间窗口（毫秒）
    pub window_ms: u64,
    /// 启用状态
    pub enabled: bool,
}

/// 限流统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStats {
    /// 总请求数
    pub total_requests: u32,
    /// 通过请求数
    pub passed_requests: u32,
    /// 被限流请求数
    pub limited_requests: u32,
    /// 限流率
    pub limit_rate: f64,
    /// 上次重置时间
    pub last_reset_time: String,
    /// 规则匹配数
    pub rule_matches: HashMap<String, u32>,
}

/// 固定窗口计数器
#[derive(Debug, Clone)]
struct FixedWindowCounter {
    count: u32,
    window_start: String,
}

/// 滑动窗口计数器
#[derive(Debug, Clone)]
struct SlidingWindowCounter {
    windows: Vec<(String, u32)>,
}

/// 令牌桶
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_refill: String,
    fill_rate: f64,
    capacity: u32,
}

/// 漏桶
#[derive(Debug, Clone)]
struct LeakyBucket {
    level: f64,
    last_leak: String,
    leak_rate: f64,
    capacity: u32,
}

/// 限流器
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: Arc<RwLock<RateLimiterConfig>>,
    stats: Arc<RwLock<RateLimitStats>>,
    fixed_windows: Arc<RwLock<HashMap<String, FixedWindowCounter>>>,
    sliding_windows: Arc<RwLock<HashMap<String, SlidingWindowCounter>>>,
    token_buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    leaky_buckets: Arc<RwLock<HashMap<String, LeakyBucket>>>,
}

impl RateLimiter {
    /// 创建新的限流器
    pub fn new(config: RateLimiterConfig) -> Self {
        let stats = RateLimitStats {
            total_requests: 0,
            passed_requests: 0,
            limited_requests: 0,
            limit_rate: 0.0,
            last_reset_time: chrono::Utc::now().to_string(),
            rule_matches: HashMap::new(),
        };

        Self {
            config: Arc::new(RwLock::new(config)),
            stats: Arc::new(RwLock::new(stats)),
            fixed_windows: Arc::new(RwLock::new(HashMap::new())),
            sliding_windows: Arc::new(RwLock::new(HashMap::new())),
            token_buckets: Arc::new(RwLock::new(HashMap::new())),
            leaky_buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 初始化限流器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化限流器
        Ok(())
    }

    /// 检查是否允许请求
    pub async fn allow_request(
        &self,
        path: &str,
        method: &str,
        headers: &[(String, String)],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let config = self.config.read().await;

        if !config.enabled {
            self.record_pass().await;
            return Ok(true);
        }

        // 查找匹配的规则
        let matching_rule = self
            .find_matching_rule(&config, path, method, headers)
            .await;

        // 检查是否限流
        let allowed = match matching_rule {
            Some(rule) => self.check_rate_limit_with_rule(&rule, path, method).await,
            None => self.check_rate_limit_default(&config, path, method).await,
        };

        // 记录统计信息
        if allowed {
            self.record_pass().await;
        } else {
            self.record_limit().await;
        }

        Ok(allowed)
    }

    /// 查找匹配的规则
    async fn find_matching_rule(
        &self,
        config: &RateLimiterConfig,
        path: &str,
        method: &str,
        headers: &[(String, String)],
    ) -> Option<RateLimitRule> {
        for rule in &config.rules {
            if rule.enabled {
                // 检查路径匹配
                if !self.matches_pattern(path, &rule.path_pattern) {
                    continue;
                }

                // 检查方法匹配
                if !self.matches_pattern(method, &rule.method_pattern) {
                    continue;
                }

                // 检查头部匹配
                let headers_match = rule.header_patterns.iter().all(|(name, value)| {
                    headers.iter().any(|(h_name, h_value)| {
                        self.matches_pattern(h_name, name) && self.matches_pattern(h_value, value)
                    })
                });

                if headers_match {
                    // 记录规则匹配
                    let mut stats = self.stats.write().await;
                    *stats.rule_matches.entry(rule.name.clone()).or_insert(0) += 1;
                    return Some(rule.clone());
                }
            }
        }

        None
    }

    /// 检查是否匹配模式
    fn matches_pattern(&self, value: &str, pattern: &str) -> bool {
        // 简单的通配符匹配
        if pattern == "*" {
            return true;
        }

        // 精确匹配
        value == pattern
    }

    /// 使用规则检查限流
    async fn check_rate_limit_with_rule(
        &self,
        rule: &RateLimitRule,
        path: &str,
        method: &str,
    ) -> bool {
        let key = format!("{}_{}", path, method);
        let config = self.config.read().await;

        match config.strategy {
            RateLimitStrategy::FixedWindow => {
                self.check_fixed_window(&key, rule.limit, rule.window_ms)
                    .await
            }
            RateLimitStrategy::SlidingWindow => {
                self.check_sliding_window(&key, rule.limit, rule.window_ms)
                    .await
            }
            RateLimitStrategy::TokenBucket => {
                self.check_token_bucket(&key, rule.limit, rule.window_ms)
                    .await
            }
            RateLimitStrategy::LeakyBucket => {
                self.check_leaky_bucket(&key, rule.limit, rule.window_ms)
                    .await
            }
        }
    }

    /// 使用默认配置检查限流
    async fn check_rate_limit_default(
        &self,
        config: &RateLimiterConfig,
        path: &str,
        method: &str,
    ) -> bool {
        let key = format!("{}_{}", path, method);

        match config.strategy {
            RateLimitStrategy::FixedWindow => {
                self.check_fixed_window(&key, config.limit, config.window_ms)
                    .await
            }
            RateLimitStrategy::SlidingWindow => {
                self.check_sliding_window(&key, config.limit, config.window_ms)
                    .await
            }
            RateLimitStrategy::TokenBucket => {
                self.check_token_bucket(&key, config.limit, config.window_ms)
                    .await
            }
            RateLimitStrategy::LeakyBucket => {
                self.check_leaky_bucket(&key, config.limit, config.window_ms)
                    .await
            }
        }
    }

    /// 检查固定窗口
    async fn check_fixed_window(&self, key: &str, limit: u32, window_ms: u64) -> bool {
        let mut fixed_windows = self.fixed_windows.write().await;
        let now = chrono::Utc::now();
        let current_window = now.to_string();

        let counter = fixed_windows
            .entry(key.to_string())
            .or_insert(FixedWindowCounter {
                count: 0,
                window_start: current_window.clone(),
            });

        // 检查是否需要重置窗口
        let window_start = chrono::DateTime::parse_from_rfc3339(&counter.window_start).unwrap();
        let window_duration = now.signed_duration_since(window_start);

        if window_duration.num_milliseconds() >= window_ms as i64 {
            counter.count = 0;
            counter.window_start = current_window;
        }

        // 检查是否超过限制
        if counter.count >= limit {
            false
        } else {
            counter.count += 1;
            true
        }
    }

    /// 检查滑动窗口
    async fn check_sliding_window(&self, key: &str, limit: u32, window_ms: u64) -> bool {
        let mut sliding_windows = self.sliding_windows.write().await;
        let now = chrono::Utc::now();

        let counter = sliding_windows
            .entry(key.to_string())
            .or_insert(SlidingWindowCounter {
                windows: Vec::new(),
            });

        // 移除过期的窗口
        counter.windows.retain(|(window_time, _)| {
            let window_time = chrono::DateTime::parse_from_rfc3339(window_time).unwrap();
            let window_duration = now.signed_duration_since(window_time);
            window_duration.num_milliseconds() < window_ms as i64
        });

        // 计算当前窗口的请求数
        let total_count: u32 = counter.windows.iter().map(|(_, count)| count).sum();

        // 检查是否超过限制
        if total_count >= limit {
            false
        } else {
            // 添加当前请求到窗口
            counter.windows.push((now.to_string(), 1));
            true
        }
    }

    /// 检查令牌桶
    async fn check_token_bucket(&self, key: &str, _limit: u32, _window_ms: u64) -> bool {
        let config = self.config.read().await;
        let mut token_buckets = self.token_buckets.write().await;
        let now = chrono::Utc::now();

        let bucket = token_buckets.entry(key.to_string()).or_insert(TokenBucket {
            tokens: config.token_bucket_capacity as f64,
            last_refill: now.to_string(),
            fill_rate: config.token_fill_rate,
            capacity: config.token_bucket_capacity,
        });

        // 填充令牌
        let last_refill = chrono::DateTime::parse_from_rfc3339(&bucket.last_refill).unwrap();
        let duration = now.signed_duration_since(last_refill);
        let elapsed_seconds = duration.num_milliseconds() as f64 / 1000.0;

        let tokens_to_add = elapsed_seconds * bucket.fill_rate;
        bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.capacity as f64);
        bucket.last_refill = now.to_string();

        // 检查是否有足够的令牌
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// 检查漏桶
    async fn check_leaky_bucket(&self, key: &str, _limit: u32, _window_ms: u64) -> bool {
        let config = self.config.read().await;
        let mut leaky_buckets = self.leaky_buckets.write().await;
        let now = chrono::Utc::now();

        let bucket = leaky_buckets.entry(key.to_string()).or_insert(LeakyBucket {
            level: 0.0,
            last_leak: now.to_string(),
            leak_rate: config.leak_rate,
            capacity: config.leak_bucket_capacity,
        });

        // 漏水
        let last_leak = chrono::DateTime::parse_from_rfc3339(&bucket.last_leak).unwrap();
        let duration = now.signed_duration_since(last_leak);
        let elapsed_seconds = duration.num_milliseconds() as f64 / 1000.0;

        let leaked = elapsed_seconds * bucket.leak_rate;
        bucket.level = (bucket.level - leaked).max(0.0);
        bucket.last_leak = now.to_string();

        // 检查是否可以添加新请求
        if bucket.level < bucket.capacity as f64 {
            bucket.level += 1.0;
            true
        } else {
            false
        }
    }

    /// 记录通过的请求
    async fn record_pass(&self) {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        stats.passed_requests += 1;
        stats.limit_rate = (stats.limited_requests as f64 / stats.total_requests as f64) * 100.0;
    }

    /// 记录被限流的请求
    async fn record_limit(&self) {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        stats.limited_requests += 1;
        stats.limit_rate = (stats.limited_requests as f64 / stats.total_requests as f64) * 100.0;
    }

    /// 获取配置
    pub async fn get_config(&self) -> RateLimiterConfig {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update_config(
        &self,
        config: RateLimiterConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> RateLimitStats {
        self.stats.read().await.clone()
    }

    /// 重置统计信息
    pub async fn reset_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stats = self.stats.write().await;
        stats.total_requests = 0;
        stats.passed_requests = 0;
        stats.limited_requests = 0;
        stats.limit_rate = 0.0;
        stats.last_reset_time = chrono::Utc::now().to_string();
        stats.rule_matches.clear();

        // 重置计数器
        self.fixed_windows.write().await.clear();
        self.sliding_windows.write().await.clear();
        self.token_buckets.write().await.clear();
        self.leaky_buckets.write().await.clear();

        Ok(())
    }
}
