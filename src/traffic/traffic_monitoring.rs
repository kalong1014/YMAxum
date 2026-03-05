//! 流量监控模块
//!
//! 提供流量数据收集、分析和可视化等功能

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 流量监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMonitorConfig {
    /// 启用流量监控
    pub enabled: bool,
    /// 采样率
    pub sampling_rate: f64,
    /// 数据保留时间（毫秒）
    pub data_retention_ms: u64,
    /// 监控指标
    pub metrics: Vec<MonitorMetric>,
    /// 告警配置
    pub alert: AlertConfig,
    /// 导出配置
    pub export: ExportConfig,
}

/// 监控指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorMetric {
    /// 指标名称
    pub name: String,
    /// 指标描述
    pub description: String,
    /// 指标类型
    pub type_: String,
    /// 启用状态
    pub enabled: bool,
}

/// 告警配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// 启用告警
    pub enabled: bool,
    /// 告警规则
    pub rules: Vec<AlertRule>,
}

/// 告警规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 指标名称
    pub metric_name: String,
    /// 阈值
    pub threshold: f64,
    /// 比较操作
    pub operation: String,
    /// 持续时间（毫秒）
    pub duration_ms: u64,
    /// 启用状态
    pub enabled: bool,
}

/// 导出配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// 启用导出
    pub enabled: bool,
    /// 导出目标
    pub targets: Vec<String>,
    /// 导出间隔（毫秒）
    pub interval_ms: u64,
}

/// 流量统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficStats {
    /// 总请求数
    pub total_requests: u32,
    /// 成功请求数
    pub success_requests: u32,
    /// 失败请求数
    pub failure_requests: u32,
    /// 平均响应时间（毫秒）
    pub avg_response_time_ms: f64,
    /// 最大响应时间（毫秒）
    pub max_response_time_ms: f64,
    /// 最小响应时间（毫秒）
    pub min_response_time_ms: f64,
    /// 请求速率（请求/秒）
    pub request_rate: f64,
    /// 错误率
    pub error_rate: f64,
    /// 带宽使用（字节/秒）
    pub bandwidth_usage: f64,
    /// 路径统计
    pub path_stats: HashMap<String, PathStats>,
    /// 方法统计
    pub method_stats: HashMap<String, MethodStats>,
    /// 状态码统计
    pub status_code_stats: HashMap<u16, u32>,
    /// 时间戳
    pub timestamp: String,
}

/// 路径统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathStats {
    /// 请求数
    pub requests: u32,
    /// 平均响应时间（毫秒）
    pub avg_response_time_ms: f64,
    /// 错误率
    pub error_rate: f64,
}

/// 方法统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodStats {
    /// 请求数
    pub requests: u32,
    /// 平均响应时间（毫秒）
    pub avg_response_time_ms: f64,
    /// 错误率
    pub error_rate: f64,
}

/// 流量监控
#[derive(Debug, Clone)]
pub struct TrafficMonitor {
    config: Arc<RwLock<TrafficMonitorConfig>>,
    stats: Arc<RwLock<TrafficStats>>,
    last_reset_time: Arc<RwLock<String>>,
    request_timestamps: Arc<RwLock<Vec<String>>>,
}

impl TrafficMonitor {
    /// 创建新的流量监控
    pub fn new(config: TrafficMonitorConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            stats: Arc::new(RwLock::new(TrafficStats {
                total_requests: 0,
                success_requests: 0,
                failure_requests: 0,
                avg_response_time_ms: 0.0,
                max_response_time_ms: 0.0,
                min_response_time_ms: f64::MAX,
                request_rate: 0.0,
                error_rate: 0.0,
                bandwidth_usage: 0.0,
                path_stats: HashMap::new(),
                method_stats: HashMap::new(),
                status_code_stats: HashMap::new(),
                timestamp: chrono::Utc::now().to_string(),
            })),
            last_reset_time: Arc::new(RwLock::new(chrono::Utc::now().to_string())),
            request_timestamps: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 初始化流量监控
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化流量监控
        Ok(())
    }

