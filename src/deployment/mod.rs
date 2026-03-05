//! 部署自动化模块
//! 用于一键部署脚本、部署状态监控和部署回滚机制

pub mod cloud_platform;
pub mod deployment_script;
pub mod rollback_mechanism;
pub mod status_monitoring;

/// 部署自动化管理器
#[derive(Debug, Clone)]
pub struct DeploymentManager {
    deployment_script: deployment_script::DeploymentScriptExecutor,
    status_monitoring: status_monitoring::DeploymentStatusMonitor,
    rollback_mechanism: rollback_mechanism::RollbackManager,
    cloud_platform: cloud_platform::CloudDeploymentManager,
}

impl DeploymentManager {
    /// 创建新的部署自动化管理器
    pub fn new() -> Self {
        Self {
            deployment_script: deployment_script::DeploymentScriptExecutor::new(),
            status_monitoring: status_monitoring::DeploymentStatusMonitor::new(),
            rollback_mechanism: rollback_mechanism::RollbackManager::new(),
            cloud_platform: cloud_platform::CloudDeploymentManager::new(),
        }
    }

    /// 初始化部署自动化
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.deployment_script.initialize().await?;
        self.status_monitoring.initialize().await?;
        self.rollback_mechanism.initialize().await?;
        self.cloud_platform.initialize().await?;
        Ok(())
    }

    /// 执行部署脚本
    pub async fn execute_deployment(
        &self,
        config: deployment_script::DeploymentConfig,
    ) -> Result<deployment_script::DeploymentResult, Box<dyn std::error::Error>> {
        self.deployment_script.execute_deployment(config).await
    }

    /// 监控部署状态
    pub async fn monitor_status(
        &self,
        config: status_monitoring::MonitoringConfig,
    ) -> Result<status_monitoring::MonitoringResult, Box<dyn std::error::Error>> {
        self.status_monitoring.monitor_status(config).await
    }

    /// 执行部署回滚
    pub async fn execute_rollback(
        &self,
        config: rollback_mechanism::RollbackConfig,
    ) -> Result<rollback_mechanism::RollbackResult, Box<dyn std::error::Error>> {
        self.rollback_mechanism.execute_rollback(config).await
    }

    /// 部署到云平台
    pub async fn deploy_to_cloud(
        &self,
        config: cloud_platform::CloudDeploymentConfig,
    ) -> Result<cloud_platform::CloudDeploymentResult, Box<dyn std::error::Error>> {
        self.cloud_platform.deploy_to_cloud(config).await
    }

    /// 获取云平台部署结果
    pub async fn get_cloud_deployment_results(
        &self,
    ) -> Result<Vec<cloud_platform::CloudDeploymentResult>, Box<dyn std::error::Error>> {
        self.cloud_platform.get_cloud_deployment_results().await
    }

    /// 获取云平台部署状态
    pub async fn get_cloud_deployment_status(
        &self,
        deployment_id: &str,
    ) -> Result<Option<cloud_platform::CloudDeploymentResult>, Box<dyn std::error::Error>> {
        self.cloud_platform.get_cloud_deployment_status(deployment_id).await
    }

    /// 取消云平台部署
    pub async fn cancel_cloud_deployment(
        &self,
        deployment_id: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.cloud_platform.cancel_cloud_deployment(deployment_id).await
    }
}
