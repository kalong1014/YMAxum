pub mod stylesheets;
/// 样式系统模块
///
/// 包含主题定义、样式表和样式管理功能
pub mod themes;

/// 样式系统配置
pub struct StyleSystemConfig {
    /// 是否启用主题
    pub enable_theme: bool,
    /// 默认主题
    pub default_theme: String,
    /// 是否启用响应式设计
    pub enable_responsive: bool,
    /// 是否启用动画
    pub enable_animation: bool,
}

impl Default for StyleSystemConfig {
    fn default() -> Self {
        Self {
            enable_theme: true,
            default_theme: "default".to_string(),
            enable_responsive: true,
            enable_animation: true,
        }
    }
}

/// 初始化样式系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化主题系统
    themes::initialize().await?;

    // 初始化样式表系统
    stylesheets::initialize().await?;

    Ok(())
}

/// 获取当前主题
pub async fn get_current_theme() -> String {
    themes::get_current_theme().await
}

/// 设置当前主题
pub async fn set_current_theme(theme_name: &str) -> Result<(), crate::error::Error> {
    themes::set_current_theme(theme_name).await
}

/// 应用样式到组件
pub async fn apply_style(
    component_id: &str,
    style: serde_json::Value,
) -> Result<(), crate::error::Error> {
    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 应用样式
    adapter.update_component(component_id, style).await
}