    /// 记录请求
    pub async fn record_request(
        &self,
        path: &str,
        method: &str,
        status_code: u16,
        response_time_ms: f64,
        bytes_sent: u64,
    ) {
        let config = self.config.read().await;

        if !config.enabled {
            return;
        }

        // 采样
        let mut rng = rand::thread_rng();
        if rand::Rng::gen_range(&mut rng, 0.0..1.0) > config.sampling_rate {
            return;
        }

        let mut stats = self.stats.write().await;
        let mut request_timestamps = self.request_timestamps.write().await;

        // 更新总请求数
        stats.total_requests += 1;

        // 更新成功/失败请求数
        if (200..400).contains(&status_code) {
            stats.success_requests += 1;
        } else {
            stats.failure_requests += 1;
        }

        // 更新响应时间
        stats.avg_response_time_ms =
            (stats.avg_response_time_ms * (stats.total_requests - 1) as f64 + response_time_ms)
                / stats.total_requests as f64;

        stats.max_response_time_ms = stats.max_response_time_ms.max(response_time_ms);
        stats.min_response_time_ms = stats.min_response_time_ms.min(response_time_ms);

        // 更新状态码统计
        *stats.status_code_stats.entry(status_code).or_insert(0) += 1;

        // 更新路径统计
        let path_stats = stats
            .path_stats
            .entry(path.to_string())
            .or_insert(PathStats {
                requests: 0,
                avg_response_time_ms: 0.0,
                error_rate: 0.0,
            });
        path_stats.requests += 1;
        path_stats.avg_response_time_ms =
            (path_stats.avg_response_time_ms * (path_stats.requests - 1) as f64 + response_time_ms)
                / path_stats.requests as f64;
        path_stats.error_rate = if status_code >= 400 {
            (path_stats.error_rate * (path_stats.requests - 1) as f64 + 1.0)
                / path_stats.requests as f64
        } else {
            path_stats.error_rate * (path_stats.requests - 1) as f64 / path_stats.requests as f64
        };

        // 更新方法统计
        let method_stats = stats
            .method_stats
            .entry(method.to_string())
            .or_insert(MethodStats {
                requests: 0,
                avg_response_time_ms: 0.0,
                error_rate: 0.0,
            });
        method_stats.requests += 1;
        method_stats.avg_response_time_ms = (method_stats.avg_response_time_ms
            * (method_stats.requests - 1) as f64
            + response_time_ms)
            / method_stats.requests as f64;
        method_stats.error_rate = if status_code >= 400 {
            (method_stats.error_rate * (method_stats.requests - 1) as f64 + 1.0)
                / method_stats.requests as f64
        } else {
            method_stats.error_rate * (method_stats.requests - 1) as f64
                / method_stats.requests as f64
        };

        // 更新带宽使用
        stats.bandwidth_usage = bytes_sent as f64 / response_time_ms * 1000.0;

        // 更新时间戳
        let now = chrono::Utc::now().to_string();
        stats.timestamp = now.clone();
        request_timestamps.push(now);

        // 移除过期的时间戳
        let retention_ms = config.data_retention_ms;
        request_timestamps.retain(|timestamp| {
            let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(timestamp);
            duration.num_milliseconds() < retention_ms as i64
        });

        // 更新请求速率
        let request_count = request_timestamps.len() as f64;
        stats.request_rate = request_count / (retention_ms as f64 / 1000.0);

        // 更新错误率
        stats.error_rate = stats.failure_requests as f64 / stats.total_requests as f64;
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> TrafficStats {
        self.stats.read().await.clone()
    }

    /// 重置统计信息
    pub async fn reset_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stats = self.stats.write().await;
        *stats = TrafficStats {
            total_requests: 0,
            success_requests: 0,
            failure_requests: 0,
            avg_response_time_ms: 0.0,
            max_response_time_ms: 0.0,
            min_response_time_ms: f64::MAX,
            request_rate: 0.0,
            error_rate: 0.0,
            bandwidth_usage: 0.0,
            path_stats: HashMap::new(),
            method_stats: HashMap::new(),
            status_code_stats: HashMap::new(),
            timestamp: chrono::Utc::now().to_string(),
        };

        let mut last_reset_time = self.last_reset_time.write().await;
        *last_reset_time = chrono::Utc::now().to_string();

        let mut request_timestamps = self.request_timestamps.write().await;
        request_timestamps.clear();

        Ok(())
    }

    /// 获取配置
    pub async fn get_config(&self) -> TrafficMonitorConfig {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update_config(
        &self,
        config: TrafficMonitorConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 检查告警
    pub async fn check_alerts(&self) -> Vec<String> {
        let config = self.config.read().await;
        let stats = self.stats.read().await;

        if !config.alert.enabled {
            return Vec::new();
        }

        let mut alerts = Vec::new();

        for rule in &config.alert.rules {
            if rule.enabled {
                let metric_value = self.get_metric_value(&rule.metric_name, &stats).await;
                if self.check_alert_rule(rule, metric_value).await {
                    alerts.push(format!(
                        "Alert triggered: {} - Metric {} {} threshold {}",
                        rule.name, rule.metric_name, rule.operation, rule.threshold
                    ));
                }
            }
        }

        alerts
    }

    /// 获取指标值
    async fn get_metric_value(&self, metric_name: &str, stats: &TrafficStats) -> f64 {
        match metric_name {
            "total_requests" => stats.total_requests as f64,
            "success_requests" => stats.success_requests as f64,
            "failure_requests" => stats.failure_requests as f64,
            "avg_response_time_ms" => stats.avg_response_time_ms,
            "max_response_time_ms" => stats.max_response_time_ms,
            "min_response_time_ms" => stats.min_response_time_ms,
            "request_rate" => stats.request_rate,
            "error_rate" => stats.error_rate,
            "bandwidth_usage" => stats.bandwidth_usage,
            _ => 0.0,
        }
    }

    /// 检查告警规则
    async fn check_alert_rule(&self, rule: &AlertRule, metric_value: f64) -> bool {
        match rule.operation.as_str() {
            "gt" => metric_value > rule.threshold,
            "ge" => metric_value >= rule.threshold,
            "lt" => metric_value < rule.threshold,
            "le" => metric_value <= rule.threshold,
            "eq" => metric_value == rule.threshold,
            _ => false,
        }
    }

    /// 导出统计信息
    pub async fn export_stats(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let stats_guard = self.stats.read().await;
        let stats = &*stats_guard;
        Ok(serde_json::to_value(stats)?)
    }
}
