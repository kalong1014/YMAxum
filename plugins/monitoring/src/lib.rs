use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::info;
use ymaxum::core::iterate_api::{PluginLifecycle, IterateError};
use ymaxum::core::state::AppState;

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// 监控间隔（秒）
    pub monitoring_interval: u64,
    /// 告警阈值配置
    pub alert_thresholds: AlertThresholds,
    /// 通知配置
    pub notification_config: NotificationConfig,
    /// 告警历史保留天数
    pub alert_history_days: u32,
}

/// 告警阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU 使用率阈值（%）
    pub cpu_usage_threshold: f64,
    /// 内存使用率阈值（%）
    pub memory_usage_threshold: f64,
    /// 磁盘使用率阈值（%）
    pub disk_usage_threshold: f64,
    /// 组件响应时间阈值（ms）
    pub component_response_time_threshold: f64,
    /// 组件错误率阈值（%）
    pub component_error_rate_threshold: f64,
    /// 事件处理时间阈值（ms）
    pub event_processing_time_threshold: f64,
}

/// 通知配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 是否启用邮件通知
    pub enable_email: bool,
    /// 邮件服务器配置
    pub email_config: Option<EmailConfig>,
    /// 是否启用 Webhook 通知
    pub enable_webhook: bool,
    /// Webhook 配置
    pub webhook_config: Option<WebhookConfig>,
    /// 是否启用短信通知
    pub enable_sms: bool,
    /// 短信配置
    pub sms_config: Option<SmsConfig>,
}

/// 邮件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP 服务器地址
    pub smtp_server: String,
    /// SMTP 服务器端口
    pub smtp_port: u16,
    /// 发件人邮箱
    pub from_email: String,
    /// 发件人密码
    pub password: String,
    /// 收件人邮箱列表
    pub to_emails: Vec<String>,
    /// 是否启用 TLS
    pub enable_tls: bool,
}

/// Webhook 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// 认证令牌
    pub auth_token: Option<String>,
    /// 通知频率限制（秒）
    pub rate_limit: u64,
}

/// 短信配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsConfig {
    /// 短信服务提供商
    pub provider: String,
    /// API 密钥
    pub api_key: String,
    /// 手机号码列表
    pub phone_numbers: Vec<String>,
    /// 通知频率限制（秒）
    pub rate_limit: u64,
}

/// 监控数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringDataPoint {
    /// 数据点 ID
    pub id: String,
    /// 监控类型
    pub monitor_type: String,
    /// 监控值
    pub value: f64,
    /// 时间戳
    pub timestamp: u64,
    /// 标签
    pub tags: std::collections::HashMap<String, String>,
}

/// 告警级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 严重
    Critical,
}

/// 告警状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    /// 触发
    Triggered,
    /// 已解决
    Resolved,
    /// 已确认
    Acknowledged,
}

/// 告警
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// 告警 ID
    pub alert_id: String,
    /// 告警名称
    pub name: String,
    /// 告警描述
    pub description: String,
    /// 告警级别
    pub level: AlertLevel,
    /// 告警状态
    pub status: AlertStatus,
    /// 触发时间
    pub triggered_at: u64,
    /// 解决时间
    pub resolved_at: Option<u64>,
    /// 确认时间
    pub acknowledged_at: Option<u64>,
    /// 确认人
    pub acknowledged_by: Option<String>,
    /// 相关数据点
    pub related_data_points: Vec<String>,
    /// 标签
    pub tags: std::collections::HashMap<String, String>,
}

/// 监控插件
pub struct MonitoringPlugin {
    /// 插件配置
    config: MonitoringConfig,
    /// 监控数据存储
    data_store: Arc<RwLock<Vec<MonitoringDataPoint>>>,
    /// 告警存储
    alerts: Arc<RwLock<Vec<Alert>>>,
    /// 监控任务
    monitoring_task: Option<tokio::task::JoinHandle<()>>,
    /// 是否初始化
    initialized: bool,
}

