/// 按钮组件
///
/// 可复用的按钮组件，支持多种样式和状态
use serde_json::Value;

/// 按钮类型
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

/// 按钮大小
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// 按钮状态
pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled,
}

/// 按钮属性
pub struct ButtonProps {
    /// 按钮类型
    pub button_type: ButtonType,
    /// 按钮大小
    pub size: ButtonSize,
    /// 是否禁用
    pub disabled: bool,
    /// 是否加载中
    pub loading: bool,
    /// 按钮文本
    pub text: String,
    /// 图标
    pub icon: Option<String>,
    /// 点击事件
    pub on_click: Option<fn()>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            loading: false,
            text: "Button".to_string(),
            icon: None,
            on_click: None,
        }
    }
}

/// 初始化按钮组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Button component initialized");
    Ok(())
}

/// 创建按钮组件
pub async fn create_button(props: ButtonProps) -> Result<Value, crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "type": props.button_type.as_str(),
        "size": props.size.as_str(),
        "disabled": props.disabled,
        "loading": props.loading,
        "text": props.text,
        "icon": props.icon,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 创建按钮组件
    adapter.create_component("button", json_props).await
}

/// 更新按钮组件
pub async fn update_button(
    component_id: &str,
    props: ButtonProps,
) -> Result<(), crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "type": props.button_type.as_str(),
        "size": props.size.as_str(),
        "disabled": props.disabled,
        "loading": props.loading,
        "text": props.text,
        "icon": props.icon,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 更新按钮组件
    adapter.update_component(component_id, json_props).await
}

/// 触发按钮点击事件
pub async fn click_button(component_id: &str) -> Result<Value, crate::error::Error> {
    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 触发点击事件
    adapter
        .trigger_event(component_id, "click", serde_json::json!({}))
        .await
}

impl ButtonType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Primary => "primary",
            ButtonType::Secondary => "secondary",
            ButtonType::Success => "success",
            ButtonType::Warning => "warning",
            ButtonType::Danger => "danger",
            ButtonType::Info => "info",
        }
    }
}

impl ButtonSize {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

impl ButtonState {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonState::Normal => "normal",
            ButtonState::Hover => "hover",
            ButtonState::Active => "active",
            ButtonState::Disabled => "disabled",
        }
    }
}
