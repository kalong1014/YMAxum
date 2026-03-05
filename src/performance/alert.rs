use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::performance::monitor::PerformanceMonitor;

/// 告警级别
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertLevel::Info => write!(f, "Info"),
            AlertLevel::Warning => write!(f, "Warning"),
            AlertLevel::Error => write!(f, "Error"),
            AlertLevel::Critical => write!(f, "Critical"),
        }
    }
}

/// 告警类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertType {
    HttpErrorRate,
    HttpResponseTime,
    DatabaseErrorRate,
    DatabaseQueryTime,
    CacheHitRate,
    MemoryUsage,
    CpuUsage,
    NetworkUsage,
    DiskUsage,
    PluginErrorRate,
    ActiveRequests,
    DatabaseConnectionPool,
    SystemLoad,
    NetworkInbound,
    NetworkOutbound,
    DiskRead,
    DiskWrite,
    PluginLatency,
    ApiRequestRate,
    ApiErrorRate,
    SessionCount,
    CacheSize,
    QueueDepth,
    ThreadCount,
    Uptime,
    HealthCheckStatus,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::HttpErrorRate => write!(f, "HTTP错误率"),
            AlertType::HttpResponseTime => write!(f, "HTTP响应时间"),
            AlertType::DatabaseErrorRate => write!(f, "数据库错误率"),
            AlertType::DatabaseQueryTime => write!(f, "数据库查询时间"),
            AlertType::CacheHitRate => write!(f, "缓存命中率"),
            AlertType::MemoryUsage => write!(f, "内存使用"),
            AlertType::CpuUsage => write!(f, "CPU使用"),
            AlertType::NetworkUsage => write!(f, "网络使用"),
            AlertType::DiskUsage => write!(f, "磁盘使用"),
            AlertType::PluginErrorRate => write!(f, "插件错误率"),
            AlertType::ActiveRequests => write!(f, "活跃请求数"),
            AlertType::DatabaseConnectionPool => write!(f, "数据库连接池"),
            AlertType::SystemLoad => write!(f, "系统负载"),
            AlertType::NetworkInbound => write!(f, "网络入站流量"),
            AlertType::NetworkOutbound => write!(f, "网络出站流量"),
            AlertType::DiskRead => write!(f, "磁盘读取"),
            AlertType::DiskWrite => write!(f, "磁盘写入"),
            AlertType::PluginLatency => write!(f, "插件延迟"),
            AlertType::ApiRequestRate => write!(f, "API请求率"),
            AlertType::ApiErrorRate => write!(f, "API错误率"),
            AlertType::SessionCount => write!(f, "会话数"),
            AlertType::CacheSize => write!(f, "缓存大小"),
            AlertType::QueueDepth => write!(f, "队列深度"),
            AlertType::ThreadCount => write!(f, "线程数"),
            AlertType::Uptime => write!(f, "系统运行时间"),
            AlertType::HealthCheckStatus => write!(f, "健康检查状态"),
        }
    }
}

/// 告警配置
#[derive(Debug, Clone)]
pub struct AlertConfig {
    pub alert_type: AlertType,
    pub level: AlertLevel,
    pub threshold: f64,
    pub duration: Duration,
    pub cooldown: Duration,
    pub enabled: bool,
}

/// 告警实例
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub alert_type: AlertType,
    pub level: AlertLevel,
    pub message: String,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: Instant,
    pub resolved: bool,
    pub resolved_at: Option<Instant>,
}

/// 告警通知方式
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NotificationMethod {
    Console,
    Log,
    Email,
    Webhook,
    Slack,
    PagerDuty,
}

/// 告警通知配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationConfig {
    pub method: NotificationMethod,
    pub enabled: bool,
    pub config: serde_json::Value,
}

/// 告警管理器
#[derive(Debug, Clone)]
pub struct AlertManager {
    monitor: Arc<PerformanceMonitor>,
    alerts: Arc<Mutex<Vec<Alert>>>,
    configs: Vec<AlertConfig>,
    notification_configs: Vec<NotificationConfig>,
    _last_checked: Instant,
}

