//! 模拟 GUF 服务器
//! 用于测试 GUF 集成的网络连接和 API 调用

use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRequest {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectResponse {
    pub success: bool,
    pub message: String,
    pub connection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRequest {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub success: bool,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct MockGufServerState {
    pub connections: std::sync::RwLock<Vec<String>>,
    pub configs: std::sync::RwLock<std::collections::HashMap<String, serde_json::Value>>,
}

impl MockGufServerState {
    pub fn new() -> Self {
        Self {
            connections: std::sync::RwLock::new(vec![]),
            configs: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

async fn handle_connect(
    State(state): State<Arc<MockGufServerState>>,
    Json(_req): Json<ConnectRequest>,
) -> Json<ConnectResponse> {
    // 模拟连接处理
    let connection_id = format!("conn_{}", uuid::Uuid::new_v4());

    // 存储连接
    state
        .connections
        .write()
        .unwrap()
        .push(connection_id.clone());

    Json(ConnectResponse {
        success: true,
        message: "Connection established successfully".to_string(),
        connection_id,
    })
}

async fn handle_get_config(
    State(state): State<Arc<MockGufServerState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Json<ConfigResponse> {
    // 模拟获取配置
    let configs = state.configs.read().unwrap();
    let default_value = serde_json::json!({});
    let value = configs.get(&key).unwrap_or(&default_value);

    Json(ConfigResponse {
        success: true,
        data: value.clone(),
    })
}

async fn handle_set_config(
    State(state): State<Arc<MockGufServerState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
    Json(req): Json<ConfigRequest>,
) -> Json<ConfigResponse> {
    // 模拟设置配置
    state.configs.write().unwrap().insert(key, req.value);

    Json(ConfigResponse {
        success: true,
        data: serde_json::json!({"status": "ok"}),
    })
}

pub async fn start_mock_guf_server() -> (u16, Arc<MockGufServerState>) {
    // 创建服务器状态
    let state = Arc::new(MockGufServerState::new());

    // 设置路由
    let app = Router::new()
        .route("/api/v1/connect", post(handle_connect))
        .route("/api/v1/config/{key}", post(handle_set_config))
        .route(
            "/api/v1/config/{key}",
            axum::routing::get(handle_get_config),
        )
        .with_state(state.clone());

    // 绑定到随机端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // 启动服务器
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (port, state)
}

pub fn get_mock_guf_server_url(port: u16) -> String {
    format!("http://localhost:{}", port)
}
