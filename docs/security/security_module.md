# YMAxum 框架安全模块文档

## 1. 概述

YMAxum 框架的安全模块提供了全面的安全扫描和入侵检测功能，旨在保护应用程序免受各种安全威胁。本模块包含以下核心组件：

- **安全扫描器**：检测应用程序中的漏洞
- **入侵检测引擎**：实时监控和检测入侵行为
- **漏洞数据库**：管理和更新漏洞信息

## 2. 核心功能

### 2.1 安全扫描器

安全扫描器能够检测多种类型的安全漏洞，包括：

- **传统漏洞**：SQL注入、XSS、CSRF等
- **新兴威胁**：AI生成恶意代码、容器逃逸、云服务配置错误、API滥用等
- **基础设施漏洞**：服务器安全、网络安全、依赖项漏洞等

#### 2.1.1 扫描配置

```rust
use ymaxum::security::{SecurityScanConfig, ScanType};

let config = SecurityScanConfig {
    scan_types: vec![
        ScanType::SqlInjection,
        ScanType::Xss,
        ScanType::AiGeneratedMaliciousCode, // 新：AI生成恶意代码检测
        ScanType::ContainerEscape, // 新：容器逃逸检测
        ScanType::CloudServiceMisconfiguration, // 新：云服务配置错误检测
        ScanType::ApiAbuse, // 新：API滥用检测
    ],
    scan_scope: ScanScope::Full,
    enable_deep_scan: true,
    scan_timeout: 300,
};
```

#### 2.1.2 执行扫描

```rust
use ymaxum::security::SecurityScannerCore;

let mut scanner = SecurityScannerCore::new(config);
let stats = scanner.scan("http://example.com");

println!("扫描完成，发现 {} 个漏洞", stats.vulnerabilities_found);
println!("严重漏洞: {}, 高危漏洞: {}, 中危漏洞: {}", 
         stats.critical_count, stats.high_count, stats.medium_count);
```

#### 2.1.3 增量扫描

安全扫描器支持增量扫描，通过只扫描高风险漏洞类型来提高性能：

```rust
// 第一次扫描（完整扫描）
let first_stats = scanner.scan("http://example.com");

// 第二次扫描（增量扫描，更快）
let second_stats = scanner.scan("http://example.com");
```

### 2.2 入侵检测引擎

入侵检测引擎能够实时监控和检测入侵行为，包括：

- **暴力破解检测**：检测针对登录页面的暴力破解尝试
- **DoS攻击检测**：检测拒绝服务攻击
- **异常行为检测**：基于机器学习的行为分析
- **智能阈值调整**：根据行为模式自动调整检测阈值

#### 2.2.1 检测配置

```rust
use ymaxum::security::{IntrusionDetectionConfig, DetectionRule};

let config = IntrusionDetectionConfig {
    enabled: true,
    detection_rules: vec![
        DetectionRule::BruteForce,
        DetectionRule::DoSAttempt,
        DetectionRule::AnomalyDetection,
        DetectionRule::AiGeneratedMaliciousCodeAttempt, // 新：AI生成恶意代码尝试检测
        DetectionRule::ContainerEscapeAttempt, // 新：容器逃逸尝试检测
    ],
    alert_threshold: 5,
    detection_window: 3600,
};
```

#### 2.2.2 检测入侵

```rust
use ymaxum::security::IntrusionDetectionEngine;

let mut engine = IntrusionDetectionEngine::new(config);

let mut details = std::collections::HashMap::new();
details.insert("user_agent".to_string(), "Mozilla/5.0".to_string());

engine.detect_intrusion(
    "192.168.1.1",
    "/login",
    DetectionRule::BruteForce,
    details
);

// 检查检测结果
let events = engine.get_intrusion_events();
let stats = engine.get_intrusion_stats();

println!("检测到 {} 个入侵事件", stats.total_events);
println!("可疑IP数量: {}", stats.suspicious_ips_count);
```

### 2.3 漏洞数据库

漏洞数据库管理和更新漏洞信息，支持：

- **多源更新**：从多个来源自动更新漏洞信息
- **漏洞优先级排序**：基于风险评分对漏洞进行排序
- **漏洞趋势分析**：分析漏洞的时间分布趋势
- **组件漏洞映射**：跟踪每个组件的漏洞情况

#### 2.3.1 数据库配置

```rust
use ymaxum::security::VulnerabilityDbConfig;

let config = VulnerabilityDbConfig {
    db_path: "./vulnerability_db.json".to_string(),
    update_interval: 24, // 小时
    enable_auto_update: true,
    update_sources: vec![
        // 默认包含 NVD、CVE Details、OSS Index
    ],
    enable_multi_source: true,
    classification_settings: Default::default(),
};
```

