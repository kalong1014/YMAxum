//! 告警面板组件
//! 用于显示系统告警信息和处理告警

use serde::{Deserialize, Serialize};

/// 告警级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// 告警状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    New,
    Acknowledged,
    Resolved,
    Ignored,
}

/// 告警信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// 告警ID
    pub id: String,
    /// 告警标题
    pub title: String,
    /// 告警描述
    pub description: String,
    /// 告警级别
    pub level: AlertLevel,
    /// 告警状态
    pub status: AlertStatus,
    /// 告警来源
    pub source: String,
    /// 告警时间
    pub timestamp: String,
    /// 告警指标
    pub metric: String,
    /// 指标值
    pub metric_value: f64,
    /// 阈值
    pub threshold: f64,
    /// 组件名称
    pub component: String,
    /// 相关日志
    pub related_logs: Vec<String>,
    /// 处理建议
    pub recommendation: String,
    /// 处理人
    pub assignee: Option<String>,
    /// 处理时间
    pub resolved_at: Option<String>,
    /// 处理备注
    pub resolution_note: Option<String>,
}

/// 告警面板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPanelConfig {
    /// 是否启用实时更新
    pub realtime_updates: bool,
    /// 更新间隔(秒)
    pub update_interval: u32,
    /// 是否显示已解决的告警
    pub show_resolved: bool,
    /// 是否按级别分组显示
    pub group_by_level: bool,
    /// 是否按组件分组显示
    pub group_by_component: bool,
    /// 告警历史保存时间(天)
    pub history_days: u32,
    /// 是否启用自动处理
    pub enable_auto_resolution: bool,
    /// 自动处理阈值
    pub auto_resolution_threshold: f64,
    /// 是否发送通知
    pub enable_notifications: bool,
    /// 通知配置
    pub notification_config: std::collections::HashMap<String, String>,
}

/// 告警统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStats {
    /// 总告警数
    pub total_alerts: u32,
    /// 未处理告警数
    pub unhandled_alerts: u32,
    /// 按级别统计
    pub alerts_by_level: std::collections::HashMap<String, u32>,
    /// 按组件统计
    pub alerts_by_component: std::collections::HashMap<String, u32>,
    /// 最近24小时告警数
    pub alerts_last_24h: u32,
    /// 平均处理时间(分钟)
    pub avg_resolution_time: f64,
    /// 告警解决率
    pub resolution_rate: f64,
}

/// 告警面板
#[derive(Debug, Clone)]
pub struct AlertPanel {
    /// 配置
    config: AlertPanelConfig,
    /// 告警列表
    alerts: std::sync::Arc<tokio::sync::RwLock<Vec<Alert>>>,
    /// 告警统计
    stats: std::sync::Arc<tokio::sync::RwLock<AlertStats>>,
}

impl AlertPanel {
    /// 创建新的告警面板
    pub fn new() -> Self {
        let config = AlertPanelConfig {
            realtime_updates: true,
            update_interval: 5,
            show_resolved: false,
            group_by_level: true,
            group_by_component: false,
            history_days: 7,
            enable_auto_resolution: true,
            auto_resolution_threshold: 0.8,
            enable_notifications: true,
            notification_config: std::collections::HashMap::new(),
        };

        let stats = AlertStats {
            total_alerts: 0,
            unhandled_alerts: 0,
            alerts_by_level: std::collections::HashMap::new(),
            alerts_by_component: std::collections::HashMap::new(),
            alerts_last_24h: 0,
            avg_resolution_time: 0.0,
            resolution_rate: 0.0,
        };

        Self {
            config,
            alerts: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            stats: std::sync::Arc::new(tokio::sync::RwLock::new(stats)),
        }
    }

    /// 初始化告警面板
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化告警面板
        println!("Initializing alert panel...");

        // 初始化告警统计
        self.update_stats().await?;

