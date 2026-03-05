//! 性能仪表板组件
//! 用于显示系统性能指标和监控数据

use axum::extract::ws::WebSocketUpgrade;
use chrono;
use serde::{Deserialize, Serialize};
use uuid;

/// 性能仪表板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// 是否启用实时更新
    pub realtime_updates: bool,
    /// 更新间隔(秒)
    pub update_interval: u32,
    /// 显示的指标类型
    pub metrics_types: Vec<String>,
    /// 是否显示历史数据
    pub show_history: bool,
    /// 历史数据时间范围(分钟)
    pub history_time_range: u32,
    /// 是否启用告警
    pub enable_alerts: bool,
    /// 告警阈值配置
    pub alert_thresholds: std::collections::HashMap<String, f64>,
}

/// 性能数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    /// 时间戳
    pub timestamp: String,
    /// 指标值
    pub value: f64,
    /// 指标单位
    pub unit: String,
}

/// 性能指标数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    /// 指标名称
    pub name: String,
    /// 当前值
    pub current_value: f64,
    /// 单位
    pub unit: String,
    /// 最小值
    pub min_value: f64,
    /// 最大值
    pub max_value: f64,
    /// 平均值
    pub avg_value: f64,
    /// 趋势数据
    pub trend_data: Vec<PerformanceDataPoint>,
    /// 状态
    pub status: String,
    /// 告警阈值
    pub alert_threshold: Option<f64>,
}

/// 性能仪表板
#[derive(Debug, Clone)]
pub struct PerformanceDashboard {
    /// 配置
    config: DashboardConfig,
    /// 性能指标数据
    metrics:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, PerformanceMetric>>>,
    /// 后台更新任务句柄
    update_task: std::sync::Arc<tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// WebSocket 服务器
    websocket_server: std::sync::Arc<crate::realtime::websocket::WebSocketServer>,
}

impl PerformanceDashboard {
    /// 创建新的性能仪表板
    pub fn new() -> Self {
        let config = DashboardConfig {
            realtime_updates: true,
            update_interval: 5,
            metrics_types: vec![
                "cpu_usage".to_string(),
                "memory_usage".to_string(),
                "disk_usage".to_string(),
                "network_in".to_string(),
                "network_out".to_string(),
                "response_time".to_string(),
                "request_count".to_string(),
            ],
            show_history: true,
            history_time_range: 30,
            enable_alerts: true,
            alert_thresholds: std::collections::HashMap::from([
                ("cpu_usage".to_string(), 80.0),
                ("memory_usage".to_string(), 85.0),
                ("disk_usage".to_string(), 90.0),
                ("response_time".to_string(), 500.0),
            ]),
        };

        // 创建 WebSocket 服务器
        let websocket_server =
            std::sync::Arc::new(crate::realtime::websocket::WebSocketServer::new());

        Self {
            config,
            metrics: std::sync::Arc::new(
                tokio::sync::RwLock::new(std::collections::HashMap::new()),
            ),
            update_task: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
            websocket_server,
        }
    }

    /// 初始化性能仪表板
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化性能仪表板
        println!("Initializing performance dashboard...");

        // 初始化指标数据
        let mut metrics = self.metrics.write().await;
        for metric_type in &self.config.metrics_types {
            metrics.insert(metric_type.clone(), self.create_initial_metric(metric_type));
        }

