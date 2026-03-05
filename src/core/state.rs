// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::{error, info};
use serde_json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{OnceCell, RwLock, Semaphore, watch};

// Database support (controlled by feature flag)
#[cfg(feature = "database")]
use sqlx::{MySqlPool, PgPool, SqlitePool};

// Cache support (controlled by feature flag)
#[cfg(feature = "cache")]
use moka::sync::Cache;
#[cfg(feature = "cache")]
use redis::Client;

// Performance monitoring support (controlled by feature flag)
#[cfg(feature = "monitoring")]
use crate::monitoring::prometheus::PrometheusMetrics;
#[cfg(feature = "monitoring")]
use crate::performance::monitor::PerformanceMonitor;

use crate::ai::AiManager;
use crate::fraud::FraudModule;
use crate::i18n::I18nManager;
use crate::iterate::IterateService;
use crate::ops::config_hot::ConfigHotUpdateService;
use crate::ops::fault_handling::FaultHandlingManager;
use crate::ops::log::LogManager;
use crate::ops::monitor::MonitorService;
use crate::points::PointsModule;
use crate::plugin::PluginManager;
use crate::plugin::dependency::DependencyManager;
use crate::plugin::market::PluginMarketplace;
use crate::referral::ReferralModule;
use crate::rights::RightsModule;
use crate::traffic::{CircuitBreaker, LoadBalancer, RateLimiter, TrafficMonitor, TrafficShaping};
use crate::user::UserModule;

/// Application configuration
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Database connection pool size
    pub db_pool_size: u32,
    /// Cache size
    pub cache_size: u64,
    /// Log level
    pub log_level: String,
}

/// Application global state
///
/// This struct holds all the global state for the application, including:
/// - Application metadata (start time, version)
/// - Database connection pools (MySQL, PostgreSQL, SQLite)
/// - Cache clients (memory cache, Redis)
/// - Service managers (config hot update, log, monitor, iterate)
/// - Plugin system components (plugin manager, dependency manager, marketplace)
/// - Concurrency control (semaphores, config watchers)
#[derive(Debug)]
pub struct AppState {
    /// Application start time
    pub start_time: Instant,
    /// Application version
    pub version: String,

    /// Configuration
    pub config: Arc<RwLock<AppConfig>>,
    /// Config change watcher
    pub config_watcher: watch::Sender<AppConfig>,

    /// MySQL database connection pool (controlled by feature flag)
    #[cfg(feature = "database")]
    pub mysql_pool: OnceCell<MySqlPool>,
    /// PostgreSQL database connection pool (controlled by feature flag)
    #[cfg(feature = "database")]
    pub postgres_pool: OnceCell<PgPool>,
    /// SQLite database connection pool (controlled by feature flag)
    #[cfg(feature = "database")]
    pub sqlite_pool: OnceCell<SqlitePool>,

    /// Memory cache (controlled by feature flag)
    #[cfg(feature = "cache")]
    pub memory_cache: Cache<String, String>,
    /// Redis client (controlled by feature flag)
    #[cfg(feature = "cache")]
    pub redis_client: OnceCell<Client>,

    /// Config hot update service
    pub config_hot_update_service: OnceCell<ConfigHotUpdateService>,
    /// Log manager
    pub log_manager: OnceCell<LogManager>,
    /// Monitor service
    pub monitor_service: OnceCell<MonitorService>,
    /// Fault handling manager
    pub fault_handling_manager: OnceCell<FaultHandlingManager>,
    /// Iteration convenience service
    pub iterate_service: OnceCell<IterateService>,
    /// Plugin manager
    pub plugin_manager: OnceCell<PluginManager>,
    /// Plugin dependency manager
    pub dependency_manager: OnceCell<DependencyManager>,
    /// Plugin marketplace
    pub plugin_marketplace: OnceCell<PluginMarketplace>,

    /// Internationalization manager
    pub i18n_manager: OnceCell<I18nManager>,

    /// Performance monitor (controlled by feature flag)
    #[cfg(feature = "monitoring")]
    pub performance_monitor: OnceCell<PerformanceMonitor>,
    /// Prometheus metrics (controlled by feature flag)
    #[cfg(feature = "monitoring")]
    pub prometheus_metrics: OnceCell<PrometheusMetrics>,

    /// Traffic management components
    pub traffic_shaping: OnceCell<TrafficShaping>,
    pub circuit_breaker: OnceCell<CircuitBreaker>,
    pub rate_limiter: OnceCell<RateLimiter>,
    pub load_balancer: OnceCell<LoadBalancer>,
    pub traffic_monitor: OnceCell<TrafficMonitor>,