        Ok(())
    }

    /// 更新告警信息
    pub async fn update_alerts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟告警更新
        println!("Updating alerts...");

        let mut alerts = self.alerts.write().await;

        // 随机生成新告警
        if rand::random::<f64>() > 0.7 {
            // 30%的概率生成新告警
            let new_alert = self.generate_new_alert().await;
            alerts.push(new_alert);
        }

        // 随机更新告警状态
        for alert in &mut *alerts {
            if alert.status == AlertStatus::New && rand::random::<f64>() > 0.8 {
                // 20%的概率确认告警
                alert.status = AlertStatus::Acknowledged;
                alert.assignee = Some("System".to_string());
            } else if alert.status == AlertStatus::Acknowledged && rand::random::<f64>() > 0.7 {
                // 30%的概率解决告警
                alert.status = AlertStatus::Resolved;
                alert.resolved_at = Some(chrono::Utc::now().to_string());
                alert.resolution_note = Some("自动解决".to_string());
            }
        }

        // 清理旧告警
        self.cleanup_old_alerts().await?;

        // 更新告警统计
        self.update_stats().await?;

        Ok(())
    }

    /// 生成新告警
    async fn generate_new_alert(&self) -> Alert {
        let alert_levels = [
            AlertLevel::Info,
            AlertLevel::Warning,
            AlertLevel::Error,
            AlertLevel::Critical,
        ];
        let components = [
            "API Server",
            "Database",
            "Plugin System",
            "GUF Integration",
            "Cache System",
            "Security System",
        ];
        let metrics = [
            "cpu_usage",
            "memory_usage",
            "disk_usage",
            "response_time",
            "error_rate",
        ];

        let level = alert_levels[rand::random::<usize>() % alert_levels.len()].clone();
        let component = components[rand::random::<usize>() % components.len()].to_string();
        let metric = metrics[rand::random::<usize>() % metrics.len()].to_string();

        let metric_value = match metric.as_str() {
            "cpu_usage" => 85.0 + (rand::random::<f64>() * 10.0),
            "memory_usage" => 90.0 + (rand::random::<f64>() * 5.0),
            "disk_usage" => 92.0 + (rand::random::<f64>() * 5.0),
            "response_time" => 550.0 + (rand::random::<f64>() * 100.0),
            "error_rate" => 6.0 + (rand::random::<f64>() * 4.0),
            _ => 100.0,
        };

        let threshold = match metric.as_str() {
            "cpu_usage" => 80.0,
            "memory_usage" => 85.0,
            "disk_usage" => 90.0,
            "response_time" => 500.0,
            "error_rate" => 5.0,
            _ => 90.0,
        };

        let title = format!("{} {} 告警", component, metric);
        let description = format!(
            "{}的{}指标达到{}，超过阈值{}",
            component, metric, metric_value, threshold
        );

        Alert {
            id: format!(
                "alert-{}-{}",
                chrono::Utc::now().timestamp(),
                rand::random::<u32>()
            ),
            title,
            description,
            level,
            status: AlertStatus::New,
            source: "System Monitor".to_string(),
            timestamp: chrono::Utc::now().to_string(),
            metric: metric.clone(),
            metric_value,
            threshold,
            component,
            related_logs: vec![format!("{}: 指标超过阈值", chrono::Utc::now())],
            recommendation: self.generate_recommendation(&metric, metric_value, threshold),
            assignee: None,
            resolved_at: None,
            resolution_note: None,
        }
    }

    /// 生成处理建议
    fn generate_recommendation(&self, metric: &str, _value: f64, _threshold: f64) -> String {
        match metric {
            "cpu_usage" => "检查系统负载，考虑优化代码或增加资源".to_string(),
            "memory_usage" => "检查内存泄漏，考虑增加内存或优化内存使用".to_string(),
            "disk_usage" => "清理磁盘空间，考虑增加磁盘容量".to_string(),
            "response_time" => "检查API性能，考虑优化代码或增加缓存".to_string(),
            "error_rate" => "检查错误日志，修复导致错误的问题".to_string(),
            _ => "请检查相关系统和日志".to_string(),
        }
    }

    /// 清理旧告警
    async fn cleanup_old_alerts(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut alerts = self.alerts.write().await;
        let cutoff_time =
            chrono::Utc::now() - chrono::Duration::days(self.config.history_days as i64);

        alerts.retain(|alert| {
            let alert_time =
                chrono::DateTime::parse_from_rfc3339(&alert.timestamp.replace(" ", "T"))
                    .unwrap_or(chrono::Utc::now().into());
            alert_time > cutoff_time
        });

        Ok(())
    }

    /// 更新告警统计
    async fn update_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let alerts = self.alerts.read().await;
        let mut stats = self.stats.write().await;

        // 更新总告警数
        stats.total_alerts = alerts.len() as u32;

        // 更新未处理告警数
        stats.unhandled_alerts = alerts
            .iter()
            .filter(|a| a.status == AlertStatus::New)
            .count() as u32;

        // 更新按级别统计
        let mut alerts_by_level = std::collections::HashMap::new();
        for alert in &*alerts {
            let level_str = match alert.level {
                AlertLevel::Info => "info",
                AlertLevel::Warning => "warning",
                AlertLevel::Error => "error",
                AlertLevel::Critical => "critical",
            };
            *alerts_by_level.entry(level_str.to_string()).or_insert(0) += 1;
        }
        stats.alerts_by_level = alerts_by_level;

        // 更新按组件统计
        let mut alerts_by_component = std::collections::HashMap::new();
        for alert in &*alerts {
            *alerts_by_component
                .entry(alert.component.clone())
                .or_insert(0) += 1;
        }
        stats.alerts_by_component = alerts_by_component;

        // 更新最近24小时告警数
        let last_24h = chrono::Utc::now() - chrono::Duration::hours(24);
        stats.alerts_last_24h = alerts
            .iter()
            .filter(|a| {
                let alert_time =
                    chrono::DateTime::parse_from_rfc3339(&a.timestamp.replace(" ", "T"))
                        .unwrap_or(chrono::Utc::now().into());
                alert_time > last_24h
            })
            .count() as u32;

        // 更新平均处理时间和解决率
        let resolved_alerts = alerts
            .iter()
            .filter(|a| a.status == AlertStatus::Resolved && a.resolved_at.is_some())
            .collect::<Vec<_>>();
        if !resolved_alerts.is_empty() {
            let total_resolution_time = resolved_alerts
                .iter()
                .filter_map(|a| {
                    let alert_time =
                        chrono::DateTime::parse_from_rfc3339(&a.timestamp.replace(" ", "T")).ok();
                    let resolved_time = a.resolved_at.as_ref().and_then(|t| {
                        chrono::DateTime::parse_from_rfc3339(&t.replace(" ", "T")).ok()
                    });
                    alert_time.and_then(|at| resolved_time.map(|rt| (rt - at).num_minutes() as f64))
                })
                .sum::<f64>();
            stats.avg_resolution_time = total_resolution_time / resolved_alerts.len() as f64;
        }

        stats.resolution_rate = if !alerts.is_empty() {
            alerts
                .iter()
                .filter(|a| a.status == AlertStatus::Resolved)
                .count() as f64
                / alerts.len() as f64
        } else {
            0.0
        };

        Ok(())
    }

    /// 获取告警列表
    pub async fn get_alerts(&self) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        let alerts = self.alerts.read().await;
        let filtered_alerts = if self.config.show_resolved {
            alerts.clone()
        } else {
            alerts
                .iter()
                .filter(|a| a.status != AlertStatus::Resolved)
                .cloned()
                .collect()
        };
        Ok(filtered_alerts)
    }

    /// 获取告警统计
    pub async fn get_stats(&self) -> Result<AlertStats, Box<dyn std::error::Error>> {
        let stats = self.stats.read().await;
        Ok(stats.clone())
    }

    /// 处理告警
    pub async fn handle_alert(
        &self,
        alert_id: &str,
        status: AlertStatus,
        assignee: Option<String>,
        resolution_note: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut alerts = self.alerts.write().await;

        for alert in &mut *alerts {
            if alert.id == alert_id {
                alert.status = status.clone();
                alert.assignee = assignee;
                alert.resolution_note = resolution_note.clone();
                if status == AlertStatus::Resolved {
                    alert.resolved_at = Some(chrono::Utc::now().to_rfc3339());
                }
                break;
            }
        }

        // 更新告警统计
        self.update_stats().await?;

        Ok(())
    }

    /// 获取未处理的告警
    pub async fn get_unhandled_alerts(&self) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        let alerts = self.alerts.read().await;
        let unhandled_alerts = alerts
            .iter()
            .filter(|a| a.status == AlertStatus::New)
            .cloned()
            .collect();
        Ok(unhandled_alerts)
    }

    /// 获取按级别分组的告警
    pub async fn get_alerts_by_level(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<Alert>>, Box<dyn std::error::Error>> {
        let alerts = self.alerts.read().await;
        let mut alerts_by_level = std::collections::HashMap::new();

        for alert in &*alerts {
            let level_str = match alert.level {
                AlertLevel::Info => "info",
                AlertLevel::Warning => "warning",
                AlertLevel::Error => "error",
                AlertLevel::Critical => "critical",
            };
            alerts_by_level
                .entry(level_str.to_string())
                .or_insert_with(Vec::new)
                .push(alert.clone());
        }

        Ok(alerts_by_level)
    }

    /// 获取按组件分组的告警
    pub async fn get_alerts_by_component(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<Alert>>, Box<dyn std::error::Error>> {
        let alerts = self.alerts.read().await;
        let mut alerts_by_component = std::collections::HashMap::new();

        for alert in &*alerts {
            alerts_by_component
                .entry(alert.component.clone())
                .or_insert_with(Vec::new)
                .push(alert.clone());
        }

        Ok(alerts_by_component)
    }

    /// 获取告警面板配置
    pub fn get_config(&self) -> &AlertPanelConfig {
        &self.config
    }

    /// 更新告警面板配置
    pub fn update_config(&mut self, config: AlertPanelConfig) {
        self.config = config;
    }
}
