//! Cache optimization module
//! Provides cache policy optimization, cache hit rate improvement, cache preheating and other functions

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptimizerConfig {
    /// Cache size
    pub cache_size: usize,
    /// Minimum cache size
    pub min_cache_size: usize,
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Cache expiration time (seconds)
    pub cache_ttl: u64,
    /// Minimum cache expiration time (seconds)
    pub min_cache_ttl: u64,
    /// Maximum cache expiration time (seconds)
    pub max_cache_ttl: u64,
    /// Enable cache warmup
    pub enable_cache_warmup: bool,
    /// Enable cache eviction
    pub enable_cache_eviction: bool,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Enable cache statistics
    pub enable_cache_stats: bool,
    /// Enable adaptive cache size
    pub enable_adaptive_cache_size: bool,
    /// Enable dynamic TTL
    pub enable_dynamic_ttl: bool,
    /// Enable cache sharding
    pub enable_cache_sharding: bool,
    /// Number of cache shards
    pub cache_shards: usize,
    /// Target hit rate
    pub target_hit_rate: f64,
    /// Hit rate threshold for resizing
    pub hit_rate_threshold: f64,
    /// Memory usage limit (MB)
    pub memory_limit_mb: usize,
}

impl Default for CacheOptimizerConfig {
    fn default() -> Self {
        Self {
            cache_size: 10000,
            min_cache_size: 1000,
            max_cache_size: 100000,
            cache_ttl: 3600,
            min_cache_ttl: 60,
            max_cache_ttl: 86400,
            enable_cache_warmup: true,
            enable_cache_eviction: true,
            eviction_policy: EvictionPolicy::LRU,
            enable_cache_stats: true,
            enable_adaptive_cache_size: true,
            enable_dynamic_ttl: true,
            enable_cache_sharding: false,
            cache_shards: 8,
            target_hit_rate: 80.0,
            hit_rate_threshold: 5.0,
            memory_limit_mb: 100,
        }
    }
}

/// Cache eviction policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least recently used
    LRU,
    /// First in first out
    FIFO,
    /// Least frequently used
    LFU,
    /// Random eviction
    Random,
    /// Adaptive replacement cache (ARC)
    ARC,
    /// Segmented LRU
    SLRU,
    /// Time-aware LRU
    TLRU,
}

/// Cache statistics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    /// Cache hit count
    pub hits: u64,
    /// Cache miss count
    pub misses: u64,
    /// Cache eviction count
    pub evictions: u64,
    /// Cache size
    pub size: usize,
    /// Cache capacity
    pub capacity: usize,
}

impl CacheStatistics {
    /// Calculate cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        (self.hits as f64 / total as f64) * 100.0
    }

    /// Calculate cache usage rate
    pub fn usage_rate(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        (self.size as f64 / self.capacity as f64) * 100.0
    }
}

/// Cache entry
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// Cache value
    value: T,
    /// Creation time
    created_at: Instant,
    /// Last access time
    last_accessed: Instant,
    /// Access count
    access_count: u64,
    /// Expiration time (seconds)
    ttl: u64,
    /// Heat score (used for smart eviction)
    heat_score: f64,
    /// Access pattern (0: cold, 1: warm, 2: hot)
    access_pattern: u8,
    /// Memory usage estimate (bytes)
    memory_usage: usize,
}

/// Cache optimizer
pub struct CacheOptimizer<T> {
    /// Configuration
    config: CacheOptimizerConfig,
    /// Cache storage (sharded if enabled)
    caches: Vec<HashMap<String, CacheEntry<T>>>,
    /// Cache statistics
    stats: CacheStatistics,
    /// Total memory usage (bytes)
    memory_usage: usize,
    /// Last resize time
    last_resize_time: Instant,
    /// Last hit rate
    last_hit_rate: f64,
    /// Access pattern analysis
    access_patterns: HashMap<String, Vec<Instant>>,
}

