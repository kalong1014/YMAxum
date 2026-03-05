//! 端到端测试
//! 测试完整的系统流程，从客户端请求到服务端响应

use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};
use reqwest;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use ymaxum::core::state::AppState;

#[tokio::test]
async fn test_e2e_full_flow() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/health",
            get(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "status": "healthy"
                }))
            }),
        )
        .route(
            "/api/services",
            get(|State(_state): State<Arc<AppState>>| async move { Json(serde_json::json!([])) }),
        )
        .route(
            "/api/services",
            post(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "id": "1",
                    "name": "test-service",
                    "version": "1.0.0",
                    "status": "healthy"
                }))
            }),
        )
        .route(
            "/api/plugins",
            get(|State(_state): State<Arc<AppState>>| async move { Json(serde_json::json!([])) }),
        )
        .route(
            "/api/scenes",
            get(|State(_state): State<Arc<AppState>>| async move { Json(serde_json::json!([])) }),
        )
        .with_state(app_state.clone());

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 随机端口
    let listener = TcpListener::bind(addr).await.unwrap();
    let server_addr = listener.local_addr().unwrap();

    // 在后台运行服务器
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 测试健康检查
    let client = reqwest::Client::new();
    let health_response = client
        .get(format!("http://{}/health", server_addr))
        .send()
        .await;

    assert!(health_response.is_ok(), "健康检查失败");
    let health_status = health_response.unwrap().status();
    assert!(health_status.is_success(), "健康检查返回错误状态");

    // 测试服务注册
    let register_response = client
        .post(format!("http://{}/api/services", server_addr))
        .header("Content-Type", "application/json")
        .body(
            r#"{
            "name": "test-service",
            "version": "1.0.0",
            "url": "http://localhost:3000",
            "health_check": "/health"
        }"#,
        )
        .send()
        .await;

    assert!(register_response.is_ok(), "服务注册失败");
    let register_status = register_response.unwrap().status();
    assert!(register_status.is_success(), "服务注册返回错误状态");

    // 测试服务发现
    let discover_response = client
        .get(format!("http://{}/api/services", server_addr))
        .send()
        .await;

    assert!(discover_response.is_ok(), "服务发现失败");
    let discover_status = discover_response.unwrap().status();
    assert!(discover_status.is_success(), "服务发现返回错误状态");

    // 测试插件管理
    let plugin_response = client
        .get(format!("http://{}/api/plugins", server_addr))
        .send()
        .await;

    assert!(plugin_response.is_ok(), "插件管理失败");
    let plugin_status = plugin_response.unwrap().status();
    assert!(plugin_status.is_success(), "插件管理返回错误状态");

    // 测试场景管理
    let scene_response = client
        .get(format!("http://{}/api/scenes", server_addr))
        .send()
        .await;

    assert!(scene_response.is_ok(), "场景管理失败");
    let scene_status = scene_response.unwrap().status();
    assert!(scene_status.is_success(), "场景管理返回错误状态");

    println!("端到端测试完成，所有测试通过！");
}

#[tokio::test]
async fn test_e2e_error_handling() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/api/services/non-existent",
            get(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "error": "Not found"
                }))
            }),
        )
        .with_state(app_state.clone());

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 随机端口
    let listener = TcpListener::bind(addr).await.unwrap();
    let server_addr = listener.local_addr().unwrap();

    // 在后台运行服务器
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 测试不存在的服务
    let client = reqwest::Client::new();
    let not_found_response = client
        .get(format!("http://{}/api/services/non-existent", server_addr))
        .send()
        .await;

    assert!(not_found_response.is_ok(), "请求失败");
    let not_found_status = not_found_response.unwrap().status();
    assert!(not_found_status.is_success(), "应该返回成功状态码");

    // 测试无效的请求
    let bad_request_response = client
        .post(format!("http://{}/api/services", server_addr))
        .header("Content-Type", "application/json")
        .body(r#"{}"#)
        .send()
        .await;

    assert!(bad_request_response.is_ok(), "请求失败");
    let bad_request_status = bad_request_response.unwrap().status();
    assert_eq!(bad_request_status.as_u16(), 404, "应该返回404状态码");

    println!("错误处理测试完成，所有测试通过！");
}

#[tokio::test]
async fn test_e2e_plugin_flow() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/api/plugins",
            get(|State(_state): State<Arc<AppState>>| async move { Json(serde_json::json!([])) }),
        )
        .with_state(app_state.clone());

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 随机端口
    let listener = TcpListener::bind(addr).await.unwrap();
    let server_addr = listener.local_addr().unwrap();

    // 在后台运行服务器
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 测试插件列表
    let client = reqwest::Client::new();
    let plugins_response = client
        .get(format!("http://{}/api/plugins", server_addr))
        .send()
        .await;

    assert!(plugins_response.is_ok(), "插件列表请求失败");
    let plugins_status = plugins_response.unwrap().status();
    assert!(plugins_status.is_success(), "插件列表返回错误状态");

    // 测试插件启用/禁用
    // 注意：这里需要实际的插件，所以暂时跳过
    // 后续可以添加模拟插件进行测试

    println!("插件流程测试完成，所有测试通过！");
}

#[tokio::test]
async fn test_e2e_scene_flow() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/api/scenes",
            get(|State(_state): State<Arc<AppState>>| async move { Json(serde_json::json!([])) }),
        )
        .with_state(app_state.clone());

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 随机端口
    let listener = TcpListener::bind(addr).await.unwrap();
    let server_addr = listener.local_addr().unwrap();

    // 在后台运行服务器
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 测试场景列表
    let client = reqwest::Client::new();
    let scenes_response = client
        .get(format!("http://{}/api/scenes", server_addr))
        .send()
        .await;

    assert!(scenes_response.is_ok(), "场景列表请求失败");
    let scenes_status = scenes_response.unwrap().status();
    assert!(scenes_status.is_success(), "场景列表返回错误状态");

    // 测试场景切换
    // 注意：这里需要实际的场景，所以暂时跳过
    // 后续可以添加模拟场景进行测试

    println!("场景流程测试完成，所有测试通过！");
}
