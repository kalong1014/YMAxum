/// 单选框组件
///
/// 可复用的单选框组件，支持多种状态和样式
use serde_json::Value;

/// 单选框状态
pub enum RadioState {
    Normal,
    Checked,
    Disabled,
}

/// 单选框属性
pub struct RadioProps {
    /// 是否选中
    pub checked: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 标签文本
    pub label: Option<String>,
    /// 分组名称
    pub name: String,
    /// 值
    pub value: String,
    /// 变更事件
    pub on_change: Option<fn(String)>,
}

impl Default for RadioProps {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            label: None,
            name: "radio_group".to_string(),
            value: "".to_string(),
            on_change: None,
        }
    }
}

/// 初始化单选框组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Radio component initialized");
    Ok(())
}

/// 创建单选框组件
pub async fn create_radio(props: RadioProps) -> Result<Value, crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "checked": props.checked,
        "disabled": props.disabled,
        "label": props.label,
        "name": props.name,
        "value": props.value,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 创建单选框组件
    adapter.create_component("radio", json_props).await
}

/// 更新单选框组件
pub async fn update_radio(
    component_id: &str,
    props: RadioProps,
) -> Result<(), crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "checked": props.checked,
        "disabled": props.disabled,
        "label": props.label,
        "name": props.name,
        "value": props.value,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 更新单选框组件
    adapter.update_component(component_id, json_props).await
}

/// 选择单选框
pub async fn select_radio(component_id: &str) -> Result<Value, crate::error::Error> {
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

impl RadioState {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioState::Normal => "normal",
            RadioState::Checked => "checked",
            RadioState::Disabled => "disabled",
        }
    }
}