impl<T: Clone> CacheOptimizer<T> {
    /// Create a new cache optimizer
    pub fn new(config: CacheOptimizerConfig) -> Self {
        let cache_size = config.cache_size;
        let num_shards = if config.enable_cache_sharding {
            config.cache_shards
        } else {
            1
        };

        // Initialize cache shards
        let mut caches = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            caches.push(HashMap::new());
        }

        Self {
            config,
            caches,
            stats: CacheStatistics {
                hits: 0,
                misses: 0,
                evictions: 0,
                size: 0,
                capacity: cache_size,
            },
            memory_usage: 0,
            last_resize_time: Instant::now(),
            last_hit_rate: 0.0,
            access_patterns: HashMap::new(),
        }
    }

    /// Get cache value
    pub fn get(&mut self, key: &str) -> Option<T> {
        // Get shard index
        let shard_index = self.get_shard_index(key);

        // 先检查缓存项是否存在
        let cache = &self.caches[shard_index];
        if !cache.contains_key(key) {
            self.stats.misses += 1;
            info!("Cache miss: {}", key);
            return None;
        }

        // 检查缓存项是否过期
        let entry = cache.get(key).unwrap();
        if entry.created_at.elapsed() > Duration::from_secs(entry.ttl) {
            // 过期了，移除缓存项
            let _ = cache; // 释放不可变借用
            self.remove_from_shard(shard_index, key);
            self.stats.misses += 1;
            return None;
        }

        // 缓存项有效，更新信息
        let now = Instant::now();
        let heat_score = self.calculate_heat_score(entry);
        let access_pattern = self.determine_access_pattern(entry);

        // 释放不可变借用，获取可变借用
        let _ = cache;
        let cache = &mut self.caches[shard_index];
        let entry = cache.get_mut(key).unwrap();

        // 更新访问信息
        entry.last_accessed = now;
        entry.access_count += 1;
        entry.heat_score = heat_score;
        entry.access_pattern = access_pattern;

        // 保存需要返回的值
        let value_clone = entry.value.clone();

        // 释放可变借用，调用其他方法
        let _ = cache;

        // 检查是否需要调整缓存大小或 TTL
        self.adaptive_adjustments();

        // 更新访问模式
        self.update_access_pattern(key, now);

        self.stats.hits += 1;
        info!("Cache hit: {}, heat score: {:.2}", key, heat_score);

        Some(value_clone)
    }

    /// Set cache value
    pub fn set(&mut self, key: String, value: T) {
        // Get shard index
        let shard_index = self.get_shard_index(&key);

        // Calculate total cache size across all shards
        let total_size = self.caches.iter().map(|c| c.len()).sum::<usize>();

        // Check if eviction is needed
        if total_size >= self.config.cache_size {
            if self.config.enable_cache_eviction {
                self.evict();
            } else {
                warn!("Cache is full, cannot add new entry: {}", key);
                return;
            }
        }

        // Calculate dynamic TTL if enabled
        let ttl = if self.config.enable_dynamic_ttl {
            self.calculate_dynamic_ttl(&key)
        } else {
            self.config.cache_ttl
        };

        // Estimate memory usage
        let memory_usage = self.estimate_memory_usage(&key, &value);

        // Check memory limit
        if self.memory_usage + memory_usage > self.config.memory_limit_mb * 1024 * 1024 {
            if self.config.enable_cache_eviction {
                self.evict_by_memory(memory_usage);
            } else {
                warn!("Memory limit exceeded, cannot add new entry: {}", key);
                return;
            }
        }

        // Remove existing entry if it exists
        if let Some(old_entry) = self
            .caches
            .get_mut(shard_index)
            .and_then(|c| c.remove(&key))
        {
            self.memory_usage -= old_entry.memory_usage;
            self.stats.size -= 1;
        }

        // Add cache entry
        let now = Instant::now();
        let cache = &mut self.caches[shard_index];
        cache.insert(
            key.clone(),
            CacheEntry {
                value,
                created_at: now,
                last_accessed: now,
                access_count: 0,
                ttl,
                heat_score: 0.0,
                access_pattern: 0,
                memory_usage,
            },
        );

        // Update statistics
        self.stats.size = self.caches.iter().map(|c| c.len()).sum();
        self.memory_usage += memory_usage;

        // Update access pattern
        self.update_access_pattern(&key, now);

        info!(
            "Cache added: {}, TTL: {}s, Memory: {} bytes",
            key, ttl, memory_usage
        );
    }

    /// Remove cache value
    pub fn remove(&mut self, key: &str) {
        let shard_index = self.get_shard_index(key);
        self.remove_from_shard(shard_index, key);
    }

    /// Remove cache value from specific shard
    fn remove_from_shard(&mut self, shard_index: usize, key: &str) {
        let cache = &mut self.caches[shard_index];
        if let Some(entry) = cache.remove(key) {
            self.memory_usage -= entry.memory_usage;
            // 更新 stats.size，而不是重新计算所有缓存的大小
            if self.stats.size > 0 {
                self.stats.size -= 1;
            }
            info!(
                "Cache removed: {}, Memory freed: {} bytes",
                key, entry.memory_usage
            );
        }
    }

    /// Clear cache
    pub fn clear(&mut self) {
        for cache in &mut self.caches {
            cache.clear();
        }
        self.stats.size = 0;
        self.memory_usage = 0;
        self.access_patterns.clear();
        info!("Cache cleared, Memory freed: {} bytes", self.memory_usage);
    }

    /// Evict cache entry
    fn evict(&mut self) {
        if self.stats.size == 0 {
            return;
        }

        // Find the best entry to evict across all shards
        let mut best_entry: Option<(usize, String, f64)> = None; // (shard_index, key, eviction_score)

        for (shard_index, cache) in self.caches.iter().enumerate() {
            for (key, entry) in cache.iter() {
                let eviction_score = self.calculate_eviction_score(entry);

                // Select the entry with the lowest score (most evictable)
                if best_entry.is_none() || eviction_score < best_entry.as_ref().unwrap().2 {
                    best_entry = Some((shard_index, key.clone(), eviction_score));
                }
            }
        }

        if let Some((shard_index, key, score)) = best_entry {
            self.remove_from_shard(shard_index, &key);
            self.stats.evictions += 1;
            info!(
                "Cache evicted: {}, policy: {:?}, score: {:.2}",
                key, self.config.eviction_policy, score
            );
        }
    }

    /// Evict cache entries by memory usage
    fn evict_by_memory(&mut self, required_memory: usize) {
        let target_memory = self.memory_usage + required_memory;
        let max_memory = self.config.memory_limit_mb * 1024 * 1024;

        if target_memory <= max_memory {
            return;
        }

        let mut evict_memory = target_memory - max_memory;

        // Evict entries until we have enough memory
        while evict_memory > 0 && self.stats.size > 0 {
            // Find the largest entry to evict
            let mut largest_entry: Option<(usize, String, usize)> = None; // (shard_index, key, memory_usage)

            for (shard_index, cache) in self.caches.iter().enumerate() {
                for (key, entry) in cache.iter() {
                    if largest_entry.is_none()
                        || entry.memory_usage > largest_entry.as_ref().unwrap().2
                    {
                        largest_entry = Some((shard_index, key.clone(), entry.memory_usage));
                    }
                }
            }

            if let Some((shard_index, key, memory)) = largest_entry {
                self.remove_from_shard(shard_index, &key);
                self.stats.evictions += 1;
                evict_memory -= memory;
                info!(
                    "Cache evicted by memory: {}, Memory freed: {} bytes",
                    key, memory
                );
            } else {
                break;
            }
        }
    }

    /// Calculate eviction score for an entry
    fn calculate_eviction_score<U>(&self, entry: &CacheEntry<U>) -> f64 {
        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                // Higher score means more recently used (less evictable)
                entry.last_accessed.elapsed().as_secs_f64()
            }
            EvictionPolicy::FIFO => {
                // Higher score means more recently created (less evictable)
                entry.created_at.elapsed().as_secs_f64()
            }
            EvictionPolicy::LFU => {
                // Higher score means more frequently used (less evictable)
                -(entry.access_count as f64)
            }
            EvictionPolicy::Random => {
                // Random score
                #[cfg(feature = "rand")]
                {
                    rand::random::<f64>()
                }
                #[cfg(not(feature = "rand"))]
                {
                    // Fallback to a simple time-based random score
                    let now = Instant::now();
                    let seed = now.elapsed().as_nanos() as u64;
                    let hash = seed.wrapping_mul(1103515245).wrapping_add(12345);
                    (hash % 1000) as f64 / 1000.0
                }
            }
            EvictionPolicy::ARC => {
                // Adaptive replacement cache score
                // Combine LRU and LFU scores
                let lru_score = entry.last_accessed.elapsed().as_secs_f64();
                let lfu_score = entry.access_count as f64;
                lru_score * 0.5 + lfu_score * 0.5
            }
            EvictionPolicy::SLRU => {
                // Segmented LRU score
                // Cold entries are more evictable
                entry.access_pattern as f64 * 10.0 + entry.last_accessed.elapsed().as_secs_f64()
            }
            EvictionPolicy::TLRU => {
                // Time-aware LRU score
                // Consider both access time and TTL
                let time_score = entry.last_accessed.elapsed().as_secs_f64();
                let ttl_score = entry.ttl as f64;
                time_score / ttl_score
            }
        }
    }

    /// Calculate heat score for an entry
    fn calculate_heat_score<U>(&self, entry: &CacheEntry<U>) -> f64 {
        // Calculate heat based on access frequency and recency
        let recency = 1.0 / (1.0 + entry.last_accessed.elapsed().as_secs_f64() / 3600.0);
        let frequency =
            entry.access_count as f64 / (1.0 + entry.created_at.elapsed().as_secs_f64() / 3600.0);
        let time_since_creation = entry.created_at.elapsed().as_secs_f64() / 3600.0;

        // Weight factors
        let recency_weight = 0.4;
        let frequency_weight = 0.4;
        let age_weight = 0.2;

        // Calculate weighted score
        (recency * recency_weight)
            + (frequency * frequency_weight)
            + (1.0 / (1.0 + time_since_creation) * age_weight)
    }

    /// Determine access pattern for an entry
    fn determine_access_pattern<U>(&self, entry: &CacheEntry<U>) -> u8 {
        let heat = entry.heat_score;
        if heat > 0.7 {
            2 // Hot
        } else if heat > 0.3 {
            1 // Warm
        } else {
            0 // Cold
        }
    }

    /// Calculate dynamic TTL for a key
    fn calculate_dynamic_ttl(&self, key: &str) -> u64 {
        // Base TTL on access pattern
        let access_history = self.access_patterns.get(key);

        if let Some(history) = access_history {
            // Calculate access frequency
            let now = Instant::now();
            let recent_accesses = history
                .iter()
                .filter(|&time| now.duration_since(*time).as_secs() < 3600)
                .count();

            // Adjust TTL based on access frequency
            match recent_accesses {
                0 => self.config.min_cache_ttl, // No recent accesses
                1..=5 => self.config.cache_ttl, // Moderate accesses
                _ => self.config.max_cache_ttl, // Frequent accesses
            }
        } else {
            // Default TTL for new keys
            self.config.cache_ttl
        }
    }

    /// Update access pattern for a key
    fn update_access_pattern(&mut self, key: &str, time: Instant) {
        let history = self.access_patterns.entry(key.to_string()).or_default();
        history.push(time);

        // Keep only the last 100 access times
        if history.len() > 100 {
            history.drain(0..history.len() - 100);
        }
    }

    /// Estimate memory usage for a key-value pair
    fn estimate_memory_usage<U>(&self, key: &str, _value: &U) -> usize {
        // Simple memory usage estimate
        let key_size = key.len();
        let value_size = std::mem::size_of::<T>();
        key_size + value_size
    }

    /// Get shard index for a key
    fn get_shard_index(&self, key: &str) -> usize {
        if !self.config.enable_cache_sharding || self.caches.len() == 1 {
            return 0;
        }

        // Simple hash-based sharding
        let hash = key
            .bytes()
            .fold(0usize, |acc, b| acc.wrapping_add(b as usize));
        hash % self.caches.len()
    }

    /// Adaptive adjustments to cache size and TTL
    fn adaptive_adjustments(&mut self) {
        // Check if we need to adjust cache size
        if self.config.enable_adaptive_cache_size && self.last_resize_time.elapsed().as_secs() > 300
        {
            // Adjust every 5 minutes

            let current_hit_rate = self.stats.hit_rate();
            let hit_rate_diff = (current_hit_rate - self.last_hit_rate).abs();

            if hit_rate_diff > self.config.hit_rate_threshold {
                let mut new_size = self.config.cache_size;

                if current_hit_rate < self.config.target_hit_rate {
                    // Increase cache size
                    new_size =
                        (new_size as f64 * 1.1).min(self.config.max_cache_size as f64) as usize;
                } else if current_hit_rate > self.config.target_hit_rate + 10.0 {
                    // Decrease cache size
                    new_size =
                        (new_size as f64 * 0.9).max(self.config.min_cache_size as f64) as usize;
                }

                if new_size != self.config.cache_size {
                    info!(
                        "Adjusting cache size from {} to {} based on hit rate: {:.2}%",
                        self.config.cache_size, new_size, current_hit_rate
                    );
                    self.config.cache_size = new_size;
                    self.stats.capacity = new_size;
                }

                self.last_hit_rate = current_hit_rate;
                self.last_resize_time = Instant::now();
            }
        }
    }

    /// Warmup cache
    pub fn warmup(&mut self, keys: Vec<String>, values: Vec<T>) {
        if !self.config.enable_cache_warmup {
            info!("Cache warmup is disabled");
            return;
        }

        info!("Starting cache warmup, entry count: {}", keys.len());

        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            self.set(key, value);
        }

        let total_size = self.caches.iter().map(|c| c.len()).sum::<usize>();
        info!(
            "Cache warmup completed, current cache size: {}, Memory usage: {} bytes",
            total_size, self.memory_usage
        );
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> &CacheStatistics {
        &self.stats
    }

    /// Generate optimization report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Cache Optimization Report ===\n\n");

        // Cache statistics
        report.push_str("[Cache Statistics]\n");
        report.push_str(&format!(
            "Cache size: {}/{}\n",
            self.stats.size, self.stats.capacity
        ));
        report.push_str(&format!("Cache hit rate: {:.2}%\n", self.stats.hit_rate()));
        report.push_str(&format!(
            "Cache usage rate: {:.2}%\n",
            self.stats.usage_rate()
        ));
        report.push_str(&format!("Cache hit count: {}\n", self.stats.hits));
        report.push_str(&format!("Cache miss count: {}\n", self.stats.misses));
        report.push_str(&format!("Cache eviction count: {}\n", self.stats.evictions));
        report.push_str(&format!(
            "Memory usage: {} bytes / {} MB\n\n",
            self.memory_usage, self.config.memory_limit_mb
        ));

        // Cache configuration
        report.push_str("[Cache Configuration]\n");
        report.push_str(&format!(
            "Eviction policy: {:?}\n",
            self.config.eviction_policy
        ));
        report.push_str(&format!(
            "Enable adaptive cache size: {}\n",
            self.config.enable_adaptive_cache_size
        ));
        report.push_str(&format!(
            "Enable dynamic TTL: {}\n",
            self.config.enable_dynamic_ttl
        ));
        report.push_str(&format!(
            "Enable cache sharding: {}\n",
            self.config.enable_cache_sharding
        ));
        report.push_str(&format!("Cache shards: {}\n\n", self.config.cache_shards));

        // Performance evaluation
        let hit_rate = self.stats.hit_rate();
        if hit_rate >= 80.0 {
            report.push_str("[Performance Evaluation] Cache performance is excellent\n\n");
        } else if hit_rate >= 60.0 {
            report.push_str("[Performance Evaluation] Cache performance is good\n\n");
        } else if hit_rate >= 40.0 {
            report.push_str(
                "[Performance Evaluation] Cache performance is average, suggest optimization\n\n",
            );
        } else {
            report.push_str(
                "[Performance Evaluation] Cache performance is poor, needs improvement\n\n",
            );
        }

        // Optimization suggestions
        report.push_str("[Optimization Suggestions]\n");

        if hit_rate < 60.0 {
            report.push_str("1. Increase cache size\n");
            report.push_str("2. Optimize cache key design\n");
            report.push_str("3. Use multi-level cache\n");
            report.push_str("4. Preheat hot data\n");
            report.push_str("5. Enable adaptive cache size\n");
        }

        if self.stats.evictions > self.stats.hits / 10 {
            report.push_str("6. Adjust cache eviction policy\n");
            report.push_str("7. Increase cache capacity\n");
            report.push_str("8. Enable cache sharding\n");
        }

        if self.stats.usage_rate() > 90.0 {
            report.push_str("9. Clean expired cache regularly\n");
            report.push_str("10. Implement cache sharding\n");
        }

        if self.memory_usage > self.config.memory_limit_mb * 1024 * 1024 * 90 / 100 {
            report.push_str("11. Increase memory limit\n");
            report.push_str("12. Optimize memory usage of cached objects\n");
        }

        report.push_str("13. Regularly monitor cache hit rate\n");
        report.push_str("14. Use cache analysis tools\n");
        report.push_str("15. Enable dynamic TTL for better cache efficiency\n");

        report
    }
}

