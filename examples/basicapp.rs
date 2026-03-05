use ymaxum::core::state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::{Router, routing::get, response::Json, middleware::from_fn, extract::State};
use ymaxum::core::middleware::{logger_middleware, cors_middleware, exception_catch_middleware};
use ymaxum::core::network::client::Client;
use serde_json;

#[tokio::main]
async fn main() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());
    
    // 创建路由
    let app = Router::new()
        .route("/", get(|| async {
            Json(serde_json::json!({
                "message": "Hello, YMAxum Framework!", 
                "status": "ok"
            }))
        }))
        .route("/health", get(|State(state): State<Arc<AppState>>| async move {
            let uptime = state.uptime();
            Json(serde_json::json!({
                "status": "ok",
                "service": "ymaxum-framework",
                "version": state.version.clone(),
                "uptime_seconds": uptime
            }))
        }))
        .route("/api/example", get(|| async {
            // 创建网络客户端
            let client = Client::new();
            
            // 发送GET请求到example.com
            match client.get("https://example.com").await {
                Ok(response) => {
                    Json(serde_json::json!({
                        "status": "ok",
                        "response_status": response.status.code,
                        "message": "Network request successful"
                    }))
                },
                Err(e) => {
                    Json(serde_json::json!({
                        "status": "error",
                        "message": format!("Network request failed: {:?}", e)
                    }))
                }
            }
        }))
        .layer(from_fn(logger_middleware))
        .layer(from_fn(cors_middleware))
        .layer(from_fn(exception_catch_middleware))
        .with_state(app_state.clone());
    
    // 启动服务器
    println!("Server starting on port 3000...");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}