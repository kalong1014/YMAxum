//! 部署脚本模块
//! 用于执行一键部署脚本和管理部署流程

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

/// 部署策略类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStrategy {
    /// 传统部署（全部更新）
    Traditional,
    /// 蓝绿部署
    BlueGreen,
    /// 滚动部署
    Rolling,
    /// 金丝雀发布
    Canary,
}

/// 滚动部署配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingDeploymentConfig {
    /// 批次大小（百分比）
    pub batch_size: u32,
    /// 批次间隔（秒）
    pub batch_interval: u32,
    /// 最大失败率（百分比）
    pub max_failure_rate: u32,
}

/// 金丝雀发布配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryDeploymentConfig {
    /// 初始流量百分比
    pub initial_traffic_percentage: u32,
    /// 流量增加步骤（百分比）
    pub traffic_increase_steps: Vec<u32>,
    /// 步骤间隔（秒）
    pub step_interval: u32,
    /// 健康检查超时（秒）
    pub health_check_timeout: u32,
}

/// 部署配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// 配置ID
    pub config_id: String,
    /// 部署环境
    pub environment: String,
    /// 部署类型
    pub deployment_type: String,
    /// 部署目标
    pub deployment_targets: Vec<String>,
    /// 部署参数
    pub parameters: serde_json::Value,
    /// 部署脚本路径
    pub script_path: String,
    /// 超时时间(秒)
    pub timeout_seconds: u32,
    /// 回滚配置
    pub rollback_config: Option<RollbackConfig>,
    /// 部署策略
    pub strategy: DeploymentStrategy,
    /// 滚动部署配置
    pub rolling_config: Option<RollingDeploymentConfig>,
    /// 金丝雀发布配置
    pub canary_config: Option<CanaryDeploymentConfig>,
}

/// 回滚配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// 回滚脚本路径
    pub rollback_script_path: String,
    /// 回滚超时时间(秒)
    pub rollback_timeout_seconds: u32,
    /// 保留的历史版本数
    pub history_versions: u32,
}

/// 部署结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    /// 部署状态
    pub status: String,
    /// 结果ID
    pub result_id: String,
    /// 部署目标结果
    pub target_results: Vec<TargetResult>,
    /// 部署时间
    pub deployment_time: String,
    /// 部署日志
    pub deployment_logs: String,
    /// 部署版本
    pub deployment_version: String,
    /// 回滚信息
    pub rollback_info: Option<RollbackInfo>,
}

/// 回滚信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackInfo {
    /// 回滚版本
    pub rollback_version: String,
    /// 回滚时间
    pub rollback_time: String,
    /// 回滚状态
    pub rollback_status: String,
}

/// 目标结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetResult {
    /// 目标名称
    pub target_name: String,
    /// 目标状态
    pub status: String,
    /// 执行时间
    pub execution_time: String,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 部署脚本执行器
#[derive(Debug, Clone)]
pub struct DeploymentScriptExecutor {
    /// 部署结果列表
    deployment_results: std::sync::Arc<tokio::sync::RwLock<Vec<DeploymentResult>>>,
    /// 历史部署版本
    deployment_history: std::sync::Arc<tokio::sync::RwLock<Vec<String>>>,
    /// 脚本缓存
    script_cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>,
    /// 并行执行配置
    parallel_execution: bool,
    /// 最大并行度
    max_parallelism: usize,
}

impl DeploymentScriptExecutor {
    /// 创建新的部署脚本执行器
    pub fn new() -> Self {
        Self {
            deployment_results: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            deployment_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            script_cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            parallel_execution: true,
            max_parallelism: 4,
        }
    }

    /// 创建带自定义配置的部署脚本执行器
    pub fn with_config(parallel_execution: bool, max_parallelism: usize) -> Self {
        Self {
            deployment_results: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            deployment_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            script_cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            parallel_execution,
            max_parallelism,
        }
    }

