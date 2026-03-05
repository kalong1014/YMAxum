//! 指标显示组件
//! 用于显示详细的监控指标数据

use serde::{Deserialize, Serialize};

/// 指标显示配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDisplayConfig {
    /// 是否启用实时更新
    pub realtime_updates: bool,
    /// 更新间隔(秒)
    pub update_interval: u32,
    /// 显示的指标类型
    pub metrics_types: Vec<String>,
    /// 是否显示详细信息
    pub show_details: bool,
    /// 是否显示历史数据
    pub show_history: bool,
    /// 历史数据时间范围(分钟)
    pub history_time_range: u32,
    /// 是否显示趋势图表
    pub show_trend_charts: bool,
    /// 是否显示统计数据
    pub show_statistics: bool,
    /// 是否按组件分组显示
    pub group_by_component: bool,
}

/// 指标数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// 时间戳
    pub timestamp: String,
    /// 指标值
    pub value: f64,
    /// 指标单位
    pub unit: String,
}

/// 指标详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDetails {
    /// 指标名称
    pub name: String,
    /// 指标描述
    pub description: String,
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
    /// 95分位数
    pub p95_value: f64,
    /// 99分位数
    pub p99_value: f64,
    /// 趋势数据
    pub trend_data: Vec<MetricDataPoint>,
    /// 最近变化
    pub recent_change: f64,
    /// 变化百分比
    pub change_percentage: f64,
    /// 状态
    pub status: String,
    /// 组件名称
    pub component: String,
    /// 采集时间
    pub collection_time: String,
}

/// 指标组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricGroup {
    /// 组名称
    pub group_name: String,
    /// 指标列表
    pub metrics: Vec<MetricDetails>,
    /// 组状态
    pub status: String,
    /// 组描述
    pub description: String,
}

/// 指标显示组件
#[derive(Debug, Clone)]
pub struct MetricsDisplay {
    /// 配置
    config: MetricsDisplayConfig,
    /// 指标数据
    metrics: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, MetricDetails>>>,
    /// 指标组
    metric_groups:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, MetricGroup>>>,
}

impl MetricsDisplay {
    /// 创建新的指标显示组件
    pub fn new() -> Self {
        let config = MetricsDisplayConfig {
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
                "error_rate".to_string(),
                "throughput".to_string(),
                "latency".to_string(),
            ],
            show_details: true,
            show_history: true,
            history_time_range: 30,
            show_trend_charts: true,
            show_statistics: true,
            group_by_component: true,
        };

        Self {
            config,
            metrics: std::sync::Arc::new(
                tokio::sync::RwLock::new(std::collections::HashMap::new()),
            ),
            metric_groups: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// 初始化指标显示组件
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化指标显示组件
        println!("Initializing metrics display...");

        // 初始化指标数据
        let mut metrics = self.metrics.write().await;
        for metric_type in &self.config.metrics_types {
            metrics.insert(metric_type.clone(), self.create_initial_metric(metric_type));
        }

        // 初始化指标组
        self.initialize_metric_groups().await?;

        Ok(())
    }

    /// 初始化指标组
    async fn initialize_metric_groups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut metric_groups = self.metric_groups.write().await;

        // 创建系统资源组
        metric_groups.insert(
            "system_resources".to_string(),
            MetricGroup {
                group_name: "系统资源".to_string(),
                metrics: vec![],
                status: "normal".to_string(),
                description: "系统资源使用情况".to_string(),
            },
        );

        // 创建网络组
        metric_groups.insert(
            "network".to_string(),
            MetricGroup {
                group_name: "网络".to_string(),
                metrics: vec![],
                status: "normal".to_string(),
                description: "网络性能指标".to_string(),
            },
        );

        // 创建API性能组
        metric_groups.insert(
            "api_performance".to_string(),
            MetricGroup {
                group_name: "API性能".to_string(),
                metrics: vec![],
                status: "normal".to_string(),
                description: "API性能指标".to_string(),
            },
        );

