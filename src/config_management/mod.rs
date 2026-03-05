//! 配置管理自动化模块
//! 用于配置自动生成和验证、配置版本管理和配置变更审计

pub mod change_audit;
pub mod config_generation;
pub mod config_validation;
pub mod version_management;

/// 配置管理自动化管理器
#[derive(Debug, Clone)]
pub struct ConfigManagementManager {
    config_generation: config_generation::ConfigGenerator,
    config_validation: config_validation::ConfigValidator,
    version_management: version_management::VersionManager,
    change_audit: change_audit::ChangeAuditor,
}

impl ConfigManagementManager {
    /// 创建新的配置管理自动化管理器
    pub fn new() -> Self {
        Self {
            config_generation: config_generation::ConfigGenerator::new(),
            config_validation: config_validation::ConfigValidator::new(),
            version_management: version_management::VersionManager::new(),
            change_audit: change_audit::ChangeAuditor::new(),
        }
    }

    /// 初始化配置管理自动化
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.config_generation.initialize().await?;
        self.config_validation.initialize().await?;
        self.version_management.initialize().await?;
        self.change_audit.initialize().await?;
        Ok(())
    }

    /// 生成配置
    pub async fn generate_config(
        &self,
        config: config_generation::GenerationConfig,
    ) -> Result<config_generation::GenerationResult, Box<dyn std::error::Error>> {
        self.config_generation.generate_config(config).await
    }

    /// 验证配置
    pub async fn validate_config(
        &self,
        config: config_validation::ValidationConfig,
    ) -> Result<config_validation::ValidationResult, Box<dyn std::error::Error>> {
        self.config_validation.validate_config(config).await
    }

    /// 管理配置版本
    pub async fn manage_version(
        &self,
        config: version_management::VersionConfig,
    ) -> Result<version_management::VersionResult, Box<dyn std::error::Error>> {
        self.version_management.manage_version(config).await
    }

    /// 审计配置变更
    pub async fn audit_change(
        &self,
        config: change_audit::AuditConfig,
    ) -> Result<change_audit::AuditResult, Box<dyn std::error::Error>> {
        self.change_audit.audit_change(config).await
    }
}
