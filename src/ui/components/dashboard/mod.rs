//! 仪表板组件模块
//! 用于显示系统性能指标、监控数据和系统状态

pub mod alert_panel;
pub mod metrics_display;
pub mod performance_dashboard;
pub mod system_status;

/// 仪表板管理器
#[derive(Debug, Clone)]
pub struct DashboardManager {
    performance_dashboard: performance_dashboard::PerformanceDashboard,
    system_status: system_status::SystemStatusPanel,
    metrics_display: metrics_display::MetricsDisplay,
    alert_panel: alert_panel::AlertPanel,
}

impl DashboardManager {
    /// 创建新的仪表板管理器
    pub fn new() -> Self {
        Self {
            performance_dashboard: performance_dashboard::PerformanceDashboard::new(),
            system_status: system_status::SystemStatusPanel::new(),
            metrics_display: metrics_display::MetricsDisplay::new(),
            alert_panel: alert_panel::AlertPanel::new(),
        }
    }

    /// 初始化仪表板
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.performance_dashboard.initialize().await?;
        self.system_status.initialize().await?;
        self.metrics_display.initialize().await?;
        self.alert_panel.initialize().await?;
        Ok(())
    }

    /// 获取性能仪表板
    pub fn get_performance_dashboard(&self) -> &performance_dashboard::PerformanceDashboard {
        &self.performance_dashboard
    }

    /// 获取系统状态面板
    pub fn get_system_status(&self) -> &system_status::SystemStatusPanel {
        &self.system_status
    }

    /// 获取指标显示组件
    pub fn get_metrics_display(&self) -> &metrics_display::MetricsDisplay {
        &self.metrics_display
    }

    /// 获取告警面板
    pub fn get_alert_panel(&self) -> &alert_panel::AlertPanel {
        &self.alert_panel
    }

    /// 更新所有仪表板数据
    pub async fn update_dashboards(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.performance_dashboard.update_data().await?;
        self.system_status.update_status().await?;
        self.metrics_display.update_metrics().await?;
        self.alert_panel.update_alerts().await?;
        Ok(())
    }
}
