/// Reducers模块
///
/// 负责处理状态更新逻辑，根据动作类型更新相应的状态
use super::actions::{Action, ActionType};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Reducer类型
pub type Reducer = fn(&Value, &Action) -> Value;

/// Reducer注册表
pub struct ReducerRegistry {
    /// 动作类型到reducer的映射
    reducers: HashMap<String, Reducer>,
}

impl ReducerRegistry {
    /// 创建新的reducer注册表
    pub fn new() -> Self {
        Self {
            reducers: HashMap::new(),
        }
    }

    /// 注册reducer
    pub fn register(&mut self, action_type: &str, reducer: Reducer) {
        self.reducers.insert(action_type.to_string(), reducer);
    }

    /// 获取reducer
    pub fn get(&self, action_type: &str) -> Option<Reducer> {
        self.reducers.get(action_type).copied()
    }
}

/// 全局reducer注册表
static REDUCER_REGISTRY: tokio::sync::OnceCell<RwLock<ReducerRegistry>> =
    tokio::sync::OnceCell::const_new();

/// 获取reducer注册表
pub async fn get_reducer_registry() -> &'static RwLock<ReducerRegistry> {
    REDUCER_REGISTRY
        .get_or_init(|| async {
            let mut registry = ReducerRegistry::new();

            // 注册默认reducers
            registry.register("SetTheme", set_theme_reducer);
            registry.register("SetLanguage", set_language_reducer);
            registry.register("SetDarkMode", set_dark_mode_reducer);
            registry.register("SetLoading", set_loading_reducer);
            registry.register("UpdateUI", update_ui_reducer);
            registry.register("UpdateComponent", update_component_reducer);
            registry.register("CreateComponent", create_component_reducer);
            registry.register("DestroyComponent", destroy_component_reducer);
            registry.register("SetFormValue", set_form_value_reducer);
            registry.register("ResetForm", reset_form_reducer);
            registry.register("ValidateForm", validate_form_reducer);
            registry.register("Navigate", navigate_reducer);
            registry.register("SetActiveTab", set_active_tab_reducer);

            RwLock::new(registry)
        })
        .await
}

/// 初始化reducers
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化reducer注册表
    let registry = get_reducer_registry().await;
    let reducers = registry.read().await;

    log::info!(
        "Reducer system initialized with {} reducers",
        reducers.reducers.len()
    );

    Ok(())
}

/// 应用reducer
pub async fn apply_reducer(state: &Value, action: &Action) -> Value {
    let registry = get_reducer_registry().await;
    let reducers = registry.read().await;

    let action_type_str = match &action.r#type {
        ActionType::SetTheme => "SetTheme",
        ActionType::SetLanguage => "SetLanguage",
        ActionType::SetDarkMode => "SetDarkMode",
        ActionType::SetLoading => "SetLoading",
        ActionType::UpdateUI => "UpdateUI",
        ActionType::UpdateComponent => "UpdateComponent",
        ActionType::CreateComponent => "CreateComponent",
        ActionType::DestroyComponent => "DestroyComponent",
        ActionType::SetFormValue => "SetFormValue",
        ActionType::ResetForm => "ResetForm",
        ActionType::ValidateForm => "ValidateForm",
        ActionType::Navigate => "Navigate",
        ActionType::SetActiveTab => "SetActiveTab",
        ActionType::Custom(name) => name,
    };

    if let Some(reducer) = reducers.get(action_type_str) {
        reducer(state, action)
    } else {
        // 默认reducer，不做任何改变
        state.clone()
    }
}

/// 设置主题reducer
fn set_theme_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(theme_name) = payload.get("theme_name").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("theme".to_string(), Value::String(theme_name.to_string()));
        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 设置语言reducer
fn set_language_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(language) = payload.get("language").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("language".to_string(), Value::String(language.to_string()));
        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 设置暗黑模式reducer
fn set_dark_mode_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(enabled) = payload.get("enabled").and_then(|v| v.as_bool())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("dark_mode".to_string(), Value::Bool(enabled));
        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 设置加载状态reducer
fn set_loading_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(loading) = payload.get("loading").and_then(|v| v.as_bool())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("loading".to_string(), Value::Bool(loading));
        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 更新UIreducer