impl AlertManager {
    /// 创建新的告警管理器
    pub fn new(monitor: Arc<PerformanceMonitor>) -> Self {
        let default_configs = vec![
            // HTTP 错误率告警
            AlertConfig {
                alert_type: AlertType::HttpErrorRate,
                level: AlertLevel::Warning,
                threshold: 0.05, // 5%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::HttpErrorRate,
                level: AlertLevel::Error,
                threshold: 0.1, // 10%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // HTTP 响应时间告警
            AlertConfig {
                alert_type: AlertType::HttpResponseTime,
                level: AlertLevel::Warning,
                threshold: 1.0, // 1秒
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::HttpResponseTime,
                level: AlertLevel::Error,
                threshold: 3.0, // 3秒
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 数据库错误率告警
            AlertConfig {
                alert_type: AlertType::DatabaseErrorRate,
                level: AlertLevel::Warning,
                threshold: 0.05, // 5%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::DatabaseErrorRate,
                level: AlertLevel::Error,
                threshold: 0.1, // 10%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 缓存命中率告警
            AlertConfig {
                alert_type: AlertType::CacheHitRate,
                level: AlertLevel::Warning,
                threshold: 0.7, // 70%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 内存使用告警
            AlertConfig {
                alert_type: AlertType::MemoryUsage,
                level: AlertLevel::Warning,
                threshold: 80.0, // 80%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::MemoryUsage,
                level: AlertLevel::Error,
                threshold: 90.0, // 90%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // CPU 使用告警
            AlertConfig {
                alert_type: AlertType::CpuUsage,
                level: AlertLevel::Warning,
                threshold: 70.0, // 70%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::CpuUsage,
                level: AlertLevel::Error,
                threshold: 85.0, // 85%
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 活跃请求数告警
            AlertConfig {
                alert_type: AlertType::ActiveRequests,
                level: AlertLevel::Warning,
                threshold: 100.0, // 100个
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            AlertConfig {
                alert_type: AlertType::ActiveRequests,
                level: AlertLevel::Error,
                threshold: 200.0, // 200个
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 网络入站流量告警
            AlertConfig {
                alert_type: AlertType::NetworkInbound,
                level: AlertLevel::Warning,
                threshold: 10.0, // 10 Mbps
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 网络出站流量告警
            AlertConfig {
                alert_type: AlertType::NetworkOutbound,
                level: AlertLevel::Warning,
                threshold: 10.0, // 10 Mbps
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 磁盘读取告警
            AlertConfig {
                alert_type: AlertType::DiskRead,
                level: AlertLevel::Warning,
                threshold: 50.0, // 50 MB/s
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 磁盘写入告警
            AlertConfig {
                alert_type: AlertType::DiskWrite,
                level: AlertLevel::Warning,
                threshold: 30.0, // 30 MB/s
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 插件延迟告警
            AlertConfig {
                alert_type: AlertType::PluginLatency,
                level: AlertLevel::Warning,
                threshold: 500.0, // 500 ms
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // API请求率告警
            AlertConfig {
                alert_type: AlertType::ApiRequestRate,
                level: AlertLevel::Warning,
                threshold: 1000.0, // 1000 req/s
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 会话数告警
            AlertConfig {
                alert_type: AlertType::SessionCount,
                level: AlertLevel::Warning,
                threshold: 1000.0, // 1000个
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 缓存大小告警
            AlertConfig {
                alert_type: AlertType::CacheSize,
                level: AlertLevel::Warning,
                threshold: 500.0, // 500 MB
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 队列深度告警
            AlertConfig {
                alert_type: AlertType::QueueDepth,
                level: AlertLevel::Warning,
                threshold: 100.0, // 100个
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
            // 线程数告警
            AlertConfig {
                alert_type: AlertType::ThreadCount,
                level: AlertLevel::Warning,
                threshold: 100.0, // 100个
                duration: Duration::from_secs(60),
                cooldown: Duration::from_secs(300),
                enabled: true,
            },
        ];

        let default_notifications = vec![
            NotificationConfig {
                method: NotificationMethod::Console,
                enabled: true,
                config: serde_json::json!({}),
            },
            NotificationConfig {
                method: NotificationMethod::Log,
                enabled: true,
                config: serde_json::json!({}),
            },
            NotificationConfig {
                method: NotificationMethod::Email,
                enabled: false,
                config: serde_json::json!({
                    "smtp_server": "smtp.example.com",
                    "smtp_port": 587,
                    "username": "alert@example.com",
                    "password": "password",
                    "from": "alert@example.com",
                    "to": ["admin@example.com"],
                    "subject": "YMAxum Alert"
                }),
            },
            NotificationConfig {
                method: NotificationMethod::Webhook,
                enabled: false,
                config: serde_json::json!({
                    "url": "https://example.com/webhook",
                    "headers": {
                        "Content-Type": "application/json",
                        "Authorization": "Bearer token"
                    }
                }),
            },
        ];

        Self {
            monitor,
            alerts: Arc::new(Mutex::new(Vec::new())),
            configs: default_configs,
            notification_configs: default_notifications,
            _last_checked: Instant::now(),
        }
    }

    /// 添加告警配置
    pub fn add_alert_config(&mut self, config: AlertConfig) {
        self.configs.push(config);
    }

    /// 更新告警配置
    pub fn update_alert_config(
        &mut self,
        alert_type: AlertType,
        level: AlertLevel,
        threshold: f64,
    ) {
        for config in &mut self.configs {
            if config.alert_type == alert_type && config.level == level {
                config.threshold = threshold;
            }
        }
    }

    /// 启用/禁用告警类型
    pub fn set_alert_enabled(&mut self, alert_type: AlertType, enabled: bool) {
        for config in &mut self.configs {
            if config.alert_type == alert_type {
                config.enabled = enabled;
            }
        }
    }

    /// 检查告警
    pub fn check_alerts(&self) -> Vec<Alert> {
        let mut new_alerts = Vec::new();
        let mut alerts = self.alerts.lock().unwrap();

        // 更新系统指标
        self.monitor.update_memory_usage();
        self.monitor.update_cpu_usage();

        // 检查每个告警配置
        for config in &self.configs {
            if !config.enabled {
                continue;
            }

            let (value, message) = match config.alert_type {
                AlertType::HttpErrorRate => {
                    let (total, errors) = self.monitor.get_http_metrics();
                    if total > 0 {
                        let error_rate = errors as f64 / total as f64 * 100.0;
                        (error_rate, format!("HTTP错误率: {:.2}%", error_rate))
                    } else {
                        (0.0, "HTTP错误率: 0%".to_string())
                    }
                }
                AlertType::HttpResponseTime => {
                    let response_times = self.monitor.get_response_times();
                    if !response_times.is_empty() {
                        let avg_time =
                            response_times.iter().sum::<f64>() / response_times.len() as f64;
                        (avg_time, format!("HTTP平均响应时间: {:.2}ms", avg_time))
                    } else {
                        (0.0, "HTTP平均响应时间: 0ms".to_string())
                    }
                }
                AlertType::DatabaseErrorRate => {
                    let (total, errors, _) = self.monitor.get_database_metrics();
                    if total > 0 {
                        let error_rate = errors as f64 / total as f64 * 100.0;
                        (error_rate, format!("数据库错误率: {:.2}%", error_rate))
                    } else {
                        (0.0, "数据库错误率: 0%".to_string())
                    }
                }
                AlertType::CacheHitRate => {
                    let hit_rate = self.monitor.get_cache_hit_rate();
                    (hit_rate, format!("缓存命中率: {:.2}%", hit_rate))
                }
                AlertType::MemoryUsage => {
                    // 模拟内存使用率（0-100%）
                    #[cfg(feature = "system_info")]
                    {
                        use sysinfo::System;
                        let mut system = System::new_all();
                        system.refresh_all();
                        let memory_usage = ((system.total_memory() - system.available_memory())
                            as f64
                            / system.total_memory() as f64)
                            * 100.0;
                        (memory_usage, format!("内存使用率: {:.2}%", memory_usage))
                    }
                    #[cfg(not(feature = "system_info"))]
                    {
                        (0.0, "内存使用率: 未知".to_string())
                    }
                }
                AlertType::CpuUsage => {
                    // 模拟CPU使用率（0-100%）
                    #[cfg(feature = "system_info")]
                    {
                        use sysinfo::System;
                        let mut system = System::new_all();
                        system.refresh_all();
                        let cpu_usage = system.global_cpu_usage() as f64;
                        (cpu_usage, format!("CPU使用率: {:.2}%", cpu_usage))
                    }
                    #[cfg(not(feature = "system_info"))]
                    {
                        (0.0, "CPU使用率: 未知".to_string())
                    }
                }
                AlertType::ActiveRequests => {
                    let active = self.monitor.get_active_connections() as f64;
                    (active, format!("活跃请求数: {:.0}", active))
                }
                AlertType::NetworkInbound => {
                    // 模拟网络入站流量（Mbps）
                    (5.0, "网络入站流量: 5.0 Mbps".to_string())
                }
                AlertType::NetworkOutbound => {
                    // 模拟网络出站流量（Mbps）
                    (3.0, "网络出站流量: 3.0 Mbps".to_string())
                }
                AlertType::DiskRead => {
                    // 模拟磁盘读取速度（MB/s）
                    (20.0, "磁盘读取: 20.0 MB/s".to_string())
                }
                AlertType::DiskWrite => {
                    // 模拟磁盘写入速度（MB/s）
                    (10.0, "磁盘写入: 10.0 MB/s".to_string())
                }
                AlertType::PluginLatency => {
                    // 模拟插件延迟（ms）
                    (100.0, "插件延迟: 100.0 ms".to_string())
                }
                AlertType::ApiRequestRate => {
                    // 模拟API请求率（req/s）
                    (500.0, "API请求率: 500.0 req/s".to_string())
                }
                AlertType::SessionCount => {
                    // 模拟会话数
                    (500.0, "会话数: 500".to_string())
                }
                AlertType::CacheSize => {
                    // 模拟缓存大小（MB）
                    (200.0, "缓存大小: 200.0 MB".to_string())
                }
                AlertType::QueueDepth => {
                    // 模拟队列深度
                    (50.0, "队列深度: 50".to_string())
                }
                AlertType::ThreadCount => {
                    // 模拟线程数
                    (50.0, "线程数: 50".to_string())
                }
                _ => (0.0, "未实现的告警类型".to_string()),
            };

            // 检查是否超过阈值
            let is_breach = match config.alert_type {
                AlertType::CacheHitRate => value < config.threshold, // 缓存命中率是越低越严重
                _ => value > config.threshold,                       // 其他都是越高越严重
            };

            if is_breach {
                // 检查是否已经有未解决的相同类型和级别的告警
                let existing_alert = alerts.iter().find(|a| {
                    !a.resolved && a.alert_type == config.alert_type && a.level == config.level
                });

                if existing_alert.is_none() {
                    // 创建新告警
                    let alert = Alert {
                        id: format!(
                            "{:?}_{:?}_{:?}",
                            config.alert_type,
                            config.level,
                            Instant::now()
                        ),
                        alert_type: config.alert_type.clone(),
                        level: config.level.clone(),
                        message,
                        value,
                        threshold: config.threshold,
                        timestamp: Instant::now(),
                        resolved: false,
                        resolved_at: None,
                    };
                    new_alerts.push(alert.clone());
                    alerts.push(alert);
                }
            } else {
                // 检查是否需要解决告警
                for alert in &mut *alerts {
                    if !alert.resolved
                        && alert.alert_type == config.alert_type
                        && alert.level == config.level
                    {
                        alert.resolved = true;
                        alert.resolved_at = Some(Instant::now());
                    }
                }
            }
        }

        new_alerts
    }

    /// 获取未解决的告警
    pub fn get_unresolved_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.lock().unwrap();
        alerts.iter().filter(|a| !a.resolved).cloned().collect()
    }

    /// 获取已解决的告警
    pub fn get_resolved_alerts(&self, since: Option<Instant>) -> Vec<Alert> {
        let alerts = self.alerts.lock().unwrap();
        let mut filtered = alerts
            .iter()
            .filter(|a| a.resolved)
            .cloned()
            .collect::<Vec<_>>();

        if let Some(since) = since {
            filtered.retain(|a| a.resolved_at.map(|t| t >= since).unwrap_or(false));
        }

        filtered
    }

    /// 获取所有告警
    pub fn get_all_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.lock().unwrap();
        alerts.clone()
    }

    /// 清除已解决的告警
    pub fn clear_resolved_alerts(&self, older_than: Duration) {
        let mut alerts = self.alerts.lock().unwrap();
        let now = Instant::now();
        alerts.retain(|a| {
            if a.resolved {
                if let Some(resolved_at) = a.resolved_at {
                    now.duration_since(resolved_at) < older_than
                } else {
                    true
                }
            } else {
                true
            }
        });
    }

    /// 获取告警统计
    pub fn get_alert_stats(&self) -> (usize, usize, usize, usize) {
        let alerts = self.alerts.lock().unwrap();
        let info = alerts
            .iter()
            .filter(|a| !a.resolved && a.level == AlertLevel::Info)
            .count();
        let warning = alerts
            .iter()
            .filter(|a| !a.resolved && a.level == AlertLevel::Warning)
            .count();
        let error = alerts
            .iter()
            .filter(|a| !a.resolved && a.level == AlertLevel::Error)
            .count();
        let critical = alerts
            .iter()
            .filter(|a| !a.resolved && a.level == AlertLevel::Critical)
            .count();
        (info, warning, error, critical)
    }

    /// 获取告警配置
    pub fn get_alert_configs(&self) -> Vec<AlertConfig> {
        self.configs.clone()
    }

    /// 启动告警监控任务
    pub async fn start_alert_monitoring(&self, check_interval: Duration) {
        use tokio::time::interval;
        let mut interval = interval(check_interval);
        let manager = self.clone();

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let alerts = manager.check_alerts();

                // 处理新告警
                for alert in alerts {
                    manager.handle_alert(alert).await;
                }
            }
        });
    }

    /// 处理告警
    async fn handle_alert(&self, alert: Alert) {
        // 遍历所有启用的通知方式
        for notification in &self.notification_configs {
            if notification.enabled {
                self.send_notification(notification, &alert).await;
            }
        }
    }

    /// 发送告警通知
    async fn send_notification(&self, config: &NotificationConfig, alert: &Alert) {
        match config.method {
            NotificationMethod::Console => {
                println!("[{}] {}: {}", alert.level, alert.alert_type, alert.message);
            }
            NotificationMethod::Log => match alert.level {
                AlertLevel::Info => log::info!("{}: {}", alert.alert_type, alert.message),
                AlertLevel::Warning => log::warn!("{}: {}", alert.alert_type, alert.message),
                AlertLevel::Error => log::error!("{}: {}", alert.alert_type, alert.message),
                AlertLevel::Critical => {
                    log::error!("[CRITICAL] {}: {}", alert.alert_type, alert.message)
                }
            },
            NotificationMethod::Email => {
                // 邮件通知实现
                if let Some(smtp_server) = config.config.get("smtp_server").and_then(|v| v.as_str())
                {
                    // 这里可以集成实际的邮件发送库
                    log::info!("[Email Notification] Sending alert via {}", smtp_server);
                }
            }
            NotificationMethod::Webhook => {
                // Webhook通知实现
                if let Some(url) = config.config.get("url").and_then(|v| v.as_str()) {
                    let _headers = config
                        .config
                        .get("headers")
                        .and_then(|v| v.as_object())
                        .unwrap_or(&serde_json::Map::new());

                    // 构建告警数据
                    let _alert_data = serde_json::json!({
                        "id": alert.id,
                        "type": format!("{}", alert.alert_type),
                        "level": format!("{}", alert.level),
                        "message": alert.message,
                        "value": alert.value,
                        "threshold": alert.threshold,
                        "timestamp": alert.timestamp.elapsed().as_secs(),
                        "resolved": alert.resolved
                    });

                    // 这里可以集成实际的HTTP客户端库
                    log::info!("[Webhook Notification] Sending alert to {}", url);
                }
            }
            NotificationMethod::Slack => {
                // Slack通知实现
                if let Some(_webhook_url) =
                    config.config.get("webhook_url").and_then(|v| v.as_str())
                {
                    // 构建Slack消息
                    let _slack_message = serde_json::json!({
                        "text": format!("[{}] {}: {}", alert.level, alert.alert_type, alert.message),
                        "attachments": [{
                            "color": match alert.level {
                                AlertLevel::Info => "#36a64f",
                                AlertLevel::Warning => "#ffcc00",
                                AlertLevel::Error => "#ff0000",
                                AlertLevel::Critical => "#990000",
                            },
                            "fields": [
                                {"title": "Value", "value": format!("{:.2}", alert.value), "short": true},
                                {"title": "Threshold", "value": format!("{:.2}", alert.threshold), "short": true},
                                {"title": "Type", "value": format!("{}", alert.alert_type), "short": true},
                                {"title": "Level", "value": format!("{}", alert.level), "short": true},
                            ]
                        }]
                    });

                    // 这里可以集成实际的HTTP客户端库
                    log::info!("[Slack Notification] Sending alert to Slack webhook");
                }
            }
            NotificationMethod::PagerDuty => {
                // PagerDuty通知实现
                if let Some(_integration_key) = config
                    .config
                    .get("integration_key")
                    .and_then(|v| v.as_str())
                {
                    // 构建PagerDuty事件
                    let _pagerduty_event = serde_json::json!({
                        "service_key": _integration_key,
                        "event_type": if alert.resolved { "resolve" } else { "trigger" },
                        "description": format!("{}: {}", alert.alert_type, alert.message),
                        "details": {
                            "level": format!("{}", alert.level),
                            "value": alert.value,
                            "threshold": alert.threshold,
                            "timestamp": alert.timestamp.elapsed().as_secs()
                        }
                    });

                    // 这里可以集成实际的HTTP客户端库
                    log::info!("[PagerDuty Notification] Sending alert to PagerDuty");
                }
            }
        }
    }

    /// 获取告警状态的HTTP处理函数
    pub async fn alerts_handler(&self) -> String {
        let unresolved = self.get_unresolved_alerts();
        let (info, warning, error, critical) = self.get_alert_stats();

        let mut response = format!(
            "# 告警状态\n\n## 统计\n\n- Info: {}\n- Warning: {}\n- Error: {}\n- Critical: {}\n\n## 未解决的告警\n\n",
            info, warning, error, critical
        );

        if unresolved.is_empty() {
            response.push_str("无未解决的告警\n");
        } else {
            for alert in unresolved {
                response.push_str(&format!(
                    "- [{}] {}: {} (值: {:.2}, 阈值: {:.2})\n",
                    alert.level, alert.alert_type, alert.message, alert.value, alert.threshold
                ));
            }
        }

        response
    }

    /// 创建告警路由
    pub fn create_router(&self) -> axum::Router {
        use std::sync::Arc;
        let manager = Arc::new(self.clone());
        axum::Router::new().route(
            "/alerts",
            axum::routing::get(move || {
                let manager = manager.clone();
                async move { manager.alerts_handler().await }
            }),
        )
    }
}

/// 扩展PerformanceMonitor以支持告警
impl PerformanceMonitor {
    /// 创建告警管理器
    pub fn create_alert_manager(&self) -> AlertManager {
        AlertManager::new(Arc::new(self.clone()))
    }
}