impl Default for MonitoringPlugin {
    fn default() -> Self {
        Self {
            config: MonitoringConfig {
                monitoring_interval: 30,
                alert_thresholds: AlertThresholds {
                    cpu_usage_threshold: 80.0,
                    memory_usage_threshold: 85.0,
                    disk_usage_threshold: 90.0,
                    component_response_time_threshold: 500.0,
                    component_error_rate_threshold: 5.0,
                    event_processing_time_threshold: 100.0,
                },
                notification_config: NotificationConfig {
                    enable_email: false,
                    email_config: None,
                    enable_webhook: false,
                    webhook_config: None,
                    enable_sms: false,
                    sms_config: None,
                },
                alert_history_days: 30,
            },
            data_store: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            monitoring_task: None,
            initialized: false,
        }
    }
}

impl Clone for MonitoringPlugin {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            data_store: self.data_store.clone(),
            alerts: self.alerts.clone(),
            monitoring_task: None,
            initialized: self.initialized,
        }
    }
}

#[async_trait::async_trait]
impl PluginLifecycle for MonitoringPlugin {
    async fn init(&self, state: Arc<AppState>) -> Result<(), IterateError> {
        info!("Initializing GUF Monitoring Plugin v1.0.0");
        
        // 初始化监控存储目录
        std::fs::create_dir_all("./monitoring_data")
            .map_err(|e| IterateError::InitFailed(format!("Failed to create storage directory: {}", e)))?;
        
        info!("GUF Monitoring Plugin initialized successfully");
        Ok(())
    }
    
    async fn start(&self, state: Arc<AppState>) -> Result<(), IterateError> {
        info!("Starting GUF Monitoring Plugin");
        
        // 启动监控任务
        let mut self_mut = self.clone();
        self_mut.start_monitoring_task().await
            .map_err(|e| IterateError::StartFailed(e))?;
        
        info!("GUF Monitoring Plugin started successfully");
        Ok(())
    }
    
    async fn stop(&self, state: Arc<AppState>) -> Result<(), IterateError> {
        info!("Stopping GUF Monitoring Plugin");
        
        // 停止监控任务
        let mut self_mut = self.clone();
        
        if let Some(task) = self_mut.monitoring_task.take() {
            task.abort();
            if let Err(e) = task.await {
                info!("Monitoring task aborted: {:?}", e);
            }
        }
        
        // 保存数据到磁盘
        self_mut.save_data().await
            .map_err(|e| IterateError::StopFailed(e))?;
        
        info!("GUF Monitoring Plugin stopped successfully");
        Ok(())
    }
    
    fn name(&self) -> &'static str {
        "guf_monitoring_plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "GUF Monitoring Plugin - Monitoring and alerting for GUF ecosystem"
    }
    
    fn plugin_type(&self) -> &'static str {
        "monitoring"
    }
}

impl MonitoringPlugin {
    /// 创建新的监控插件
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 启动监控任务
    async fn start_monitoring_task(&mut self) -> Result<(), String> {
        let config = self.config.clone();
        let data_store = self.data_store.clone();
        let alerts = self.alerts.clone();
        
        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(config.monitoring_interval));
            
