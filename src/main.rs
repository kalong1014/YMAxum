// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use clap::{Arg, Command};
use log::{error, info};

// 导入命令模块
use ymaxum::command::doc_generator::DocGeneratorCommand;
use ymaxum::command::performance_optimizer::PerformanceOptimizerCommand;
use ymaxum::command::security_scanner::SecurityScannerCommand;
use ymaxum::command::test_generator::TestGeneratorCommand;
use ymaxum::command::version_manager::VersionManagerCommand;

use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;

// 导入核心模块

use ymaxum::core::deps::{DependencyManager, DependencyType, check_dependency_versions};
use ymaxum::core::iterate_api::IterateApi;
use ymaxum::core::middleware::{
    RateLimiter, cors_middleware, exception_catch_middleware, logger_middleware,
};
use ymaxum::core::state::AppState;

// 导入业务模块
use ymaxum::fraud::FraudModule;
use ymaxum::points::PointsModule;
use ymaxum::referral::ReferralModule;
use ymaxum::rights::RightsModule;
use ymaxum::user::UserModule;

// 导入安全模块
// use ymaxum::security::{EncryptionService, DesensitizationService, HttpsService, EncryptConfig, HttpsConfig};

// 导入运维管理模块
use ymaxum::ops::config_hot::{ConfigHotUpdateConfig, ConfigHotUpdateService};
use ymaxum::ops::fault_handling::{FaultHandlingConfig, FaultHandlingManager};
use ymaxum::ops::log::{LogConfig, LogManager};
use ymaxum::ops::monitor::{MonitorConfig, MonitorService};

// 导入替代测试架构模块
use ymaxum::iterate::IterateService;

// 导入插件管理模块
use ymaxum::plugin::PluginManager;
use ymaxum::plugin::market::PluginMarketplace;

// 导入场景模块
use ymaxum::scene::SceneManager;
use ymaxum::scene::game::adapter::GameScene;
use ymaxum::scene::mall::adapter::MallSceneAdapter;
use ymaxum::scene::newbie::NewbieScene;
use ymaxum::scene::saas::adapter::SaasScene;
use ymaxum::scene::social::adapter::SocialScene;

// 导入性能测试模块
use ymaxum::performance::run_default_benchmark;
// 导入性能监控模块
use ymaxum::performance::monitor::PerformanceMonitor;
// 导入告警模块
use ymaxum::performance::alert::AlertManager;

