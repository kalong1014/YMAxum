/// 主题管理模块
///
/// 包含主题定义、加载和切换功能
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// 主题名称枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeName {
    /// 默认主题
    Default,
    /// 暗黑主题
    Dark,
}

impl ThemeName {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeName::Default => "default",
            ThemeName::Dark => "dark",
        }
    }
}

impl From<ThemeName> for String {
    fn from(theme_name: ThemeName) -> Self {
        theme_name.as_str().to_string()
    }
}

/// 主题定义
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题描述
    pub description: String,
    /// 主题变量
    pub variables: HashMap<String, String>,
    /// 组件样式
    pub component_styles: HashMap<String, Value>,
}

impl Theme {
    /// 创建新主题
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            variables: HashMap::new(),
            component_styles: HashMap::new(),
        }
    }

    /// 添加主题变量
    pub fn add_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// 添加组件样式
    pub fn add_component_style(&mut self, component_type: &str, style: Value) {
        self.component_styles
            .insert(component_type.to_string(), style);
    }

    /// 获取主题变量
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// 获取组件样式
    pub fn get_component_style(&self, component_type: &str) -> Option<&Value> {
        self.component_styles.get(component_type)
    }
}

/// 主题管理器
pub struct ThemeManager {
    /// 主题列表
    themes: HashMap<String, Theme>,
    /// 当前主题
    current_theme: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            themes: HashMap::new(),
            current_theme: "default".to_string(),
        }
    }
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加主题
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// 获取主题
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme)
    }

    /// 设置当前主题
    pub fn set_current_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    /// 获取所有主题
    pub fn get_all_themes(&self) -> &HashMap<String, Theme> {
        &self.themes
    }
}

/// 全局主题管理器
static THEME_MANAGER: tokio::sync::OnceCell<RwLock<ThemeManager>> =
    tokio::sync::OnceCell::const_new();

/// 获取主题管理器
pub async fn get_theme_manager() -> &'static RwLock<ThemeManager> {
    THEME_MANAGER
        .get_or_init(|| async {
            let mut manager = ThemeManager::new();

            // 添加默认主题
            let mut default_theme = Theme::new("default", "Default theme");
            default_theme.add_variable("primary-color", "#3498db");
            default_theme.add_variable("secondary-color", "#2ecc71");
            default_theme.add_variable("danger-color", "#e74c3c");
            default_theme.add_variable("warning-color", "#f39c12");
            default_theme.add_variable("info-color", "#3498db");
            default_theme.add_variable("text-color", "#333333");
            default_theme.add_variable("background-color", "#ffffff");

            // 添加按钮样式
            default_theme.add_component_style(
                "button",
                serde_json::json!({
                    "primary": {
                        "background": "#3498db",
                        "color": "#ffffff",
                        "padding": "8px 16px",
                        "border-radius": "4px",
                        "border": "none",
                        "cursor": "pointer"
                    },
                    "secondary": {
                        "background": "#2ecc71",
                        "color": "#ffffff",
                        "padding": "8px 16px",
                        "border-radius": "4px",
                        "border": "none",
                        "cursor": "pointer"
                    }
                }),
            );

            // 添加输入框样式
            default_theme.add_component_style(
                "input",
                serde_json::json!({
                    "normal": {
                        "border": "1px solid #ddd",
                        "padding": "8px 12px",
                        "border-radius": "4px",
                        "font-size": "14px"
                    },
                    "focused": {
                        "border": "1px solid #3498db",
                        "outline": "none",
                        "box-shadow": "0 0 0 2px rgba(52, 152, 219, 0.2)"
                    }
                }),
            );

            manager.add_theme(default_theme);

            // 添加暗色主题
            let mut dark_theme = Theme::new("dark", "Dark theme");
            dark_theme.add_variable("primary-color", "#3498db");
            dark_theme.add_variable("secondary-color", "#2ecc71");
            dark_theme.add_variable("danger-color", "#e74c3c");
            dark_theme.add_variable("warning-color", "#f39c12");
            dark_theme.add_variable("info-color", "#3498db");
            dark_theme.add_variable("text-color", "#ffffff");
            dark_theme.add_variable("background-color", "#333333");

            // 添加按钮样式
            dark_theme.add_component_style(
                "button",
                serde_json::json!({
                    "primary": {
                        "background": "#3498db",
                        "color": "#ffffff",
                        "padding": "8px 16px",
                        "border-radius": "4px",
                        "border": "none",
                        "cursor": "pointer"
                    },
                    "secondary": {
                        "background": "#2ecc71",
                        "color": "#ffffff",
                        "padding": "8px 16px",
                        "border-radius": "4px",
                        "border": "none",
                        "cursor": "pointer"
                    }
                }),
            );

            // 添加输入框样式
            dark_theme.add_component_style(
                "input",
                serde_json::json!({
                    "normal": {
                        "border": "1px solid #555",
                        "padding": "8px 12px",
                        "border-radius": "4px",
                        "font-size": "14px",
                        "background": "#444",
                        "color": "#fff"
                    },
                    "focused": {
                        "border": "1px solid #3498db",
                        "outline": "none",
                        "box-shadow": "0 0 0 2px rgba(52, 152, 219, 0.2)"
                    }
                }),
            );

            manager.add_theme(dark_theme);

            RwLock::new(manager)
        })
        .await
}

/// 初始化主题系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化主题管理器
    let manager = get_theme_manager().await;
    let themes = manager.read().await;

    log::info!(
        "Theme system initialized with {} themes",
        themes.themes.len()
    );
    log::info!("Current theme: {}", themes.current_theme);

    Ok(())
}

/// 获取当前主题
pub async fn get_current_theme() -> String {
    let manager = get_theme_manager().await;
    let themes = manager.read().await;
    themes.current_theme.clone()
}

/// 设置当前主题
pub async fn set_current_theme(theme_name: &str) -> Result<(), crate::error::Error> {
    let manager = get_theme_manager().await;
    let mut themes = manager.write().await;

    if themes.set_current_theme(theme_name) {
        log::info!("Theme changed to: {}", theme_name);
        Ok(())
    } else {
        Err(crate::error::Error::from(
            crate::error::YMAxumError::service_error(format!("Theme '{}' not found", theme_name)),
        ))
    }
}

/// 使用枚举设置当前主题
pub async fn set_theme(theme_name: ThemeName) -> Result<(), crate::error::Error> {
    set_current_theme(theme_name.as_str()).await
}

/// 获取主题样式
pub async fn get_theme_style(theme_name: &str, component_type: &str) -> Option<Value> {
    let manager = get_theme_manager().await;
    let themes = manager.read().await;

    themes
        .get_theme(theme_name)
        .and_then(|theme| theme.get_component_style(component_type))
        .cloned()
}

/// 获取当前主题样式
pub async fn get_current_theme_style(component_type: &str) -> Option<Value> {
    let theme_name = get_current_theme().await;
    get_theme_style(&theme_name, component_type).await
}