#### 2.3.2 使用漏洞数据库

```rust
use ymaxum::security::VulnerabilityDatabase;

let mut db = VulnerabilityDatabase::new(config);

// 更新数据库
tokio_test::block_on(async {
    db.update().await.unwrap();
});

// 获取优先级排序的漏洞
let prioritized = db.get_prioritized_vulnerabilities();
println!("优先级最高的漏洞: {}", prioritized[0].name);

// 获取漏洞趋势
let trends = db.get_vulnerability_trends();
println!("漏洞趋势: {:?}", trends);

// 获取组件的漏洞
let component_vulns = db.get_prioritized_vulnerabilities_by_component("login.php");
println!("login.php 的漏洞数量: {}", component_vulns.len());

// 生成漏洞报告
let report = db.generate_report();
println!("{}", report);
```

## 3. 性能优化

安全模块经过优化，提供以下性能特性：

- **并行扫描**：使用线程池并行执行扫描任务
- **增量扫描**：只扫描高风险漏洞类型，提高扫描速度
- **智能缓存**：缓存扫描结果，避免重复扫描
- **误报过滤**：基于历史数据和上下文减少误报

## 4. 集成使用

### 4.1 与Web框架集成

```rust
use ymaxum::App;
use ymaxum::security::{SecurityScanner, SecurityScanConfig, IntrusionDetectionConfig};

// 创建安全扫描器
let scanner = SecurityScanner::builder()
    .with_scan_config(SecurityScanConfig::default())
    .with_intrusion_config(IntrusionDetectionConfig::default())
    .build();

// 创建应用
let app = App::new()
    .with_security_scanner(scanner)
    .build();

// 启动应用
app.run("127.0.0.1:3000").await;
```

### 4.2 自定义安全规则

```rust
// 自定义扫描配置
let mut scan_config = SecurityScanConfig::default();
// 添加特定的扫描类型
scan_config.scan_types.push(ScanType::AiGeneratedMaliciousCode);
scan_config.scan_types.push(ScanType::ContainerEscape);

// 自定义入侵检测配置
let mut intrusion_config = IntrusionDetectionConfig::default();
// 添加特定的检测规则
intrusion_config.detection_rules.push(DetectionRule::AiGeneratedMaliciousCodeAttempt);
intrusion_config.detection_rules.push(DetectionRule::ContainerEscapeAttempt);

// 创建安全扫描器
let scanner = SecurityScanner::builder()
    .with_scan_config(scan_config)
    .with_intrusion_config(intrusion_config)
    .build();
```

## 5. 最佳实践

1. **定期扫描**：定期执行安全扫描，及时发现和修复漏洞
2. **实时监控**：启用入侵检测引擎，实时监控可疑行为
3. **自动更新**：启用漏洞数据库自动更新，保持漏洞信息最新
4. **分层防御**：结合多种安全措施，构建多层次防御体系
5. **安全培训**：定期对开发人员进行安全培训，提高安全意识

## 6. 故障排除

### 6.1 常见问题

| 问题 | 可能原因 | 解决方案 |
|------|---------|----------|
| 扫描速度慢 | 扫描范围过大 | 使用增量扫描或缩小扫描范围 |
| 误报过多 | 检测规则过于敏感 | 调整检测阈值或使用误报过滤 |
| 漏报 | 扫描类型不完整 | 添加更多扫描类型 |
| 数据库更新失败 | 网络连接问题 | 检查网络连接或手动更新 |

### 6.2 日志和监控

安全模块会生成详细的日志，记录扫描结果和检测到的入侵事件。建议：

- 配置日志系统，确保安全事件得到及时记录
- 设置监控告警，当检测到严重漏洞或入侵时及时通知
- 定期分析安全日志，发现潜在的安全趋势

## 7. 版本历史

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| 1.0.0 | 2024-01-01 | 初始版本 |
| 1.1.0 | 2024-07-01 | 增加新的攻击类型检测规则 |
| 1.2.0 | 2024-07-15 | 实现智能异常检测算法 |
| 1.3.0 | 2024-07-30 | 增强漏洞数据库管理和更新机制 |
| 1.4.0 | 2024-08-15 | 优化安全扫描器性能和准确性 |

## 8. 总结

YMAxum 框架的安全模块提供了全面的安全防护功能，包括漏洞扫描、入侵检测和漏洞管理。通过持续更新和优化，该模块能够有效应对不断演变的安全威胁，为应用程序提供强大的安全保障。

建议开发者在开发过程中充分利用安全模块的功能，构建更加安全可靠的应用程序。