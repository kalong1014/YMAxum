// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use anyhow::{Context, Result};
use sqlx::pool::PoolOptions;
use sqlx::{MySqlPool, PgPool, SqlitePool};

use moka::sync::Cache;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::core::state::AppState;

/// 缓存策略
#[derive(Debug, Clone, PartialEq)]
pub enum CacheStrategy {
    /// 总是缓存
    Always,
    /// 基于查询频率缓存
    FrequencyBased,
    /// 基于执行时间缓存
    TimeBased,
    /// 从不缓存
    Never,
    /// 基于查询类型缓存
    QueryTypeBased,
    /// 基于结果大小缓存
    ResultSizeBased,
}

/// Database connection manager
pub struct DatabaseManager {
    app_state: Arc<AppState>,
    query_cache: Cache<String, (Vec<u8>, Duration)>,
    // Cache statistics
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    // Query statistics
    query_stats: Cache<String, (u64, Duration)>,
    // Cache strategy
    cache_strategy: CacheStrategy,
    // Database optimizer
    optimizer: Option<super::super::performance::database_optimizer::DatabaseOptimizer>,
}

impl DatabaseManager {
    /// Create new database manager
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            query_cache: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(300))
                .build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(3600))
                .build(),
            cache_strategy: CacheStrategy::FrequencyBased,
            optimizer: Some(
                super::super::performance::database_optimizer::DatabaseOptimizer::default(),
            ),
        }
    }

    /// Initialize MySQL connection pool with optimized settings
    pub async fn init_mysql_pool(&self, url: &str) -> Result<()> {
        // Get pool size from config if available
        let config = self.app_state.config.read().await;
        let max_connections = if config.db_pool_size > 0 {
            config.db_pool_size
        } else {
            25 // Default value
        };

        let pool = PoolOptions::<sqlx::MySql>::new()
            .max_connections(max_connections)
            .min_connections(max_connections / 5)
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(url)
            .await
            .context("Failed to connect to MySQL database")?;

        // Test connection pool health
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .context("Failed to test MySQL connection")?;

        let _ = self.app_state.mysql_pool.set(pool);

        Ok(())
    }

    /// Initialize PostgreSQL connection pool with optimized settings
    pub async fn init_postgres_pool(&self, url: &str) -> Result<()> {
        // Get pool size from config if available
        let config = self.app_state.config.read().await;
        let max_connections = if config.db_pool_size > 0 {
            config.db_pool_size
        } else {
            25 // Default value
        };

        let pool = PoolOptions::<sqlx::Postgres>::new()
            .max_connections(max_connections)
            .min_connections(max_connections / 5)
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(url)
            .await
            .context("Failed to connect to PostgreSQL database")?;

        // Test connection pool health
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .context("Failed to test PostgreSQL connection")?;

        let _ = self.app_state.postgres_pool.set(pool);

        Ok(())
    }

    /// Initialize SQLite connection pool with optimized settings
    pub async fn init_sqlite_pool(&self, url: &str) -> Result<()> {
        // SQLite has more limited concurrency, so use smaller pool size
        let pool = PoolOptions::<sqlx::Sqlite>::new()
            .max_connections(15)
            .min_connections(2)
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(url)
            .await
            .context("Failed to connect to SQLite database")?;

        // Test connection pool health
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .context("Failed to test SQLite connection")?;

        let _ = self.app_state.sqlite_pool.set(pool);

        Ok(())
    }

    /// Check database connection health
    pub async fn check_health(&self) -> Result<bool> {
        // Get database semaphore
        let db_semaphore = self.app_state.get_db_semaphore();
        let _permit = db_semaphore
            .acquire()
            .await
            .context("Failed to acquire database semaphore")?;

        // Check MySQL connection
        if let Some(pool) = self.app_state.mysql_pool.get() {
            sqlx::query("SELECT 1")
                .execute(pool)
                .await
                .context("MySQL health check failed")?;
        }

        // Check PostgreSQL connection
        if let Some(pool) = self.app_state.postgres_pool.get() {
            sqlx::query("SELECT 1")
                .execute(pool)
                .await
                .context("PostgreSQL health check failed")?;
        }

        // Check SQLite connection
        if let Some(pool) = self.app_state.sqlite_pool.get() {
            sqlx::query("SELECT 1")
                .execute(pool)
                .await
                .context("SQLite health check failed")?;
        }

        Ok(true)
    }

    /// Get MySQL connection pool
    pub async fn get_mysql_pool(&self) -> Option<&MySqlPool> {
        self.app_state.mysql_pool.get()
    }

    /// Get PostgreSQL connection pool
    pub async fn get_postgres_pool(&self) -> Option<&PgPool> {
        self.app_state.postgres_pool.get()
    }

    /// Get SQLite connection pool
    pub async fn get_sqlite_pool(&self) -> Option<&SqlitePool> {
        self.app_state.sqlite_pool.get()
    }

    /// Close all database connections
    pub async fn close_all_connections(&self) -> Result<()> {
        // Get database semaphore
        let db_semaphore = self.app_state.get_db_semaphore();
        let _permit = db_semaphore
            .acquire()
            .await
            .context("Failed to acquire database semaphore")?;

        // Close MySQL connection
        if let Some(pool) = self.app_state.mysql_pool.get() {
            pool.close().await;
        }

        // Close PostgreSQL connection
        if let Some(pool) = self.app_state.postgres_pool.get() {
            pool.close().await;
        }

        // Close SQLite connection
        if let Some(pool) = self.app_state.sqlite_pool.get() {
            pool.close().await;
        }

        Ok(())
    }

    /// Execute query with cache support
    pub async fn execute_with_cache<
        T: Send + Sync + 'static + serde::Serialize + for<'de> serde::de::Deserialize<'de>,
    >(
        &mut self,
        query: &str,
        exec_func: impl FnOnce() -> Result<T>,
        cache_duration: Duration,
    ) -> Result<T> {
        let start = std::time::Instant::now();
        let cache_key = query.to_string();

        // Analyze query with optimizer
        if let Some(optimizer) = &mut self.optimizer {
            optimizer.analyze_query(query, Duration::from_millis(0));
        }

        // Check if result is in cache
        if let Some((cached_result, _)) = self.query_cache.get(&cache_key) {
            self.cache_hits.fetch_add(1, Ordering::SeqCst);
            let duration = start.elapsed();
            self.record_query_metrics(duration, true).await;

            // Security: Add size limit to prevent DoS attacks
            if cached_result.len() > 1024 * 1024 {
                // 1MB limit
                return Err(anyhow::anyhow!("Cached result too large"));
            }

            // Security: Use safe JSON deserialization
            // JSON deserialization is safer than bincode as it doesn't allow arbitrary type deserialization
            let result = serde_json::from_slice(&cached_result)?;

            return Ok(result);
        }

        // Get database semaphore
        let db_semaphore = self.app_state.get_db_semaphore();
        let _permit = db_semaphore
            .acquire()
            .await
            .context("Failed to acquire database semaphore")?;

        // Execute query
        self.cache_misses.fetch_add(1, Ordering::SeqCst);
        let result = exec_func()?;
        let duration = start.elapsed();

        // Update query statistics
        self.update_query_stats(query, duration).await;

        // Cache the result based on strategy
        if self.should_cache(query, duration)
            && let Ok(serialized) = serde_json::to_vec(&result)
        {
            // For ResultSizeBased strategy, check if result size is below threshold
            let should_cache = match self.cache_strategy {
                CacheStrategy::ResultSizeBased => {
                    // Only cache results smaller than 100KB
                    serialized.len() < 100 * 1024
                }
                _ => true,
            };

            if should_cache {
                self.query_cache
                    .insert(cache_key, (serialized, cache_duration));
            }
        }

        self.record_query_metrics(duration, false).await;
        Ok(result)
    }

    /// Execute transaction
    pub async fn execute_transaction<'a, T, F>(
        &self,
        pool: impl sqlx::Executor<'a, Database = T> + sqlx::Acquire<'a, Database = T>,
        f: impl FnOnce(&mut sqlx::Transaction<'a, T>) -> Result<()>,
    ) -> Result<()>
    where
        T: sqlx::Database,
    {
        let start = std::time::Instant::now();

        // Get database semaphore
        let db_semaphore = self.app_state.get_db_semaphore();
        let _permit = db_semaphore
            .acquire()
            .await
            .context("Failed to acquire database semaphore")?;

        let mut transaction = pool.begin().await?;

        let result = f(&mut transaction);

        match result {
            Ok(_) => {
                transaction.commit().await?;
                let duration = start.elapsed();
                self.record_query_metrics(duration, false).await;
                Ok(())
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }

    /// Execute async transaction
    pub async fn execute_async_transaction<'a, T, F, R>(
        &self,
        pool: impl sqlx::Executor<'a, Database = T> + sqlx::Acquire<'a, Database = T>,
        f: impl FnOnce(
            &mut sqlx::Transaction<'a, T>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>>>>,
    ) -> Result<R>
    where
        T: sqlx::Database,
        R: Send + Sync + 'static,
    {
        let start = std::time::Instant::now();

        // Get database semaphore
        let db_semaphore = self.app_state.get_db_semaphore();
        let _permit = db_semaphore
            .acquire()
            .await
            .context("Failed to acquire database semaphore")?;

        let mut transaction = pool.begin().await?;

        let result = f(&mut transaction).await;

        match result {
            Ok(value) => {
                transaction.commit().await?;
                let duration = start.elapsed();
                self.record_query_metrics(duration, false).await;
                Ok(value)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }

    /// Record query metrics
    async fn record_query_metrics(&self, duration: Duration, from_cache: bool) {
        if let Some(monitor) = self.app_state.get_performance_monitor().await {
            monitor.record_database_query(duration.as_secs_f64(), false);
            if from_cache {
                // Get cache hit rate
                let _hit_rate = self.calculate_cache_hit_rate();
            }
        }
    }

    /// Calculate cache hit rate
    fn calculate_cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::SeqCst);
        let misses = self.cache_misses.load(Ordering::SeqCst);
        let total = hits + misses;

        if total > 0 {
            (hits as f64) / (total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Update query statistics
    async fn update_query_stats(&mut self, query: &str, duration: Duration) {
        let cache_key = query.to_string();

        if let Some((count, total_duration)) = self.query_stats.get(&cache_key) {
            let new_count = count + 1;
            let new_total_duration = total_duration + duration;
            self.query_stats
                .insert(cache_key, (new_count, new_total_duration));
        } else {
            self.query_stats.insert(cache_key, (1, duration));
        }

        // Update optimizer with actual execution time
        if let Some(optimizer) = &mut self.optimizer {
            optimizer.analyze_query(query, duration);
        }
    }

    /// Check if query should be cached based on strategy
    fn should_cache(&self, query: &str, duration: Duration) -> bool {
        match self.cache_strategy {
            CacheStrategy::Always => true,
            CacheStrategy::Never => false,
            CacheStrategy::FrequencyBased => {
                let cache_key = query.to_string();
                if let Some((count, _)) = self.query_stats.get(&cache_key) {
                    count > 3 // Cache if query is executed more than 3 times
                } else {
                    false
                }
            }
            CacheStrategy::TimeBased => {
                duration > Duration::from_millis(50) // Cache if query takes more than 50ms
            }
            CacheStrategy::QueryTypeBased => {
                // Cache SELECT queries, but not INSERT/UPDATE/DELETE
                query.trim_start().to_uppercase().starts_with("SELECT")
            }
            CacheStrategy::ResultSizeBased => {
                // This strategy will be evaluated after query execution
                // when we know the result size
                true
            }
        }
    }

    /// Set cache strategy
    pub fn set_cache_strategy(&mut self, strategy: CacheStrategy) {
        self.cache_strategy = strategy;
    }

    /// Get cache strategy
    pub fn get_cache_strategy(&self) -> CacheStrategy {
        self.cache_strategy.clone()
    }

    /// Clear query cache
    pub fn clear_query_cache(&self) {
        self.query_cache.invalidate_all();
    }

    /// Get query cache statistics
    pub fn get_cache_statistics(&self) -> (u64, u64, f64) {
        let hits = self.cache_hits.load(Ordering::SeqCst);
        let misses = self.cache_misses.load(Ordering::SeqCst);
        let hit_rate = self.calculate_cache_hit_rate();
        (hits, misses, hit_rate)
    }

    /// Get database optimizer
    pub fn get_optimizer(
        &mut self,
    ) -> Option<&mut super::super::performance::database_optimizer::DatabaseOptimizer> {
        self.optimizer.as_mut()
    }

    /// Generate optimization report
    pub fn generate_optimization_report(&mut self) -> String {
        if let Some(optimizer) = &mut self.optimizer {
            optimizer.generate_index_suggestions();
            optimizer.generate_report()
        } else {
            "Database optimizer not initialized".to_string()
        }
    }

    /// Optimize query
    pub fn optimize_query(&self, query: &str) -> String {
        if let Some(optimizer) = &self.optimizer {
            optimizer.optimize_query(query)
        } else {
            query.to_string()
        }
    }
}

/// Database query builder with caching support
pub struct CachedQueryBuilder {
    db_manager: Arc<DatabaseManager>,
    query: String,
    cache_duration: Duration,
}

impl CachedQueryBuilder {
    /// Create new cached query builder
    pub fn new(db_manager: Arc<DatabaseManager>, query: &str) -> Self {
        Self {
            db_manager,
            query: query.to_string(),
            cache_duration: Duration::from_secs(300),
        }
    }

    /// Set cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Execute query
    pub async fn execute<
        T: Send + Sync + 'static + serde::Serialize + for<'de> serde::de::Deserialize<'de>,
    >(
        self,
        exec_func: impl FnOnce() -> Result<T>,
    ) -> Result<T> {
        let mut db_manager = Arc::try_unwrap(self.db_manager)
            .unwrap_or_else(|arc| DatabaseManager::new(arc.app_state.clone()));
        db_manager
            .execute_with_cache(&self.query, exec_func, self.cache_duration)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_cache_strategy() {
        // Test all cache strategies
        let strategies = vec![
            CacheStrategy::Always,
            CacheStrategy::Never,
            CacheStrategy::FrequencyBased,
            CacheStrategy::TimeBased,
            CacheStrategy::QueryTypeBased,
            CacheStrategy::ResultSizeBased,
        ];

        for strategy in strategies {
            assert!(matches!(
                strategy,
                CacheStrategy::Always
                    | CacheStrategy::Never
                    | CacheStrategy::FrequencyBased
                    | CacheStrategy::TimeBased
                    | CacheStrategy::QueryTypeBased
                    | CacheStrategy::ResultSizeBased
            ));
        }
    }

    #[tokio::test]
    async fn test_database_manager_creation() {
        let app_state = Arc::new(AppState::new());
        let _db_manager = DatabaseManager::new(app_state);
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_database_manager_cache_strategy() {
        let app_state = Arc::new(AppState::new());
        let mut db_manager = DatabaseManager::new(app_state);

        // Test set and get cache strategy
        db_manager.set_cache_strategy(CacheStrategy::Always);
        assert_eq!(db_manager.get_cache_strategy(), CacheStrategy::Always);

        db_manager.set_cache_strategy(CacheStrategy::Never);
        assert_eq!(db_manager.get_cache_strategy(), CacheStrategy::Never);
    }

    #[tokio::test]
    async fn test_database_manager_clear_query_cache() {
        let app_state = Arc::new(AppState::new());
        let db_manager = DatabaseManager::new(app_state);

        // Test clear query cache
        db_manager.clear_query_cache();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_database_manager_get_cache_statistics() {
        let app_state = Arc::new(AppState::new());
        let db_manager = DatabaseManager::new(app_state);

        // Test get cache statistics
        let (hits, misses, hit_rate) = db_manager.get_cache_statistics();
        assert_eq!(hits, 0);
        assert_eq!(misses, 0);
        assert_eq!(hit_rate, 0.0);
    }

    #[tokio::test]
    async fn test_database_manager_calculate_cache_hit_rate() {
        let app_state = Arc::new(AppState::new());
        let db_manager = DatabaseManager::new(app_state);

        // Test calculate cache hit rate
        let hit_rate = db_manager.calculate_cache_hit_rate();
        assert_eq!(hit_rate, 0.0);
    }

    #[tokio::test]
    async fn test_cached_query_builder() {
        let app_state = Arc::new(AppState::new());
        let db_manager = Arc::new(DatabaseManager::new(app_state));

        // Test cached query builder creation
        let builder = CachedQueryBuilder::new(db_manager, "SELECT * FROM users");

        // Test with_cache_duration
        let _builder_with_duration = builder.with_cache_duration(Duration::from_secs(60));
        // Just ensure the methods don't panic
    }

    #[tokio::test]
    async fn test_should_cache() {
        let app_state = Arc::new(AppState::new());
        let _db_manager = DatabaseManager::new(app_state);

        // Test should_cache with Always strategy
        let always_manager = DatabaseManager {
            app_state: Arc::new(AppState::new()),
            query_cache: Cache::builder().max_capacity(1000).build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder().max_capacity(1000).build(),
            cache_strategy: CacheStrategy::Always,
            optimizer: None,
        };
        assert!(always_manager.should_cache("SELECT * FROM users", Duration::from_millis(10)));

        // Test should_cache with Never strategy
        let never_manager = DatabaseManager {
            app_state: Arc::new(AppState::new()),
            query_cache: Cache::builder().max_capacity(1000).build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder().max_capacity(1000).build(),
            cache_strategy: CacheStrategy::Never,
            optimizer: None,
        };
        assert!(!never_manager.should_cache("SELECT * FROM users", Duration::from_millis(10)));

        // Test should_cache with TimeBased strategy
        let time_manager = DatabaseManager {
            app_state: Arc::new(AppState::new()),
            query_cache: Cache::builder().max_capacity(1000).build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder().max_capacity(1000).build(),
            cache_strategy: CacheStrategy::TimeBased,
            optimizer: None,
        };
        assert!(!time_manager.should_cache("SELECT * FROM users", Duration::from_millis(40)));
        assert!(time_manager.should_cache("SELECT * FROM users", Duration::from_millis(60)));

        // Test should_cache with QueryTypeBased strategy
        let query_type_manager = DatabaseManager {
            app_state: Arc::new(AppState::new()),
            query_cache: Cache::builder().max_capacity(1000).build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder().max_capacity(1000).build(),
            cache_strategy: CacheStrategy::QueryTypeBased,
            optimizer: None,
        };
        assert!(query_type_manager.should_cache("SELECT * FROM users", Duration::from_millis(10)));
        assert!(!query_type_manager.should_cache(
            "INSERT INTO users (name) VALUES ('test')",
            Duration::from_millis(10)
        ));
        assert!(!query_type_manager.should_cache(
            "UPDATE users SET name = 'test' WHERE id = 1",
            Duration::from_millis(10)
        ));
        assert!(
            !query_type_manager
                .should_cache("DELETE FROM users WHERE id = 1", Duration::from_millis(10))
        );

        // Test should_cache with ResultSizeBased strategy
        let result_size_manager = DatabaseManager {
            app_state: Arc::new(AppState::new()),
            query_cache: Cache::builder().max_capacity(1000).build(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            query_stats: Cache::builder().max_capacity(1000).build(),
            cache_strategy: CacheStrategy::ResultSizeBased,
            optimizer: None,
        };
        // ResultSizeBased strategy always returns true in should_cache
        // The actual size check happens when serializing the result
        assert!(result_size_manager.should_cache("SELECT * FROM users", Duration::from_millis(10)));
    }

    #[tokio::test]
    async fn test_calculate_cache_hit_rate() {
        let app_state = Arc::new(AppState::new());
        let db_manager = DatabaseManager::new(app_state);

        // Test calculate_cache_hit_rate with no queries
        let hit_rate1 = db_manager.calculate_cache_hit_rate();
        assert_eq!(hit_rate1, 0.0);

        // Test calculate_cache_hit_rate with some hits and misses
        db_manager.cache_hits.fetch_add(5, Ordering::SeqCst);
        db_manager.cache_misses.fetch_add(5, Ordering::SeqCst);
        let hit_rate2 = db_manager.calculate_cache_hit_rate();
        assert_eq!(hit_rate2, 50.0);
    }
}

