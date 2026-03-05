/// 动作定义模块
///
/// 定义状态管理系统中的各种动作类型和创建动作的函数
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 动作类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    // UI相关动作
    SetTheme,
    SetLanguage,
    SetDarkMode,
    SetLoading,
    UpdateUI,

    // 组件相关动作
    UpdateComponent,
    CreateComponent,
    DestroyComponent,

    // 表单相关动作
    SetFormValue,
    ResetForm,
    ValidateForm,

    // 导航相关动作
    Navigate,
    SetActiveTab,

    // 自定义动作
    Custom(String),
}

/// 动作定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// 动作类型
    pub r#type: ActionType,
    /// 动作数据
    pub payload: Option<Value>,
    /// 动作元数据
    pub meta: Option<Value>,
}

impl Action {
    /// 创建新动作
    pub fn new(r#type: ActionType, payload: Option<Value>, meta: Option<Value>) -> Self {
        Self {
            r#type,
            payload,
            meta,
        }
    }

    /// 创建无数据的动作
    pub fn new_simple(r#type: ActionType) -> Self {
        Self {
            r#type,
            payload: None,
            meta: None,
        }
    }

    /// 创建带数据的动作
    pub fn new_with_payload(r#type: ActionType, payload: Value) -> Self {
        Self {
            r#type,
            payload: Some(payload),
            meta: None,
        }
    }
}

/// 初始化动作系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    log::info!("Action system initialized");
    Ok(())
}

/// 创建设置主题动作
pub fn set_theme(theme_name: &str) -> Action {
    Action::new_with_payload(
        ActionType::SetTheme,
        serde_json::json!({
            "theme_name": theme_name
        }),
    )
}

/// 创建设置语言动作
pub fn set_language(language: &str) -> Action {
    Action::new_with_payload(
        ActionType::SetLanguage,
        serde_json::json!({
            "language": language
        }),
    )
}

/// 创建设置暗黑模式动作
pub fn set_dark_mode(enabled: bool) -> Action {
    Action::new_with_payload(
        ActionType::SetDarkMode,
        serde_json::json!({
            "enabled": enabled
        }),
    )
}

/// 创建设置加载状态动作
pub fn set_loading(loading: bool) -> Action {
    Action::new_with_payload(
        ActionType::SetLoading,
        serde_json::json!({
            "loading": loading
        }),
    )
}

/// 创建更新组件动作
pub fn update_component(component_id: &str, props: Value) -> Action {
    Action::new_with_payload(
        ActionType::UpdateComponent,
        serde_json::json!({
            "component_id": component_id,
            "props": props
        }),
    )
}

/// 创建创建组件动作
pub fn create_component(component_type: &str, props: Value) -> Action {
    Action::new_with_payload(
        ActionType::CreateComponent,
        serde_json::json!({
            "component_type": component_type,
            "props": props
        }),
    )
}

/// 创建销毁组件动作
pub fn destroy_component(component_id: &str) -> Action {
    Action::new_with_payload(
        ActionType::DestroyComponent,
        serde_json::json!({
            "component_id": component_id
        }),
    )
}

/// 创建设置表单值动作
pub fn set_form_value(form_name: &str, field_name: &str, value: Value) -> Action {
    Action::new_with_payload(
        ActionType::SetFormValue,
        serde_json::json!({
            "form_name": form_name,
            "field_name": field_name,
            "value": value
        }),
    )
}

/// 创建重置表单动作
pub fn reset_form(form_name: &str) -> Action {
    Action::new_with_payload(
        ActionType::ResetForm,
        serde_json::json!({
            "form_name": form_name
        }),
    )
}

/// 创建验证表单动作
pub fn validate_form(form_name: &str) -> Action {
    Action::new_with_payload(
        ActionType::ValidateForm,
        serde_json::json!({
            "form_name": form_name
        }),
    )
}

/// 创建导航动作
pub fn navigate(path: &str) -> Action {
    Action::new_with_payload(
        ActionType::Navigate,
        serde_json::json!({
            "path": path
        }),
    )
}

/// 创建设置活动标签动作
pub fn set_active_tab(tab_id: &str) -> Action {
    Action::new_with_payload(
        ActionType::SetActiveTab,
        serde_json::json!({
            "tab_id": tab_id
        }),
    )
}

/// 创建自定义动作
pub fn custom_action(action_name: &str, payload: Value) -> Action {
    Action::new_with_payload(ActionType::Custom(action_name.to_string()), payload)
}

/// 创建更新UI动作
pub fn update_ui(component_id: &str, props: Value) -> Action {
    Action::new_with_payload(
        ActionType::UpdateUI,
        serde_json::json!({
            "component_id": component_id,
            "props": props
        }),
    )
}
