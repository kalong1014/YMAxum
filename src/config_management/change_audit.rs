//! 配置变更审计模块
//! 用于审计配置文件的变更和跟踪变更历史

use serde::{Deserialize, Serialize};
use std::path::Path;
use super::version_management::VersionManager;

/// 审计配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// 配置ID
    pub config_id: String,
    /// 配置文件路径
    pub config_file_path: String,
    /// 变更类型
    pub change_type: String,
    /// 变更内容
    pub change_content: serde_json::Value,
    /// 变更者
    pub changed_by: String,
    /// 审计参数
    pub parameters: serde_json::Value,
    /// 关联的版本ID
    pub version_id: Option<String>,
}

/// 审计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    /// 审计状态
    pub status: String,
    /// 结果ID
    pub result_id: String,
    /// 审计日志
    pub audit_logs: Vec<AuditLog>,
    /// 审计时间
    pub audit_time: String,
    /// 审计文件
    pub audited_file: String,
    /// 审计建议
    pub audit_suggestions: Vec<String>,
    /// 关联的版本ID
    pub version_id: Option<String>,
}

/// 审计日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// 日志ID
    pub log_id: String,
    /// 变更类型
    pub change_type: String,
    /// 变更内容
    pub change_content: serde_json::Value,
    /// 变更时间
    pub change_time: String,
    /// 变更者
    pub changed_by: String,
    /// 变更状态
    pub change_status: String,
    /// 关联的版本ID
    pub version_id: Option<String>,
}

/// 变更审计器
#[derive(Debug, Clone)]
pub struct ChangeAuditor {
    /// 审计结果列表
    audit_results: std::sync::Arc<tokio::sync::RwLock<Vec<AuditResult>>>,
    /// 审计日志列表
    audit_logs: std::sync::Arc<tokio::sync::RwLock<Vec<AuditLog>>>,
    /// 版本管理器
    version_manager: VersionManager,
    /// 审计存储路径
    audit_storage_path: String,
}

impl ChangeAuditor {
    /// 创建新的变更审计器
    pub fn new() -> Self {
        let audit_storage_path = "config/audit".to_string();
        // 确保审计存储目录存在
        std::fs::create_dir_all(&audit_storage_path).ok();
        
        Self {
            audit_results: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            audit_logs: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            version_manager: VersionManager::new(),
            audit_storage_path,
        }
    }

    /// 初始化变更审计器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化版本管理器
        self.version_manager.initialize().await?;
        
        // 加载已存储的审计日志
        self.load_audit_logs().await?;
        
