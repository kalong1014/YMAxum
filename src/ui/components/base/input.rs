/// 输入框组件
///
/// 可复用的输入框组件，支持多种类型和验证
use serde_json::Value;

/// 输入框类型
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DatetimeLocal,
}

/// 输入框大小
pub enum InputSize {
    Small,
    Medium,
    Large,
}

/// 输入框状态
pub enum InputState {
    Normal,
    Focused,
    Error,
    Disabled,
}

/// 输入框属性
pub struct InputProps {
    /// 输入框类型
    pub input_type: InputType,
    /// 占位符
    pub placeholder: String,
    /// 默认值
    pub value: String,
    /// 是否禁用
    pub disabled: bool,
    /// 是否必填
    pub required: bool,
    /// 变更事件
    pub on_change: Option<fn(String)>,
    /// 提交事件
    pub on_submit: Option<fn()>,
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            input_type: InputType::Text,
            placeholder: "Enter text".to_string(),
            value: "".to_string(),
            disabled: false,
            required: false,
            on_change: None,
            on_submit: None,
        }
    }
}

/// 初始化输入框组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Input component initialized");
    Ok(())
}

/// 创建输入框组件
pub async fn create_input(props: InputProps) -> Result<Value, crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "type": props.input_type.as_str(),
        "placeholder": props.placeholder,
        "value": props.value,
        "disabled": props.disabled,
        "required": props.required,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 创建输入框组件
    adapter.create_component("input", json_props).await
}

/// 更新输入框组件
pub async fn update_input(
    component_id: &str,
    props: InputProps,
) -> Result<(), crate::error::Error> {
    // 转换为JSON属性
    let json_props = serde_json::json!({
        "type": props.input_type.as_str(),
        "placeholder": props.placeholder,
        "value": props.value,
        "disabled": props.disabled,
        "required": props.required,
    });

    // 获取GUF适配器
    let registry = crate::ui::core::adapter::get_adapter_registry().await;
    let version = crate::ui::core::adapter::GufVersion::parse("4.4.0").unwrap();
    let adapter = registry.get_adapter(&version).unwrap_or_else(|| {
        registry
            .get_adapter(&crate::ui::core::adapter::GufVersion::parse("4.0.0").unwrap())
            .unwrap()
    });

    // 更新输入框组件
    adapter.update_component(component_id, json_props).await
}

/// 触发输入框变更事件
pub async fn change_input(component_id: &str, value: &str) -> Result<Value, crate::error::Error> {
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

impl InputType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
        }
    }
}

impl InputSize {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            InputSize::Small => "small",
            InputSize::Medium => "medium",
            InputSize::Large => "large",
        }
    }
}

impl InputState {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            InputState::Normal => "normal",
            InputState::Focused => "focused",
            InputState::Error => "error",
            InputState::Disabled => "disabled",
        }
    }
}
