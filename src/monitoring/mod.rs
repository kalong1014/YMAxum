//! 监控和告警自动化模块
//! 用于监控指标自动采集、智能告警系统和告警处理自动化流程

pub mod alert_processing;
pub mod alert_system;
pub mod metric_collection;
pub mod prometheus;

/// 监控和告警自动化管理器
#[derive(Debug, Clone)]
pub struct MonitoringAutomationManager {
    metric_collection: metric_collection::MetricCollector,
    alert_system: alert_system::AlertSystem,
    alert_processing: alert_processing::AlertProcessor,
    prometheus_metrics: Option<prometheus::PrometheusMetrics>,
}

impl MonitoringAutomationManager {
    /// 创建新的监控和告警自动化管理器
    pub fn new() -> Self {
        Self {
            metric_collection: metric_collection::MetricCollector::new(),
            alert_system: alert_system::AlertSystem::new(),
            alert_processing: alert_processing::AlertProcessor::new(),
            prometheus_metrics: None,
        }
    }

    /// 初始化监控和告警自动化
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.metric_collection.initialize().await?;
        self.alert_system.initialize().await?;
        self.alert_processing.initialize().await?;

        // 初始化Prometheus指标
        self.prometheus_metrics = Some(prometheus::PrometheusMetrics::new());

        Ok(())
    }

    /// 采集监控指标
    pub async fn collect_metrics(
        &self,
        config: metric_collection::CollectionConfig,
    ) -> Result<metric_collection::CollectionResult, Box<dyn std::error::Error>> {
        self.metric_collection.collect_metrics(config).await
    }

    /// 处理告警
    pub async fn process_alerts(
        &self,
        config: alert_processing::ProcessingConfig,
    ) -> Result<alert_processing::ProcessingResult, Box<dyn std::error::Error>> {
        self.alert_processing.process_alerts(config).await
    }

    /// 触发告警
    pub async fn trigger_alert(
        &self,
        config: alert_system::AlertConfig,
    ) -> Result<alert_system::AlertResult, Box<dyn std::error::Error>> {
        self.alert_system.trigger_alert(config).await
    }

    /// 获取Prometheus指标
    pub fn prometheus_metrics(&self) -> Option<&prometheus::PrometheusMetrics> {
        self.prometheus_metrics.as_ref()
    }

    /// 获取Prometheus指标的可变引用
    pub fn prometheus_metrics_mut(&mut self) -> Option<&mut prometheus::PrometheusMetrics> {
        self.prometheus_metrics.as_mut()
    }
}
