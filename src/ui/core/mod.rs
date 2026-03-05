/// UI核心模块
///
/// 包含GUF集成的核心功能，包括版本适配、连接器和管理器
pub mod adapter;
pub mod connector;
pub mod manager;

/// UI核心系统初始化
///
/// # Returns
/// * `Result<()>` - 初始化结果
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化适配器系统
    adapter::initialize().await?;

    // 初始化连接器
    connector::initialize().await?;

    // 初始化UI管理器
    manager::initialize().await?;

    Ok(())
}
