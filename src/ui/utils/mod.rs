pub mod html_escape;
/// 工具模块
///
/// 包含响应式设计、设备检测和其他UI辅助工具
pub mod responsive;

/// 初始化工具模块
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化响应式工具
    responsive::initialize_responsive().await;

    log::info!("UI utils initialized");
    Ok(())
}

/// 获取设备类型
pub async fn get_device_type(user_agent: &str) -> responsive::DeviceType {
    let utils = responsive::get_responsive_utils().await;
    let utils_guard = utils.read().await;
    utils_guard.detect_device_type(user_agent)
}

/// 更新屏幕信息
pub async fn update_screen_info(width: u32, height: u32, pixel_ratio: f64, user_agent: &str) {
    responsive::update_screen_info(width, height, pixel_ratio, user_agent, false).await;
}

/// 生成响应式样式
pub async fn generate_responsive_styles(base_styles: &serde_json::Value) -> serde_json::Value {
    responsive::generate_responsive_styles(base_styles).await
}

/// 生成自适应布局
pub async fn generate_adaptive_layout(layout: &serde_json::Value) -> serde_json::Value {
    responsive::generate_adaptive_layout(layout).await
}
