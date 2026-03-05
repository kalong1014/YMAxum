// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改


use ymaxum::config_management::version_management::{VersionConfig, VersionManager, VersionDiff};
use ymaxum::config_management::change_audit::{AuditConfig, ChangeAuditor};
use ymaxum::core::config::loader::{BaseConfig, InheritanceStrategy};
use ymaxum::core::config::{Config, ConfigSource};

#[tokio::test]
async fn test_config_version_control() {
    // 测试配置版本控制功能
    let version_manager = VersionManager::new();
    version_manager.initialize().await.unwrap();

    // 创建测试配置文件
    let test_config = r#"
[server]
port = 8080
host = "localhost"
"#;
    std::fs::write("config/test_version.toml", test_config).unwrap();

    // 创建版本配置
    let version_config = VersionConfig {
        config_id: "test_config_1".to_string(),
        operation_type: "create".to_string(),
        version: "1.0.0".to_string(),
        config_file_path: "config/test_version.toml".to_string(),
        operation_description: "Initial config version".to_string(),
        parameters: serde_json::json!({
            "tags": ["initial", "production"]
        }),
    };

    // 管理版本
    let result = version_manager.manage_version(version_config).await.unwrap();
    assert_eq!(result.status, "completed");
    assert!(!result.result_id.is_empty());
    assert!(!result.version_info.tags.is_empty());

    // 更新配置文件
    let updated_config = r#"
[server]
port = 8081
host = "localhost"
[database]
url = "postgres://localhost:5432/db"
"#;
    std::fs::write("config/test_version.toml", updated_config).unwrap();

    // 创建新版本
    let version_config2 = VersionConfig {
        config_id: "test_config_2".to_string(),
        operation_type: "update".to_string(),
        version: "1.1.0".to_string(),
        config_file_path: "config/test_version.toml".to_string(),
        operation_description: "Updated config version".to_string(),
        parameters: serde_json::json!({
            "tags": ["updated", "production"]
        }),
    };

    let result2 = version_manager.manage_version(version_config2).await.unwrap();
    assert_eq!(result2.status, "completed");

    // 比较版本
    let diff: VersionDiff = version_manager.compare_versions("1.0.0".to_string(), "1.1.0".to_string(), "config/test_version.toml".to_string()).await.unwrap();
    assert!(!diff.added.is_empty() || !diff.modified.is_empty());

    // 回滚到版本
    let rollback_result = version_manager.rollback_to_version("1.0.0".to_string(), "config/test_version.toml".to_string()).await.unwrap();
    assert_eq!(rollback_result.status, "completed");

    // 清理测试文件
    std::fs::remove_file("config/test_version.toml").unwrap();
}

#[tokio::test]
async fn test_config_inheritance() {
    // 测试配置继承功能
    let mut parent_config = BaseConfig::new();
    parent_config.set("server.port", 8080).unwrap();
    parent_config.set("server.host", "localhost").unwrap();
    parent_config.set("database.url", "postgres://localhost:5432/db").unwrap();

    let mut parent_config2 = BaseConfig::new();
    parent_config2.set("database.url", "mysql://localhost:3306/db").unwrap();
    parent_config2.set("redis.url", "redis://localhost:6379").unwrap();

    let mut child_config = BaseConfig::new();
    child_config.set("server.port", 8081).unwrap();
    child_config.add_parent(parent_config);
    child_config.add_parent(parent_config2);

    // 测试优先级继承策略
    let port: u16 = child_config.get("server.port").unwrap();
    assert_eq!(port, 8081); // 子配置优先

    let host: String = child_config.get("server.host").unwrap();
    assert_eq!(host, "localhost"); // 从父配置继承

    let db_url: String = child_config.get("database.url").unwrap();
    assert_eq!(db_url, "postgres://localhost:5432/db"); // 第一个父配置优先

    let redis_url: String = child_config.get("redis.url").unwrap();
    assert_eq!(redis_url, "redis://localhost:6379"); // 从第二个父配置继承

    // 测试合并继承策略
    let mut parent_config3 = BaseConfig::new();
    parent_config3.set("database.url", "postgres://localhost:5432/db").unwrap();
    parent_config3.set("database.pool_size", 10).unwrap();

    let mut parent_config4 = BaseConfig::new();
    parent_config4.set("database.pool_size", 20).unwrap();
    parent_config4.set("redis.url", "redis://localhost:6379").unwrap();

    let mut child_config2 = BaseConfig::new();
    child_config2.set_inheritance_strategy(InheritanceStrategy::Merge);
    child_config2.add_parent(parent_config3);
    child_config2.add_parent(parent_config4);

    let db_url2: String = child_config2.get("database.url").unwrap();
    assert_eq!(db_url2, "postgres://localhost:5432/db");

    let pool_size: u32 = child_config2.get("database.pool_size").unwrap();
    assert_eq!(pool_size, 20); // 第二个父配置覆盖第一个

    let redis_url2: String = child_config2.get("redis.url").unwrap();
    assert_eq!(redis_url2, "redis://localhost:6379");
}