fn update_ui_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let (Some(component_id), Some(props)) = (
            payload.get("component_id").and_then(|v| v.as_str()),
            payload.get("props"),
        )
        && let Value::Object(mut state_obj) = new_state
    {
        // 确保components字段存在
        if !state_obj.contains_key("components") {
            state_obj.insert(
                "components".to_string(),
                Value::Object(serde_json::Map::new()),
            );
        }

        if let Some(Value::Object(components)) = state_obj.get_mut("components") {
            components.insert(component_id.to_string(), props.clone());
        }

        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 更新组件reducer
fn update_component_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let (Some(component_id), Some(props)) = (
            payload.get("component_id").and_then(|v| v.as_str()),
            payload.get("props"),
        )
        && let Value::Object(mut state_obj) = new_state
    {
        // 确保components字段存在
        if !state_obj.contains_key("components") {
            state_obj.insert(
                "components".to_string(),
                Value::Object(serde_json::Map::new()),
            );
        }

        if let Some(Value::Object(components)) = state_obj.get_mut("components") {
            components.insert(component_id.to_string(), props.clone());
        }

        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 创建组件reducer
fn create_component_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let (Some(component_type), Some(props)) = (
            payload.get("component_type").and_then(|v| v.as_str()),
            payload.get("props"),
        )
    {
        let component_id = format!("{}_{}", component_type, uuid::Uuid::new_v4());

        if let Value::Object(mut state_obj) = new_state {
            // 确保components字段存在
            if !state_obj.contains_key("components") {
                state_obj.insert(
                    "components".to_string(),
                    Value::Object(serde_json::Map::new()),
                );
            }

            if let Some(Value::Object(components)) = state_obj.get_mut("components") {
                components.insert(component_id, props.clone());
            }

            new_state = Value::Object(state_obj);
        }
    }

    new_state
}

/// 销毁组件reducer
fn destroy_component_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(component_id) = payload.get("component_id").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        if let Some(Value::Object(components)) = state_obj.get_mut("components") {
            components.remove(component_id);
        }

        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 设置表单值reducer
fn set_form_value_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let (Some(form_name), Some(field_name), Some(value)) = (
            payload.get("form_name").and_then(|v| v.as_str()),
            payload.get("field_name").and_then(|v| v.as_str()),
            payload.get("value"),
        )
        && let Value::Object(mut state_obj) = new_state
    {
        // 确保forms字段存在
        if !state_obj.contains_key("forms") {
            state_obj.insert("forms".to_string(), Value::Object(serde_json::Map::new()));
        }

        if let Some(Value::Object(forms)) = state_obj.get_mut("forms") {
            // 确保表单存在
            if !forms.contains_key(form_name) {
                forms.insert(form_name.to_string(), Value::Object(serde_json::Map::new()));
            }

            if let Some(Value::Object(form)) = forms.get_mut(form_name) {
                form.insert(field_name.to_string(), value.clone());
            }
        }

        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 重置表单reducer
fn reset_form_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(form_name) = payload.get("form_name").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        if let Some(Value::Object(forms)) = state_obj.get_mut("forms") {
            forms.insert(form_name.to_string(), Value::Object(serde_json::Map::new()));
        }

        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 验证表单reducer
fn validate_form_reducer(state: &Value, _action: &Action) -> Value {
    // 这里可以添加表单验证逻辑
    state.clone()
}

/// 导航reducer
fn navigate_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(path) = payload.get("path").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("current_path".to_string(), Value::String(path.to_string()));
        new_state = Value::Object(state_obj);
    }

    new_state
}

/// 设置活动标签reducer
fn set_active_tab_reducer(state: &Value, action: &Action) -> Value {
    let mut new_state = state.clone();

    if let Some(payload) = &action.payload
        && let Some(tab_id) = payload.get("tab_id").and_then(|v| v.as_str())
        && let Value::Object(mut state_obj) = new_state
    {
        state_obj.insert("active_tab".to_string(), Value::String(tab_id.to_string()));
        new_state = Value::Object(state_obj);
    }

    new_state
}
