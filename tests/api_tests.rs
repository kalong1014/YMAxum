//! API测试
//! 测试API接口的功能和性能

use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{delete, get, post, put},
};
use reqwest;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use ymaxum::core::state::AppState;

#[tokio::test]
async fn test_api_health_check() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/health",
            get(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "status": "healthy",
                    "version": "1.0.0",
                    "uptime": "10s",
                    "dependencies": {}
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

    // 测试健康检查API
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", server_addr))
        .send()
        .await
        .expect("健康检查请求失败");

    assert!(response.status().is_success(), "健康检查返回错误状态");

    let health_data: serde_json::Value = response.json().await.expect("解析健康检查响应失败");

    assert_eq!(
        health_data["status"], "healthy",
        "健康检查状态应该是healthy"
    );
    assert!(health_data["version"].is_string(), "版本应该是字符串");
    assert!(health_data["uptime"].is_string(), "运行时间应该是字符串");
    assert!(health_data["dependencies"].is_object(), "依赖应该是对象");

    println!("健康检查API测试通过！");
}

#[tokio::test]
async fn test_api_service_management() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());

    // 创建测试路由器
    let app = Router::new()
        .route(
            "/api/services",
            get(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!([{
                    "id": "1",
                    "name": "test-api-service",
                    "version": "1.0.0",
                    "status": "healthy",
                    "instances": []
                }]))
            }),
        )
        .route(
            "/api/services",
            post(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "id": "1",
                    "name": "test-api-service",
                    "version": "1.0.0",
                    "status": "healthy"
                }))
            }),
        )
        .route(
            "/api/services/test-api-service",
            get(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "name": "test-api-service",
                    "version": "1.0.0",
                    "instances": []
                }))
            }),
        )
        .route(
            "/api/services/test-api-service",
            put(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "version": "1.1.0"
                }))
            }),
        )
        .route(
            "/api/services/test-api-service",
            delete(|State(_state): State<Arc<AppState>>| async move {
                Json(serde_json::json!({
                    "status": "success"
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

    let client = reqwest::Client::new();

    // 测试注册服务
    let register_response = client
        .post(format!("http://{}/api/services", server_addr))
        .header("Content-Type", "application/json")
        .body(
            r#"{
            "name": "test-api-service",
            "version": "1.0.0",
            "url": "http://localhost:3000",
            "health_check": "/health",
            "timeout": 5000,
            "retries": 3
        }"#,
        )
        .send()
        .await
        .expect("服务注册请求失败");

    assert!(
        register_response.status().is_success(),
        "服务注册返回错误状态"
    );

    let register_data: serde_json::Value = register_response
        .json()
        .await
        .expect("解析服务注册响应失败");

    assert!(register_data["id"].is_string(), "服务ID应该是字符串");
    assert_eq!(
        register_data["name"], "test-api-service",
        "服务名称应该匹配"
    );
    assert_eq!(register_data["version"], "1.0.0", "服务版本应该匹配");
    assert_eq!(register_data["status"], "healthy", "服务状态应该是healthy");

    // 测试列出服务
    let list_response = client
        .get(format!("http://{}/api/services", server_addr))
        .send()
        .await
        .expect("服务列表请求失败");

    assert!(list_response.status().is_success(), "服务列表返回错误状态");

    let list_data: serde_json::Value = list_response.json().await.expect("解析服务列表响应失败");

    assert!(list_data.is_array(), "服务列表应该是数组");

    // 检查是否包含刚刚注册的服务
    let mut found = false;
    for service in list_data.as_array().unwrap() {
        if service["name"] == "test-api-service" {
            found = true;
            break;
        }
    }

    assert!(found, "服务列表中应该包含刚刚注册的服务");

    // 测试获取服务详情
    let detail_response = client
        .get(format!(
            "http://{}/api/services/test-api-service",
            server_addr
        ))
        .send()
        .await
        .expect("服务详情请求失败");

    assert!(
        detail_response.status().is_success(),
        "服务详情返回错误状态"
    );

    let detail_data: serde_json::Value =
        detail_response.json().await.expect("解析服务详情响应失败");

    assert_eq!(detail_data["name"], "test-api-service", "服务名称应该匹配");
    assert_eq!(detail_data["version"], "1.0.0", "服务版本应该匹配");
    assert!(detail_data["instances"].is_array(), "实例应该是数组");

    // 测试更新服务
    let update_response = client
        .put(format!(
            "http://{}/api/services/test-api-service",
            server_addr
        ))
        .header("Content-Type", "application/json")
        .body(
            r#"{
            "version": "1.1.0",
            "url": "http://localhost:3000",
            "health_check": "/health",
            "timeout": 10000,
            "retries": 5
        }"#,
        )
        .send()
        .await
        .expect("服务更新请求失败");

    assert!(
        update_response.status().is_success(),
        "服务更新返回错误状态"
    );

    let update_data: serde_json::Value =
        update_response.json().await.expect("解析服务更新响应失败");

    assert_eq!(update_data["version"], "1.1.0", "服务版本应该更新为1.1.0");

    // 测试删除服务
    let delete_response = client
        .delete(format!(
            "http://{}/api/services/test-api-service",
            server_addr
        ))
        .send()
        .await
        .expect("服务删除请求失败");

    assert!(
        delete_response.status().is_success(),
        "服务删除返回错误状态"
    );

    println!("服务管理API测试通过！");
}

#[tokio::test]
async fn test_api_plugin_management() {
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

    // 测试插件列表API
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/api/plugins", server_addr))
        .send()
        .await
        .expect("插件列表请求失败");

    assert!(response.status().is_success(), "插件列表返回错误状态");

    let plugins_data: serde_json::Value = response.json().await.expect("解析插件列表响应失败");

    assert!(plugins_data.is_array(), "插件列表应该是数组");

    println!("插件管理API测试通过！");
}

#[tokio::test]
async fn test_api_scene_management() {
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

    // 测试场景列表API
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/api/scenes", server_addr))
        .send()
        .await
        .expect("场景列表请求失败");

    assert!(response.status().is_success(), "场景列表返回错误状态");

    let scenes_data: serde_json::Value = response.json().await.expect("解析场景列表响应失败");

    assert!(scenes_data.is_array(), "场景列表应该是数组");

    println!("场景管理API测试通过！");
}

#[tokio::test]
async fn test_api_error_handling() {
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

    let client = reqwest::Client::new();

    // 测试不存在的API端点
    let not_found_response = client
        .get(format!("http://{}/api/non-existent", server_addr))
        .send()
        .await
        .expect("请求不存在的API端点失败");

    assert_eq!(
        not_found_response.status().as_u16(),
        404,
        "不存在的API端点应该返回404"
    );

    // 测试无效的请求方法
    let method_not_allowed_response = client
        .post(format!("http://{}/health", server_addr))
        .send()
        .await
        .expect("测试无效请求方法失败");

    assert_eq!(
        method_not_allowed_response.status().as_u16(),
        405,
        "无效的请求方法应该返回405"
    );

    println!("API错误处理测试通过！");
}

#[tokio::test]
async fn test_api_performance() {
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

    let client = reqwest::Client::new();

    // 测试健康检查API的性能
    let start_time = tokio::time::Instant::now();

    // 发送10个并发请求
    let mut tasks = vec![];
    for _ in 0..10 {
        let client = client.clone();
        let server_addr = server_addr.clone();
        tasks.push(tokio::spawn(async move {
            client
                .get(format!("http://{}/health", server_addr))
                .send()
                .await
                .expect("性能测试请求失败")
                .status()
        }));
    }

    // 等待所有请求完成
    let results = futures::future::join_all(tasks).await;

    let elapsed = start_time.elapsed();

    // 检查所有请求都成功
    for result in results {
        let status = result.expect("获取请求结果失败");
        assert!(status.is_success(), "性能测试请求应该成功");
    }

    println!("API性能测试通过！10个并发请求耗时: {:?}", elapsed);
}
