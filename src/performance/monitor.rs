use anyhow::Result;
use axum::{Router, body::Body, extract::State, routing::get};
use hyper::Request;
use prometheus::{Counter, Encoder, Gauge, Histogram, HistogramOpts, Opts, Registry, TextEncoder};

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::state::AppState;

/// 性能监控管理器
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    _app_state: Arc<AppState>,
    registry: Registry,
    // 计数器
    http_requests_total: Counter,
    http_errors_total: Counter,
    database_queries_total: Counter,
    database_errors_total: Counter,
    cache_operations_total: Counter,
    plugin_operations_total: Counter,
    // 仪表
    active_requests: Gauge,
    memory_usage: Gauge,
    cpu_usage: Gauge,
    network_rx_bytes: Gauge,
    network_tx_bytes: Gauge,
    disk_read_bytes: Gauge,
    disk_write_bytes: Gauge,
    cache_hit_rate: Gauge,
    database_connection_pool_usage: Gauge,
    plugin_active_count: Gauge,
    // 直方图
    http_request_duration_seconds: Histogram,
    database_query_duration_seconds: Histogram,
    cache_response_duration_seconds: Histogram,
    plugin_operation_duration_seconds: Histogram,
    // 历史数据
    response_times: Arc<Mutex<VecDeque<f64>>>,
    query_times: Arc<Mutex<VecDeque<f64>>>,
    cache_response_times: Arc<Mutex<VecDeque<f64>>>,
    plugin_response_times: Arc<Mutex<VecDeque<f64>>>,
    // 缓存统计
    cache_hits: Arc<AtomicU64>,
    cache_misses: Arc<AtomicU64>,
    // 数据库统计
    database_hits: Arc<AtomicU64>,
    database_misses: Arc<AtomicU64>,
    // 插件统计
    plugin_hits: Arc<AtomicU64>,
    plugin_misses: Arc<AtomicU64>,
}

impl PerformanceMonitor {
    /// 创建新的性能监控管理器
    pub fn new(app_state: Arc<AppState>) -> Result<Self> {
        let registry = Registry::default();

        // 创建计数器
        let http_requests_total = Counter::with_opts(Opts::new(
            "http_requests_total",
            "Total number of HTTP requests",
        ))?;

        let http_errors_total = Counter::with_opts(Opts::new(
            "http_errors_total",
            "Total number of HTTP errors",
        ))?;

        let database_queries_total = Counter::with_opts(Opts::new(
            "database_queries_total",
            "Total number of database queries",
        ))?;

        let database_errors_total = Counter::with_opts(Opts::new(
            "database_errors_total",
            "Total number of database errors",
        ))?;

        let cache_operations_total = Counter::with_opts(Opts::new(
            "cache_operations_total",
            "Total number of cache operations",
        ))?;

        let plugin_operations_total = Counter::with_opts(Opts::new(
            "plugin_operations_total",
            "Total number of plugin operations",
        ))?;

        // 创建仪表
        let active_requests = Gauge::with_opts(Opts::new(
            "active_requests",
            "Number of active HTTP requests",
        ))?;

        let memory_usage = Gauge::with_opts(Opts::new("memory_usage", "Memory usage in bytes"))?;

        let cpu_usage = Gauge::with_opts(Opts::new("cpu_usage", "CPU usage percentage"))?;

        let network_rx_bytes =
            Gauge::with_opts(Opts::new("network_rx_bytes", "Network receive bytes"))?;

        let network_tx_bytes =
            Gauge::with_opts(Opts::new("network_tx_bytes", "Network transmit bytes"))?;

        let disk_read_bytes = Gauge::with_opts(Opts::new("disk_read_bytes", "Disk read bytes"))?;

        let disk_write_bytes = Gauge::with_opts(Opts::new("disk_write_bytes", "Disk write bytes"))?;

        let cache_hit_rate =
            Gauge::with_opts(Opts::new("cache_hit_rate", "Cache hit rate percentage"))?;

        let database_connection_pool_usage = Gauge::with_opts(Opts::new(
            "database_connection_pool_usage",
            "Database connection pool usage percentage",
        ))?;

        let plugin_active_count =
            Gauge::with_opts(Opts::new("plugin_active_count", "Number of active plugins"))?;

        // 创建直方图
        let http_request_duration_seconds = Histogram::with_opts(HistogramOpts::from(Opts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
        )))?;

        let database_query_duration_seconds =
            Histogram::with_opts(HistogramOpts::from(Opts::new(
                "database_query_duration_seconds",
                "Database query duration in seconds",
            )))?;

