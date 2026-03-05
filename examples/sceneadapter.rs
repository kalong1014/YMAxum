use ymaxum::core::state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::{Router, routing::get, response::Json, middleware::from_fn, extract::State};
use ymaxum::core::middleware::{logger_middleware, cors_middleware, exception_catch_middleware};
use serde_json;

#[tokio::main]
async fn main() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());
    
    // 创建路由
    let app = Router::new()
        .route("/", get(|| async {
            Json(serde_json::json!({
                "message": "Scene adapter server", 
                "status": "ok"
            }))
        }))
        .route("/health", get(|State(state): State<Arc<AppState>>| async move {
            let uptime = state.uptime();
            Json(serde_json::json!({
                "status": "ok",
                "service": "ymaxum-scene-adapter",
                "version": state.version.clone(),
                "uptime_seconds": uptime
            }))
        }))
        .layer(from_fn(logger_middleware))
        .layer(from_fn(cors_middleware))
        .layer(from_fn(exception_catch_middleware))
        .with_state(app_state.clone());
    
    // 启动服务器
    println!("Scene adapter server starting on port 3000...");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}