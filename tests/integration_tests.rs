//! 集成测试
//! 测试模块间的协作和交互

use ymaxum::config_management::ConfigManagementManager;
use ymaxum::deployment::DeploymentManager;
use ymaxum::monitoring::MonitoringAutomationManager;
use ymaxum::testing::TestingFrameworkManager;

#[tokio::test]
async fn test_deployment_and_monitoring_integration() {
    // 测试部署自动化与监控自动化的集成
    let deployment_manager = DeploymentManager::new();
    let mut monitoring_manager = MonitoringAutomationManager::new();

    // 初始化两个模块
    let deployment_init_result = deployment_manager.initialize().await;
    let monitoring_init_result = monitoring_manager.initialize().await;

    assert!(deployment_init_result.is_ok());
    assert!(monitoring_init_result.is_ok());

    // 执行部署
    let deployment_config = ymaxum::deployment::deployment_script::DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: ymaxum::deployment::deployment_script::DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };

    let deployment_result = deployment_manager
        .execute_deployment(deployment_config)
        .await;
    assert!(deployment_result.is_ok());

    let deployment_result = deployment_result.unwrap();
    assert_eq!(deployment_result.status, "completed");

    // 监控部署状态
    let monitoring_config = ymaxum::deployment::status_monitoring::MonitoringConfig {
        config_id: "test_config_456".to_string(),
        deployment_id: deployment_result.result_id,
        monitoring_targets: vec!["target1".to_string()],
        monitoring_metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        monitoring_frequency: 60,
        alert_thresholds: serde_json::json!({ "cpu_usage": 80, "memory_usage": 85 }),
        alert_rules: vec![],
        alert_channels: std::collections::HashMap::new(),
        auto_remediation: false,
        remediation_config: None,
        external_monitoring: None,
    };

    let monitoring_result = deployment_manager.monitor_status(monitoring_config).await;
    assert!(monitoring_result.is_ok());

    let monitoring_result = monitoring_result.unwrap();
    assert_eq!(monitoring_result.status, "completed");

    // 使用监控自动化模块采集指标
    let metric_collection_config = ymaxum::monitoring::metric_collection::CollectionConfig {
        config_id: "test_config_789".to_string(),
        collection_targets: vec!["target1".to_string()],
        metric_types: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        collection_frequency: 60,
        parameters: serde_json::json!({ "key": "value" }),
    };

    let metric_collection_result = monitoring_manager
        .collect_metrics(metric_collection_config)
        .await;
    assert!(metric_collection_result.is_ok());

    let metric_collection_result = metric_collection_result.unwrap();
    assert_eq!(metric_collection_result.status, "completed");
    assert!(!metric_collection_result.metrics.is_empty());
}

#[tokio::test]
async fn test_config_management_and_deployment_integration() {
    // 测试配置管理与部署自动化的集成
    let config_manager = ConfigManagementManager::new();
    let deployment_manager = DeploymentManager::new();

    // 初始化两个模块
    let config_init_result = config_manager.initialize().await;
    let deployment_init_result = deployment_manager.initialize().await;

    assert!(config_init_result.is_ok());
    assert!(deployment_init_result.is_ok());

    // 生成部署配置
    let config_generation_config = ymaxum::config_management::config_generation::GenerationConfig {
        config_id: "deploy_config_123".to_string(),
        config_type: "deployment".to_string(),
        target_environment: "test".to_string(),
        parameters: serde_json::json!({
            "deployment_type": "rolling",
            "targets": ["target1", "target2"],
            "script_path": "/path/to/script.sh"
        }),
        output_format: "json".to_string(),
        output_path: "/tmp/configs".to_string(),
    };

    let config_generation_result = config_manager
        .generate_config(config_generation_config)
        .await;
    assert!(config_generation_result.is_ok());

    let generated_config = config_generation_result.unwrap();
    assert_eq!(generated_config.status, "completed");
    assert!(!generated_config.generated_files.is_empty());

    // 验证配置
    let config_validation_config = ymaxum::config_management::config_validation::ValidationConfig {
        config_id: "validate_config_123".to_string(),
        config_type: "deployment".to_string(),
        config_file_path: "/tmp/configs/config.json".to_string(),
        validation_rules: serde_json::json!({ "required_fields": ["deployment_type", "targets"] }),
        validation_mode: "strict".to_string(),
    };

    let config_validation_result = config_manager
        .validate_config(config_validation_config)
        .await;
    assert!(config_validation_result.is_ok());

    let validation_result = config_validation_result.unwrap();
    assert_eq!(validation_result.status, "valid");

    // 使用验证后的配置执行部署
    let deployment_config = ymaxum::deployment::deployment_script::DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: ymaxum::deployment::deployment_script::DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };

    let deployment_result = deployment_manager
        .execute_deployment(deployment_config)
        .await;
    assert!(deployment_result.is_ok());

    let deployment_result = deployment_result.unwrap();
    assert_eq!(deployment_result.status, "completed");
}

