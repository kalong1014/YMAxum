//! GUF 集成测试（使用模拟服务器）
//! 使用模拟的 GUF 服务器测试网络连接和 API 调用

mod mock_guf_server;

use ymaxum::guf::GufIntegration;
use ymaxum::guf::adapter::{GufAdapter, GufAuthInfo, GufRuntimeConfig};
use ymaxum::guf::config_sync::GufConfigSync;

#[tokio::test]
async fn test_guf_adapter_with_mock_server() {
    // 启动模拟 GUF 服务器
    let (port, _server_state) = mock_guf_server::start_mock_guf_server().await;
    let server_url = mock_guf_server::get_mock_guf_server_url(port);

    // 创建 GUF 适配器
    let mut adapter = GufAdapter::new();

    // 创建运行时配置
    let config = GufRuntimeConfig {
        server_address: server_url,
        auth_info: GufAuthInfo {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            auth_token: None,
        },
        connection_timeout: 5,
        heartbeat_interval: 30,
        pool_size: 2,
    };

    // 初始化适配器
    let init_result = adapter.initialize(config).await;
    assert!(init_result.is_ok());

    // 检查适配器状态
    assert!(adapter.is_initialized());
    assert!(adapter.is_connected());
}

#[tokio::test]
async fn test_guf_config_sync_with_mock_server() {
    // 启动模拟 GUF 服务器
    let (port, _server_state) = mock_guf_server::start_mock_guf_server().await;
    let server_url = mock_guf_server::get_mock_guf_server_url(port);

    // 创建配置同步服务
    let config_sync = GufConfigSync::new_with_config(server_url, "test_token".to_string());

    // 初始化配置同步
    let init_result = config_sync.initialize("default").await;
    assert!(init_result.is_ok());

    // 启动同步服务
    tokio::spawn(async move {
        config_sync.start_sync_service("default", 10).await;
    });

    // 等待同步服务启动
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
}

#[tokio::test]
async fn test_guf_integration_with_mock_server() {
    // 启动模拟 GUF 服务器
    let (port, _server_state) = mock_guf_server::start_mock_guf_server().await;
    let server_url = mock_guf_server::get_mock_guf_server_url(port);

    // 创建 GUF 集成实例
    let mut integration = GufIntegration::new_with_config(server_url, "test_token".to_string());

    // 初始化集成
    let init_result = integration.initialize().await;
    assert!(init_result.is_ok());

    // 启动集成
    let start_result = integration.start().await;
    assert!(start_result.is_ok());

    // 等待一段时间
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // 停止集成
    let _ = integration.stop().await;
}

#[tokio::test]
async fn test_guf_performance_with_mock_server() {
    // 启动模拟 GUF 服务器
    let (port, _server_state) = mock_guf_server::start_mock_guf_server().await;
    let server_url = mock_guf_server::get_mock_guf_server_url(port);

    // 创建 GUF 适配器
    let mut adapter = GufAdapter::new();

    // 创建运行时配置
    let config = GufRuntimeConfig {
        server_address: server_url.clone(),
        auth_info: GufAuthInfo {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            auth_token: None,
        },
        connection_timeout: 5,
        heartbeat_interval: 30,
        pool_size: 5,
    };

    // 初始化适配器
    let init_result = adapter.initialize(config).await;
    assert!(init_result.is_ok());

    // 测试性能：连续发送多个请求
    let start_time = std::time::Instant::now();

    // 模拟发送多个初始化请求
    for i in 0..10 {
        // 重新创建配置
        let config = GufRuntimeConfig {
            server_address: server_url.clone(),
            auth_info: GufAuthInfo {
                client_id: format!("test_client_{}", i),
                client_secret: "test_secret".to_string(),
                auth_token: None,
            },
            connection_timeout: 5,
            heartbeat_interval: 30,
            pool_size: 2,
        };

        // 初始化适配器
        let result = adapter.initialize(config).await;
        assert!(result.is_ok());

        // 等待一小段时间
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    let elapsed = start_time.elapsed();
    println!("Performed 10 initialization requests in {:?}", elapsed);
    // 确保性能合理（10个请求在10秒内完成）
    assert!(elapsed < std::time::Duration::from_secs(10));
}