        Ok(())
    }

    /// 创建初始指标数据
    fn create_initial_metric(&self, metric_type: &str) -> PerformanceMetric {
        let unit = match metric_type {
            "cpu_usage" | "memory_usage" | "disk_usage" => "%",
            "network_in" | "network_out" => "MB/s",
            "response_time" => "ms",
            "request_count" => "req/min",
            _ => "",
        };

        PerformanceMetric {
            name: metric_type.to_string(),
            current_value: 0.0,
            unit: unit.to_string(),
            min_value: 0.0,
            max_value: 0.0,
            avg_value: 0.0,
            trend_data: Vec::new(),
            status: "normal".to_string(),
            alert_threshold: self.config.alert_thresholds.get(metric_type).copied(),
        }
    }

    /// 更新性能数据
    pub async fn update_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟性能数据更新
        println!("Updating performance dashboard data...");

        let mut metrics = self.metrics.write().await;
        for metric_type in &self.config.metrics_types {
            if let Some(metric) = metrics.get_mut(metric_type) {
                self.update_metric(metric).await;
            }
        }

        Ok(())
    }

    /// 更新单个指标
    async fn update_metric(&self, metric: &mut PerformanceMetric) {
        // 生成随机值模拟性能数据
        let new_value = self.generate_metric_value(&metric.name);

        // 更新当前值
        metric.current_value = new_value;

        // 更新趋势数据
        metric.trend_data.push(PerformanceDataPoint {
            timestamp: chrono::Utc::now().to_string(),
            value: new_value,
            unit: metric.unit.clone(),
        });

        // 限制趋势数据点数量
        if metric.trend_data.len() > 100 {
            metric.trend_data.remove(0);
        }

        // 更新统计数据
        metric.min_value = metric
            .trend_data
            .iter()
            .map(|d| d.value)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        metric.max_value = metric
            .trend_data
            .iter()
            .map(|d| d.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        metric.avg_value =
            metric.trend_data.iter().map(|d| d.value).sum::<f64>() / metric.trend_data.len() as f64;

        // 更新状态
        if let Some(threshold) = metric.alert_threshold {
            if new_value > threshold {
                metric.status = "alert".to_string();
            } else if new_value > threshold * 0.8 {
                metric.status = "warning".to_string();
            } else {
                metric.status = "normal".to_string();
            }
        }
    }

    /// 生成指标值
    fn generate_metric_value(&self, metric_type: &str) -> f64 {
        match metric_type {
            "cpu_usage" => 30.0 + (rand::random::<f64>() * 20.0),
            "memory_usage" => 40.0 + (rand::random::<f64>() * 25.0),
            "disk_usage" => 20.0 + (rand::random::<f64>() * 15.0),
            "network_in" => 100.0 + (rand::random::<f64>() * 500.0),
            "network_out" => 80.0 + (rand::random::<f64>() * 400.0),
            "response_time" => 100.0 + (rand::random::<f64>() * 50.0),
            "request_count" => 100.0 + (rand::random::<f64>() * 200.0),
            _ => rand::random::<f64>() * 100.0,
        }
    }

    /// 获取性能指标数据
    pub async fn get_metrics(
        &self,
    ) -> Result<std::collections::HashMap<String, PerformanceMetric>, Box<dyn std::error::Error>>
    {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// 获取单个指标数据
    pub async fn get_metric(
        &self,
        metric_name: &str,
    ) -> Result<Option<PerformanceMetric>, Box<dyn std::error::Error>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.get(metric_name).cloned())
    }

    /// 获取仪表板配置
    pub fn get_config(&self) -> &DashboardConfig {
        &self.config
    }

    /// 更新仪表板配置
    pub fn update_config(&mut self, config: DashboardConfig) {
        self.config = config;
    }

    /// 启动后台更新任务
    pub async fn start_update_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut update_task = self.update_task.lock().await;
        if update_task.is_none() {
            let dashboard_clone = self.clone();
            let handle = tokio::spawn(async move {
                loop {
                    // 记录开始时间
                    let start_time = tokio::time::Instant::now();

                    // 更新数据
                    if let Err(e) = dashboard_clone.update_data().await {
                        eprintln!("Error updating dashboard data: {:?}", e);
                    }

                    // 广播数据更新
                    dashboard_clone.broadcast_updates().await;

                    // 计算执行时间
                    let execution_time = start_time.elapsed().as_secs_f64();
                    
                    // 动态调整等待时间，确保更新间隔稳定
                    let wait_time = (dashboard_clone.config.update_interval as f64) - execution_time;
                    if wait_time > 0.0 {
                        tokio::time::sleep(tokio::time::Duration::from_secs_f64(wait_time)).await;
                    }
                }
            });
            *update_task = Some(handle);
        }
        Ok(())
    }

    /// 停止后台更新任务
    pub async fn stop_update_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut update_task = self.update_task.lock().await;
        if let Some(handle) = update_task.take() {
            handle.abort();
        }
        Ok(())
    }

    /// 广播数据更新
    async fn broadcast_updates(&self) {
        let metrics = self.metrics.read().await;
        let metrics_data = metrics.clone();

        // 序列化指标数据
        if let Ok(json_data) = serde_json::to_string(&metrics_data) {
            // 创建 WebSocket 消息
            let message = crate::realtime::websocket::WebSocketMessage {
                message_id: uuid::Uuid::new_v4().to_string(),
                connection_id: "broadcast".to_string(),
                message_type: "performance_update".to_string(),
                content: serde_json::from_str(&json_data).unwrap_or(serde_json::Value::Null),
                timestamp: chrono::Utc::now().to_string(),
            };

            // 广播消息
            // 注意：这里简化处理，实际应该遍历所有连接并发送
            let active_connections = self
                .websocket_server
                .get_active_connections()
                .await
                .unwrap_or_default();
            for connection in active_connections {
                let mut msg = message.clone();
                msg.connection_id = connection.connection_id;
                let _ = self.websocket_server.send_message(msg).await;
            }
        }
    }

    /// 处理 WebSocket 连接
    pub async fn handle_websocket(&self, ws: WebSocketUpgrade) -> axum::response::Response {
        let websocket_server = self.websocket_server.clone();
        ws.on_upgrade(move |socket| async move {
            // 创建 WebSocket 连接信息
            let connection = crate::realtime::websocket::WebSocketConnection {
                connection_id: uuid::Uuid::new_v4().to_string(),
                client_info: crate::realtime::websocket::ClientInfo {
                    client_id: uuid::Uuid::new_v4().to_string(),
                    client_type: "dashboard".to_string(),
                    client_version: "1.0.0".to_string(),
                    client_ip: "127.0.0.1".to_string(),
                    browser_info: None,
                },
                connection_params: serde_json::Value::Null,
                connection_time: chrono::Utc::now().to_string(),
            };

            // 处理连接
            let _ = websocket_server.handle_connection(connection, socket).await;
        })
    }
}