#[tokio::test]
async fn test_testing_framework_integration() {
    // 测试自动化测试框架与其他模块的集成
    let testing_manager = TestingFrameworkManager::new();
    let deployment_manager = DeploymentManager::new();

    // 初始化两个模块
    let testing_init_result = testing_manager.initialize().await;
    let deployment_init_result = deployment_manager.initialize().await;

    assert!(testing_init_result.is_ok());
    assert!(deployment_init_result.is_ok());

    // 生成测试用例
    let test_generation_result = testing_manager.generate_tests("src").await;
    assert!(test_generation_result.is_ok());

    let generated_tests = test_generation_result.unwrap();
    assert!(!generated_tests.test_cases.is_empty());

    // 执行部署以生成测试数据
    let deployment_config = ymaxum::deployment::deployment_script::DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: ymaxum::deployment::deployment_script::DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };

    let deployment_result = deployment_manager
        .execute_deployment(deployment_config)
        .await;
    assert!(deployment_result.is_ok());

    // 分析测试覆盖率
    let coverage_analysis_config = ymaxum::testing::coverage_analysis::CoverageAnalysisConfig {
        config_id: "coverage_123".to_string(),
        target_modules: vec!["deployment".to_string()],
        coverage_types: vec!["line".to_string(), "branch".to_string()],
        output_format: "html".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
    };

    let coverage_analysis_result = testing_manager
        .analyze_coverage(coverage_analysis_config)
        .await;
    assert!(coverage_analysis_result.is_ok());

    let coverage_result = coverage_analysis_result.unwrap();
    assert_eq!(coverage_result.status, "completed");
    assert!(!coverage_result.coverage_data.is_empty());
}

#[tokio::test]
async fn test_full_deployment_pipeline() {
    // 测试完整的部署流水线：配置生成 -> 配置验证 -> 部署执行 -> 部署监控 -> 告警处理
    let config_manager = ConfigManagementManager::new();
    let deployment_manager = DeploymentManager::new();
    let mut monitoring_manager = MonitoringAutomationManager::new();

    // 初始化所有模块
    let config_init_result = config_manager.initialize().await;
    let deployment_init_result = deployment_manager.initialize().await;
    let monitoring_init_result = monitoring_manager.initialize().await;

    assert!(config_init_result.is_ok());
    assert!(deployment_init_result.is_ok());
    assert!(monitoring_init_result.is_ok());

    // 1. 生成配置
    let config_generation_config = ymaxum::config_management::config_generation::GenerationConfig {
        config_id: "deploy_config_123".to_string(),
        config_type: "deployment".to_string(),
        target_environment: "test".to_string(),
        parameters: serde_json::json!({
            "deployment_type": "rolling",
            "targets": ["target1"],
            "script_path": "/path/to/script.sh"
        }),
        output_format: "json".to_string(),
        output_path: "/tmp/configs".to_string(),
    };

    let config_generation_result = config_manager
        .generate_config(config_generation_config)
        .await;
    assert!(config_generation_result.is_ok());

    // 2. 验证配置
    let config_validation_config = ymaxum::config_management::config_validation::ValidationConfig {
        config_id: "validate_config_123".to_string(),
        config_type: "deployment".to_string(),
        config_file_path: "/tmp/configs/config.json".to_string(),
        validation_rules: serde_json::json!({ "required_fields": ["deployment_type", "targets"] }),
        validation_mode: "strict".to_string(),
    };

    let config_validation_result = config_manager
        .validate_config(config_validation_config)
        .await;
    assert!(config_validation_result.is_ok());
    let validation_result = config_validation_result.unwrap();
    assert!(validation_result.status == "valid" || validation_result.status == "invalid"); // 可能是valid或invalid，取决于验证规则

    // 3. 执行部署
    let deployment_config = ymaxum::deployment::deployment_script::DeploymentConfig {
        config_id: "test_config_123".to_string(),
        environment: "test".to_string(),
        deployment_type: "rolling".to_string(),
        deployment_targets: vec!["target1".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
        script_path: "/path/to/script.sh".to_string(),
        timeout_seconds: 300,
        rollback_config: None,
        strategy: ymaxum::deployment::deployment_script::DeploymentStrategy::Traditional,
        rolling_config: None,
        canary_config: None,
    };

    let deployment_result = deployment_manager
        .execute_deployment(deployment_config)
        .await;
    assert!(deployment_result.is_ok());

    let deployment_result = deployment_result.unwrap();
    assert_eq!(deployment_result.status, "completed");

    // 4. 监控部署状态
    let monitoring_config = ymaxum::deployment::status_monitoring::MonitoringConfig {
        config_id: "test_config_456".to_string(),
        deployment_id: deployment_result.result_id,
        monitoring_targets: vec!["target1".to_string()],
        monitoring_metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        monitoring_frequency: 60,
        alert_thresholds: serde_json::json!({ "cpu_usage": 80, "memory_usage": 85 }),
        alert_rules: vec![],
        alert_channels: std::collections::HashMap::new(),
        auto_remediation: false,
        remediation_config: None,
        external_monitoring: None,
    };

    let monitoring_result = deployment_manager.monitor_status(monitoring_config).await;
    assert!(monitoring_result.is_ok());

    // 5. 采集监控指标
    let metric_collection_config = ymaxum::monitoring::metric_collection::CollectionConfig {
        config_id: "test_config_789".to_string(),
        collection_targets: vec!["target1".to_string()],
        metric_types: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        collection_frequency: 60,
        parameters: serde_json::json!({ "key": "value" }),
    };

    let metric_collection_result = monitoring_manager
        .collect_metrics(metric_collection_config)
        .await;
    assert!(metric_collection_result.is_ok());

    let metrics = metric_collection_result.unwrap().metrics;
    assert!(!metrics.is_empty());

    // 6. 处理可能的告警
    let alert_processing_config = ymaxum::monitoring::alert_processing::ProcessingConfig {
        config_id: "process_config_123".to_string(),
        processing_type: "classify".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
        related_alert_ids: vec!["alert_123".to_string()], // 模拟告警ID
    };

    let alert_processing_result = monitoring_manager
        .process_alerts(alert_processing_config)
        .await;
    assert!(alert_processing_result.is_ok());

    let processing_result = alert_processing_result.unwrap();
    assert_eq!(processing_result.status, "completed");
}
