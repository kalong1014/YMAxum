/// 复选框组件
///
/// 可复用的复选框组件，支持多种状态和样式
use serde_json::Value;

/// 复选框状态
pub enum CheckboxState {
    Normal,
    Checked,
    Disabled,
    Indeterminate,
}

/// 复选框属性
#[derive(Default)]
pub struct CheckboxProps {
    /// 是否选中
    pub checked: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 是否为不确定状态
    pub indeterminate: bool,
    /// 标签文本
    pub label: Option<String>,
    /// 变更事件
    pub on_change: Option<fn(bool)>,
}

/// 初始化复选框组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Checkbox component initialized");
    Ok(())
}

/// 创建复选框组件
pub async fn create_checkbox(props: CheckboxProps) -> Result<Value, crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "checked": props.checked,
        "disabled": props.disabled,
        "indeterminate": props.indeterminate,
        "label": props.label,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 创建复选框组件
    adapter.create_component("checkbox", json_props).await
}

/// 更新复选框组件
pub async fn update_checkbox(
    component_id: &str,
    props: CheckboxProps,
) -> Result<(), crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "checked": props.checked,
        "disabled": props.disabled,
        "indeterminate": props.indeterminate,
        "label": props.label,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 更新复选框组件
    adapter.update_component(component_id, json_props).await
}

/// 切换复选框状态
pub async fn toggle_checkbox(component_id: &str) -> Result<Value, crate::error::Error> {
    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 触发变更事件
    adapter
        .trigger_event(component_id, "change", serde_json::json!({}))
        .await
}

impl CheckboxState {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxState::Normal => "normal",
            CheckboxState::Checked => "checked",
            CheckboxState::Disabled => "disabled",
            CheckboxState::Indeterminate => "indeterminate",
        }
    }
}
