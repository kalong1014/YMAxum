//! GUF 插件模板示例

use anyhow::Result;
use guf_plugin_template::{GufPluginTemplate, PluginManifest};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== GUF Plugin Template Example ===");

    // 创建插件实例
    let mut plugin = GufPluginTemplate::new();
    println!("Created plugin: {}", plugin.manifest.name);
    println!("Version: {}", plugin.manifest.version);
    println!("Description: {}", plugin.manifest.description);
    println!("GUF Compatible: {}", plugin.manifest.guf_compatible);
    println!("GUF Version: {}", plugin.manifest.guf_version);

    // 初始化插件
    println!("\nInitializing plugin...");
    if let Err(e) = plugin.initialize().await {
        eprintln!("Failed to initialize plugin: {}", e);
        return Err(e);
    }
    println!("Plugin initialized successfully!");

    // 检查 GUF 集成状态
    println!("\nChecking GUF integration status...");
    let guf_status = plugin.check_guf_status().await;
    println!("GUF Integration Status: {:?}", guf_status);

    // 处理 GUF 事件
    println!("\nHandling GUF event...");
    let event_data = serde_json::json!({
        "event_id": "12345",
        "timestamp": "2026-02-04T12:00:00Z",
        "data": {
            "key": "value"
        }
    });
    if let Err(e) = plugin.handle_guf_event("test_event".to_string(), event_data).await {
        eprintln!("Failed to handle GUF event: {}", e);
    } else {
        println!("GUF event handled successfully!");
    }

    // 调用 GUF 服务
    println!("\nCalling GUF service...");
    let service_params = serde_json::json!({
        "service_id": "test_service",
        "params": {
            "name": "test",
            "value": 123
        }
    });
    match plugin.call_guf_service("test_service".to_string(), service_params).await {
        Ok(response) => {
            println!("GUF service called successfully!");
            println!("Response: {:?}", response);
        }
        Err(e) => {
            eprintln!("Failed to call GUF service: {}", e);
        }
    }

    // 停止插件
    println!("\nStopping plugin...");
    if let Err(e) = plugin.stop().await {
        eprintln!("Failed to stop plugin: {}", e);
        return Err(e);
    }
    println!("Plugin stopped successfully!");

    println!("\n=== GUF Plugin Template Example Complete ===");
    Ok(())
}
