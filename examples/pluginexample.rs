use ymaxum::core::state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::{Router, routing::get, response::Json, middleware::from_fn, extract::State};
use ymaxum::core::middleware::{logger_middleware, cors_middleware, exception_catch_middleware};
use ymaxum::plugin::PluginManager;
use serde_json;

#[tokio::main]
async fn main() {
    // 初始化应用状态
    let app_state = Arc::new(AppState::new());
    
    // 初始化插件管理器
    println!("Initializing plugin manager...");
    let plugin_manager = PluginManager::new().unwrap();
    app_state.set_plugin_manager(plugin_manager).await;
    println!("Plugin manager initialized");
    
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
        .route("/plugins", get(|State(state): State<Arc<AppState>>| async move {
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