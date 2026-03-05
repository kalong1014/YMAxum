use axum::{Router, response::Json, routing::get};
use log::{error, info};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 创建简单的路由
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                Json(serde_json::json!({
                    "message": "Hello, YMAxum Framework!",
                    "status": "ok"
                }))
            }),
        )
        .route(
            "/health",
            get(|| async {
                Json(serde_json::json!({
                    "status": "ok",
                    "service": "ymaxum-framework",
                    "version": "0.1.0"
                }))
            }),
        );

    // 启动HTTP服务器
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

    // 启动服务器
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
