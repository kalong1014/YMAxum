pub mod components;
/// UI系统模块
///
/// 包含Godot UI Framework (GUF)集成的核心功能，
/// 提供可复用的用户界面组件和管理系统
pub mod core;
pub mod state;
pub mod styles;
pub mod utils;

/// UI系统初始化函数
///
/// # Returns
/// * `Result<()>` - 初始化结果
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化UI核心系统
    core::initialize().await
}

/// UI系统版本信息
pub const UI_SYSTEM_VERSION: &str = "1.0.0";
