/// 选择器组件
///
/// 可复用的选择器组件，支持多种状态和样式
use serde_json::Value;

/// 选择器状态
pub enum SelectState {
    Normal,
    Focused,
    Disabled,
    Error,
}

/// 选项
pub struct SelectOption {
    /// 值
    pub value: String,
    /// 文本
    pub label: String,
    /// 是否禁用
    pub disabled: bool,
    /// 是否选中
    pub selected: bool,
}

impl SelectOption {
    /// 创建新选项
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            disabled: false,
            selected: false,
        }
    }
}

/// 选择器属性
#[derive(Default)]
pub struct SelectProps {
    /// 选项列表
    pub options: Vec<SelectOption>,
    /// 是否禁用
    pub disabled: bool,
    /// 是否必填
    pub required: bool,
    /// 占位符
    pub placeholder: Option<String>,
    /// 当前值
    pub value: Option<String>,
    /// 变更事件
    pub on_change: Option<fn(String)>,
}

/// 初始化选择器组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Select component initialized");
    Ok(())
}

/// 创建选择器组件
pub async fn create_select(props: SelectProps) -> Result<Value, crate::error::Error> {
    // 转换为JSON属性
    let options_json: Vec<Value> = props
        .options
        .iter()
        .map(|opt| {
            serde_json::json!({
                "value": opt.value,
                "label": opt.label,
                "disabled": opt.disabled,
                "selected": opt.selected,
            })
        })
        .collect();

    let json_props = serde_json::json!({
        "options": options_json,
        "disabled": props.disabled,
        "required": props.required,
        "placeholder": props.placeholder,
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

    // 创建选择器组件
    adapter.create_component("select", json_props).await
}

/// 更新选择器组件
pub async fn update_select(
    component_id: &str,
    props: SelectProps,
) -> Result<(), crate::error::Error> {
    // 转换为JSON属性
    let options_json: Vec<Value> = props
        .options
        .iter()
        .map(|opt| {
            serde_json::json!({
                "value": opt.value,
                "label": opt.label,
                "disabled": opt.disabled,
                "selected": opt.selected,
            })
        })
        .collect();

    let json_props = serde_json::json!({
        "options": options_json,
        "disabled": props.disabled,
        "required": props.required,
        "placeholder": props.placeholder,
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

    // 更新选择器组件
    adapter.update_component(component_id, json_props).await
}

/// 选择选项
pub async fn select_option(component_id: &str, value: &str) -> Result<Value, crate::error::Error> {
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
        .trigger_event(
            component_id,
            "change",
            serde_json::json!({
                "value": value
            }),
        )
        .await
}

impl SelectState {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectState::Normal => "normal",
            SelectState::Focused => "focused",
            SelectState::Disabled => "disabled",
            SelectState::Error => "error",
        }
    }
}