impl<T: Clone> Default for CacheOptimizer<T> {
    fn default() -> Self {
        Self::new(CacheOptimizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_optimizer() {
        let mut optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());

        // Test setting and getting
        optimizer.set("key1".to_string(), "value1");
        assert_eq!(optimizer.get("key1"), Some("value1"));
        assert_eq!(optimizer.get("key2"), None);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());

        // Set some caches
        optimizer.set("key1".to_string(), "value1");
        optimizer.set("key2".to_string(), "value2");
        optimizer.set("key3".to_string(), "value3");

        // Test hit rate
        optimizer.get("key1");
        optimizer.get("key2");
        optimizer.get("key3");
        optimizer.get("key4");

        let stats = optimizer.get_statistics();
        assert_eq!(stats.hits, 3);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate(), 75.0);
    }

    #[test]
    fn test_cache_eviction() {
        let mut config = CacheOptimizerConfig::default();
        config.cache_size = 2;
        let mut optimizer = CacheOptimizer::new(config);

        // Test cache eviction
        optimizer.set("key1".to_string(), "value1");
        optimizer.set("key2".to_string(), "value2");
        optimizer.set("key3".to_string(), "value3");

        assert_eq!(optimizer.get_statistics().size, 2);
        assert_eq!(optimizer.get_statistics().evictions, 1);
    }

    #[test]
    fn test_cache_warmup() {
        let mut optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());

        // Test cache warmup
        let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
        let values = vec!["value1", "value2", "value3"];

        optimizer.warmup(keys, values);

        assert_eq!(optimizer.get_statistics().size, 3);
    }

    #[test]
    fn test_generate_report() {
        let mut optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());

        // Set some caches
        optimizer.set("key1".to_string(), "value1");
        optimizer.get("key1");
        optimizer.get("key2");

        let report = optimizer.generate_report();

        assert!(report.contains("Cache Optimization Report"));
        assert!(report.contains("Cache Statistics"));
    }
}