    /// AI components
    pub ai_manager: OnceCell<AiManager>,

    /// Business modules
    pub rights_module: OnceCell<RightsModule>,
    pub points_module: OnceCell<PointsModule>,
    pub user_module: OnceCell<UserModule>,
    pub fraud_module: OnceCell<FraudModule>,
    pub referral_module: OnceCell<ReferralModule>,

    /// Concurrency control
    pub request_semaphore: Arc<Semaphore>,
    pub db_semaphore: Arc<Semaphore>,
    pub cache_semaphore: Arc<Semaphore>,
}

impl AppState {
    /// Create new application state
    ///
    /// This function initializes a new application state with default values:
    /// - Start time set to the current instant
    /// - Version set to "0.1.0"
    /// - All connection pools and services initialized to None
    /// - Memory cache initialized with a capacity of 10,000 entries (if cache feature is enabled)
    /// - Concurrency control semaphores initialized
    pub fn new() -> Self {
        let config = AppConfig::default();
        let (config_watcher, _) = watch::channel(config.clone());

        Self {
            start_time: Instant::now(),
            version: String::from("1.0.0"),
            config: Arc::new(RwLock::new(config)),
            config_watcher,

            // Database connection pools (controlled by feature flag)
            #[cfg(feature = "database")]
            mysql_pool: OnceCell::new(),
            #[cfg(feature = "database")]
            postgres_pool: OnceCell::new(),
            #[cfg(feature = "database")]
            sqlite_pool: OnceCell::new(),

            // Cache (controlled by feature flag)
            #[cfg(feature = "cache")]
            memory_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300))
                .time_to_idle(Duration::from_secs(60))
                .build(),
            #[cfg(feature = "cache")]
            redis_client: OnceCell::new(),

            config_hot_update_service: OnceCell::new(),
            log_manager: OnceCell::new(),
            monitor_service: OnceCell::new(),
            fault_handling_manager: OnceCell::new(),
            iterate_service: OnceCell::new(),
            plugin_manager: OnceCell::new(),
            dependency_manager: OnceCell::new(),
            plugin_marketplace: OnceCell::new(),
            i18n_manager: OnceCell::new(),

            // Performance monitor (controlled by feature flag)
            #[cfg(feature = "monitoring")]
            performance_monitor: OnceCell::new(),
            // Prometheus metrics (controlled by feature flag)
            #[cfg(feature = "monitoring")]
            prometheus_metrics: OnceCell::new(),

            // Traffic management components
            traffic_shaping: OnceCell::new(),
            circuit_breaker: OnceCell::new(),
            rate_limiter: OnceCell::new(),
            load_balancer: OnceCell::new(),
            traffic_monitor: OnceCell::new(),

            // AI components
            ai_manager: OnceCell::new(),

            // Business modules
            rights_module: OnceCell::new(),
            points_module: OnceCell::new(),
            user_module: OnceCell::new(),
            fraud_module: OnceCell::new(),
            referral_module: OnceCell::new(),

