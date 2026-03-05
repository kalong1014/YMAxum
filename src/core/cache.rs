// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use anyhow::{Context, Result, anyhow};
use moka::sync::Cache;
use redis::{Client, Commands};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use tokio::time::interval;

use crate::core::state::AppState;


/// 缓存淘汰策略
#[derive(Debug, Clone, PartialEq)]
pub enum CacheEvictionPolicy {
    /// 最近最少使用
    LRU,
    /// 最不经常使用
    LFU,
    /// 先进先出
    FIFO,
    /// 基于时间的淘汰
    TimeBased,
    /// 混合策略（结合LRU和LFU）
    Hybrid,
}

/// Cache manager
pub struct CacheManager {
    app_state: Arc<AppState>,
    // 缓存分片
    memory_caches: Vec<Cache<String, (String, Option<Duration>)>>,
    num_shards: usize,

    // 访问模式分析
    access_patterns: Cache<String, (u64, Instant)>,
    // Cache statistics
    hits: AtomicU64,
    misses: AtomicU64,
    requests: AtomicU64,
    // 缓存类型配置
    cache_configs: HashMap<String, CacheConfig>,
    // 智能缓存策略
    auto_optimize_enabled: bool,
    optimization_interval: Duration,
    last_optimization: Instant,
    cache_eviction_policy: CacheEvictionPolicy,
    smart_ttl_enabled: bool,
    // 缓存预热配置
    warm_up_enabled: bool,
    warm_up_interval: Duration,
    // 缓存压缩配置
    compression_enabled: bool,
    compression_threshold: usize,
}

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub ttl: Duration,
    pub tti: Duration,
    pub max_capacity: u64,
    pub priority: u32,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(3600),
            tti: Duration::from_secs(1800),
            max_capacity: 1000,
            priority: 1,
        }
    }
}

impl Clone for CacheManager {
    fn clone(&self) -> Self {
        Self {
            app_state: self.app_state.clone(),
            memory_caches: self.memory_caches.clone(),
            num_shards: self.num_shards,

            access_patterns: self.access_patterns.clone(),
            hits: AtomicU64::new(self.hits.load(Ordering::SeqCst)),
            misses: AtomicU64::new(self.misses.load(Ordering::SeqCst)),
            requests: AtomicU64::new(self.requests.load(Ordering::SeqCst)),
            cache_configs: self.cache_configs.clone(),
            auto_optimize_enabled: self.auto_optimize_enabled,
            optimization_interval: self.optimization_interval,
            last_optimization: self.last_optimization,
            cache_eviction_policy: self.cache_eviction_policy.clone(),
            smart_ttl_enabled: self.smart_ttl_enabled,
            warm_up_enabled: self.warm_up_enabled,
            warm_up_interval: self.warm_up_interval,
            compression_enabled: self.compression_enabled,
            compression_threshold: self.compression_threshold,
        }
    }
}

impl CacheManager {
    /// Create new cache manager
    pub fn new(app_state: Arc<AppState>) -> Self {
        const NUM_SHARDS: usize = 8;
        let mut memory_caches = Vec::with_capacity(NUM_SHARDS);

        for _ in 0..NUM_SHARDS {
            memory_caches.push(
                Cache::builder()
                    .max_capacity(10_000)
                    .time_to_live(Duration::from_secs(3600))
                    .time_to_idle(Duration::from_secs(1800))
                    .build(),
            );
        }

        let now = Instant::now();


        Self {
            app_state,
            memory_caches,
            num_shards: NUM_SHARDS,

            access_patterns: Cache::builder()
                .max_capacity(100_000)
                .time_to_live(Duration::from_secs(86400))
                .build(),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            requests: AtomicU64::new(0),
            cache_configs: HashMap::from([
                (
                    "hot".to_string(),
                    CacheConfig {
                        ttl: Duration::from_secs(7200),
                        tti: Duration::from_secs(3600),
                        max_capacity: 5000,
                        priority: 3,
                    },
                ),
                (
                    "warm".to_string(),
                    CacheConfig {
                        ttl: Duration::from_secs(3600),
                        tti: Duration::from_secs(1800),
                        max_capacity: 3000,
                        priority: 2,
                    },
                ),
                (
                    "cold".to_string(),
                    CacheConfig {
                        ttl: Duration::from_secs(1800),
                        tti: Duration::from_secs(900),
                        max_capacity: 2000,
                        priority: 1,
                    },
                ),
            ]),
            auto_optimize_enabled: true,
            optimization_interval: Duration::from_secs(3600),
            last_optimization: now,
            cache_eviction_policy: CacheEvictionPolicy::Hybrid,
            smart_ttl_enabled: true,
            warm_up_enabled: true,
            warm_up_interval: Duration::from_secs(86400),
            compression_enabled: false,
            compression_threshold: 1024,
        }
    }