#[tokio::test]
async fn test_config_change_audit() {
    // 测试配置变更审计功能
    let change_auditor = ChangeAuditor::new();
    change_auditor.initialize().await.unwrap();

    // 创建测试配置文件
    let test_config = r#"
[server]
port = 8080
host = "localhost"
"#;
    std::fs::write("config/test_audit.toml", test_config).unwrap();

    // 创建审计配置
    let audit_config = AuditConfig {
        config_id: "test_audit_1".to_string(),
        config_file_path: "config/test_audit.toml".to_string(),
        change_type: "update".to_string(),
        change_content: serde_json::json!({
            "server.port": 8081
        }),
        changed_by: "test_user".to_string(),
        parameters: serde_json::json!({}),
        version_id: Some("1.0.0".to_string()),
    };

    // 审计变更
    let result = change_auditor.audit_change(audit_config).await.unwrap();
    assert_eq!(result.status, "completed");
    assert!(!result.result_id.is_empty());
    assert!(!result.audit_logs.is_empty());
    assert!(!result.audit_suggestions.is_empty());

    // 回滚变更
    let rollback_result = change_auditor.rollback_change(result.audit_logs[0].log_id.clone(), "config/test_audit.toml".to_string()).await.unwrap();
    assert_eq!(rollback_result.status, "completed");
    assert!(!rollback_result.result_id.is_empty());

    // 清理测试文件
    std::fs::remove_file("config/test_audit.toml").unwrap();
}

#[tokio::test]
async fn test_config_cache() {
    // 测试配置缓存功能
    let mut config = BaseConfig::new();
    config.set("server.port", 8080).unwrap();
    config.set("server.host", "localhost").unwrap();

    // 启用缓存
    config.set_cache_enabled(true);
    assert!(config.is_cache_enabled());

    // 第一次获取（应该缓存）
    let port1: u16 = config.get("server.port").unwrap();
    assert_eq!(port1, 8080);

    // 检查缓存大小
    assert_eq!(config.get_cache_size(), 1);

    // 第二次获取（应该从缓存获取）
    let port2: u16 = config.get("server.port").unwrap();
    assert_eq!(port2, 8080);

    // 清除缓存
    config.clear_cache();
    assert_eq!(config.get_cache_size(), 0);

    // 禁用缓存
    config.set_cache_enabled(false);
    assert!(!config.is_cache_enabled());
}

#[tokio::test]
async fn test_config_async_load() {
    // 测试异步加载功能
    let mut config = BaseConfig::new();
    
    // 创建测试配置文件
    let test_config = r#"
[server]
port = 8080
host = "localhost"
"#;
    
    std::fs::write("config/test_async.toml", test_config).unwrap();
    
    // 异步加载
    let source = ConfigSource::File("config/test_async.toml".to_string());
    config.load(source).unwrap();
    
    // 验证加载结果
    let port: u16 = config.get("server.port").unwrap();
    assert_eq!(port, 8080);
    
    let host: String = config.get("server.host").unwrap();
    assert_eq!(host, "localhost");
    
    // 清理测试文件
    std::fs::remove_file("config/test_async.toml").unwrap();
}

#[tokio::test]
async fn test_config_parallel_load() {
    // 测试并行加载功能
    let mut config = BaseConfig::new();
    
    // 创建多个测试配置文件
    let test_config1 = r#"
[server]
port = 8080
host = "localhost"
"#;
    
    let test_config2 = r#"
[database]
url = "postgres://localhost:5432/db"
pool_size = 10
"#;
    
    std::fs::write("config/test_parallel1.toml", test_config1).unwrap();
    std::fs::write("config/test_parallel2.toml", test_config2).unwrap();
    
    // 并行加载
    let sources = vec![
        ConfigSource::File("config/test_parallel1.toml".to_string()),
        ConfigSource::File("config/test_parallel2.toml".to_string())
    ];
    
    config.parallel_load(sources).await.unwrap();
    
    // 验证加载结果
    let port: u16 = config.get("server.port").unwrap();
    assert_eq!(port, 8080);
    
    let db_url: String = config.get("database.url").unwrap();
    assert_eq!(db_url, "postgres://localhost:5432/db");
    
    // 清理测试文件
    std::fs::remove_file("config/test_parallel1.toml").unwrap();
    std::fs::remove_file("config/test_parallel2.toml").unwrap();
}