            loop {
                interval.tick().await;
                
                // 收集监控数据
                if let Err(e) = Self::collect_monitoring_data(&data_store).await {
                    log::error!("Failed to collect monitoring data: {}", e);
                }
                
                // 检查告警阈值
                if let Err(e) = Self::check_alert_thresholds(&data_store, &alerts, &config).await {
                    log::error!("Failed to check alert thresholds: {}", e);
                }
            }
        });
        
        self.monitoring_task = Some(task);
        Ok(())
    }
    
    /// 收集监控数据
    async fn collect_monitoring_data(data_store: &Arc<RwLock<Vec<MonitoringDataPoint>>>) -> Result<(), String> {
        // 收集系统数据
        let system_data = Self::collect_system_data().await?;
        let component_data = Self::collect_component_data().await?;
        let event_data = Self::collect_event_data().await?;
        
        // 存储数据
        let mut store = data_store.write().await;
        store.extend(system_data);
        store.extend(component_data);
        store.extend(event_data);
        
        // 限制数据存储大小
        let current_len = store.len();
        if current_len > 10000 {
            store.drain(0..current_len - 10000);
        }
        
        Ok(())
    }
    
    /// 收集系统数据
    async fn collect_system_data() -> Result<Vec<MonitoringDataPoint>, String> {
        let mut data_points = Vec::new();
        
        // 收集 CPU 使用率
        let cpu_usage = Self::get_cpu_usage().await?;
        data_points.push(MonitoringDataPoint {
            id: format!("sys_cpu_{}", chrono::Utc::now().timestamp()),
            monitor_type: "system_cpu_usage".to_string(),
            value: cpu_usage,
            timestamp: chrono::Utc::now().timestamp() as u64,
            tags: std::collections::HashMap::from([("source".to_string(), "system".to_string())]),
        });
        
        // 收集内存使用率
        let memory_usage = Self::get_memory_usage().await?;
        data_points.push(MonitoringDataPoint {
            id: format!("sys_mem_{}", chrono::Utc::now().timestamp()),
            monitor_type: "system_memory_usage".to_string(),
            value: memory_usage,
            timestamp: chrono::Utc::now().timestamp() as u64,
            tags: std::collections::HashMap::from([("source".to_string(), "system".to_string())]),
        });
        
        // 收集磁盘使用率
        let disk_usage = Self::get_disk_usage().await?;
        data_points.push(MonitoringDataPoint {
            id: format!("sys_disk_{}", chrono::Utc::now().timestamp()),
            monitor_type: "system_disk_usage".to_string(),
            value: disk_usage,
            timestamp: chrono::Utc::now().timestamp() as u64,
            tags: std::collections::HashMap::from([("source".to_string(), "system".to_string())]),
        });
        
        Ok(data_points)
    }
    
    /// 收集组件数据
    async fn collect_component_data() -> Result<Vec<MonitoringDataPoint>, String> {
        let mut data_points = Vec::new();
        
        // 这里应该从 GUF 组件管理器获取组件状态
        // 暂时生成模拟数据
        for i in 0..5 {
            let response_time = rand::random::<f64>() * 1000.0;
            let error_rate = rand::random::<f64>() * 10.0;
            
            data_points.push(MonitoringDataPoint {
                id: format!("comp_response_{}_{}", i, chrono::Utc::now().timestamp()),
                monitor_type: "component_response_time".to_string(),
                value: response_time,
                timestamp: chrono::Utc::now().timestamp() as u64,
                tags: std::collections::HashMap::from([
                    ("source".to_string(), "component".to_string()),
                    ("component_id".to_string(), format!("component_{}", i)),
                ]),
            });
            
            data_points.push(MonitoringDataPoint {
                id: format!("comp_error_{}_{}", i, chrono::Utc::now().timestamp()),
                monitor_type: "component_error_rate".to_string(),
                value: error_rate,
                timestamp: chrono::Utc::now().timestamp() as u64,
                tags: std::collections::HashMap::from([
                    ("source".to_string(), "component".to_string()),
                    ("component_id".to_string(), format!("component_{}", i)),
                ]),
            });
        }
        
        Ok(data_points)
    }
    
    /// 收集事件数据
    async fn collect_event_data() -> Result<Vec<MonitoringDataPoint>, String> {
        let mut data_points = Vec::new();
        
        // 这里应该从 GUF 事件总线获取事件数据
        // 暂时生成模拟数据
        for i in 0..3 {
            let processing_time = rand::random::<f64>() * 200.0;
            
            data_points.push(MonitoringDataPoint {
                id: format!("event_processing_{}_{}", i, chrono::Utc::now().timestamp()),
                monitor_type: "event_processing_time".to_string(),
                value: processing_time,
                timestamp: chrono::Utc::now().timestamp() as u64,
                tags: std::collections::HashMap::from([
                    ("source".to_string(), "event".to_string()),
                    ("event_type".to_string(), format!("event_type_{}", i)),
                ]),
            });
        }
        
        Ok(data_points)
    }
    
    /// 获取 CPU 使用率
    async fn get_cpu_usage() -> Result<f64, String> {
        // 这里应该实现真实的 CPU 使用率检测
        // 暂时返回模拟数据
        Ok(rand::random::<f64>() * 100.0)
    }
    
    /// 获取内存使用率
    async fn get_memory_usage() -> Result<f64, String> {
        // 这里应该实现真实的内存使用率检测
        // 暂时返回模拟数据
        Ok(rand::random::<f64>() * 100.0)
    }
    
    /// 获取磁盘使用率
    async fn get_disk_usage() -> Result<f64, String> {
        // 这里应该实现真实的磁盘使用率检测
        // 暂时返回模拟数据
        Ok(rand::random::<f64>() * 100.0)
    }
    
    /// 检查告警阈值
    async fn check_alert_thresholds(
        data_store: &Arc<RwLock<Vec<MonitoringDataPoint>>>,
        alerts: &Arc<RwLock<Vec<Alert>>>,
        config: &MonitoringConfig,
    ) -> Result<(), String> {
        let data = data_store.read().await;
        
        // 检查系统 CPU 使用率
        if let Some(cpu_data) = data.last() {
            if cpu_data.monitor_type == "system_cpu_usage" && cpu_data.value > config.alert_thresholds.cpu_usage_threshold {
                Self::create_alert(
                    alerts,
                    "High CPU Usage",
                    format!("CPU usage is {:.2}%, exceeding threshold of {:.2}%", cpu_data.value, config.alert_thresholds.cpu_usage_threshold),
                    AlertLevel::Warning,
                    &[cpu_data.id.clone()],
                    &[("monitor_type".to_string(), "system_cpu_usage".to_string())],
                ).await?;
            }
        }
        
        // 检查系统内存使用率
        if let Some(mem_data) = data.iter().find(|p| p.monitor_type == "system_memory_usage") {
            if mem_data.value > config.alert_thresholds.memory_usage_threshold {
                Self::create_alert(
                    alerts,
                    "High Memory Usage",
                    format!("Memory usage is {:.2}%, exceeding threshold of {:.2}%", mem_data.value, config.alert_thresholds.memory_usage_threshold),
                    AlertLevel::Warning,
                    &[mem_data.id.clone()],
                    &[("monitor_type".to_string(), "system_memory_usage".to_string())],
                ).await?;
            }
        }
        
        // 检查系统磁盘使用率
        if let Some(disk_data) = data.iter().find(|p| p.monitor_type == "system_disk_usage") {
            if disk_data.value > config.alert_thresholds.disk_usage_threshold {
                Self::create_alert(
                    alerts,
                    "High Disk Usage",
                    format!("Disk usage is {:.2}%, exceeding threshold of {:.2}%", disk_data.value, config.alert_thresholds.disk_usage_threshold),
                    AlertLevel::Error,
                    &[disk_data.id.clone()],
                    &[("monitor_type".to_string(), "system_disk_usage".to_string())],
                ).await?;
            }
        }
        
        // 检查组件响应时间
        for data_point in data.iter().filter(|p| p.monitor_type == "component_response_time") {
            if data_point.value > config.alert_thresholds.component_response_time_threshold {
                let component_id = data_point.tags.get("component_id").unwrap_or(&"unknown".to_string());
                Self::create_alert(
                    alerts,
                    format!("High Component Response Time - {}", component_id),
                    format!("Component {} response time is {:.2}ms, exceeding threshold of {:.2}ms", component_id, data_point.value, config.alert_thresholds.component_response_time_threshold),
                    AlertLevel::Warning,
                    &[data_point.id.clone()],
                    &[
                        ("monitor_type".to_string(), "component_response_time".to_string()),
                        ("component_id".to_string(), component_id.clone()),
                    ],
                ).await?;
            }
        }
        
        // 检查组件错误率
        for data_point in data.iter().filter(|p| p.monitor_type == "component_error_rate") {
            if data_point.value > config.alert_thresholds.component_error_rate_threshold {
                let component_id = data_point.tags.get("component_id").unwrap_or(&"unknown".to_string());
                Self::create_alert(
                    alerts,
                    format!("High Component Error Rate - {}", component_id),
                    format!("Component {} error rate is {:.2}%, exceeding threshold of {:.2}%", component_id, data_point.value, config.alert_thresholds.component_error_rate_threshold),
                    AlertLevel::Error,
                    &[data_point.id.clone()],
                    &[
                        ("monitor_type".to_string(), "component_error_rate".to_string()),
                        ("component_id".to_string(), component_id.clone()),
                    ],
                ).await?;
            }
        }
        
        // 检查事件处理时间
        for data_point in data.iter().filter(|p| p.monitor_type == "event_processing_time") {
            if data_point.value > config.alert_thresholds.event_processing_time_threshold {
                let event_type = data_point.tags.get("event_type").unwrap_or(&"unknown".to_string());
                Self::create_alert(
                    alerts,
                    format!("High Event Processing Time - {}", event_type),
                    format!("Event {} processing time is {:.2}ms, exceeding threshold of {:.2}ms", event_type, data_point.value, config.alert_thresholds.event_processing_time_threshold),
                    AlertLevel::Info,
                    &[data_point.id.clone()],
                    &[
                        ("monitor_type".to_string(), "event_processing_time".to_string()),
                        ("event_type".to_string(), event_type.clone()),
                    ],
                ).await?;
            }
        }
        
        Ok(())
    }
    
    /// 创建告警
    async fn create_alert(
        alerts: &Arc<RwLock<Vec<Alert>>>,
        name: &str,
        description: String,
        level: AlertLevel,
        related_data_points: &[String],
        tags: &[(String, String)],
    ) -> Result<(), String> {
        let alert_id = format!("alert_{}_{}", name.to_lowercase().replace(" ", "_"), chrono::Utc::now().timestamp());
        
        let mut tags_map = std::collections::HashMap::new();
        for (key, value) in tags {
            tags_map.insert(key.clone(), value.clone());
        }
        
        let alert = Alert {
            alert_id,
            name: name.to_string(),
            description,
            level,
            status: AlertStatus::Triggered,
            triggered_at: chrono::Utc::now().timestamp() as u64,
            resolved_at: None,
            acknowledged_at: None,
            acknowledged_by: None,
            related_data_points: related_data_points.to_vec(),
            tags: tags_map,
        };
        
        let mut alerts_lock = alerts.write().await;
        alerts_lock.push(alert);
        
        // 发送告警通知
        Self::send_notification(&alert).await?;
        
        Ok(())
    }
    
    /// 发送告警通知
    async fn send_notification(alert: &Alert) -> Result<(), String> {
        // 这里应该实现真实的通知发送逻辑
        // 暂时只记录日志
        log::info!("Sending notification for alert: {}", alert.name);
        log::info!("Alert level: {:?}, description: {}", alert.level, alert.description);
        
        Ok(())
    }
    
    /// 保存数据到磁盘
    async fn save_data(&self) -> Result<(), String> {
        // 保存监控数据
        let data = self.data_store.read().await;
        let data_path = "./monitoring_data/monitoring_data.json";
        let data_json = serde_json::to_string_pretty(&*data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        
        std::fs::write(data_path, data_json)
            .map_err(|e| format!("Failed to write data: {}", e))?;
        
        // 保存告警数据
        let alerts_data = self.alerts.read().await;
        let alerts_path = "./monitoring_data/alerts.json";
        let alerts_json = serde_json::to_string_pretty(&*alerts_data)
            .map_err(|e| format!("Failed to serialize alerts: {}", e))?;
        
        std::fs::write(alerts_path, alerts_json)
            .map_err(|e| format!("Failed to write alerts: {}", e))?;
        
        Ok(())
    }
    
    /// 获取告警列表
    pub async fn get_alerts(&self) -> Result<Vec<Alert>, String> {
        let alerts = self.alerts.read().await;
        Ok(alerts.clone())
    }
    
    /// 获取监控数据
    pub async fn get_monitoring_data(&self, monitor_type: Option<&str>) -> Result<Vec<MonitoringDataPoint>, String> {
        let data = self.data_store.read().await;
        
        if let Some(m_type) = monitor_type {
            Ok(data.iter().filter(|p| p.monitor_type == m_type).cloned().collect())
        } else {
            Ok(data.clone())
        }
    }
    
    /// 解决告警
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<(), String> {
        let mut alerts = self.alerts.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.alert_id == alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(chrono::Utc::now().timestamp() as u64);
            Ok(())
        } else {
            Err(format!("Alert not found: {}", alert_id))
        }
    }
    
    /// 确认告警
    pub async fn acknowledge_alert(&self, alert_id: &str, user: &str) -> Result<(), String> {
        let mut alerts = self.alerts.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.alert_id == alert_id) {
            alert.status = AlertStatus::Acknowledged;
            alert.acknowledged_at = Some(chrono::Utc::now().timestamp() as u64);
            alert.acknowledged_by = Some(user.to_string());
            Ok(())
        } else {
            Err(format!("Alert not found: {}", alert_id))
        }
    }
}

/// 创建监控插件实例
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn PluginLifecycle {
    let plugin = Box::new(MonitoringPlugin::new());
    Box::into_raw(plugin)
}