        Ok(())
    }

    /// 审计配置变更
    pub async fn audit_change(
        &self,
        config: AuditConfig,
    ) -> Result<AuditResult, Box<dyn std::error::Error>> {

        // 读取配置文件的当前状态
        let current_content = match tokio::fs::read_to_string(&config.config_file_path).await {
            Ok(content) => content,
            Err(_) => String::new()
        };

        // 生成审计日志
        let audit_log = AuditLog {
            log_id: format!(
                "log_{}_{}",
                config.config_id,
                chrono::Utc::now().timestamp()
            ),
            change_type: config.change_type.clone(),
            change_content: serde_json::json!({
                "file_path": config.config_file_path,
                "changes": config.change_content,
                "current_content": current_content,
                "parameters": config.parameters
            }),
            change_time: chrono::Utc::now().to_string(),
            changed_by: config.changed_by.clone(),
            change_status: "approved".to_string(),
            version_id: config.version_id.clone(),
        };

        // 添加到审计日志列表
        let mut audit_logs = self.audit_logs.write().await;
        audit_logs.push(audit_log.clone());

        // 生成审计建议
        let mut audit_suggestions = Vec::new();
        match config.change_type.as_str() {
            "security" => {
                audit_suggestions.push(
                    "Consider implementing encryption for sensitive configuration values".to_string(),
                );
                audit_suggestions
                    .push("Review access control policies for configuration files".to_string());
            }
            "performance" => {
                audit_suggestions.push(
                    "Consider optimizing configuration values for better performance".to_string(),
                );
            }
            "network" => {
                audit_suggestions.push(
                    "Review network configuration for security and performance".to_string(),
                );
            }
            _ => {
                audit_suggestions.push(
                    "Review configuration changes for potential impact".to_string(),
                );
            }
        }

        // 生成审计结果
        let result = AuditResult {
            status: "completed".to_string(),
            result_id: format!(
                "audit_{}_{}",
                config.config_id,
                chrono::Utc::now().timestamp()
            ),
            audit_logs: vec![audit_log],
            audit_time: chrono::Utc::now().to_string(),
            audited_file: config.config_file_path,
            audit_suggestions,
            version_id: config.version_id,
        };

        // 添加到审计结果列表
        let mut audit_results = self.audit_results.write().await;
        audit_results.push(result.clone());

        // 持久化审计日志
        self.save_audit_logs(&*audit_logs).await?;

        Ok(result)
    }

    /// 获取审计日志
    pub async fn get_audit_logs(
        &self,
        config_file_path: String,
    ) -> Result<Vec<AuditLog>, Box<dyn std::error::Error>> {
        let audit_logs = self.audit_logs.read().await;
        let filtered_logs = (&*audit_logs)
            .iter()
            .filter(|log| {
                log.change_content
                    .get("file_path")
                    .unwrap_or(&serde_json::json!(""))
                    .as_str()
                    .unwrap_or("")
                    == config_file_path
            })
            .cloned()
            .collect();
        Ok(filtered_logs)
    }

    /// 获取审计结果列表
    pub async fn get_audit_results(&self) -> Result<Vec<AuditResult>, Box<dyn std::error::Error>> {
        let audit_results = self.audit_results.read().await;
        Ok((&*audit_results).clone())
    }

    /// 回滚配置变更
    pub async fn rollback_change(
        &self,
        audit_log_id: String,
        config_file_path: String,
    ) -> Result<AuditResult, Box<dyn std::error::Error>> {
        // 查找对应的审计日志
        let target_log = {
            let audit_logs = self.audit_logs.read().await;
            audit_logs
                .iter()
                .find(|log| log.log_id == audit_log_id)
                .cloned()
        };

        if let Some(log) = target_log {
            // 回滚到之前的版本
            let rollback_version_id = if let Some(version_id) = &log.version_id {
                // 使用版本管理器回滚
                match self.version_manager.rollback_to_version(version_id.clone(), config_file_path.clone()).await {
                    Ok(result) => Some(result.version_info.version),
                    Err(_) => {
                        // 版本回滚失败，尝试从审计日志中恢复配置
                        if let Some(current_content) = log.change_content.get("current_content").and_then(|v| v.as_str()) {
                            tokio::fs::write(&config_file_path, current_content).await?;
                            None
                        } else {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "No rollback information available in audit log",
                            )));
                        }
                    }
                }
            } else {
                // 尝试从审计日志中恢复配置
                if let Some(current_content) = log.change_content.get("current_content").and_then(|v| v.as_str()) {
                    tokio::fs::write(&config_file_path, current_content).await?;
                    None
                } else {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "No rollback information available in audit log",
                    )));
                }
            };

            // 生成回滚审计日志
            let rollback_log = AuditLog {
                log_id: format!(
                    "log_rollback_{}_{}",
                    audit_log_id,
                    chrono::Utc::now().timestamp()
                ),
                change_type: "rollback".to_string(),
                change_content: serde_json::json!({
                    "original_log_id": audit_log_id,
                    "rollback_time": chrono::Utc::now().to_string(),
                    "original_change_type": log.change_type,
                    "original_change_time": log.change_time,
                    "original_changed_by": log.changed_by
                }),
                change_time: chrono::Utc::now().to_string(),
                changed_by: "system".to_string(),
                change_status: "completed".to_string(),
                version_id: rollback_version_id.clone(),
            };

            // 添加到审计日志列表
            let mut audit_logs = self.audit_logs.write().await;
            audit_logs.push(rollback_log.clone());

            // 生成回滚审计结果
            let result = AuditResult {
                status: "completed".to_string(),
                result_id: format!(
                    "audit_rollback_{}_{}",
                    audit_log_id,
                    chrono::Utc::now().timestamp()
                ),
                audit_logs: vec![rollback_log],
                audit_time: chrono::Utc::now().to_string(),
                audited_file: config_file_path,
                audit_suggestions: vec!["Review the rollback results to ensure configuration is correct".to_string()],
                version_id: rollback_version_id,
            };

            // 添加到审计结果列表
            let mut audit_results = self.audit_results.write().await;
            audit_results.push(result.clone());

            // 持久化审计日志
            self.save_audit_logs(&*audit_logs).await?;

            Ok(result)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Audit log with id {} not found", audit_log_id),
            )))
        }
    }

    /// 保存审计日志到文件
    async fn save_audit_logs(&self, audit_logs: &[AuditLog]) -> Result<(), Box<dyn std::error::Error>> {
        let logs_path = Path::new(&self.audit_storage_path).join("audit_logs.json");
        let content = serde_json::to_string_pretty(audit_logs)?;
        tokio::fs::write(logs_path, content).await?;
        Ok(())
    }

    /// 从文件加载审计日志
    async fn load_audit_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let logs_path = Path::new(&self.audit_storage_path).join("audit_logs.json");
        if tokio::fs::try_exists(logs_path.clone()).await? {
            let content = tokio::fs::read_to_string(logs_path).await?;
            let logs: Vec<AuditLog> = serde_json::from_str(&content)?;
            let mut audit_logs = self.audit_logs.write().await;
            audit_logs.extend(logs);
        }
        Ok(())
    }
}