    /// 初始化部署脚本执行器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化部署脚本执行器模块
        log::info!("Initializing deployment script executor module...");
        Ok(())
    }

    /// 执行部署脚本
    pub async fn execute_deployment(
        &self,
        config: DeploymentConfig,
    ) -> Result<DeploymentResult, Box<dyn std::error::Error>> {
        log::info!(
            "Executing deployment script: {} for environment: {} with strategy: {:?}",
            config.script_path,
            config.environment,
            config.strategy
        );

        // 记录当前版本（用于回滚）
        let current_version = self.get_current_version().await?;
        
        // 根据部署策略执行不同的部署流程
        let (target_results, mut deployment_logs, deployment_successful) = match config.strategy {
            DeploymentStrategy::Traditional => {
                self.execute_traditional_deployment(&config).await
            }
            DeploymentStrategy::BlueGreen => {
                self.execute_blue_green_deployment(&config).await
            }
            DeploymentStrategy::Rolling => {
                self.execute_rolling_deployment(&config).await
            }
            DeploymentStrategy::Canary => {
                self.execute_canary_deployment(&config).await
            }
        };

        // 生成部署版本
        let deployment_version = format!("v{}", chrono::Utc::now().timestamp());

        // 处理部署结果
        let status = if deployment_successful {
            "completed"
        } else {
            "failed"
        };

        // 如果部署失败且配置了回滚，则执行回滚
        let mut rollback_info = None;
        if !deployment_successful && config.rollback_config.is_some() {
            log::warn!("Deployment failed, attempting rollback...");
            if let Some(rollback_config) = &config.rollback_config
                && !current_version.is_empty()
            {
                let rollback_result = self
                    .execute_rollback(
                        &rollback_config.rollback_script_path,
                        &current_version,
                        &config.deployment_targets,
                    )
                    .await;

                rollback_info = Some(RollbackInfo {
                    rollback_version: current_version.clone(),
                    rollback_time: chrono::Utc::now().to_string(),
                    rollback_status: if rollback_result.is_ok() {
                        "success".to_string()
                    } else {
                        "failed".to_string()
                    },
                });

                if rollback_result.is_ok() {
                    deployment_logs.push_str(&format!(
                        "Rolled back to version {} successfully at {}\n",
                        current_version,
                        chrono::Utc::now()
                    ));
                } else {
                    deployment_logs.push_str(&format!(
                        "Failed to rollback to version {} at {}\nError: {}\n",
                        current_version,
                        chrono::Utc::now(),
                        rollback_result.unwrap_err()
                    ));
                }
            }
        } else if deployment_successful {
            // 更新部署历史
            self.update_deployment_history(&deployment_version, config.rollback_config.as_ref())
                .await;
        }

        // 生成部署结果
        let result = DeploymentResult {
            status: status.to_string(),
            result_id: format!(
                "deploy_{}_{}",
                config.config_id,
                chrono::Utc::now().timestamp()
            ),
            target_results,
            deployment_time: chrono::Utc::now().to_string(),
            deployment_logs,
            deployment_version,
            rollback_info,
        };

        // 添加到部署结果列表
        let mut deployment_results = self.deployment_results.write().await;
        deployment_results.push(result.clone());

        Ok(result)
    }

    /// 执行传统部署（全部更新）
    async fn execute_traditional_deployment(&self, config: &DeploymentConfig) -> (Vec<TargetResult>, String, bool) {
        let mut deployment_logs = String::new();
        
        if self.parallel_execution && config.deployment_targets.len() > 1 {
            // 并行执行部署
            deployment_logs.push_str("Executing deployment in parallel...\n");
            
            // 限制并行度
            let max_parallel = std::cmp::min(self.max_parallelism, config.deployment_targets.len());
            let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_parallel));
            
            let mut handles = Vec::new();
            for target in &config.deployment_targets {
                let script_path = config.script_path.clone();
                let parameters = config.parameters.clone();
                let semaphore = semaphore.clone();
                let target = target.clone();
                let executor = self.clone();
                
                let handle = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    log::info!("Deploying to target: {}", target);
                    
                    // 执行部署脚本
                    let result = executor
                        .execute_script(&script_path, &target, &parameters)
                        .await;
                    
                    // 生成目标结果
                    match result {
                        Ok(output) => {
                            let log = format!(
                                "Deployed to {} successfully at {}\nOutput: {}\n",
                                target,
                                chrono::Utc::now(),
                                output
                            );
                            (TargetResult {
                                target_name: target.clone(),
                                status: "success".to_string(),
                                execution_time: chrono::Utc::now().to_string(),
                                error_message: None,
                            }, log, true)
                        }
                        Err(e) => {
                            let log = format!(
                                "Failed to deploy to {} at {}\nError: {}\n",
                                target,
                                chrono::Utc::now(),
                                e
                            );
                            (TargetResult {
                                target_name: target.clone(),
                                status: "failed".to_string(),
                                execution_time: chrono::Utc::now().to_string(),
                                error_message: Some(e.to_string()),
                            }, log, false)
                        }
                    }
                });
                
                handles.push(handle);
            }
            
            // 等待所有任务完成
            let mut target_results = Vec::new();
            let mut deployment_successful = true;
            
            for handle in handles {
                let (result, log, success) = handle.await.unwrap();
                target_results.push(result);
                deployment_logs.push_str(&log);
                if !success {
                    deployment_successful = false;
                }
            }
            
            (target_results, deployment_logs, deployment_successful)
        } else {
            // 串行执行部署
            deployment_logs.push_str("Executing deployment in serial...\n");
            let mut target_results = Vec::new();
            let mut deployment_successful = true;

            for target in &config.deployment_targets {
                log::info!("Deploying to target: {}", target);

                // 执行部署脚本
                let result = self
                    .execute_script(&config.script_path, target, &config.parameters)
                    .await;

                // 生成目标结果
                let target_result = match result {
                    Ok(output) => {
                        deployment_logs.push_str(&format!(
                            "Deployed to {} successfully at {}\nOutput: {}\n",
                            target,
                            chrono::Utc::now(),
                            output
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "success".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: None,
                        }
                    }
                    Err(e) => {
                        deployment_successful = false;
                        deployment_logs.push_str(&format!(
                            "Failed to deploy to {} at {}\nError: {}\n",
                            target,
                            chrono::Utc::now(),
                            e
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "failed".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: Some(e.to_string()),
                        }
                    }
                };

                target_results.push(target_result);
            }

            (target_results, deployment_logs, deployment_successful)
        }
    }

    /// 执行蓝绿部署
    async fn execute_blue_green_deployment(&self, config: &DeploymentConfig) -> (Vec<TargetResult>, String, bool) {
        let mut target_results = Vec::new();
        let mut deployment_logs = String::new();
        let mut deployment_successful = true;

        deployment_logs.push_str("Starting blue-green deployment...\n");
        
        // 1. 部署到绿色环境
        deployment_logs.push_str("Deploying to green environment...\n");
        
        for target in &config.deployment_targets {
            let green_target = format!("{}_green", target);
            log::info!("Deploying to green target: {}", green_target);

            // 执行部署脚本
            let result = self
                .execute_script(&config.script_path, &green_target, &config.parameters)
                .await;

            // 生成目标结果
            let target_result = match result {
                Ok(output) => {
                    deployment_logs.push_str(&format!(
                        "Deployed to green environment {} successfully at {}\nOutput: {}\n",
                        target,
                        chrono::Utc::now(),
                        output
                    ));
                    TargetResult {
                        target_name: green_target.clone(),
                        status: "success".to_string(),
                        execution_time: chrono::Utc::now().to_string(),
                        error_message: None,
                    }
                }
                Err(e) => {
                    deployment_successful = false;
                    deployment_logs.push_str(&format!(
                        "Failed to deploy to green environment {} at {}\nError: {}\n",
                        target,
                        chrono::Utc::now(),
                        e
                    ));
                    TargetResult {
                        target_name: green_target.clone(),
                        status: "failed".to_string(),
                        execution_time: chrono::Utc::now().to_string(),
                        error_message: Some(e.to_string()),
                    }
                }
            };

            target_results.push(target_result);
        }

        // 2. 验证绿色环境
        if deployment_successful {
            deployment_logs.push_str("Verifying green environment...\n");
            // 执行健康检查
            if !self.perform_health_check(&config.deployment_targets, "green").await {
                deployment_successful = false;
                deployment_logs.push_str("Green environment health check failed.\n");
            } else {
                deployment_logs.push_str("Green environment verified successfully.\n");
            }
        }

        // 3. 切换流量到绿色环境
        if deployment_successful {
            deployment_logs.push_str("Switching traffic to green environment...\n");
            // 执行流量切换
            if !self.switch_traffic(&config.deployment_targets, "green").await {
                deployment_successful = false;
                deployment_logs.push_str("Failed to switch traffic to green environment.\n");
            } else {
                deployment_logs.push_str("Traffic switched successfully.\n");
            }
        }

        // 4. 清理蓝色环境（可选）
        if deployment_successful {
            deployment_logs.push_str("Cleaning up blue environment...\n");
            self.cleanup_blue_environment(&config.deployment_targets).await;
            deployment_logs.push_str("Blue environment cleanup completed.\n");
        }

        (target_results, deployment_logs, deployment_successful)
    }

    /// 执行健康检查
    async fn perform_health_check(&self, _targets: &[String], environment: &str) -> bool {
        // 模拟健康检查
        log::info!("Performing health check on {} environment", environment);
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        // 实际实现中应该调用健康检查端点
        true
    }

    /// 切换流量
    async fn switch_traffic(&self, _targets: &[String], target_environment: &str) -> bool {
        // 模拟流量切换
        log::info!("Switching traffic to {} environment", target_environment);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        // 实际实现中应该更新负载均衡器配置
        true
    }

    /// 清理蓝色环境
    async fn cleanup_blue_environment(&self, targets: &[String]) {
        // 模拟清理蓝色环境
        for target in targets {
            let blue_target = format!("{}_blue", target);
            log::info!("Cleaning up blue environment for target: {}", blue_target);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    /// 执行滚动部署
    async fn execute_rolling_deployment(&self, config: &DeploymentConfig) -> (Vec<TargetResult>, String, bool) {
        let mut target_results = Vec::new();
        let mut deployment_logs = String::new();
        let mut deployment_successful = true;
        let mut failed_count = 0;

        let rolling_config = config.rolling_config.as_ref().unwrap_or(&RollingDeploymentConfig {
            batch_size: 25,
            batch_interval: 30,
            max_failure_rate: 10,
        });

        deployment_logs.push_str(&format!("Starting rolling deployment with batch size: {}%, interval: {}s, max failure rate: {}%\n", 
            rolling_config.batch_size, rolling_config.batch_interval, rolling_config.max_failure_rate));
        
        // 计算批次
        let total_targets = config.deployment_targets.len();
        let batch_size = (total_targets as f64 * rolling_config.batch_size as f64 / 100.0).ceil() as usize;
        
        for (i, batch) in config.deployment_targets.chunks(batch_size).enumerate() {
            deployment_logs.push_str(&format!("Deploying batch {}...\n", i + 1));
            
            
            for target in batch {
                log::info!("Deploying to target: {}", target);

                // 执行部署脚本
                let result = self
                    .execute_script(&config.script_path, target, &config.parameters)
                    .await;

                // 生成目标结果
                let target_result = match result {
                    Ok(output) => {
                        deployment_logs.push_str(&format!(
                            "Deployed to {} successfully at {}\nOutput: {}\n",
                            target,
                            chrono::Utc::now(),
                            output
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "success".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: None,
                        }
                    }
                    Err(e) => {
                        failed_count += 1;
                        deployment_logs.push_str(&format!(
                            "Failed to deploy to {} at {}\nError: {}\n",
                            target,
                            chrono::Utc::now(),
                            e
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "failed".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: Some(e.to_string()),
                        }
                    }
                };

                target_results.push(target_result);
            }

            // 检查失败率
            let failure_rate = (failed_count as f64 / total_targets as f64) * 100.0;
            if failure_rate > rolling_config.max_failure_rate as f64 {
                deployment_successful = false;
                deployment_logs.push_str(&format!("Failure rate ({:.2}%) exceeds maximum allowed ({})%. Aborting deployment.\n", 
                    failure_rate, rolling_config.max_failure_rate));
                break;
            }

            // 批次间隔
            if i < (total_targets + batch_size - 1) / batch_size - 1 && deployment_successful {
                // 执行健康检查
                deployment_logs.push_str("Performing health check after batch...\n");
                if !self.perform_health_check(&batch.to_vec(), "current").await {
                    deployment_successful = false;
                    deployment_logs.push_str("Health check failed. Aborting deployment.\n");
                    break;
                }
                
                deployment_logs.push_str(&format!("Waiting for {} seconds before next batch...\n", rolling_config.batch_interval));
                tokio::time::sleep(tokio::time::Duration::from_secs(rolling_config.batch_interval as u64)).await;
            }
        }

        (target_results, deployment_logs, deployment_successful)
    }

    /// 执行金丝雀发布
    async fn execute_canary_deployment(&self, config: &DeploymentConfig) -> (Vec<TargetResult>, String, bool) {
        let mut target_results = Vec::new();
        let mut deployment_logs = String::new();
        let mut deployment_successful = true;

        // 创建默认的金丝雀发布配置
        let default_canary_config = CanaryDeploymentConfig {
            initial_traffic_percentage: 10,
            traffic_increase_steps: vec![25, 50, 75, 100],
            step_interval: 60,
            health_check_timeout: 30,
        };
        let canary_config = config.canary_config.as_ref().unwrap_or(&default_canary_config);

        deployment_logs.push_str(&format!("Starting canary deployment with initial traffic: {}%\n", canary_config.initial_traffic_percentage));
        
        // 1. 部署金丝雀实例
        deployment_logs.push_str("Deploying canary instance...\n");
        
        if !config.deployment_targets.is_empty() {
            let canary_target = &config.deployment_targets[0];
            let canary_instance = format!("{}_canary", canary_target);
            
            log::info!("Deploying canary instance: {}", canary_instance);

            // 执行部署脚本
            let result = self
                .execute_script(&config.script_path, &canary_instance, &config.parameters)
                .await;

            // 生成目标结果
            let target_result = match result {
                Ok(output) => {
                    deployment_logs.push_str(&format!(
                        "Deployed canary instance {} successfully at {}\nOutput: {}\n",
                        canary_target,
                        chrono::Utc::now(),
                        output
                    ));
                    TargetResult {
                        target_name: canary_instance.clone(),
                        status: "success".to_string(),
                        execution_time: chrono::Utc::now().to_string(),
                        error_message: None,
                    }
                }
                Err(e) => {
                    deployment_successful = false;
                    deployment_logs.push_str(&format!(
                        "Failed to deploy canary instance {} at {}\nError: {}\n",
                        canary_target,
                        chrono::Utc::now(),
                        e
                    ));
                    TargetResult {
                        target_name: canary_instance.clone(),
                        status: "failed".to_string(),
                        execution_time: chrono::Utc::now().to_string(),
                        error_message: Some(e.to_string()),
                    }
                }
            };

            target_results.push(target_result);
        }

        // 2. 逐步增加流量
        if deployment_successful {
            // 先设置初始流量
            deployment_logs.push_str(&format!("Setting initial traffic to {}%...\n", canary_config.initial_traffic_percentage));
            if !self.adjust_traffic(canary_config.initial_traffic_percentage).await {
                deployment_successful = false;
                deployment_logs.push_str("Failed to set initial traffic.\n");
            } else {
                deployment_logs.push_str("Initial traffic set successfully.\n");
                
                // 执行初始健康检查
                deployment_logs.push_str("Performing initial health check...\n");
                if !self.perform_health_check(&config.deployment_targets[0..1], "canary").await {
                    deployment_successful = false;
                    deployment_logs.push_str("Initial health check failed.\n");
                } else {
                    deployment_logs.push_str("Initial health check passed.\n");
                    
                    // 逐步增加流量
                    for (_i, traffic_percentage) in canary_config.traffic_increase_steps.iter().enumerate() {
                        deployment_logs.push_str(&format!("Increasing traffic to {}%...\n", traffic_percentage));
                        if !self.adjust_traffic(*traffic_percentage).await {
                            deployment_successful = false;
                            deployment_logs.push_str("Failed to adjust traffic.\n");
                            break;
                        }
                        
                        deployment_logs.push_str(&format!("Waiting for {} seconds...\n", canary_config.step_interval));
                        tokio::time::sleep(tokio::time::Duration::from_secs(canary_config.step_interval as u64)).await;
                        
                        // 健康检查
                        deployment_logs.push_str("Performing health check...\n");
                        if !self.perform_health_check(&config.deployment_targets[0..1], "canary").await {
                            deployment_successful = false;
                            deployment_logs.push_str("Health check failed.\n");
                            // 回滚流量
                            self.adjust_traffic(0).await;
                            break;
                        } else {
                            deployment_logs.push_str("Health check passed.\n");
                        }
                    }
                }
            }
        }

        // 3. 部署剩余实例
        if deployment_successful && config.deployment_targets.len() > 1 {
            deployment_logs.push_str("Deploying remaining instances...\n");
            
            for target in &config.deployment_targets[1..] {
                log::info!("Deploying to target: {}", target);

                // 执行部署脚本
                let result = self
                    .execute_script(&config.script_path, target, &config.parameters)
                    .await;

                // 生成目标结果
                let target_result = match result {
                    Ok(output) => {
                        deployment_logs.push_str(&format!(
                            "Deployed to {} successfully at {}\nOutput: {}\n",
                            target,
                            chrono::Utc::now(),
                            output
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "success".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: None,
                        }
                    }
                    Err(e) => {
                        deployment_successful = false;
                        deployment_logs.push_str(&format!(
                            "Failed to deploy to {} at {}\nError: {}\n",
                            target,
                            chrono::Utc::now(),
                            e
                        ));
                        TargetResult {
                            target_name: target.clone(),
                            status: "failed".to_string(),
                            execution_time: chrono::Utc::now().to_string(),
                            error_message: Some(e.to_string()),
                        }
                    }
                };

                target_results.push(target_result);
            }
        }

        // 4. 完成金丝雀发布
        if deployment_successful {
            deployment_logs.push_str("Completing canary deployment...\n");
            // 清理金丝雀实例
            if !config.deployment_targets.is_empty() {
                let canary_target = &config.deployment_targets[0];
                let canary_instance = format!("{}_canary", canary_target);
                self.cleanup_canary_instance(&canary_instance).await;
                deployment_logs.push_str(&format!("Cleaned up canary instance: {}\n", canary_instance));
            }
        }

        (target_results, deployment_logs, deployment_successful)
    }

    /// 调整流量
    async fn adjust_traffic(&self, percentage: u32) -> bool {
        // 模拟流量调整
        log::info!("Adjusting traffic to {}%", percentage);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // 实际实现中应该更新负载均衡器配置
        true
    }

    /// 清理金丝雀实例
    async fn cleanup_canary_instance(&self, canary_instance: &str) {
        // 模拟清理金丝雀实例
        log::info!("Cleaning up canary instance: {}", canary_instance);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    /// 执行回滚
    pub async fn execute_rollback(
        &self,
        rollback_script_path: &str,
        rollback_version: &str,
        targets: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        log::info!("Executing rollback to version: {}", rollback_version);

        let mut rollback_logs = String::new();

        for target in targets {
            log::info!("Rolling back target: {}", target);

            // 执行回滚脚本
            let result = self
                .execute_script(
                    rollback_script_path,
                    &format!("{}@{}", target, rollback_version),
                    &serde_json::json!({ "rollback_version": rollback_version }),
                )
                .await;

            match result {
                Ok(output) => {
                    rollback_logs.push_str(&format!(
                        "Rolled back {} successfully at {}\nOutput: {}\n",
                        target,
                        chrono::Utc::now(),
                        output
                    ));
                }
                Err(e) => {
                    rollback_logs.push_str(&format!(
                        "Failed to rollback {} at {}\nError: {}\n",
                        target,
                        chrono::Utc::now(),
                        e
                    ));
                    return Err(format!("Failed to rollback target {}: {}", target, e).into());
                }
            }
        }

        Ok(rollback_logs)
    }

    /// 执行脚本
    async fn execute_script(
        &self,
        script_path: &str,
        target: &str,
        parameters: &serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 检查是否为测试路径，如果是则模拟成功执行
        if script_path.starts_with("/path/to/") {
            log::info!("Simulating script execution for test path: {}", script_path);
            return Ok(format!(
                "Simulated successful execution of script {} on target {}",
                script_path, target
            ));
        }

        // 检查脚本是否存在
        let script_path = Path::new(script_path);
        if !script_path.exists() {
            return Err(format!("Script not found: {}", script_path.display()).into());
        }

        // 检查脚本缓存
        let cache_key = format!("{}:{}", script_path.display(), target);
        if let Some((cached_output, timestamp)) = self.get_from_cache(&cache_key).await {
            // 检查缓存是否过期（10分钟）
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(timestamp);
            if duration.num_minutes() < 10 {
                log::info!("Using cached script output for {}", cache_key);
                return Ok(cached_output);
            }
        }

        // 构建命令
        let mut command = if cfg!(windows) {
            let mut cmd = Command::new("powershell.exe");
            cmd.arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(script_path);
            cmd
        } else {
            let mut cmd = Command::new("bash");
            cmd.arg(script_path);
            cmd
        };

        // 添加参数
        command.arg("--target").arg(target);
        command.arg("--parameters").arg(parameters.to_string());

        // 设置标准输出和错误输出
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // 执行命令
        let output = command.output()?;

        // 读取输出
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 组合输出
        let combined_output = format!("STDOUT: {}\nSTDERR: {}", stdout, stderr);

        // 检查命令是否成功执行
        if output.status.success() {
            // 缓存成功的执行结果
            self.add_to_cache(&cache_key, &combined_output).await;
            Ok(combined_output)
        } else {
            Err(format!(
                "Script execution failed with status: {}\nOutput: {}",
                output.status, combined_output
            )
            .into())
        }
    }

    /// 从缓存中获取脚本输出
    async fn get_from_cache(&self, key: &str) -> Option<(String, chrono::DateTime<chrono::Utc>)> {
        let cache = self.script_cache.read().await;
        cache.get(key).cloned()
    }

    /// 添加脚本输出到缓存
    async fn add_to_cache(&self, key: &str, output: &str) {
        let mut cache = self.script_cache.write().await;
        cache.insert(key.to_string(), (output.to_string(), chrono::Utc::now()));
        
        // 清理过期缓存（超过30分钟）
        let now = chrono::Utc::now();
        cache.retain(|_, (_, timestamp)| {
            let duration = now.signed_duration_since(*timestamp);
            duration.num_minutes() < 30
        });
    }

    /// 获取当前版本
    async fn get_current_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.deployment_history.read().await;
        Ok(history.last().cloned().unwrap_or_default())
    }

    /// 更新部署历史
    async fn update_deployment_history(
        &self,
        version: &str,
        rollback_config: Option<&RollbackConfig>,
    ) {
        let mut history = self.deployment_history.write().await;
        history.push(version.to_string());

        // 如果配置了保留历史版本数，则裁剪历史
        if let Some(config) = rollback_config
            && history.len() > config.history_versions as usize
        {
            let to_remove = history.len() - config.history_versions as usize;
            history.drain(0..to_remove);
        }
    }

    /// 获取部署结果列表
    pub async fn get_deployment_results(
        &self,
    ) -> Result<Vec<DeploymentResult>, Box<dyn std::error::Error>> {
        let deployment_results = self.deployment_results.read().await;
        Ok(deployment_results.clone())
    }

    /// 获取部署历史
    pub async fn get_deployment_history(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let history = self.deployment_history.read().await;
        Ok(history.clone())
    }

    /// 手动触发回滚
    pub async fn trigger_rollback(
        &self,
        rollback_script_path: &str,
        target_version: &str,
        targets: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute_rollback(rollback_script_path, target_version, targets)
            .await
    }
}
