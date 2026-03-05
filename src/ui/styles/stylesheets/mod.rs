/// 样式表管理模块
///
/// 包含样式表加载、解析和应用功能
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// 样式表定义
pub struct Stylesheet {
    /// 样式表名称
    pub name: String,
    /// 样式表内容
    pub content: String,
    /// 样式规则
    pub rules: HashMap<String, Value>,
}

impl Stylesheet {
    /// 创建新样式表
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            rules: HashMap::new(),
        }
    }

    /// 添加样式规则
    pub fn add_rule(&mut self, selector: &str, style: Value) {
        self.rules.insert(selector.to_string(), style);
    }

    /// 获取样式规则
    pub fn get_rule(&self, selector: &str) -> Option<&Value> {
        self.rules.get(selector)
    }
}

/// 样式表管理器
#[derive(Default)]
pub struct StylesheetManager {
    /// 样式表列表
    stylesheets: HashMap<String, Stylesheet>,
    /// 已加载的样式表
    loaded_stylesheets: Vec<String>,
}

impl StylesheetManager {
    /// 创建新的样式表管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加样式表
    pub fn add_stylesheet(&mut self, stylesheet: Stylesheet) {
        self.stylesheets.insert(stylesheet.name.clone(), stylesheet);
    }

    /// 加载样式表
    pub fn load_stylesheet(&mut self, name: &str) -> bool {
        if self.stylesheets.contains_key(name)
            && !self.loaded_stylesheets.contains(&name.to_string())
        {
            self.loaded_stylesheets.push(name.to_string());
            true
        } else {
            false
        }
    }

    /// 卸载样式表
    pub fn unload_stylesheet(&mut self, name: &str) -> bool {
        if let Some(index) = self.loaded_stylesheets.iter().position(|n| n == name) {
            self.loaded_stylesheets.remove(index);
            true
        } else {
            false
        }
    }

    /// 获取样式表
    pub fn get_stylesheet(&self, name: &str) -> Option<&Stylesheet> {
        self.stylesheets.get(name)
    }

    /// 获取已加载的样式表
    pub fn get_loaded_stylesheets(&self) -> &Vec<String> {
        &self.loaded_stylesheets
    }

    /// 获取合并后的样式
    pub fn get_combined_style(&self, selector: &str) -> Option<Value> {
        let mut combined_style = serde_json::json!({});
        let mut has_style = false;

        // 合并所有已加载样式表的规则
        for stylesheet_name in &self.loaded_stylesheets {
            if let Some(stylesheet) = self.stylesheets.get(stylesheet_name)
                && let Some(style) = stylesheet.get_rule(selector)
            {
                // 合并样式
                if let Value::Object(style_obj) = style
                    && let Value::Object(combined_obj) = &mut combined_style
                {
                    for (key, value) in style_obj {
                        combined_obj.insert(key.clone(), value.clone());
                    }
                }
                has_style = true;
            }
        }

        if has_style {
            Some(combined_style)
        } else {
            None
        }
    }
}

/// 全局样式表管理器
static STYLESHEET_MANAGER: tokio::sync::OnceCell<RwLock<StylesheetManager>> =
    tokio::sync::OnceCell::const_new();

/// 获取样式表管理器
pub async fn get_stylesheet_manager() -> &'static RwLock<StylesheetManager> {
    STYLESHEET_MANAGER
        .get_or_init(|| async {
            let mut manager = StylesheetManager::new();

            // 添加默认样式表
            let mut default_stylesheet = Stylesheet::new("default", "Default stylesheet");

            // 添加基础样式规则
            default_stylesheet.add_rule(
                "body",
                serde_json::json!({
                    "font-family": "Arial, sans-serif",
                    "font-size": "14px",
                    "color": "#333333",
                    "background-color": "#ffffff"
                }),
            );

            // 添加按钮样式规则
            default_stylesheet.add_rule(
                ".button",
                serde_json::json!({
                    "display": "inline-block",
                    "padding": "8px 16px",
                    "border-radius": "4px",
                    "border": "none",
                    "cursor": "pointer",
                    "font-size": "14px",
                    "font-weight": "500",
                    "text-align": "center",
                    "text-decoration": "none",
                    "transition": "all 0.2s ease"
                }),
            );

            // 添加输入框样式规则
            default_stylesheet.add_rule(
                ".input",
                serde_json::json!({
                    "display": "block",
                    "width": "100%",
                    "padding": "8px 12px",
                    "border": "1px solid #ddd",
                    "border-radius": "4px",
                    "font-size": "14px",
                    "transition": "border-color 0.2s ease"
                }),
            );

            manager.add_stylesheet(default_stylesheet);

            // 添加响应式样式表
            let mut responsive_stylesheet = Stylesheet::new("responsive", "Responsive stylesheet");

            // 添加响应式样式规则
            responsive_stylesheet.add_rule(
                ".container",
                serde_json::json!({
                    "width": "100%",
                    "max-width": "1200px",
                    "margin": "0 auto",
                    "padding": "0 15px"
                }),
            );

            responsive_stylesheet.add_rule(
                ".row",
                serde_json::json!({
                    "display": "flex",
                    "flex-wrap": "wrap",
                    "margin": "0 -15px"
                }),
            );

            responsive_stylesheet.add_rule(
                ".col",
                serde_json::json!({
                    "flex": "1",
                    "padding": "0 15px"
                }),
            );

            manager.add_stylesheet(responsive_stylesheet);

            RwLock::new(manager)
        })
        .await
}

/// 初始化样式表系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化样式表管理器
    let manager = get_stylesheet_manager().await;
    let stylesheets = manager.read().await;

    log::info!(
        "Stylesheet system initialized with {} stylesheets",
        stylesheets.stylesheets.len()
    );

    // 加载默认样式表
    let mut stylesheets_write = manager.write().await;
    stylesheets_write.load_stylesheet("default");
    stylesheets_write.load_stylesheet("responsive");

    log::info!(
        "Loaded stylesheets: {:?}",
        stylesheets_write.get_loaded_stylesheets()
    );

    Ok(())
}

/// 加载样式表
pub async fn load_stylesheet(name: &str) -> Result<(), crate::error::Error> {
    let manager = get_stylesheet_manager().await;
    let mut stylesheets = manager.write().await;

    if stylesheets.load_stylesheet(name) {
        log::info!("Stylesheet loaded: {}", name);
        Ok(())
    } else {
        Err(crate::error::Error::from(
            crate::error::YMAxumError::service_error(format!(
                "Stylesheet '{}' not found or already loaded",
                name
            )),
        ))
    }
}

/// 卸载样式表
pub async fn unload_stylesheet(name: &str) -> Result<(), crate::error::Error> {
    let manager = get_stylesheet_manager().await;
    let mut stylesheets = manager.write().await;

    if stylesheets.unload_stylesheet(name) {
        log::info!("Stylesheet unloaded: {}", name);
        Ok(())
    } else {
        Err(crate::error::Error::from(
            crate::error::YMAxumError::service_error(format!("Stylesheet '{}' not loaded", name)),
        ))
    }
}

/// 获取样式规则
pub async fn get_style_rule(selector: &str) -> Option<Value> {
    let manager = get_stylesheet_manager().await;
    let stylesheets = manager.read().await;
    stylesheets.get_combined_style(selector)
}

/// 应用样式规则
pub async fn apply_style_rule(
    component_id: &str,
    selector: &str,
) -> Result<(), crate::error::Error> {
    if let Some(style) = get_style_rule(selector).await {
        crate::ui::styles::apply_style(component_id, style).await
    } else {
        Ok(())
    }
}