        Ok(())
    }

    /// 创建初始指标数据
    fn create_initial_metric(&self, metric_type: &str) -> MetricDetails {
        let (description, unit, component) = match metric_type {
            "cpu_usage" => ("CPU使用率", "%", "系统"),
            "memory_usage" => ("内存使用率", "%", "系统"),
            "disk_usage" => ("磁盘使用率", "%", "系统"),
            "network_in" => ("网络入流量", "MB/s", "网络"),
            "network_out" => ("网络出流量", "MB/s", "网络"),
            "response_time" => ("响应时间", "ms", "API"),
            "request_count" => ("请求数量", "req/min", "API"),
            "error_rate" => ("错误率", "%", "API"),
            "throughput" => ("吞吐量", "req/s", "API"),
            "latency" => ("延迟", "ms", "API"),
            _ => ("未知指标", "", "系统"),
        };

        MetricDetails {
            name: metric_type.to_string(),
            description: description.to_string(),
            current_value: 0.0,
            unit: unit.to_string(),
            min_value: 0.0,
            max_value: 0.0,
            avg_value: 0.0,
            p95_value: 0.0,
            p99_value: 0.0,
            trend_data: Vec::new(),
            recent_change: 0.0,
            change_percentage: 0.0,
            status: "normal".to_string(),
            component: component.to_string(),
            collection_time: chrono::Utc::now().to_string(),
        }
    }

    /// 更新指标数据
    pub async fn update_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟指标数据更新
        println!("Updating metrics...");

        let mut metrics = self.metrics.write().await;
        for metric_type in &self.config.metrics_types {
            if let Some(metric) = metrics.get_mut(metric_type) {
                self.update_metric(metric).await;
            }
        }

        // 更新指标组
        self.update_metric_groups().await?;

        Ok(())
    }

    /// 更新单个指标
    async fn update_metric(&self, metric: &mut MetricDetails) {
        let old_value = metric.current_value;

        // 生成随机值模拟指标数据
        let new_value = self.generate_metric_value(&metric.name);

        // 更新当前值
        metric.current_value = new_value;

        // 更新趋势数据
        metric.trend_data.push(MetricDataPoint {
            timestamp: chrono::Utc::now().to_string(),
            value: new_value,
            unit: metric.unit.clone(),
        });

        // 限制趋势数据点数量
        if metric.trend_data.len() > 100 {
            metric.trend_data.remove(0);
        }

        // 更新统计数据
        if !metric.trend_data.is_empty() {
            let values: Vec<f64> = metric.trend_data.iter().map(|d| d.value).collect();
            metric.min_value = *values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metric.max_value = *values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metric.avg_value = values.iter().sum::<f64>() / values.len() as f64;

            // 计算分位数
            let mut sorted_values = values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p95_index = (sorted_values.len() as f64 * 0.95) as usize;
            let p99_index = (sorted_values.len() as f64 * 0.99) as usize;
            metric.p95_value = *sorted_values.get(p95_index).unwrap_or(&0.0);
            metric.p99_value = *sorted_values.get(p99_index).unwrap_or(&0.0);
        }

        // 更新变化数据
        metric.recent_change = new_value - old_value;
        if old_value > 0.0 {
            metric.change_percentage = (metric.recent_change / old_value) * 100.0;
        }

        // 更新状态
        metric.status = self.calculate_metric_status(&metric.name, new_value);

        // 更新采集时间
        metric.collection_time = chrono::Utc::now().to_string();
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
            "error_rate" => 0.1 + (rand::random::<f64>() * 1.0),
            "throughput" => 50.0 + (rand::random::<f64>() * 100.0),
            "latency" => 50.0 + (rand::random::<f64>() * 100.0),
            _ => rand::random::<f64>() * 100.0,
        }
    }

    /// 计算指标状态
    fn calculate_metric_status(&self, metric_type: &str, value: f64) -> String {
        match metric_type {
            "cpu_usage" if value > 80.0 => "alert".to_string(),
            "cpu_usage" if value > 60.0 => "warning".to_string(),
            "memory_usage" if value > 85.0 => "alert".to_string(),
            "memory_usage" if value > 70.0 => "warning".to_string(),
            "disk_usage" if value > 90.0 => "alert".to_string(),
            "disk_usage" if value > 75.0 => "warning".to_string(),
            "response_time" if value > 500.0 => "alert".to_string(),
            "response_time" if value > 300.0 => "warning".to_string(),
            "error_rate" if value > 5.0 => "alert".to_string(),
            "error_rate" if value > 2.0 => "warning".to_string(),
            _ => "normal".to_string(),
        }
    }

    /// 更新指标组
    async fn update_metric_groups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let metrics = self.metrics.read().await;
        let mut metric_groups = self.metric_groups.write().await;

        // 清空现有指标组数据
        for group in metric_groups.values_mut() {
            group.metrics.clear();
        }

        // 重新分配指标到组
        for metric in metrics.values() {
            let group_key = match metric.component.as_str() {
                "系统" => "system_resources",
                "网络" => "network",
                "API" => "api_performance",
                _ => "system_resources",
            };

            if let Some(group) = metric_groups.get_mut(group_key) {
                group.metrics.push(metric.clone());

                // 更新组状态
                if metric.status == "alert" {
                    group.status = "alert".to_string();
                } else if metric.status == "warning" && group.status == "normal" {
                    group.status = "warning".to_string();
                }
            }
        }

        Ok(())
    }

    /// 获取指标数据
    pub async fn get_metrics(
        &self,
    ) -> Result<std::collections::HashMap<String, MetricDetails>, Box<dyn std::error::Error>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// 获取单个指标数据
    pub async fn get_metric(
        &self,
        metric_name: &str,
    ) -> Result<Option<MetricDetails>, Box<dyn std::error::Error>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.get(metric_name).cloned())
    }

    /// 获取指标组
    pub async fn get_metric_groups(
        &self,
    ) -> Result<std::collections::HashMap<String, MetricGroup>, Box<dyn std::error::Error>> {
        let metric_groups = self.metric_groups.read().await;
        Ok(metric_groups.clone())
    }

    /// 获取单个指标组
    pub async fn get_metric_group(
        &self,
        group_name: &str,
    ) -> Result<Option<MetricGroup>, Box<dyn std::error::Error>> {
        let metric_groups = self.metric_groups.read().await;
        Ok(metric_groups.get(group_name).cloned())
    }

    /// 获取指标显示配置
    pub fn get_config(&self) -> &MetricsDisplayConfig {
        &self.config
    }

    /// 更新指标显示配置
    pub fn update_config(&mut self, config: MetricsDisplayConfig) {
        self.config = config;
    }
}