        let cache_response_duration_seconds =
            Histogram::with_opts(HistogramOpts::from(Opts::new(
                "cache_response_duration_seconds",
                "Cache response duration in seconds",
            )))?;

        let plugin_operation_duration_seconds =
            Histogram::with_opts(HistogramOpts::from(Opts::new(
                "plugin_operation_duration_seconds",
                "Plugin operation duration in seconds",
            )))?;

        // 注册指标
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_errors_total.clone()))?;
        registry.register(Box::new(database_queries_total.clone()))?;
        registry.register(Box::new(database_errors_total.clone()))?;
        registry.register(Box::new(cache_operations_total.clone()))?;
        registry.register(Box::new(plugin_operations_total.clone()))?;
        registry.register(Box::new(active_requests.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(network_rx_bytes.clone()))?;
        registry.register(Box::new(network_tx_bytes.clone()))?;
        registry.register(Box::new(disk_read_bytes.clone()))?;
        registry.register(Box::new(disk_write_bytes.clone()))?;
        registry.register(Box::new(cache_hit_rate.clone()))?;
        registry.register(Box::new(database_connection_pool_usage.clone()))?;
        registry.register(Box::new(plugin_active_count.clone()))?;
        registry.register(Box::new(http_request_duration_seconds.clone()))?;
        registry.register(Box::new(database_query_duration_seconds.clone()))?;
        registry.register(Box::new(cache_response_duration_seconds.clone()))?;
        registry.register(Box::new(plugin_operation_duration_seconds.clone()))?;

        Ok(Self {
            _app_state: app_state,
            registry,
            http_requests_total,
            http_errors_total,
            database_queries_total,
            database_errors_total,
            cache_operations_total,
            plugin_operations_total,
            active_requests,
            memory_usage,
            cpu_usage,
            network_rx_bytes,
            network_tx_bytes,
            disk_read_bytes,
            disk_write_bytes,
            cache_hit_rate,
            database_connection_pool_usage,
            plugin_active_count,
            http_request_duration_seconds,
            database_query_duration_seconds,
            cache_response_duration_seconds,
            plugin_operation_duration_seconds,
            // 历史数据，最多存储1000条
            response_times: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            query_times: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            cache_response_times: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            plugin_response_times: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            // 缓存统计
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
            // 数据库统计
            database_hits: Arc::new(AtomicU64::new(0)),
            database_misses: Arc::new(AtomicU64::new(0)),
            // 插件统计
            plugin_hits: Arc::new(AtomicU64::new(0)),
            plugin_misses: Arc::new(AtomicU64::new(0)),
        })
    }

    /// 记录HTTP请求开始
    pub fn record_request_start(&self) {
        self.http_requests_total.inc();
        self.active_requests.inc();
    }

    /// 记录HTTP请求结束
    pub fn record_request_end(&self, duration: f64, is_error: bool) {
        self.active_requests.dec();
        self.http_request_duration_seconds.observe(duration);
        if is_error {
            self.http_errors_total.inc();
        }

        // 记录响应时间
        let mut response_times = self.response_times.lock().unwrap();
        if response_times.len() >= 1000 {
            response_times.pop_front();
        }
        response_times.push_back(duration * 1000.0); // 转换为毫秒
    }

    /// 记录数据库查询
    pub fn record_database_query(&self, duration: f64, is_error: bool) {
        self.database_queries_total.inc();
        self.database_query_duration_seconds.observe(duration);
        if is_error {
            self.database_errors_total.inc();
        }

        // 记录查询时间
        let mut query_times = self.query_times.lock().unwrap();
        if query_times.len() >= 1000 {
            query_times.pop_front();
        }
        query_times.push_back(duration * 1000.0); // 转换为毫秒
    }

    /// 记录缓存操作
    pub fn record_cache_operation(&self, duration: f64, is_hit: bool) {
        self.cache_operations_total.inc();
        self.cache_response_duration_seconds.observe(duration);
        if is_hit {
            self.cache_hits.fetch_add(1, Ordering::SeqCst);
        } else {
            self.cache_misses.fetch_add(1, Ordering::SeqCst);
        }

        // 记录缓存响应时间
        let mut cache_response_times = self.cache_response_times.lock().unwrap();
        if cache_response_times.len() >= 1000 {
            cache_response_times.pop_front();
        }
        cache_response_times.push_back(duration * 1000.0); // 转换为毫秒

        // 更新缓存命中率
        let hits = self.cache_hits.load(Ordering::SeqCst);
        let misses = self.cache_misses.load(Ordering::SeqCst);
        let total = hits + misses;
        if total > 0 {
            let hit_rate = (hits as f64 / total as f64) * 100.0;
            self.cache_hit_rate.set(hit_rate);
        }
    }

    /// 记录插件操作
    pub fn record_plugin_operation(&self, duration: f64, is_error: bool) {
        self.plugin_operations_total.inc();
        self.plugin_operation_duration_seconds.observe(duration);
        if !is_error {
            self.plugin_hits.fetch_add(1, Ordering::SeqCst);
        } else {
            self.plugin_misses.fetch_add(1, Ordering::SeqCst);
        }

        // 记录插件响应时间
        let mut plugin_response_times = self.plugin_response_times.lock().unwrap();
        if plugin_response_times.len() >= 1000 {
            plugin_response_times.pop_front();
        }
        plugin_response_times.push_back(duration * 1000.0); // 转换为毫秒
    }

    /// 更新内存使用情况
    pub fn update_memory_usage(&self) {
        #[cfg(feature = "system_info")]
        {
            use sysinfo::System;
            let mut system = System::new_all();
            system.refresh_all();
            let memory = system.total_memory() - system.available_memory();
            self.memory_usage.set(memory as f64);
        }
    }

    /// 更新CPU使用情况
    pub fn update_cpu_usage(&self) {
        #[cfg(feature = "system_info")]
        {
            use sysinfo::System;
            let mut system = System::new_all();
            system.refresh_all();
            let cpu_usage = system.global_cpu_usage() as f64;
            self.cpu_usage.set(cpu_usage);
        }
    }

    /// 更新网络使用情况
    pub fn update_network_usage(&self, rx_bytes: u64, tx_bytes: u64) {
        self.network_rx_bytes.set(rx_bytes as f64);
        self.network_tx_bytes.set(tx_bytes as f64);
    }

    /// 更新磁盘使用情况
    pub fn update_disk_usage(&self, read_bytes: u64, write_bytes: u64) {
        self.disk_read_bytes.set(read_bytes as f64);
        self.disk_write_bytes.set(write_bytes as f64);
    }

    /// 更新数据库连接池使用情况
    pub fn update_database_connection_pool_usage(&self, usage: f64) {
        self.database_connection_pool_usage.set(usage);
    }

    /// 更新活跃插件数量
    pub fn update_plugin_active_count(&self, count: u64) {
        self.plugin_active_count.set(count as f64);
    }

    /// 记录缓存命中
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录缓存未命中
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录数据库命中
    pub fn record_database_hit(&self) {
        self.database_hits.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录数据库未命中
    pub fn record_database_miss(&self) {
        self.database_misses.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录插件命中
    pub fn record_plugin_hit(&self) {
        self.plugin_hits.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录插件未命中
    pub fn record_plugin_miss(&self) {
        self.plugin_misses.fetch_add(1, Ordering::SeqCst);
    }

    /// 获取HTTP指标
    pub fn get_http_metrics(&self) -> (u64, u64) {
        let total_requests = self.http_requests_total.get() as u64;
        let error_requests = self.http_errors_total.get() as u64;
        (total_requests, error_requests)
    }

    /// 获取HTTP响应时间
    pub fn get_response_times(&self) -> Vec<f64> {
        self.response_times
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect()
    }

    /// 获取缓存指标
    pub fn get_cache_metrics(&self) -> (u64, u64) {
        let hits = self.cache_hits.load(Ordering::SeqCst);
        let misses = self.cache_misses.load(Ordering::SeqCst);
        (hits, misses)
    }

    /// 获取缓存响应时间
    pub fn get_cache_response_times(&self) -> Vec<f64> {
        self.cache_response_times
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect()
    }

    /// 获取插件响应时间
    pub fn get_plugin_response_times(&self) -> Vec<f64> {
        self.plugin_response_times
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect()
    }

    /// 获取数据库查询时间
    pub fn get_database_query_times(&self) -> Vec<f64> {
        self.query_times
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect()
    }

    /// 获取旧版本的数据库指标（兼容analyzer.rs）
    pub fn get_database_metrics_old(&self) -> (u64, Vec<f64>) {
        let total_queries = self.query_times.lock().unwrap().len() as u64;
        let query_times = self.get_database_query_times();
        (total_queries, query_times)
    }

    /// 获取活跃连接数
    pub fn get_active_connections(&self) -> u64 {
        self.active_requests.get() as u64
    }

    /// 获取缓存命中率
    pub fn get_cache_hit_rate(&self) -> f64 {
        self.cache_hit_rate.get()
    }

    /// 获取数据库指标
    pub fn get_database_metrics(&self) -> (u64, u64, u64) {
        let total_queries = self.database_queries_total.get() as u64;
        let error_queries = self.database_errors_total.get() as u64;
        let active_connections = self.database_connection_pool_usage.get() as u64;
        (total_queries, error_queries, active_connections)
    }

    /// 获取插件指标
    pub fn get_plugin_metrics(&self) -> (u64, u64, u64) {
        let total_operations = self.plugin_operations_total.get() as u64;
        let active_plugins = self.plugin_active_count.get() as u64;
        let successful_operations = self.plugin_hits.load(Ordering::SeqCst);
        (total_operations, active_plugins, successful_operations)
    }

    /// 获取系统指标
    pub fn get_system_metrics(&self) -> (f64, f64, f64, f64) {
        let cpu_usage = self.cpu_usage.get();
        let memory_usage = self.memory_usage.get();
        let network_rx = self.network_rx_bytes.get();
        let network_tx = self.network_tx_bytes.get();
        (cpu_usage, memory_usage, network_rx, network_tx)
    }

    /// 获取磁盘指标
    pub fn get_disk_metrics(&self) -> (f64, f64) {
        let read_bytes = self.disk_read_bytes.get();
        let write_bytes = self.disk_write_bytes.get();
        (read_bytes, write_bytes)
    }

    /// 获取所有指标的摘要
    pub fn get_metrics_summary(&self) -> String {
        let (total_requests, error_requests) = self.get_http_metrics();
        let (total_queries, error_queries, _) = self.get_database_metrics();
        let (total_operations, active_plugins, _) = self.get_plugin_metrics();
        let (cpu_usage, memory_usage, _, _) = self.get_system_metrics();
        let cache_hit_rate = self.get_cache_hit_rate();

        format!(
            "HTTP: {} requests, {} errors\nDatabase: {} queries, {} errors\nPlugins: {} operations, {} active\nSystem: {:.2}% CPU, {:.2} MB memory\nCache: {:.2}% hit rate",
            total_requests,
            error_requests,
            total_queries,
            error_queries,
            total_operations,
            active_plugins,
            cpu_usage,
            memory_usage / 1024.0 / 1024.0,
            cache_hit_rate
        )
    }

    /// 获取监控指标的HTTP处理函数
    pub async fn metrics_handler(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    /// 创建监控路由
    pub fn create_router(&self) -> Router {
        use std::sync::Arc;
        let monitor = Arc::new(self.clone());
        Router::new().route(
            "/metrics",
            get(move || {
                let monitor = monitor.clone();
                async move {
                    let monitor = monitor;
                    monitor.metrics_handler().await
                }
            }),
        )
    }

    /// 集成到应用状态
    pub fn integrate_with_app_state(&self) {
        // 将监控器实例存储到AppState中
        // 这里需要在应用启动时调用app_state.set_performance_monitor()
    }
}

/// 监控中间件
pub async fn monitoring_middleware(
    request: Request<Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // 记录请求开始时间
    let start = std::time::Instant::now();

    // 调用下一个中间件
    let response = next.run(request).await;

    // 计算请求处理时间
    let _duration = start.elapsed().as_secs_f64();

    // 检查是否为错误响应
    let _is_error = response.status().is_client_error() || response.status().is_server_error();

    // 记录请求指标
    // 这里需要从AppState中获取PerformanceMonitor实例
    // 暂时跳过，后续需要集成到AppState中

    response
}

/// 带状态的监控中间件
///
/// 此中间件可以从请求中提取AppState，并使用其中的PerformanceMonitor实例记录指标
pub async fn monitoring_middleware_with_state(
    State(app_state): State<Arc<AppState>>,
    request: Request<Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // 记录请求开始时间
    let start = std::time::Instant::now();

    // 尝试获取PerformanceMonitor实例
    #[cfg(feature = "monitoring")]
    if let Some(monitor) = app_state.get_performance_monitor().await {
        monitor.record_request_start();
    }

    // 调用下一个中间件
    let response = next.run(request).await;

    // 计算请求处理时间
    let duration = start.elapsed().as_secs_f64();

    // 检查是否为错误响应
    let is_error = response.status().is_client_error() || response.status().is_server_error();

    // 记录请求指标
    #[cfg(feature = "monitoring")]
    if let Some(monitor) = app_state.get_performance_monitor().await {
        monitor.record_request_end(duration, is_error);
        // 定期更新内存使用情况
        if rand::random::<f64>() < 0.01 {
            // 1%的概率更新
            monitor.update_memory_usage();
        }
    }

    response
}
