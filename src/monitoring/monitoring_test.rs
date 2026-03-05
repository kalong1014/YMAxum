use super::*;

#[tokio::test]
async fn test_metric_collector_initialization() {
    let collector = MetricCollector::new();
    let result = collector.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_metric_collection() {
    let collector = MetricCollector::new();
    
    let config = CollectionConfig {
        config_id: "test_config_123".to_string(),
        collection_targets: vec!["target1".to_string(), "target2".to_string()],
        metric_types: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        collection_frequency: 60,
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = collector.collect_metrics(config).await;
    assert!(result.is_ok());
    
    let collection_result = result.unwrap();
    assert_eq!(collection_result.status, "completed");
    assert_eq!(collection_result.metrics.len(), 4); // 2 targets * 2 metrics
    assert!(!collection_result.collection_logs.is_empty());
}

#[tokio::test]
async fn test_alert_system_initialization() {
    let alert_system = AlertSystem::new();
    let result = alert_system.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_alert_triggering() {
    let alert_system = AlertSystem::new();
    
    let config = AlertConfig {
        config_id: "test_config_456".to_string(),
        alert_type: "test_alert".to_string(),
        severity: "warning".to_string(),
        message: "Test alert message".to_string(),
        source: "test_source".to_string(),
        related_metrics: vec!["cpu_usage".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = alert_system.trigger_alert(config).await;
    assert!(result.is_ok());
    
    let alert_result = result.unwrap();
    assert_eq!(alert_result.status, "completed");
    assert_eq!(alert_result.alerts.len(), 1);
    assert!(!alert_result.processing_logs.is_empty());
}

#[tokio::test]
async fn test_alert_processor_initialization() {
    let processor = AlertProcessor::new();
    let result = processor.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_alert_processing() {
    let processor = AlertProcessor::new();
    
    let config = ProcessingConfig {
        config_id: "test_config_789".to_string(),
        processing_type: "classify".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
        related_alert_ids: vec!["alert_123".to_string(), "alert_456".to_string()],
    };
    
    let result = processor.process_alerts(config).await;
    assert!(result.is_ok());
    
    let processing_result = result.unwrap();
    assert_eq!(processing_result.status, "completed");
    assert_eq!(processing_result.processed_alerts.len(), 2);
    assert!(!processing_result.processing_logs.is_empty());
}

#[tokio::test]
async fn test_monitoring_automation_manager_initialization() {
    let manager = MonitoringAutomationManager::new();
    let result = manager.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_monitoring_automation_manager_metric_collection() {
    let manager = MonitoringAutomationManager::new();
    
    let config = CollectionConfig {
        config_id: "test_config_123".to_string(),
        collection_targets: vec!["target1".to_string()],
        metric_types: vec!["cpu_usage".to_string()],
        collection_frequency: 60,
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = manager.collect_metrics(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_monitoring_automation_manager_alert_triggering() {
    let manager = MonitoringAutomationManager::new();
    
    let config = AlertConfig {
        config_id: "test_config_456".to_string(),
        alert_type: "test_alert".to_string(),
        severity: "warning".to_string(),
        message: "Test alert message".to_string(),
        source: "test_source".to_string(),
        related_metrics: vec!["cpu_usage".to_string()],
        parameters: serde_json::json!({ "key": "value" }),
    };
    
    let result = manager.trigger_alert(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_monitoring_automation_manager_alert_processing() {
    let manager = MonitoringAutomationManager::new();
    
    let config = ProcessingConfig {
        config_id: "test_config_789".to_string(),
        processing_type: "classify".to_string(),
        parameters: serde_json::json!({ "key": "value" }),
        related_alert_ids: vec!["alert_123".to_string()],
    };
    
    let result = manager.process_alerts(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_alert_system_metric_evaluation() {
    let alert_system = AlertSystem::new();
    
    // Create test metrics
    let metrics = vec![
        metric_collection::Metric {
            name: "cpu_usage".to_string(),
            value: 90.0, // Above threshold
            unit: "%".to_string(),
            metric_type: "cpu_usage".to_string(),
            target: "target1".to_string(),
            timestamp: chrono::Utc::now().to_string(),
            tags: serde_json::json!({ "key": "value" }),
        },
        metric_collection::Metric {
            name: "memory_usage".to_string(),
            value: 70.0, // Below threshold
            unit: "%".to_string(),
            metric_type: "memory_usage".to_string(),
            target: "target1".to_string(),
            timestamp: chrono::Utc::now().to_string(),
            tags: serde_json::json!({ "key": "value" }),
        },
    ];
    
    let result = alert_system.evaluate_metrics(metrics).await;
    assert!(result.is_ok());
    
    let alerts = result.unwrap();
    // Should have at least one alert for cpu_usage
    assert!(!alerts.is_empty());
}

#[tokio::test]
async fn test_alert_processor_auto_processing() {
    let processor = AlertProcessor::new();
    
    // Create test alert
    let alert = alert_system::Alert {
        alert_id: "alert_123".to_string(),
        alert_type: "test_alert".to_string(),
        severity: "critical".to_string(),
        message: "Test alert message".to_string(),
        source: "test_source".to_string(),
        timestamp: chrono::Utc::now().to_string(),
        related_metrics: vec!["cpu_usage".to_string()],
        tags: serde_json::json!({ "key": "value" }),
        status: "active".to_string(),
    };
    
    let result = processor.auto_process_alert(alert).await;
    assert!(result.is_ok());
    
    let processed_alert = result.unwrap();
    assert_eq!(processed_alert.alert_id, "alert_123");
    assert_eq!(processed_alert.processing_status, "completed");
    assert!(!processed_alert.actions.is_empty());
}