    /// Initialize Redis client
    pub async fn init_redis_client(&self, url: &str) -> Result<()> {
        let client = Client::open(url).context("Failed to create Redis client")?;

        // Test connection
        let mut connection = client
            .get_connection()
            .context("Failed to connect to Redis")?;

        let _: String = redis::cmd("PING")
            .query(&mut connection)
            .context("Failed to ping Redis")?;

        self.app_state
            .redis_client
            .set(client)
            .map_err(|_| anyhow!("Failed to set Redis client"))?;

        Ok(())
    }

    /// Get Redis client
    pub async fn get_redis_client(&self) -> Option<Client> {
        self.app_state.redis_client.get().cloned()
    }

    /// 计算缓存分片索引
    fn get_shard_index(&self, key: &str) -> usize {
        let hash = key
            .bytes()
            .fold(0usize, |acc, b| acc.wrapping_add(b as usize));
        hash % self.num_shards
    }

    /// Set value in cache (supports both memory and Redis)
    pub async fn set(&mut self, key: &str, value: &str, expire_seconds: Option<u64>) -> Result<()> {
        let start = Instant::now();

        // 计算TTL
        let expire_duration = if let Some(seconds) = expire_seconds {
            Some(Duration::from_secs(seconds))
        } else if self.smart_ttl_enabled {
            Some(self.calculate_smart_ttl(key))
        } else {
            None
        };

        let shard_index = self.get_shard_index(key);

        // 分析访问模式
        let access_pattern = self.analyze_access_pattern(key);
        let _cache_config = self.get_cache_config(&access_pattern);

        // Set in memory cache
        let memory_cache = &self.memory_caches[shard_index];
        memory_cache.insert(key.to_string(), (value.to_string(), expire_duration));



        // Set in Redis if available
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            if let Some(expire) = expire_duration {
                let expire_seconds = expire.as_secs();
                let _: () = connection
                    .set_ex(key, value, expire_seconds)
                    .context("Failed to set value in Redis with expiration")?;
            } else {
                let _: () = connection
                    .set(key, value)
                    .context("Failed to set value in Redis")?;
            }
        }

        // 记录访问模式
        self.update_access_pattern(key);

        // 记录性能指标
        let duration = start.elapsed();
        self.record_cache_metrics(duration, true).await;