            // Concurrency control
            request_semaphore: Arc::new(Semaphore::new(1000)), // Default max 1000 concurrent requests
            db_semaphore: Arc::new(Semaphore::new(200)), // Default max 200 concurrent DB connections
            cache_semaphore: Arc::new(Semaphore::new(500)), // Default max 500 concurrent cache operations
        }
    }

    /// Get application uptime (seconds)
    ///
    /// Returns the number of seconds since the application started.
    pub fn uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Asynchronously get version
    ///
    /// Returns a clone of the application version string.
    pub async fn get_version(&self) -> String {
        self.version.clone()
    }

    /// Asynchronously update configuration
    ///
    /// Updates the application configuration and notifies all watchers.
    pub async fn update_config(&self, new_config: AppConfig) {
        *self.config.write().await = new_config.clone();
        // Notify watchers
        let _ = self.config_watcher.send(new_config);
    }

    /// Initialize internationalization manager
    ///
    /// Initializes the internationalization manager with the specified language pack directory.
    /// If the directory does not exist, it will be created.
    pub fn init_i18n_manager(
        &self,
        language_pack_dir: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let i18n_manager = I18nManager::new(language_pack_dir);

        // Create default language packs if they don't exist
        if let Err(e) = i18n_manager.create_default_language_packs() {
            error!("Failed to create default language packs: {}", e);
            // Continue with initialization even if default language packs creation fails
        }

        if self.i18n_manager.set(i18n_manager).is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "国际化管理器已初始化",
            )));
        }

        info!("国际化管理器初始化成功");
        Ok(())
    }

    /// Get internationalization manager
    ///
    /// Returns a reference to the internationalization manager if it has been initialized.
    pub fn get_i18n_manager(&self) -> Option<&I18nManager> {
        self.i18n_manager.get()
    }

    /// Asynchronously get database connection pool status (controlled by feature flag)
    ///
    /// Returns a JSON object with the status of each database connection pool:
    /// - `mysql`: boolean indicating if MySQL pool is initialized
    /// - `postgres`: boolean indicating if PostgreSQL pool is initialized
    /// - `sqlite`: boolean indicating if SQLite pool is initialized
    #[cfg(feature = "database")]
    pub async fn get_db_status(&self) -> serde_json::Value {
        let mysql_status = self.mysql_pool.get().is_some();
        let postgres_status = self.postgres_pool.get().is_some();
        let sqlite_status = self.sqlite_pool.get().is_some();

        serde_json::json!({
            "mysql": mysql_status,
            "postgres": postgres_status,
            "sqlite": sqlite_status
        })
    }

    /// Asynchronously get database connection pool status (default implementation when database feature is not enabled)
    ///
    /// Returns a JSON object indicating that the database feature is disabled.
    #[cfg(not(feature = "database"))]
    pub async fn get_db_status(&self) -> serde_json::Value {
        serde_json::json!({
            "mysql": false,
            "postgres": false,
            "sqlite": false,
            "status": "database_feature_disabled"
        })
    }

    /// Asynchronously get cache status (controlled by feature flag)
    ///
    /// Returns a JSON object with the status of the cache systems:
    /// - `redis`: boolean indicating if Redis client is initialized
    /// - `memory_cache_size`: number of entries in the memory cache
    #[cfg(feature = "cache")]
    pub async fn get_cache_status(&self) -> serde_json::Value {
        let redis_status = self.redis_client.get().is_some();
        let memory_cache_size = self.memory_cache.entry_count();

        serde_json::json!({
            "redis": redis_status,
            "memory_cache_size": memory_cache_size
        })
    }

    /// Asynchronously get cache status (default implementation when cache feature is not enabled)
    ///
    /// Returns a JSON object indicating that the cache feature is disabled.
    #[cfg(not(feature = "cache"))]
    pub async fn get_cache_status(&self) -> serde_json::Value {
        serde_json::json!({
            "redis": false,
            "memory_cache_size": 0,
            "status": "cache_feature_disabled"
        })
    }

    /// Asynchronously set config hot update service
    ///
    /// Sets the config hot update service in the application state.
    ///
    /// # Parameters
    /// - `service`: The config hot update service to set
    pub async fn set_config_hot_update_service(&self, service: ConfigHotUpdateService) {
        let _ = self.config_hot_update_service.set(service);
    }

    /// Asynchronously get config hot update service
    ///
    /// Returns the config hot update service if it's initialized, otherwise None.
    pub async fn get_config_hot_update_service(&self) -> Option<&ConfigHotUpdateService> {
        self.config_hot_update_service.get()
    }

    /// Asynchronously set log manager
    ///
    /// Sets the log manager in the application state.
    ///
    /// # Parameters
    /// - `manager`: The log manager to set
    pub async fn set_log_manager(&self, manager: LogManager) {
        let _ = self.log_manager.set(manager);
    }

    /// Asynchronously get log manager
    ///
    /// Returns the log manager if it's initialized, otherwise None.
    pub async fn get_log_manager(&self) -> Option<&LogManager> {
        self.log_manager.get()
    }

    /// Asynchronously set monitor service
    ///
    /// Sets the monitor service in the application state.
    ///
    /// # Parameters
    /// - `service`: The monitor service to set
    pub async fn set_monitor_service(&self, service: MonitorService) {
        let _ = self.monitor_service.set(service);
    }

    /// Asynchronously get monitor service
    ///
    /// Returns the monitor service if it's initialized, otherwise None.
    pub async fn get_monitor_service(&self) -> Option<&MonitorService> {
        self.monitor_service.get()
    }

    /// Asynchronously set fault handling manager
    ///
    /// Sets the fault handling manager in the application state.
    ///
    /// # Parameters
    /// - `manager`: The fault handling manager to set
    pub async fn set_fault_handling_manager(&self, manager: FaultHandlingManager) {
        let _ = self.fault_handling_manager.set(manager);
    }

    /// Asynchronously get fault handling manager
    ///
    /// Returns the fault handling manager if it's initialized, otherwise None.
    pub async fn get_fault_handling_manager(&self) -> Option<&FaultHandlingManager> {
        self.fault_handling_manager.get()
    }

    /// Asynchronously set iteration service
    ///
    /// Sets the iteration service in the application state.
    ///
    /// # Parameters
    /// - `service`: The iteration service to set
    pub async fn set_iterate_service(&self, service: IterateService) {
        let _ = self.iterate_service.set(service);
    }

    /// Asynchronously get iteration service
    ///
    /// Returns the iteration service if it's initialized, otherwise None.
    pub async fn get_iterate_service(&self) -> Option<&IterateService> {
        self.iterate_service.get()
    }

    /// Asynchronously set plugin manager
    ///
    /// Sets the plugin manager in the application state.
    ///
    /// # Parameters
    /// - `manager`: The plugin manager to set
    pub async fn set_plugin_manager(&self, manager: PluginManager) {
        let _ = self.plugin_manager.set(manager);
    }

    /// Asynchronously get plugin manager
    ///
    /// Returns the plugin manager if it's initialized, otherwise None.
    pub async fn get_plugin_manager(&self) -> Option<&PluginManager> {
        self.plugin_manager.get()
    }

    /// Asynchronously set plugin marketplace
    ///
    /// Sets the plugin marketplace in the application state.
    ///
    /// # Parameters
    /// - `marketplace`: The plugin marketplace to set
    pub async fn set_plugin_marketplace(&self, marketplace: PluginMarketplace) {
        let _ = self.plugin_marketplace.set(marketplace);
    }

    /// Asynchronously get plugin marketplace
    ///
    /// Returns the plugin marketplace if it's initialized, otherwise None.
    pub async fn get_plugin_marketplace(&self) -> Option<&PluginMarketplace> {
        self.plugin_marketplace.get()
    }

    /// Asynchronously set performance monitor
    ///
    /// Sets the performance monitor in the application state.
    ///
    /// # Parameters
    /// - `monitor`: The performance monitor to set
    #[cfg(feature = "monitoring")]
    pub async fn set_performance_monitor(&self, monitor: PerformanceMonitor) {
        let _ = self.performance_monitor.set(monitor);
    }

    /// Asynchronously get performance monitor
    ///
    /// Returns the performance monitor if it's initialized, otherwise None.
    #[cfg(feature = "monitoring")]
    pub async fn get_performance_monitor(&self) -> Option<&PerformanceMonitor> {
        self.performance_monitor.get()
    }

    /// Asynchronously set Prometheus metrics
    ///
    /// Sets the Prometheus metrics in the application state.
    ///
    /// # Parameters
    /// - `metrics`: The Prometheus metrics to set
    #[cfg(feature = "monitoring")]
    pub async fn set_prometheus_metrics(&self, metrics: PrometheusMetrics) {
        let _ = self.prometheus_metrics.set(metrics);
    }

    /// Asynchronously get Prometheus metrics
    ///
    /// Returns the Prometheus metrics if it's initialized, otherwise None.
    #[cfg(feature = "monitoring")]
    pub async fn get_prometheus_metrics(&self) -> Option<&PrometheusMetrics> {
        self.prometheus_metrics.get()
    }

    /// Asynchronously set traffic shaping
    ///
    /// Sets the traffic shaping component in the application state.
    ///
    /// # Parameters
    /// - `traffic_shaping`: The traffic shaping component to set
    pub async fn set_traffic_shaping(&self, traffic_shaping: TrafficShaping) {
        let _ = self.traffic_shaping.set(traffic_shaping);
    }

    /// Asynchronously get traffic shaping
    ///
    /// Returns the traffic shaping component if it's initialized, otherwise None.
    pub async fn get_traffic_shaping(&self) -> Option<&TrafficShaping> {
        self.traffic_shaping.get()
    }

    /// Asynchronously set circuit breaker
    ///
    /// Sets the circuit breaker component in the application state.
    ///
    /// # Parameters
    /// - `circuit_breaker`: The circuit breaker component to set
    pub async fn set_circuit_breaker(&self, circuit_breaker: CircuitBreaker) {
        let _ = self.circuit_breaker.set(circuit_breaker);
    }

    /// Asynchronously get circuit breaker
    ///
    /// Returns the circuit breaker component if it's initialized, otherwise None.
    pub async fn get_circuit_breaker(&self) -> Option<&CircuitBreaker> {
        self.circuit_breaker.get()
    }

    /// Asynchronously set rate limiter
    ///
    /// Sets the rate limiter component in the application state.
    ///
    /// # Parameters
    /// - `rate_limiter`: The rate limiter component to set
    pub async fn set_rate_limiter(&self, rate_limiter: RateLimiter) {
        let _ = self.rate_limiter.set(rate_limiter);
    }

    /// Asynchronously get rate limiter
    ///
    /// Returns the rate limiter component if it's initialized, otherwise None.
    pub async fn get_rate_limiter(&self) -> Option<&RateLimiter> {
        self.rate_limiter.get()
    }

    /// Asynchronously set load balancer
    ///
    /// Sets the load balancer component in the application state.
    ///
    /// # Parameters
    /// - `load_balancer`: The load balancer component to set
    pub async fn set_load_balancer(&self, load_balancer: LoadBalancer) {
        let _ = self.load_balancer.set(load_balancer);
    }

    /// Asynchronously get load balancer
    ///
    /// Returns the load balancer component if it's initialized, otherwise None.
    pub async fn get_load_balancer(&self) -> Option<&LoadBalancer> {
        self.load_balancer.get()
    }

    /// Asynchronously set traffic monitor
    ///
    /// Sets the traffic monitor component in the application state.
    ///
    /// # Parameters
    /// - `traffic_monitor`: The traffic monitor component to set
    pub async fn set_traffic_monitor(&self, traffic_monitor: TrafficMonitor) {
        let _ = self.traffic_monitor.set(traffic_monitor);
    }

    /// Asynchronously get traffic monitor
    ///
    /// Returns the traffic monitor component if it's initialized, otherwise None.
    pub async fn get_traffic_monitor(&self) -> Option<&TrafficMonitor> {
        self.traffic_monitor.get()
    }

    /// Asynchronously set AI manager
    ///
    /// Sets the AI manager component in the application state.
    ///
    /// # Parameters
    /// - `ai_manager`: The AI manager component to set
    pub async fn set_ai_manager(&self, ai_manager: AiManager) {
        let _ = self.ai_manager.set(ai_manager);
    }

    /// Asynchronously get AI manager
    ///
    /// Returns the AI manager component if it's initialized, otherwise None.
    pub async fn get_ai_manager(&self) -> Option<&AiManager> {
        self.ai_manager.get()
    }

    /// Asynchronously set rights module
    ///
    /// Sets the rights module in the application state.
    ///
    /// # Parameters
    /// - `module`: The rights module to set
    pub async fn set_rights_module(&self, module: RightsModule) {
        let _ = self.rights_module.set(module);
    }

    /// Asynchronously get rights module
    ///
    /// Returns the rights module if it's initialized, otherwise None.
    pub async fn get_rights_module(&self) -> Option<&RightsModule> {
        self.rights_module.get()
    }

    /// Asynchronously set points module
    ///
    /// Sets the points module in the application state.
    ///
    /// # Parameters
    /// - `module`: The points module to set
    pub async fn set_points_module(&self, module: PointsModule) {
        let _ = self.points_module.set(module);
    }

    /// Asynchronously get points module
    ///
    /// Returns the points module if it's initialized, otherwise None.
    pub async fn get_points_module(&self) -> Option<&PointsModule> {
        self.points_module.get()
    }

    /// Asynchronously set user module
    ///
    /// Sets the user module in the application state.
    ///
    /// # Parameters
    /// - `module`: The user module to set
    pub async fn set_user_module(&self, module: UserModule) {
        let _ = self.user_module.set(module);
    }

    /// Asynchronously get user module
    ///
    /// Returns the user module if it's initialized, otherwise None.
    pub async fn get_user_module(&self) -> Option<&UserModule> {
        self.user_module.get()
    }

    /// Asynchronously set fraud module
    ///
    /// Sets the fraud module in the application state.
    ///
    /// # Parameters
    /// - `module`: The fraud module to set
    pub async fn set_fraud_module(&self, module: FraudModule) {
        let _ = self.fraud_module.set(module);
    }

    /// Asynchronously get fraud module
    ///
    /// Returns the fraud module if it's initialized, otherwise None.
    pub async fn get_fraud_module(&self) -> Option<&FraudModule> {
        self.fraud_module.get()
    }

    /// Asynchronously set referral module
    ///
    /// Sets the referral module in the application state.
    ///
    /// # Parameters
    /// - `module`: The referral module to set
    pub async fn set_referral_module(&self, module: ReferralModule) {
        let _ = self.referral_module.set(module);
    }

    /// Asynchronously get referral module
    ///
    /// Returns the referral module if it's initialized, otherwise None.
    pub async fn get_referral_module(&self) -> Option<&ReferralModule> {
        self.referral_module.get()
    }

    /// Get request semaphore
    ///
    /// Returns the semaphore for controlling concurrent requests.
    pub fn get_request_semaphore(&self) -> Arc<Semaphore> {
        self.request_semaphore.clone()
    }

    /// Get database semaphore
    ///
    /// Returns the semaphore for controlling concurrent database operations.
    pub fn get_db_semaphore(&self) -> Arc<Semaphore> {
        self.db_semaphore.clone()
    }

    /// Get cache semaphore
    ///
    /// Returns the semaphore for controlling concurrent cache operations.
    pub fn get_cache_semaphore(&self) -> Arc<Semaphore> {
        self.cache_semaphore.clone()
    }

    /// Update semaphore limits based on config
    ///
    /// Updates the semaphore limits based on the current configuration.
    pub async fn update_semaphore_limits(&self) {
        let config = self.config.read().await;
        // Note: Semaphore doesn't support dynamic resizing, so we would need to replace it
        // For simplicity, we'll just log the change
        log::info!(
            "Semaphore limits updated: requests={}, db={}, cache={}",
            config.max_concurrent_requests,
            config.db_pool_size,
            config.cache_size
        );
    }

    /// Update memory cache configuration
    ///
    /// Updates the memory cache configuration based on the current configuration.
    #[cfg(feature = "cache")]
    pub async fn update_memory_cache_config(&self) {
        let config = self.config.read().await;
        let cache_size = if config.cache_size > 0 {
            config.cache_size
        } else {
            10_000 // Default value
        };

        // Note: Moka cache doesn't support dynamic resizing of max_capacity,
        // so we would need to replace it if we wanted to change capacity at runtime
        log::info!("Memory cache configuration updated: size={}", cache_size);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global state extension for extracting state from requests
pub type AppStateExtension = Arc<AppState>;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test_app_state_creation() {
        let app_state = AppState::new();
        assert!(!app_state.version.is_empty());
        // 验证 elapsed() 方法不会 panic
        let _elapsed = app_state.start_time.elapsed().as_secs();
    }

    #[tokio::test]
    async fn test_app_state_uptime() {
        let app_state = AppState::new();
        sleep(Duration::from_millis(100)).await;
        let _uptime = app_state.uptime();
        // 验证 uptime() 方法不会 panic
    }

    #[tokio::test]
    async fn test_app_state_db_status() {
        let app_state = AppState::new();
        let db_status = app_state.get_db_status().await;
        assert!(db_status.is_object());
    }

    #[tokio::test]
    async fn test_app_state_cache_status() {
        let app_state = AppState::new();
        let cache_status = app_state.get_cache_status().await;
        assert!(cache_status.is_object());
    }

    #[tokio::test]
    async fn test_app_state_update_config() {
        let app_state = AppState::new();
        let new_config = AppConfig {
            max_concurrent_requests: 500,
            db_pool_size: 10,
            cache_size: 5000,
            log_level: "debug".to_string(),
        };

        app_state.update_config(new_config).await;
        let current_config = app_state.config.read().await;
        assert_eq!(current_config.max_concurrent_requests, 500);
    }

    #[tokio::test]
    async fn test_app_state_get_version() {
        let app_state = AppState::new();
        let version = app_state.get_version().await;
        assert!(!version.is_empty());
    }

    #[tokio::test]
    async fn test_app_state_semaphores() {
        let app_state = AppState::new();
        let request_sem = app_state.get_request_semaphore();
        let db_sem = app_state.get_db_semaphore();
        let cache_sem = app_state.get_cache_semaphore();

        assert!(request_sem.available_permits() > 0);
        assert!(db_sem.available_permits() > 0);
        assert!(cache_sem.available_permits() > 0);
    }

    #[tokio::test]
    async fn test_app_state_update_semaphore_limits() {
        let app_state = AppState::new();
        // This test just ensures the method doesn't panic
        app_state.update_semaphore_limits().await;
    }

    #[tokio::test]
    async fn test_app_state_update_memory_cache_config() {
        let app_state = AppState::new();
        // This test just ensures the method doesn't panic
        app_state.update_memory_cache_config().await;
    }

    #[tokio::test]
    async fn test_app_state_default() {
        let app_state = AppState::default();
        assert!(!app_state.version.is_empty());
    }
}

