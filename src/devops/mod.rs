//! DevOps工具链模块
//! 用于CI/CD流水线、自动化测试和部署、监控和告警集成

pub mod ci_cd;
pub mod monitoring;
pub mod deployment;
pub mod automation;

/// DevOps工具链管理器
#[derive(Debug, Clone)]
pub struct DevOpsManager {
    ci_cd: ci_cd::CiCdPipeline,
    monitoring: monitoring::MonitoringSystem,
    deployment: deployment::DeploymentManager,
    automation: automation::AutomationEngine,
}

impl DevOpsManager {
    /// 创建新的DevOps工具链管理器
    pub fn new() -> Self {
        Self {
            ci_cd: ci_cd::CiCdPipeline::new(),
            monitoring: monitoring::MonitoringSystem::new(),
            deployment: deployment::DeploymentManager::new(),
            automation: automation::AutomationEngine::new(),
        }
    }

    /// 初始化DevOps工具链
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.ci_cd.initialize().await?;
        self.monitoring.initialize().await?;
        self.deployment.initialize().await?;
        self.automation.initialize().await?;
        Ok(())
    }

    /// 运行CI/CD流水线
    pub async fn run_ci_cd_pipeline(&self, branch: &str, commit: &str) -> Result<ci_cd::PipelineResult, Box<dyn std::error::Error>> {
        self.ci_cd.run_pipeline(branch, commit).await
    }

    /// 部署应用
    pub async fn deploy(&self, environment: &str, version: &str) -> Result<deployment::DeploymentResult, Box<dyn std::error::Error>> {
        self.deployment.deploy(environment, version).await
    }

    /// 监控系统状态
    pub async fn monitor_system(&self) -> Result<monitoring::SystemStatus, Box<dyn std::error::Error>> {
        self.monitoring.check_status().await
    }

    /// 自动化任务
    pub async fn run_automation(&self, task: &str, params: serde_json::Value) -> Result<automation::AutomationResult, Box<dyn std::error::Error>> {
        self.automation.run_task(task, params).await
    }
}