        Ok(())
    }

    /// Get value from cache (tries memory first, then Redis)
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let start = Instant::now();
        self.requests.fetch_add(1, Ordering::SeqCst);


        let shard_index = self.get_shard_index(key);
        let memory_cache = &self.memory_caches[shard_index];

        // Try memory cache first
        if let Some((value, _)) = memory_cache.get(key) {
            self.hits.fetch_add(1, Ordering::SeqCst);
            // 更新访问模式
            self.update_access_pattern(key);
            let duration = start.elapsed();
            self.record_cache_metrics(duration, true).await;
            return Ok(Some(value));
        }

        // Try Redis if not in memory
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            let value: Option<String> = connection
                .get(key)
                .context("Failed to get value from Redis")?;

            // If found in Redis, update memory cache
            if let Some(value) = value.clone() {
                self.hits.fetch_add(1, Ordering::SeqCst);
                memory_cache.insert(key.to_string(), (value, None));
                // 更新访问模式
                self.update_access_pattern(key);
            } else {
                self.misses.fetch_add(1, Ordering::SeqCst);
                // 从布隆过滤器中移除不存在的键
                // 注意：布隆过滤器不支持删除操作，这里只是记录
            }

            let duration = start.elapsed();
            self.record_cache_metrics(duration, value.is_some()).await;
            return Ok(value);
        }

        self.misses.fetch_add(1, Ordering::SeqCst);
        let duration = start.elapsed();
        self.record_cache_metrics(duration, false).await;
        Ok(None)
    }

    /// Delete value from cache
    pub async fn delete(&self, key: &str) -> Result<()> {
        let start = Instant::now();
        let shard_index = self.get_shard_index(key);

        // Delete from memory cache
        let memory_cache = &self.memory_caches[shard_index];
        memory_cache.remove(key);

        // Delete from Redis if available
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            let _: () = connection
                .del(key)
                .context("Failed to delete value from Redis")?;
        }

        // 从访问模式中移除
        self.access_patterns.remove(key);

        let duration = start.elapsed();
        self.record_cache_metrics(duration, false).await;

        Ok(())
    }

    /// Clear all cache
    pub async fn clear(&self) -> Result<()> {
        // Clear memory cache
        for cache in &self.memory_caches {
            cache.invalidate_all();
        }

        // Clear Redis if available
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            let _: () = redis::cmd("FLUSHDB")
                .query(&mut connection)
                .context("Failed to flush Redis database")?;
        }

        // Reset statistics
        self.hits.store(0, Ordering::SeqCst);
        self.misses.store(0, Ordering::SeqCst);
        self.requests.store(0, Ordering::SeqCst);
        // 重置布隆过滤器
        // 注意：布隆过滤器不支持重置操作，这里只是创建一个新的
        // 实际实现中可能需要更复杂的处理

        Ok(())
    }

    /// Get memory cache size
    pub fn get_memory_cache_size(&self) -> usize {
        self.memory_caches
            .iter()
            .map(|c| c.entry_count() as usize)
            .sum()
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> (u64, u64, u64, f64) {
        let hits = self.hits.load(Ordering::SeqCst);
        let misses = self.misses.load(Ordering::SeqCst);
        let requests = self.requests.load(Ordering::SeqCst);
        let hit_rate = if requests > 0 {
            (hits as f64) / (requests as f64) * 100.0
        } else {
            0.0
        };

        (hits, misses, requests, hit_rate)
    }

    /// Cache预热 - 批量加载数据
    pub async fn warm_up(&mut self, data: &[(&str, &str, Option<u64>)]) -> Result<()> {
        let start = Instant::now();

        // 批量预热内存缓存
        for (key, value, expire) in data {
            let shard_index = self.get_shard_index(key);
            let memory_cache = &self.memory_caches[shard_index];
            let expire_duration = expire.map(Duration::from_secs);
            memory_cache.insert(key.to_string(), (value.to_string(), expire_duration));
            // 暂时注释掉布隆过滤器，因为API不确定
            // if let Some(filter) = &mut self.bloom_filter {
            //     filter.insert(key.to_string());
            // }
        }

        // 批量预热Redis
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            // 使用管道批量操作，减少网络往返
            let mut pipe = redis::pipe();
            for (key, value, expire) in data {
                if let Some(expire) = expire {
                    pipe.cmd("SETEX").arg(key).arg(expire).arg(value);
                } else {
                    pipe.cmd("SET").arg(key).arg(value);
                }
            }
            let _: () = pipe
                .query(&mut connection)
                .context("Failed to batch set values in Redis")?;
        }

        let duration = start.elapsed();
        self.record_cache_metrics(duration, true).await;

        Ok(())
    }

    /// 批量获取缓存
    pub async fn get_batch(&self, keys: &[&str]) -> Result<Vec<Option<String>>> {
        let start = Instant::now();
        let mut results = Vec::with_capacity(keys.len());

        // 先从内存缓存批量获取
        let mut redis_keys = Vec::new();

        for key in keys {
            let shard_index = self.get_shard_index(key);
            let memory_cache = &self.memory_caches[shard_index];

            if let Some((value, _)) = memory_cache.get(&key.to_string()) {
                results.push(Some(value));
                self.hits.fetch_add(1, Ordering::SeqCst);
                self.update_access_pattern(key);
            } else {
                redis_keys.push(key);
                results.push(None);
            }
        }

        // 从Redis批量获取剩余的键
        if !redis_keys.is_empty()
            && self.get_redis_client().await.is_some()
            && let Some(client) = self.get_redis_client().await
        {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            // 使用MGET批量获取，减少网络往返
            let mut pipe = redis::pipe();
            for key in &redis_keys {
                pipe.cmd("GET").arg(key);
            }
            let values: Vec<Option<String>> = pipe
                .query(&mut connection)
                .context("Failed to batch get values from Redis")?;

            for (i, (key, value)) in redis_keys.iter().zip(values).enumerate() {
                if let Some(value) = value.clone() {
                    let shard_index = self.get_shard_index(key);
                    let memory_cache = &self.memory_caches[shard_index];
                    memory_cache.insert(key.to_string(), (value.clone(), None));
                    results[i] = Some(value);
                    self.hits.fetch_add(1, Ordering::SeqCst);
                    self.update_access_pattern(key);
                } else {
                    self.misses.fetch_add(1, Ordering::SeqCst);
                }
            }
        }

        let duration = start.elapsed();
        self.record_cache_metrics(duration, true).await;

        Ok(results)
    }

    /// 批量删除缓存
    pub async fn delete_batch(&mut self, keys: &[&str]) -> Result<()> {
        let start = Instant::now();

        // 批量删除内存缓存
        for key in keys {
            let shard_index = self.get_shard_index(key);
            let memory_cache = &self.memory_caches[shard_index];
            memory_cache.remove(&key.to_string());
            // 从访问模式中移除
            self.access_patterns.remove(&key.to_string());
        }

        // 批量删除Redis缓存
        if !keys.is_empty()
            && self.get_redis_client().await.is_some()
            && let Some(client) = self.get_redis_client().await
        {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            // 使用DEL批量删除，减少网络往返
            let mut pipe = redis::pipe();
            for key in keys {
                pipe.cmd("DEL").arg(key);
            }
            let _: () = pipe
                .query(&mut connection)
                .context("Failed to batch delete values from Redis")?;
        }

        let duration = start.elapsed();
        self.record_cache_metrics(duration, false).await;

        Ok(())
    }

    /// 生成带前缀的缓存键
    pub fn generate_key(&self, prefix: &str, key: &str) -> String {
        format!("{}:{}", prefix, key)
    }

    /// 检查缓存是否存在
    pub async fn exists(&self, key: &str) -> Result<bool> {
        // 暂时注释掉布隆过滤器检查，因为API不确定
        // if let Some(filter) = &self.bloom_filter {
        //     if !filter.contains(&key.to_string()) {
        //         return Ok(false);
        //     }
        // } else {
        //     return Ok(false);
        // }

        // 检查内存缓存
        let shard_index = self.get_shard_index(key);
        let memory_cache = &self.memory_caches[shard_index];

        if memory_cache.contains_key(key) {
            return Ok(true);
        }

        // 检查Redis
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            let exists: bool = connection
                .exists(key)
                .context("Failed to check key existence in Redis")?;

            return Ok(exists);
        }

        Ok(false)
    }

    /// 获取缓存剩余过期时间（秒）
    pub async fn ttl(&self, key: &str) -> Result<Option<u64>> {
        if let Some(client) = self.get_redis_client().await {
            let mut connection = client
                .get_connection()
                .context("Failed to get Redis connection")?;

            let ttl: Option<u64> = connection
                .ttl(key)
                .context("Failed to get TTL from Redis")?;

            return Ok(ttl);
        }

        Ok(None)
    }

    /// 分析访问模式
    fn analyze_access_pattern(&self, key: &str) -> String {
        if let Some((count, last_access)) = self.access_patterns.get(key) {
            let now = Instant::now();
            let time_since_last_access = now.duration_since(last_access);

            // 根据访问频率和时间判断热度
            if count > 10 && time_since_last_access < Duration::from_secs(5 * 60) {
                "hot".to_string()
            } else if count > 5 && time_since_last_access < Duration::from_secs(3600) {
                "warm".to_string()
            } else {
                "cold".to_string()
            }
        } else {
            "cold".to_string()
        }
    }

    /// 更新访问模式
    fn update_access_pattern(&self, key: &str) {
        let now = Instant::now();

        if let Some((count, _)) = self.access_patterns.get(key) {
            self.access_patterns
                .insert(key.to_string(), (count + 1, now));
        } else {
            self.access_patterns.insert(key.to_string(), (1, now));
        }
    }

    /// 获取缓存配置
    fn get_cache_config(&self, pattern: &str) -> CacheConfig {
        self.cache_configs.get(pattern).cloned().unwrap_or_default()
    }

    /// 记录缓存指标
    async fn record_cache_metrics(&self, duration: Duration, hit: bool) {
        if let Some(monitor) = self.app_state.get_performance_monitor().await {
            monitor.record_cache_operation(duration.as_secs_f64(), hit);
            // 记录缓存命中/未命中
            if hit {
                monitor.record_cache_hit();
            } else {
                monitor.record_cache_miss();
            }
            // 获取缓存命中率
            let (_, _, _, _hit_rate) = self.get_statistics();
        }
    }

    /// 启动自动优化任务
    pub async fn start_auto_optimization(&self) {
        let self_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = interval(self_clone.optimization_interval);
            loop {
                interval.tick().await;
                if self_clone.auto_optimize_enabled
                    && let Err(e) = self_clone.optimize_cache().await
                {
                    eprintln!("Cache optimization failed: {:?}", e);
                }
            }
        });
    }

    /// 执行缓存优化
    pub async fn optimize_cache(&self) -> Result<()> {
        let start = Instant::now();

        // 分析缓存性能
        let performance = self.analyze_cache_performance().await;

        // 调整缓存容量
        self.adjust_cache_capacity(&performance).await;

        // 优化缓存配置
        self.optimize_cache_configs(&performance).await;

        // 更新最后优化时间
        // 注意：这里使用了可变引用，但在实际实现中可能需要使用Arc<RwLock>来保护

        let duration = start.elapsed();
        println!("Cache optimization completed in {:?}", duration);

        Ok(())
    }

    /// 分析缓存性能
    async fn analyze_cache_performance(&self) -> CachePerformance {
        let (hits, misses, requests, hit_rate) = self.get_statistics();
        let memory_cache_size = self.get_memory_cache_size();

        // 分析访问模式
        let hot_keys = self.identify_hot_keys().await;
        let cold_keys = self.identify_cold_keys().await;

        CachePerformance {
            hits,
            misses,
            requests,
            hit_rate,
            memory_cache_size,
            hot_keys_count: hot_keys.len(),
            cold_keys_count: cold_keys.len(),
        }
    }

    /// 识别热门键
    async fn identify_hot_keys(&self) -> Vec<String> {
        let mut hot_keys = Vec::new();

        // 遍历访问模式，找出访问频率高的键
        for entry in self.access_patterns.iter() {
            let (key, (count, last_access)) = entry;
            let now = Instant::now();
            let time_since_last_access = now.duration_since(last_access);

            // 定义热门键的标准：访问次数大于10且最近5分钟内有访问
            if count > 10 && time_since_last_access < Duration::from_secs(5 * 60) {
                hot_keys.push(key.to_string());
            }
        }

        hot_keys
    }

    /// 识别冷门键
    async fn identify_cold_keys(&self) -> Vec<String> {
        let mut cold_keys = Vec::new();

        // 遍历访问模式，找出访问频率低的键
        for entry in self.access_patterns.iter() {
            let (key, (count, last_access)) = entry;
            let now = Instant::now();
            let time_since_last_access = now.duration_since(last_access);

            // 定义冷门键的标准：访问次数小于3且最近1小时内没有访问
            if count < 3 && time_since_last_access > Duration::from_secs(3600) {
                cold_keys.push(key.to_string());
            }
        }

        cold_keys
    }

    /// 调整缓存容量
    async fn adjust_cache_capacity(&self, performance: &CachePerformance) {
        // 根据缓存性能调整内存缓存容量
        let hit_rate = performance.hit_rate;

        if hit_rate < 50.0 {
            // 命中率低，可能需要增加缓存容量
            println!(
                "Cache hit rate is low ({:.2}%), considering increasing cache capacity",
                hit_rate
            );
        } else if hit_rate > 90.0 {
            // 命中率高，可能可以减少缓存容量以节省内存
            println!(
                "Cache hit rate is high ({:.2}%), considering decreasing cache capacity",
                hit_rate
            );
        }
    }

    /// 优化缓存配置
    async fn optimize_cache_configs(&self, _performance: &CachePerformance) {
        // 根据缓存性能优化缓存配置
        // 例如，根据热门键和冷门键的比例调整不同类型缓存的容量
    }

    /// 计算智能TTL
    pub fn calculate_smart_ttl(&self, key: &str) -> Duration {
        if !self.smart_ttl_enabled {
            return Duration::from_secs(3600);
        }

        let access_pattern = self.analyze_access_pattern(key);

        match access_pattern.as_str() {
            "hot" => Duration::from_secs(7200),  // 热门键保存更长时间
            "warm" => Duration::from_secs(3600), // 温键保存中等时间
            "cold" => Duration::from_secs(1800), // 冷键保存较短时间
            _ => Duration::from_secs(3600),      // 默认TTL
        }
    }

    /// 获取缓存淘汰策略
    pub fn get_cache_eviction_policy(&self) -> CacheEvictionPolicy {
        self.cache_eviction_policy.clone()
    }

    /// 设置缓存淘汰策略
    pub fn set_cache_eviction_policy(&mut self, policy: CacheEvictionPolicy) {
        self.cache_eviction_policy = policy;
    }

    /// 启用自动优化
    pub fn enable_auto_optimization(&mut self) {
        self.auto_optimize_enabled = true;
    }

    /// 禁用自动优化
    pub fn disable_auto_optimization(&mut self) {
        self.auto_optimize_enabled = false;
    }

    /// 启用智能TTL
    pub fn enable_smart_ttl(&mut self) {
        self.smart_ttl_enabled = true;
    }

    /// 禁用智能TTL
    pub fn disable_smart_ttl(&mut self) {
        self.smart_ttl_enabled = false;
    }

    /// 启用缓存压缩
    pub fn enable_compression(&mut self) {
        self.compression_enabled = true;
    }

    /// 禁用缓存压缩
    pub fn disable_compression(&mut self) {
        self.compression_enabled = false;
    }

    /// 设置压缩阈值
    pub fn set_compression_threshold(&mut self, threshold: usize) {
        self.compression_threshold = threshold;
    }

    /// 获取详细的缓存统计信息
    pub fn get_cache_statistics(&self) -> CacheStatistics {
        let (hits, misses, requests, hit_rate) = self.get_statistics();
        let memory_cache_size = self.get_memory_cache_size();

        CacheStatistics {
            hits,
            misses,
            requests,
            hit_rate,
            memory_cache_size,
            auto_optimize_enabled: self.auto_optimize_enabled,
            smart_ttl_enabled: self.smart_ttl_enabled,
            compression_enabled: self.compression_enabled,
            cache_eviction_policy: self.cache_eviction_policy.clone(),
        }
    }
}

/// 缓存性能数据
#[derive(Debug)]
pub struct CachePerformance {
    pub hits: u64,
    pub misses: u64,
    pub requests: u64,
    pub hit_rate: f64,
    pub memory_cache_size: usize,
    pub hot_keys_count: usize,
    pub cold_keys_count: usize,
}

/// 缓存统计信息
#[derive(Debug)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub requests: u64,
    pub hit_rate: f64,
    pub memory_cache_size: usize,
    pub auto_optimize_enabled: bool,
    pub smart_ttl_enabled: bool,
    pub compression_enabled: bool,
    pub cache_eviction_policy: CacheEvictionPolicy,
}

