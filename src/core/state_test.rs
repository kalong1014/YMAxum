// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_app_state_creation() {
    let app_state = AppState::new();
    assert!(!app_state.version.is_empty());
    assert!(app_state.start_time > 0);
}

#[tokio::test]
async fn test_app_state_uptime() {
    let app_state = AppState::new();
    sleep(Duration::from_millis(100)).await;
    let uptime = app_state.uptime();
    assert!(uptime >= 0);
}

#[tokio::test]
async fn test_app_state_db_status() {
    let app_state = Arc::new(AppState::new());
    let db_status = app_state.get_db_status().await;
    assert!(db_status.contains("Not initialized"));
}

#[tokio::test]
async fn test_app_state_cache_status() {
    let app_state = Arc::new(AppState::new());
    let cache_status = app_state.get_cache_status().await;
    assert!(cache_status.contains("Not initialized"));
}

#[tokio::test]
async fn test_app_state_set_get_performance_monitor() {
    use crate::performance::monitor::PerformanceMonitor;
    
    let app_state = Arc::new(AppState::new());
    let monitor = PerformanceMonitor::new(app_state.clone()).unwrap();
    
    app_state.set_performance_monitor(monitor).await;
    let retrieved = app_state.get_performance_monitor().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_plugin_manager() {
    use crate::plugin::PluginManager;
    
    let app_state = Arc::new(AppState::new());
    let plugin_manager = PluginManager::new().unwrap();
    
    app_state.set_plugin_manager(plugin_manager).await;
    let retrieved = app_state.get_plugin_manager().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_plugin_marketplace() {
    use crate::plugin::market::PluginMarketplace;
    
    let app_state = Arc::new(AppState::new());
    let marketplace = PluginMarketplace::new();
    
    app_state.set_plugin_marketplace(marketplace).await;
    let retrieved = app_state.get_plugin_marketplace().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_log_manager() {
    use crate::ops::log::{LogConfig, LogManager};
    
    let app_state = Arc::new(AppState::new());
    let log_config = LogConfig {
        level: "info".to_string(),
        file_path: "logs/app.log".to_string(),
        enable_file: false,
        enable_console: true,
        max_file_size: 100,
        retain_days: 7,
        rotate_hours: 24,
        buffer_size: 1024,
        enable_structured: true,
    };
    let log_manager = LogManager::new(log_config);
    
    app_state.set_log_manager(log_manager).await;
    let retrieved = app_state.get_log_manager().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_monitor_service() {
    use crate::ops::monitor::{MonitorConfig, MonitorService};
    
    let app_state = Arc::new(AppState::new());
    let monitor_config = MonitorConfig {
        enabled: true,
        check_interval: 10,
        cpu_threshold: 80.0,
        memory_threshold: 80.0,
        disk_threshold: 80.0,
        network_threshold: 1000000,
        request_threshold: 1000,
        response_time_threshold: 500,
        error_threshold: 100,
        cs_response_delay_threshold: 1000,
        im_message_delay_threshold: 1000,
        database_connections_threshold: 100,
        cache_hit_rate_threshold: 80.0,
        enable_popup: false,
        enable_error_log: true,
        enable_monitor_api: true,
        enable_email_notifications: false,
        email_recipients: vec![],
    };
    let monitor_service = MonitorService::new(monitor_config);
    
    app_state.set_monitor_service(monitor_service).await;
    let retrieved = app_state.get_monitor_service().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_config_hot_update_service() {
    use crate::ops::config_hot::{ConfigHotUpdateConfig, ConfigHotUpdateService};
    
    let app_state = Arc::new(AppState::new());
    let config = ConfigHotUpdateConfig {
        config_file_path: "config.txt".to_string(),
        check_interval: 5,
        enabled: true,
    };
    let service = ConfigHotUpdateService::new(config);
    
    app_state.set_config_hot_update_service(service).await;
    let retrieved = app_state.get_config_hot_update_service().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_fault_handling_manager() {
    use crate::ops::fault_handling::{FaultHandlingConfig, FaultHandlingManager};
    
    let app_state = Arc::new(AppState::new());
    let config = FaultHandlingConfig {
        detection_interval: 30,
        max_fault_history: 1000,
        auto_fix: true,
        max_fix_attempts: 3,
        severity_threshold: crate::ops::fault_handling::SeverityLevel::Medium,
    };
    let manager = FaultHandlingManager::with_config(config);
    
    app_state.set_fault_handling_manager(manager).await;
    let retrieved = app_state.get_fault_handling_manager().await;
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_app_state_set_get_iterate_service() {
    use crate::iterate::IterateService;
    
    let app_state = Arc::new(AppState::new());
    let service = IterateService::new();
    
    app_state.set_iterate_service(service).await;
    let retrieved = app_state.get_iterate_service().await;
    assert!(retrieved.is_some());
}
