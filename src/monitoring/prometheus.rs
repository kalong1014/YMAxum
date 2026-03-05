//! Prometheus指标暴露模块
//! 用于暴露YMAxum的监控指标给Prometheus

use axum::Router;
use axum::routing::get;
use prometheus::{Counter, Encoder, Gauge, Histogram, Registry};
use std::sync::Arc;

/// Prometheus指标注册表
#[derive(Debug, Clone)]
pub struct PrometheusMetrics {
    registry: Registry,
    // 请求计数器
    http_requests_total: Counter,
    // 请求处理时间直方图
    http_request_duration_seconds: Histogram,
    // 活跃连接数
    http_active_connections: Gauge,
    // 错误计数器
    http_errors_total: Counter,
    // CPU使用率
    cpu_usage_percent: Gauge,
    // 内存使用率
    memory_usage_percent: Gauge,
    // 插件加载数量
    plugins_loaded: Gauge,
}

impl PrometheusMetrics {
    /// 创建新的Prometheus指标注册表
    pub fn new() -> Self {
        let registry = Registry::new();

        // 创建请求计数器
        let http_requests_total = Counter::new(
            "ymaxum_http_requests_total",
            "Total number of HTTP requests",
        )
        .unwrap();
        registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();

        // 创建请求处理时间直方图
        use prometheus::HistogramOpts;
        let histogram_opts = HistogramOpts::new(
            "ymaxum_http_request_duration_seconds",
            "HTTP request duration in seconds",
        );
        let http_request_duration_seconds = Histogram::with_opts(histogram_opts).unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();

        // 创建活跃连接数
        let http_active_connections = Gauge::new(
            "ymaxum_http_active_connections",
            "Number of active HTTP connections",
        )
        .unwrap();
        registry
            .register(Box::new(http_active_connections.clone()))
            .unwrap();

        // 创建错误计数器
        let http_errors_total =
            Counter::new("ymaxum_http_errors_total", "Total number of HTTP errors").unwrap();
        registry
            .register(Box::new(http_errors_total.clone()))
            .unwrap();

        // 创建CPU使用率
        let cpu_usage_percent =
            Gauge::new("ymaxum_cpu_usage_percent", "CPU usage percentage").unwrap();
        registry
            .register(Box::new(cpu_usage_percent.clone()))
            .unwrap();

        // 创建内存使用率
        let memory_usage_percent =
            Gauge::new("ymaxum_memory_usage_percent", "Memory usage percentage").unwrap();
        registry
            .register(Box::new(memory_usage_percent.clone()))
            .unwrap();

        // 创建插件加载数量
        let plugins_loaded =
            Gauge::new("ymaxum_plugins_loaded", "Number of loaded plugins").unwrap();
        registry.register(Box::new(plugins_loaded.clone())).unwrap();

        Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            http_active_connections,
            http_errors_total,
            cpu_usage_percent,
            memory_usage_percent,
            plugins_loaded,
        }
    }

    /// 获取指标注册表
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// 增加请求计数
    pub fn increment_request_count(&self) {
        self.http_requests_total.inc();
    }

    /// 记录请求处理时间
    pub fn observe_request_duration(&self, duration: f64) {
        self.http_request_duration_seconds.observe(duration);
    }

    /// 设置活跃连接数
    pub fn set_active_connections(&self, count: f64) {
        self.http_active_connections.set(count);
    }

    /// 增加错误计数
    pub fn increment_error_count(&self) {
        self.http_errors_total.inc();
    }

    /// 设置CPU使用率
    pub fn set_cpu_usage(&self, usage: f64) {
        self.cpu_usage_percent.set(usage);
    }

    /// 设置内存使用率
    pub fn set_memory_usage(&self, usage: f64) {
        self.memory_usage_percent.set(usage);
    }

    /// 设置插件加载数量
    pub fn set_plugins_loaded(&self, count: f64) {
        self.plugins_loaded.set(count);
    }
}

/// 创建Prometheus指标暴露路由
pub fn create_prometheus_router(metrics: Arc<PrometheusMetrics>) -> Router {
    Router::new().route(
        "/metrics",
        get(move || async move {
            let encoder = prometheus::TextEncoder::new();
            let metrics = metrics.registry().gather();
            let mut buffer = Vec::new();
            encoder.encode(&metrics, &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        }),
    )
}