// 导入Axum路由模块
use axum::{Router, extract::State, middleware::from_fn, response::Json, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let matches = Command::new("YMAxum Framework")
        .version("0.1.0")
        .about("YMAxum Framework with performance testing capabilities")
        .subcommand(
            Command::new("benchmark")
                .about("Run performance benchmarks instead of starting the server"),
        )
        .subcommand(
            Command::new("generate-tests")
                .about("Generate automated test cases")
                .arg(
                    Arg::new("code-path")
                        .short('p')
                        .long("code-path")
                        .help("Path to the code to analyze")
                        .default_value("./src"),
                )
                .arg(
                    Arg::new("output-dir")
                        .short('o')
                        .long("output-dir")
                        .help("Directory to output generated tests")
                        .default_value("./tests/generated"),
                )
                .arg(
                    Arg::new("coverage-target")
                        .short('c')
                        .long("coverage-target")
                        .help("Target test coverage")
                        .default_value("0.8"),
                )
                .arg(
                    Arg::new("unit-tests")
                        .short('u')
                        .long("unit-tests")
                        .help("Generate unit tests")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("integration-tests")
                        .short('i')
                        .long("integration-tests")
                        .help("Generate integration tests")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("e2e-tests")
                        .short('e')
                        .long("e2e-tests")
                        .help("Generate end-to-end tests")
                        .default_value("false"),
                )
                .arg(
                    Arg::new("use-ai")
                        .short('a')
                        .long("use-ai")
                        .help("Use AI assistance for test generation")
                        .default_value("false"),
                )
                .arg(
                    Arg::new("ai-model")
                        .short('m')
                        .long("ai-model")
                        .help("AI model to use for test generation")
                        .default_value("gpt-3.5-turbo"),
                ),
        )
        .subcommand(
            Command::new("optimize-performance")
                .about("Analyze and optimize performance")
                .arg(
                    Arg::new("mode")
                        .short('m')
                        .long("mode")
                        .help("Analysis mode")
                        .default_value("comprehensive"),
                )
                .arg(
                    Arg::new("output-dir")
                        .short('o')
                        .long("output-dir")
                        .help("Directory to output results")
                        .default_value("./performance_results"),
                )
                .arg(
                    Arg::new("depth")
                        .short('d')
                        .long("depth")
                        .help("Analysis depth")
                        .default_value("medium"),
                )
                .arg(
                    Arg::new("run-benchmarks")
                        .short('b')
                        .long("run-benchmarks")
                        .help("Run benchmarks")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("generate-suggestions")
                        .short('s')
                        .long("generate-suggestions")
                        .help("Generate optimization suggestions")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("apply-optimizations")
                        .short('a')
                        .long("apply-optimizations")
                        .help("Apply automatic optimizations")
                        .default_value("false"),
                ),
        )
        .subcommand(
            Command::new("scan-security")
                .about("Scan for security vulnerabilities")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .help("Target to scan")
                        .default_value("."),
                )
                .arg(
                    Arg::new("output-dir")
                        .short('o')
                        .long("output-dir")
                        .help("Directory to output results")
                        .default_value("./security_results"),
                )
                .arg(
                    Arg::new("scan-type")
                        .short('s')
                        .long("scan-type")
                        .help("Scan type")
                        .default_value("comprehensive"),
                )
                .arg(
                    Arg::new("deep-scan")
                        .short('d')
                        .long("deep-scan")
                        .help("Enable deep scan")
                        .default_value("false"),
                )
                .arg(
                    Arg::new("timeout")
                        .short('T')
                        .long("timeout")
                        .help("Timeout in seconds")
                        .default_value("300"),
                )
                .arg(
                    Arg::new("output-format")
                        .short('f')
                        .long("output-format")
                        .help("Output format")
                        .default_value("all"),
                )
                .arg(
                    Arg::new("min-severity")
                        .short('m')
                        .long("min-severity")
                        .help("Minimum severity level")
                        .default_value("low"),
                ),
        )
        .subcommand(
            Command::new("generate-docs")
                .about("Generate project documentation")
                .arg(
                    Arg::new("project-path")
                        .short('p')
                        .long("project-path")
                        .help("Project root path")
                        .default_value("./"),
                )
                .arg(
                    Arg::new("output-dir")
                        .short('o')
                        .long("output-dir")
                        .help("Directory to output documentation")
                        .default_value("./docs/generated"),
                )
                .arg(
                    Arg::new("api-docs")
                        .short('a')
                        .long("api-docs")
                        .help("Generate API documentation")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("user-docs")
                        .short('u')
                        .long("user-docs")
                        .help("Generate user documentation")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("architecture-docs")
                        .short('c')
                        .long("architecture-docs")
                        .help("Generate architecture documentation")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("api-reference")
                        .short('r')
                        .long("api-reference")
                        .help("Generate API reference")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("include-examples")
                        .short('e')
                        .long("include-examples")
                        .help("Include example code")
                        .default_value("true"),
                )
                .arg(
                    Arg::new("use-ai")
                        .short('i')
                        .long("use-ai")
                        .help("Use AI assistance for documentation generation")
                        .default_value("false"),
                )
                .arg(
                    Arg::new("ai-model")
                        .short('m')
                        .long("ai-model")
                        .help("AI model to use")
                        .default_value("gpt-3.5-turbo"),
                ),
        )
        .subcommand(
            Command::new("version")
                .about("Manage project version")
                .arg(
                    Arg::new("command")
                        .short('c')
                        .long("command")
                        .help("Command operation: bump, release, changelog, show")
                        .default_value("show"),
                )
                .arg(
                    Arg::new("part")
                        .short('p')
                        .long("part")
                        .help("Version part: major, minor, patch")
                        .default_value("patch"),
                )
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .help("Version number"),
                )
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("Release message")
                        .default_value(""),
                )
                .arg(
                    Arg::new("changelog")
                        .short('l')
                        .long("changelog")
                        .help("Changelog file path")
                        .default_value("./RELEASE_NOTES.md"),
                )
                .arg(
                    Arg::new("cargo-toml")
                        .short('t')
                        .long("cargo-toml")
                        .help("Cargo.toml file path")
                        .default_value("./Cargo.toml"),
                )
                .arg(
                    Arg::new("auto-commit")
                        .short('a')
                        .long("auto-commit")
                        .help("Auto commit changes")
                        .default_value("false"),
                )
                .arg(
                    Arg::new("prerelease")
                        .short('r')
                        .long("prerelease")
                        .help("Prerelease identifier")
                        .default_value(""),
                ),
        )
        .get_matches();

    // 处理子命令
    match matches.subcommand() {
        Some(("benchmark", _)) => {
            info!("Running performance benchmarks...");
            run_default_benchmark().await;
            info!("Performance benchmarks completed.");
            return Ok(());
        }
        Some(("generate-tests", sub_matches)) => {
            info!("Generating automated test cases...");
            let command = TestGeneratorCommand {
                code_path: sub_matches.get_one::<String>("code-path").unwrap().clone(),
                output_dir: sub_matches.get_one::<String>("output-dir").unwrap().clone(),
                coverage_target: sub_matches
                    .get_one::<String>("coverage-target")
                    .unwrap()
                    .parse()
                    .unwrap(),
                unit_tests: sub_matches
                    .get_one::<String>("unit-tests")
                    .unwrap()
                    .parse()
                    .unwrap(),
                integration_tests: sub_matches
                    .get_one::<String>("integration-tests")
                    .unwrap()
                    .parse()
                    .unwrap(),
                e2e_tests: sub_matches
                    .get_one::<String>("e2e-tests")
                    .unwrap()
                    .parse()
                    .unwrap(),
                use_ai: sub_matches
                    .get_one::<String>("use-ai")
                    .unwrap()
                    .parse()
                    .unwrap(),
                ai_model: sub_matches.get_one::<String>("ai-model").unwrap().clone(),
            };
            command.execute().await?;
            info!("Test generation completed.");
            return Ok(());
        }
        Some(("optimize-performance", sub_matches)) => {
            info!("Analyzing and optimizing performance...");
            let command = PerformanceOptimizerCommand {
                mode: sub_matches.get_one::<String>("mode").unwrap().clone(),
                output_dir: sub_matches.get_one::<String>("output-dir").unwrap().clone(),
                depth: sub_matches.get_one::<String>("depth").unwrap().clone(),
                run_benchmarks: sub_matches
                    .get_one::<String>("run-benchmarks")
                    .unwrap()
                    .parse()
                    .unwrap(),
                generate_suggestions: sub_matches
                    .get_one::<String>("generate-suggestions")
                    .unwrap()
                    .parse()
                    .unwrap(),
                apply_optimizations: sub_matches
                    .get_one::<String>("apply-optimizations")
                    .unwrap()
                    .parse()
                    .unwrap(),
            };
            command.execute().await?;
            info!("Performance optimization completed.");
            return Ok(());
        }
        Some(("scan-security", sub_matches)) => {
            info!("Scanning for security vulnerabilities...");
            let command = SecurityScannerCommand {
                name: "scan-security".to_string(),
                target: sub_matches.get_one::<String>("target").unwrap().clone(),
                scan_types: Some(vec![sub_matches.get_one::<String>("scan-type").unwrap().clone()]),
                deep_scan: sub_matches.get_one::<String>("deep-scan").map(|s| s.parse().unwrap_or(false)),
                timeout: sub_matches.get_one::<String>("timeout").map(|s| s.parse().unwrap_or(300)),
                enable_intrusion_detection: Some(true),
            };
            let result = command.execute();
            match result {
                ymaxum::command::executor::ExecuteResult::Success { message, data: _ } => {
                    info!("{}", message);
                }
                ymaxum::command::executor::ExecuteResult::Failure { message, error_code, line } => {
                    error!("Error: {}", message);
                    return Err(
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Command failed with error code {} at line {}", error_code, line)
                        )) as Box<dyn std::error::Error>
                    );
                }
            }
            info!("Security scan completed.");
            return Ok(());
        }
        Some(("generate-docs", sub_matches)) => {
            info!("Generating project documentation...");
            let command = DocGeneratorCommand {
                project_path: sub_matches
                    .get_one::<String>("project-path")
                    .unwrap()
                    .clone(),
                output_dir: sub_matches.get_one::<String>("output-dir").unwrap().clone(),
                api_docs: sub_matches
                    .get_one::<String>("api-docs")
                    .unwrap()
                    .parse()
                    .unwrap(),
                user_docs: sub_matches
                    .get_one::<String>("user-docs")
                    .unwrap()
                    .parse()
                    .unwrap(),
                architecture_docs: sub_matches
                    .get_one::<String>("architecture-docs")
                    .unwrap()
                    .parse()
                    .unwrap(),
                api_reference: sub_matches
                    .get_one::<String>("api-reference")
                    .unwrap()
                    .parse()
                    .unwrap(),
                include_examples: sub_matches
                    .get_one::<String>("include-examples")
                    .unwrap()
                    .parse()
                    .unwrap(),
                use_ai: sub_matches
                    .get_one::<String>("use-ai")
                    .unwrap()
                    .parse()
                    .unwrap(),
                ai_model: sub_matches.get_one::<String>("ai-model").unwrap().clone(),
            };
            command.execute().await?;
            info!("Documentation generation completed.");
            return Ok(());
        }
        Some(("version", sub_matches)) => {
            info!("Managing project version...");
            let command = VersionManagerCommand {
                command: sub_matches.get_one::<String>("command").unwrap().clone(),
                part: sub_matches.get_one::<String>("part").unwrap().clone(),
                version: sub_matches.get_one::<String>("version").cloned(),
                message: sub_matches.get_one::<String>("message").unwrap().clone(),
                changelog: sub_matches.get_one::<String>("changelog").unwrap().clone(),
                cargo_toml: sub_matches.get_one::<String>("cargo-toml").unwrap().clone(),
                auto_commit: sub_matches
                    .get_one::<String>("auto-commit")
                    .unwrap()
                    .parse()
                    .unwrap(),
                prerelease: sub_matches.get_one::<String>("prerelease").unwrap().clone(),
            };
            command.execute().await?;
            info!("Version management completed.");
            return Ok(());
        }
        _ => {
            // 没有指定子命令，启动服务器
        }
    }

    // 检查依赖版本
    check_dependency_versions()?;

    // 创建全局状态
    let app_state = Arc::new(AppState::new());

    // 初始化性能监控器
    let performance_monitor = PerformanceMonitor::new(app_state.clone()).unwrap();
    app_state
        .set_performance_monitor(performance_monitor.clone())
        .await;

    // 初始化告警管理器
    let alert_manager = AlertManager::new(Arc::new(performance_monitor.clone()));
    // 启动告警监控任务（每10秒检查一次）
    alert_manager
        .start_alert_monitoring(std::time::Duration::from_secs(10))
        .await;
    info!("AlertManager initialized and monitoring started");

    // 初始化运维管理服务
    // 1. 日志管理器（启动前初始化，后续启动：记录时间）
    let log_config = LogConfig {
        level: "info".to_string(),
        file_path: String::from("logs/app.log"),
        enable_file: true,
        enable_console: true,
        max_file_size: 100,
        retain_days: 7,
        rotate_hours: 24,
        buffer_size: 1024,
        enable_structured: true,
    };
    let log_manager = LogManager::new(log_config.clone());
    app_state.set_log_manager(log_manager.clone()).await;
    // 启动日志管理器
    log_manager.start().await?;

    // 日志管理器初始化完成，使用info!宏
    info!("LogManager initialized with config: {:?}", log_config);

    // 2. 配置热更新服务
    let config_hot_config = ConfigHotUpdateConfig {
        config_file_path: String::from("config.txt"),
        check_interval: 5,
        enabled: true,
    };
    let config_hot_service = ConfigHotUpdateService::new(config_hot_config.clone());
    app_state
        .set_config_hot_update_service(config_hot_service.clone())
        .await;
    // 启动配置热更新服务
    config_hot_service.start().await?;
    info!(
        "ConfigHotUpdateService initialized with config: {:?}",
        config_hot_config
    );

    // 3. 监控服务
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
    let monitor_service = MonitorService::new(monitor_config.clone());
    app_state.set_monitor_service(monitor_service.clone()).await;
    // 启动监控服务
    monitor_service.start().await?;
    info!(
        "MonitorService initialized with config: {:?}",
        monitor_config
    );

    // 4. 故障处理服务
    let fault_handling_config = FaultHandlingConfig {
        detection_interval: 30,
        max_fault_history: 1000,
        auto_fix: true,
        max_fix_attempts: 3,
        severity_threshold: ymaxum::ops::fault_handling::SeverityLevel::Medium,
    };
    let fault_handling_manager = FaultHandlingManager::with_config(fault_handling_config.clone());
    app_state
        .set_fault_handling_manager(fault_handling_manager.clone())
        .await;
    // 启动故障处理服务
    Arc::new(fault_handling_manager).start().await?;
    info!(
        "FaultHandlingManager initialized with config: {:?}",
        fault_handling_config
    );

    // 4. 替代测试架构迭代服务
    let iterate_service = IterateService::new();
    app_state.set_iterate_service(iterate_service.clone()).await;
    info!("IterateService initialized");

    // 5. 插件管理器
    let plugin_manager = PluginManager::new().unwrap();
    app_state.set_plugin_manager(plugin_manager).await;
    info!("PluginManager initialized");

    // 6. 插件市场管理器
    let plugin_marketplace = PluginMarketplace::new();
    app_state
        .set_plugin_marketplace(plugin_marketplace.clone())
        .await;
    info!("PluginMarketplace initialized");

    // 7. 国际化管理器
    info!("Initializing internationalization manager...");
    let language_pack_dir = Path::new("./i18n");
    info!("Language pack directory: {:?}", language_pack_dir);

    // 克隆app_state，避免移动
    let app_state_clone = app_state.clone();

    // 异步初始化国际化管理器，避免阻塞主线程
    tokio::spawn(async move {
        if let Err(e) = app_state_clone.init_i18n_manager(language_pack_dir) {
            info!("Failed to initialize i18n manager: {}", e);
            // Continue with startup even if i18n manager initialization fails
        } else {
            info!("Internationalization manager initialized successfully");
        }
    });
    info!("Internationalization manager initialization started in background");

    // 8. 业务模块初始化
    info!("Initializing business modules...");
    
    // 初始化去中心化确权模块
    let rights_module = RightsModule::new();
    app_state.set_rights_module(rights_module).await;
    info!("RightsModule initialized");
    
    // 初始化积分生态模块
    let points_module = PointsModule::new();
    app_state.set_points_module(points_module).await;
    info!("PointsModule initialized");
    
    // 初始化用户成长与权限模块
    let user_module = UserModule::new();
    app_state.set_user_module(user_module).await;
    info!("UserModule initialized");
    
    // 初始化防欺诈保障模块
    let fraud_module = FraudModule::new();
    app_state.set_fraud_module(fraud_module).await;
    info!("FraudModule initialized");
    
    // 初始化推广引流和刺激裂变模块
    let referral_module = ReferralModule::new();
    app_state.set_referral_module(referral_module).await;
    info!("ReferralModule initialized");
    
    info!("All business modules initialized");

    // 创建迭代API
    let _iterate_api = Arc::new(IterateApi::new());
    info!("IterateApi initialized");

    // 创建依赖管理器
    let dep_manager = Arc::new(DependencyManager::new());
    info!("DependencyManager initialized");

    // 加载核心依赖
    info!("Loading core dependencies...");
    match dep_manager
        .load_dependencies(DependencyType::All, app_state.clone())
        .await
    {
        Ok(_) => {
            info!("All core dependencies loaded");
        }
        Err(e) => {
            error!("Failed to load core dependencies: {}", e);
            // Continue with startup even if dependencies fail to load
            info!("Continuing with startup despite dependency loading failure");
        }
    }

    // 创建场景管理器
    info!("Creating scene manager...");
    let mut scene_manager = SceneManager::new();
    info!("Scene manager created");

    // 初始化并注册新手场景
    info!("Initializing newbie scene...");
    let mut newbie_scene = NewbieScene::new();
    newbie_scene.set_app_state(app_state.clone());
    scene_manager.register(Box::new(newbie_scene));
    info!("Newbie scene registered");

    // 初始化并注册游戏场景
    info!("Initializing game scene...");
    let game_scene = GameScene::new(1000, 8081, 8082).await?;
    scene_manager.register(Box::new(game_scene));
    info!("Game scene registered");

    // 初始化并注册商城场景
    info!("Initializing mall scene...");
    let mall_scene = MallSceneAdapter::new();
    scene_manager.register(Box::new(mall_scene));
    info!("Mall scene registered");

    // 初始化并注册SaaS场景（暂定10个租户级）
    info!("Initializing SaaS scene...");
    let saas_scene = SaasScene::new(10);
    scene_manager.register(Box::new(saas_scene));
    info!("SaaS scene registered");

    // 初始化并注册社交场景
    info!("Initializing social scene...");
    let social_scene = SocialScene::new();
    scene_manager.register(Box::new(social_scene));
    info!("Social scene registered");

    // 初始化所有场景
    info!("Initializing all scenes...");
    scene_manager.init_all()?;
    info!("All scenes initialized");

    // 启动所有场景
    info!("Starting all scenes...");
    scene_manager.start_all()?;
    info!("All scenes started");

    // 创建限流器
    let rate_limiter = RateLimiter::new(1000, std::time::Duration::from_secs(1));

    // 创建API路由
    let api_router = Router::new()
        .route(
            "/test",
            get(|| async {
                Json(serde_json::json!({"message": "Test API", "status": "ok", "version": "0.1.0"}))
            }),
        )
        .route(
            "/users",
            get(|| async {
                Json(serde_json::json!({
                    "users": [],
                    "total": 0,
                    "status": "ok"
                }))
            }),
        )
        // 业务模块路由
        .route(
            "/rights",
            get(|State(state): State<Arc<AppState>>| async move {
                if state.get_rights_module().await.is_some() {
                    // 这里应该调用模块的实际API方法
                    Json(serde_json::json!({
                        "status": "ok",
                        "module": "rights",
                        "message": "Rights module API"
                    }))
                } else {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Rights module not initialized"
                    }))
                }
            }),
        )
        .route(
            "/points",
            get(|State(state): State<Arc<AppState>>| async move {
                if state.get_points_module().await.is_some() {
                    // 这里应该调用模块的实际API方法
                    Json(serde_json::json!({
                        "status": "ok",
                        "module": "points",
                        "message": "Points module API"
                    }))
                } else {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Points module not initialized"
                    }))
                }
            }),
        )
        .route(
            "/user",
            get(|State(state): State<Arc<AppState>>| async move {
                if state.get_user_module().await.is_some() {
                    // 这里应该调用模块的实际API方法
                    Json(serde_json::json!({
                        "status": "ok",
                        "module": "user",
                        "message": "User module API"
                    }))
                } else {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "User module not initialized"
                    }))
                }
            }),
        )
        .route(
            "/fraud",
            get(|State(state): State<Arc<AppState>>| async move {
                if state.get_fraud_module().await.is_some() {
                    // 这里应该调用模块的实际API方法
                    Json(serde_json::json!({
                        "status": "ok",
                        "module": "fraud",
                        "message": "Fraud module API"
                    }))
                } else {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Fraud module not initialized"
                    }))
                }
            }),
        )
        .route(
            "/referral",
            get(|State(state): State<Arc<AppState>>| async move {
                if state.get_referral_module().await.is_some() {
                    // 这里应该调用模块的实际API方法
                    Json(serde_json::json!({
                        "status": "ok",
                        "module": "referral",
                        "message": "Referral module API"
                    }))
                } else {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Referral module not initialized"
                    }))
                }
            }),
        )
        .route(
            "/plugins",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(plugin_manager) = state.get_plugin_manager().await {
                    let plugins = plugin_manager.get_all_plugins().await;
                    Json(serde_json::json!({
                        "plugins": plugins,
                        "total": plugins.len(),
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "plugins": [],
                        "total": 0,
                        "status": "error",
                        "message": "Plugin manager not initialized"
                    }))
                }
            }),
        )
        // 插件市场路由
        .route(
            "/marketplace/plugins",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(marketplace) = state.get_plugin_marketplace().await {
                    let plugins = marketplace.list_plugins().await;
                    Json(serde_json::json!({
                        "plugins": plugins,
                        "total": plugins.len(),
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "plugins": [],
                        "total": 0,
                        "status": "error",
                        "message": "Plugin marketplace not initialized"
                    }))
                }
            }),
        )
        .route(
            "/marketplace/plugins/trending",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(marketplace) = state.get_plugin_marketplace().await {
                    let plugins = marketplace.get_trending_plugins(10).await;
                    Json(serde_json::json!({
                        "plugins": plugins,
                        "total": plugins.len(),
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "plugins": [],
                        "total": 0,
                        "status": "error",
                        "message": "Plugin marketplace not initialized"
                    }))
                }
            }),
        )
        .route(
            "/marketplace/plugins/top-rated",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(marketplace) = state.get_plugin_marketplace().await {
                    let plugins = marketplace.get_top_rated_plugins(10).await;
                    Json(serde_json::json!({
                        "plugins": plugins,
                        "total": plugins.len(),
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "plugins": [],
                        "total": 0,
                        "status": "error",
                        "message": "Plugin marketplace not initialized"
                    }))
                }
            }),
        )
        .route(
            "/marketplace/plugins/recently-updated",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(marketplace) = state.get_plugin_marketplace().await {
                    let plugins = marketplace.get_recently_updated_plugins(10).await;
                    Json(serde_json::json!({
                        "plugins": plugins,
                        "total": plugins.len(),
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "plugins": [],
                        "total": 0,
                        "status": "error",
                        "message": "Plugin marketplace not initialized"
                    }))
                }
            }),
        )
        .route(
            "/marketplace/stats",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(marketplace) = state.get_plugin_marketplace().await {
                    let stats = marketplace.get_stats().await;
                    Json(serde_json::json!({
                        "stats": stats,
                        "status": "ok"
                    }))
                } else {
                    Json(serde_json::json!({
                        "stats": null,
                        "status": "error",
                        "message": "Plugin marketplace not initialized"
                    }))
                }
            }),
        );

    // 构建使用Axum的完整应用，包含所有路由和中间件
    let app_state_clone = app_state.clone();
    let app = Router::new()
        // 根路径
        .route(
            "/",
            get(|| async {
                Json(serde_json::json!({"message": "Hello, YMAxum Framework!", "status": "ok"}))
            }),
        )
        // 健康检查接口
        .route(
            "/health",
            get(|State(state): State<Arc<AppState>>| async move {
                // 获取数据库状态
                let db_status = state.get_db_status().await;
                // 获取缓存状态
                let cache_status = state.get_cache_status().await;
                // 获取运行时间
                let uptime = state.uptime();
                
                // 获取业务模块状态
                let rights_status = state.get_rights_module().await.is_some();
                let points_status = state.get_points_module().await.is_some();
                let user_status = state.get_user_module().await.is_some();
                let fraud_status = state.get_fraud_module().await.is_some();
                let referral_status = state.get_referral_module().await.is_some();

                Json(serde_json::json!({
                    "status": "ok",
                    "service": "ymaxum-framework",
                    "version": state.version.clone(),
                    "uptime_seconds": uptime,
                    "database_status": db_status,
                    "cache_status": cache_status,
                    "business_modules": {
                        "rights": rights_status,
                        "points": points_status,
                        "user": user_status,
                        "fraud": fraud_status,
                        "referral": referral_status
                    }
                }))
            }),
        )
        // 性能监控端点
        .route(
            "/metrics",
            get(|State(state): State<Arc<AppState>>| async move {
                if let Some(monitor) = state.get_performance_monitor().await {
                    monitor.metrics_handler().await
                } else {
                    "Performance monitor not initialized".to_string()
                }
            }),
        )
        // 告警状态端点
        .route(
            "/alerts",
            get(|State(_state): State<Arc<AppState>>| async move {
                // 这里可以直接返回告警状态
                // 由于我们没有将alert_manager存储到AppState中，这里返回一个简单的响应
                // 实际应用中应该将alert_manager存储到AppState中
                "告警监控已启动，当前无未解决告警".to_string()
            }),
        )
        // API路由 - 添加到嵌套路由
        .nest("/api/v1", api_router)
        // 添加所有中间件
        .layer(from_fn(logger_middleware))
        .layer(from_fn(cors_middleware))
        .layer(from_fn(move |req, next| {
            let limiter = rate_limiter.clone();
            async move { limiter.middleware(req, next).await }
        }))
        .layer(from_fn(exception_catch_middleware))
        // 添加性能监控中间件
        .layer(from_fn(
            move |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| {
                let state = app_state_clone.clone();
                async move {
                    // 记录请求开始时间
                    let start = std::time::Instant::now();

                    // 尝试获取PerformanceMonitor实例
                    #[cfg(feature = "monitoring")]
                    if let Some(monitor) = state.get_performance_monitor().await {
                        monitor.record_request_start();
                    }

                    // 调用下一个中间件
                    let response = next.run(req).await;

                    // 计算请求处理时间
                    let duration = start.elapsed().as_secs_f64();

                    // 检查是否为错误响应
                    let is_error =
                        response.status().is_client_error() || response.status().is_server_error();

                    // 记录请求指标
                    #[cfg(feature = "monitoring")]
                    if let Some(monitor) = state.get_performance_monitor().await {
                        monitor.record_request_end(duration, is_error);
                        // 定期更新内存使用情况
                        if rand::random::<f64>() < 0.01 {
                            // 1%的概率更新
                            monitor.update_memory_usage();
                        }
                    }

                    response
                }
            },
        ))
        // 设置全局状态
        .with_state(app_state.clone());

    // 启动HTTP/1.1和HTTP/2服务器
    let http_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Attempting to bind to address: {}", http_addr);
    let listener = match TcpListener::bind(http_addr).await {
        Ok(l) => {
            info!("Successfully bound to address: {}", http_addr);
            l
        }
        Err(e) => {
            error!("Failed to bind to address {}: {}", http_addr, e);
            return Err(e.into());
        }
    };
    info!("HTTP/1.1/HTTP/2 server listening on http://{}", http_addr);

    // 配置并启动HTTP3服务器（如果启用）
    #[cfg(feature = "http3")]
    {
        let mut http3_config = ymaxum::core::http_3::Http3Config::default();
        http3_config.addr = ([0, 0, 0, 0], 3001).into();
        http3_config.enabled = true; // 启用HTTP3支持
        http3_config.cert_path = None; // 使用自签名证书
        http3_config.key_path = None; // 使用自签名证书
        http3_config.auto_generate_cert = true;
        http3_config.cert_validity_days = 365;
        http3_config.max_concurrent_connections = 1000;
        http3_config.connection_timeout_secs = 30;
        http3_config.max_streams = 100;

        // 创建HTTP3服务器
        let http3_server = ymaxum::core::http_3::Http3Server::new(
            http3_config,
            Arc::new(app.clone()),
            app_state.clone(),
        );

        // 在单独的tokio任务中启动HTTP3服务器
        let mut http3_server_clone = http3_server;
        tokio::spawn(async move {
            if let Err(e) = http3_server_clone.start().await {
                log::error!("HTTP3 server failed to start: {:?}", e);
            }
        });
        info!("HTTP/3 server starting on https://0.0.0.0:3001");
    }

    // 使用axum::serve启动HTTP/1.1和HTTP/2服务器
    let app_with_state = app.with_state(app_state.clone());
    axum::serve(listener, app_with_state.into_make_service()).await?;

    Ok(())
}
