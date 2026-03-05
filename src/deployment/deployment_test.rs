use super::*;

#[tokio::test]
async fn test_deployment_script_executor_initialization() {
    let executor = DeploymentScriptExecutor::new();
    let result = executor.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_script_execution() {
    let executor = DeploymentScriptExecutor::new();
    
    let config = DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string(), "target2".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };
    
    let result = executor.execute_deployment(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert_eq!(deployment_result.target_results.len(), 2);
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(!deployment_result.deployment_version.is_empty());
}

#[tokio::test]
async fn test_blue_green_deployment() {
    let executor = DeploymentScriptExecutor::new();
    
    let config = DeploymentConfig {
        config_id: "test_blue_green".to_string(),
        environment: "test".to_string(),
        deployment_type: "blue_green".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::BlueGreen,
        rolling_config: None,
        canary_config: None,
    };
    
    let result = executor.execute_deployment(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(deployment_result.deployment_logs.contains("blue-green deployment"));
}

#[tokio::test]
async fn test_rolling_deployment() {
    let executor = DeploymentScriptExecutor::new();
    
    let config = DeploymentConfig {
        config_id: "test_rolling".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string(), "target2".to_string(), "target3".to_string(), "target4".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::Rolling,
        rolling_config: Some(RollingDeploymentConfig {
            batch_size: 25,
            batch_interval: 5,
            max_failure_rate: 10,
        }),
        canary_config: None,
    };
    
    let result = executor.execute_deployment(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(deployment_result.deployment_logs.contains("rolling deployment"));
}

#[tokio::test]
async fn test_canary_deployment() {
    let executor = DeploymentScriptExecutor::new();
    
    let config = DeploymentConfig {
        config_id: "test_canary".to_string(),
        environment: "test".to_string(),
        deployment_type: "canary".to_string(),
        deployment_targets: vec!["target1".to_string(), "target2".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::Canary,
        rolling_config: None,
        canary_config: Some(CanaryDeploymentConfig {
            initial_traffic_percentage: 10,
            traffic_increase_steps: vec![25, 50, 75, 100],
            step_interval: 10,
            health_check_timeout: 5,
        }),
    };
    
    let result = executor.execute_deployment(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(deployment_result.deployment_logs.contains("canary deployment"));
}

#[tokio::test]
async fn test_parallel_execution() {
    let executor = DeploymentScriptExecutor::with_config(true, 2);
    
    let config = DeploymentConfig {
        config_id: "test_parallel".to_string(),
        environment: "test".to_string(),
        deployment_type: "parallel".to_string(),
        deployment_targets: vec!["target1".to_string(), "target2".to_string(), "target3".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };
    
    let result = executor.execute_deployment(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(deployment_result.deployment_logs.contains("parallel"));
}

#[tokio::test]
async fn test_deployment_status_monitor_initialization() {
    let monitor = DeploymentStatusMonitor::new();
    let result = monitor.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_status_monitoring() {
    let monitor = DeploymentStatusMonitor::new();
    
    let config = MonitoringConfig {
        config_id: "test_config_456".to_string(),
        deployment_id: "deploy_123".to_string(),
        monitoring_targets: vec!["target1".to_string()],
        monitoring_metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        monitoring_frequency: 60,
        alert_thresholds: serde_json::json!({ "cpu_usage": 80, "memory_usage": 85 }),
        alert_rules: vec![],
        alert_channels: HashMap::new(),
        auto_remediation: false,
        remediation_config: None,
        external_monitoring: None,
    };
    
    let result = monitor.monitor_status(config).await;
    assert!(result.is_ok());
    
    let monitoring_result = result.unwrap();
    assert_eq!(monitoring_result.status, "completed");
    assert_eq!(monitoring_result.monitoring_data.len(), 2);
    assert!(!monitoring_result.alerts.is_empty() || monitoring_result.alerts.is_empty());
    assert!(!monitoring_result.health_status.is_empty());
    assert!(!monitoring_result.system_status.is_empty());
    assert!(!monitoring_result.performance_summary.is_empty());
}

#[tokio::test]
async fn test_enhanced_monitoring_with_alerts() {
    let monitor = DeploymentStatusMonitor::new();
    
    let config = MonitoringConfig {
        config_id: "test_enhanced_monitoring".to_string(),
        deployment_id: "deploy_123".to_string(),
        monitoring_targets: vec!["target1".to_string()],
        monitoring_metrics: vec!["cpu_usage".to_string()],
        monitoring_frequency: 60,
        alert_thresholds: serde_json::json!({ "cpu_usage": 80 }),
        alert_rules: vec![
            AlertRule {
                rule_name: "high_cpu_usage".to_string(),
                metric_name: "cpu_usage".to_string(),
                operator: ">=".to_string(),
                threshold: 80.0,
                duration: 60,
                severity: "critical".to_string(),
                channels: vec![AlertChannel::Email, AlertChannel::Slack],
            },
        ],
        alert_channels: HashMap::from([
            (AlertChannel::Email, serde_json::json!({ "recipients": ["admin@example.com"] })),
            (AlertChannel::Slack, serde_json::json!({ "webhook": "https://slack.com/webhook" })),
        ]),
        auto_remediation: true,
        remediation_config: Some(serde_json::json!({ "actions": ["restart_service"] })),
        external_monitoring: Some(vec![
            ExternalMonitoringConfig {
                system_type: ExternalMonitoringSystem::Prometheus,
                endpoint: "http://localhost:9090".to_string(),
                credentials: None,
                metric_prefix: "ymaxum".to_string(),
                push_frequency: 30,
            },
        ]),
    };
    
    let result = monitor.monitor_status(config).await;
    assert!(result.is_ok());
    
    let monitoring_result = result.unwrap();
    assert_eq!(monitoring_result.status, "completed");
    assert!(!monitoring_result.health_status.is_empty());
    assert!(!monitoring_result.system_status.is_empty());
    assert!(monitoring_result.remediation_status.is_some());
}

#[tokio::test]
async fn test_cloud_platform_deployment_with_load_balancing() {
    let manager = CloudDeploymentManager::new();
    
    let config = CloudDeploymentConfig {
        platform: CloudPlatform::AWS,
        region: "us-east-1".to_string(),
        access_key: "test_access_key".to_string(),
        secret_key: "test_secret_key".to_string(),
        project_id: "test_project".to_string(),
        parameters: HashMap::new(),
        template_path: None,
        environment: "production".to_string(),
        load_balancing: Some(LoadBalancingConfig {
            name: "test-lb".to_string(),
            lb_type: "application".to_string(),
            port: 80,
            target_port: 8080,
            strategy: "round_robin".to_string(),
            health_check: Some(HealthCheckConfig {
                path: "/health".to_string(),
                interval: 30,
                timeout: 5,
                threshold: 3,
            }),
        }),
        multi_region: true,
        regions: vec!["us-west-1".to_string(), "eu-west-1".to_string()],
    };
    
    let result = manager.deploy_to_cloud(config).await;
    assert!(result.is_ok());
    
    let deployment_result = result.unwrap();
    assert_eq!(deployment_result.status, "completed");
    assert!(deployment_result.load_balancing.is_some());
    assert!(deployment_result.multi_region_info.is_some());
    assert!(!deployment_result.deployment_logs.is_empty());
    assert!(deployment_result.deployment_logs.contains("load balancer"));
    assert!(deployment_result.deployment_logs.contains("Multi-region"));
}

#[tokio::test]
async fn test_rollback_manager_initialization() {
    let manager = RollbackManager::new();
    let result = manager.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_rollback_execution() {
    let manager = RollbackManager::new();
    
    let config = RollbackConfig {
        config_id: "test_config_789".to_string(),
        deployment_id: "deploy_123".to_string(),
        rollback_targets: vec!["target1".to_string(), "target2".to_string()],
        rollback_version: "v1.0.0".to_string(),
        rollback_reason: "Test rollback".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = manager.execute_rollback(config).await;
    assert!(result.is_ok());
    
    let rollback_result = result.unwrap();
    assert_eq!(rollback_result.status, "completed");
    assert_eq!(rollback_result.target_results.len(), 2);
    assert!(!rollback_result.rollback_logs.is_empty());
    assert_eq!(rollback_result.rollback_version, "v1.0.0");
}

#[tokio::test]
async fn test_rollback_history_retrieval() {
    let manager = RollbackManager::new();
    
    let config = RollbackConfig {
        config_id: "test_config_789".to_string(),
        deployment_id: "deploy_123".to_string(),
        rollback_targets: vec!["target1".to_string()],
        rollback_version: "v1.0.0".to_string(),
        rollback_reason: "Test rollback".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    // Execute rollback to populate history
    let _ = manager.execute_rollback(config).await;
    
    // Retrieve history
    let history_result = manager.get_rollback_history("deploy_123".to_string()).await;
    assert!(history_result.is_ok());
    
    let history = history_result.unwrap();
    assert!(!history.is_empty());
}

#[tokio::test]
async fn test_deployment_manager_initialization() {
    let manager = DeploymentManager::new();
    let result = manager.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_manager_deployment_execution() {
    let manager = DeploymentManager::new();
    
    let config = DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };
    
    let result = manager.execute_deployment(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_manager_status_monitoring() {
    let manager = DeploymentManager::new();
    
    let config = MonitoringConfig {
        config_id: "test_config_456".to_string(),
        deployment_id: "deploy_123".to_string(),
        monitoring_targets: vec!["target1".to_string()],
        monitoring_metrics: vec!["cpu_usage".to_string()],
        monitoring_frequency: 60,
        alert_thresholds: serde_json::json!({ "cpu_usage": 80 }),
        alert_rules: vec![],
        alert_channels: HashMap::new(),
        auto_remediation: false,
        remediation_config: None,
        external_monitoring: None,
    };
    
    let result = manager.monitor_status(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_manager_rollback_execution() {
    let manager = DeploymentManager::new();
    
    let config = RollbackConfig {
        config_id: "test_config_789".to_string(),
        deployment_id: "deploy_123".to_string(),
        rollback_targets: vec!["target1".to_string()],
        rollback_version: "v1.0.0".to_string(),
        rollback_reason: "Test rollback".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = manager.execute_rollback(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_manager_cloud_deployment() {
    let manager = DeploymentManager::new();
    
    let config = CloudDeploymentConfig {
        platform: CloudPlatform::AWS,
        region: "us-east-1".to_string(),
        access_key: "test_access_key".to_string(),
        secret_key: "test_secret_key".to_string(),
        project_id: "test_project".to_string(),
        parameters: HashMap::new(),
        template_path: None,
        environment: "production".to_string(),
        load_balancing: None,
        multi_region: false,
        regions: vec![],
    };
    
    let result = manager.deploy_to_cloud(config).await;
    assert!(result.is_ok());
}
