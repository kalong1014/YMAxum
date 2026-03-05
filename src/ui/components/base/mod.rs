/// 基础组件模块
///
/// 包含常用的基础UI组件，如按钮、输入框、复选框等
pub mod button;
pub mod checkbox;
pub mod input;
pub mod radio;
pub mod select;
pub mod textarea;

/// 基础组件配置
pub struct BaseComponentConfig {
    /// 是否启用主题
    pub enable_theme: bool,
    /// 默认主题
    pub default_theme: String,
    /// 是否启用动画
    pub enable_animation: bool,
}

impl Default for BaseComponentConfig {
    fn default() -> Self {
        Self {
            enable_theme: true,
            default_theme: "default".to_string(),
            enable_animation: true,
        }
    }
}

/// 初始化基础组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化按钮组件
    button::initialize().await?;

    // 初始化输入框组件
    input::initialize().await?;

    // 初始化复选框组件
    checkbox::initialize().await?;

    // 初始化单选框组件
    radio::initialize().await?;

    // 初始化选择器组件
    select::initialize().await?;

    // 初始化文本域组件
    textarea::initialize().await?;

    Ok(())
}
