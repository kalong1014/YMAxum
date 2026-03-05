/// 响应式设计工具
///
/// 提供设备检测、屏幕尺寸获取和响应式布局辅助功能
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

/// 设备类型
#[derive(Debug, Clone)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    Tv,
    Unknown,
}

/// 屏幕方向
#[derive(Clone)]
pub enum ScreenOrientation {
    Portrait,
    Landscape,
}

/// 响应式断点
pub struct Breakpoint {
    /// 断点名称
    pub name: String,
    /// 最小宽度
    pub min_width: u32,
    /// 最大宽度
    pub max_width: u32,
}

impl Breakpoint {
    /// 创建新断点
    pub fn new(name: &str, min_width: u32, max_width: u32) -> Self {
        Self {
            name: name.to_string(),
            min_width,
            max_width,
        }
    }

    /// 检查宽度是否在断点范围内
    pub fn matches(&self, width: u32) -> bool {
        width >= self.min_width && width <= self.max_width
    }
}

/// 响应式配置
pub struct ResponsiveConfig {
    /// 断点定义
    pub breakpoints: Vec<Breakpoint>,
    /// 设备类型检测规则
    pub device_detection: HashMap<String, String>,
    /// 是否启用响应式设计
    pub enable_responsive: bool,
    /// 是否启用自动适配
    pub enable_auto_adapt: bool,
    /// 是否启用触摸设备优化
    pub enable_touch_optimization: bool,
    /// 是否启用字体缩放
    pub enable_font_scaling: bool,
    /// 字体缩放比例
    pub font_scaling_ratio: f32,
    /// 是否启用响应式图片
    pub enable_responsive_images: bool,
    /// 响应式图片配置
    pub responsive_images: HashMap<String, String>,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        let breakpoints = vec![
            Breakpoint::new("xs", 0, 575),
            Breakpoint::new("sm", 576, 767),
            Breakpoint::new("md", 768, 991),
            Breakpoint::new("lg", 992, 1199),
            Breakpoint::new("xl", 1200, 1399),
            Breakpoint::new("xxl", 1400, u32::MAX),
        ];

        let mut device_detection = HashMap::new();
        device_detection.insert(
            "mobile".to_string(),
            "(iPhone|iPod|Android|BlackBerry|IEMobile|Opera Mini)".to_string(),
        );
        device_detection.insert(
            "tablet".to_string(),
            "(iPad|Android(?!.*Mobile))".to_string(),
        );
        device_detection.insert("tv".to_string(), "(SmartTV|tvOS|WebOS|Tizen)".to_string());

        let mut responsive_images = HashMap::new();
        responsive_images.insert("xs".to_string(), "320w".to_string());
        responsive_images.insert("sm".to_string(), "576w".to_string());
        responsive_images.insert("md".to_string(), "768w".to_string());
        responsive_images.insert("lg".to_string(), "992w".to_string());
        responsive_images.insert("xl".to_string(), "1200w".to_string());
        responsive_images.insert("xxl".to_string(), "1400w".to_string());

        Self {
            breakpoints,
            device_detection,
            enable_responsive: true,
            enable_auto_adapt: true,
            enable_touch_optimization: true,
            enable_font_scaling: true,
            font_scaling_ratio: 1.0,
            enable_responsive_images: true,
            responsive_images,
        }
    }
}

/// 屏幕信息
#[derive(Clone)]
pub struct ScreenInfo {
    /// 屏幕宽度
    pub width: u32,
    /// 屏幕高度
    pub height: u32,
    /// 设备像素比
    pub pixel_ratio: f64,
    /// 设备类型
    pub device_type: DeviceType,
    /// 屏幕方向
    pub orientation: ScreenOrientation,
    /// 当前断点
    pub current_breakpoint: String,
    /// 是否为触摸设备
    pub is_touch_device: bool,
    /// 浏览器名称
    pub browser_name: String,
    /// 浏览器版本
    pub browser_version: String,
    /// 操作系统
    pub os: String,
    /// 字体缩放比例
    pub font_scale: f32,
}

/// 响应式工具
pub struct ResponsiveUtils {
    /// 配置
    config: ResponsiveConfig,
    /// 当前屏幕信息
    current_screen: Option<ScreenInfo>,
}

impl Default for ResponsiveUtils {
    fn default() -> Self {
        Self::new(ResponsiveConfig::default())
    }
}

impl ResponsiveUtils {
    /// 创建新的响应式工具
    pub fn new(config: ResponsiveConfig) -> Self {
        Self {
            config,
            current_screen: None,
        }
    }

    /// 检测设备类型
    pub fn detect_device_type(&self, user_agent: &str) -> DeviceType {
        for (device_type, pattern) in &self.config.device_detection {
            if regex::Regex::new(pattern).unwrap().is_match(user_agent) {
                match device_type.as_str() {
                    "mobile" => return DeviceType::Mobile,
                    "tablet" => return DeviceType::Tablet,
                    "tv" => return DeviceType::Tv,
                    _ => return DeviceType::Unknown,
                }
            }
        }
        DeviceType::Desktop
    }

    /// 获取屏幕方向
    pub fn get_screen_orientation(&self, width: u32, height: u32) -> ScreenOrientation {
        if width < height {
            ScreenOrientation::Portrait
        } else {
            ScreenOrientation::Landscape
        }
    }

    /// 获取当前断点
    pub fn get_current_breakpoint(&self, width: u32) -> String {
        for breakpoint in &self.config.breakpoints {
            if breakpoint.matches(width) {
                return breakpoint.name.clone();
            }
        }
        "unknown".to_string()
    }

    /// 更新屏幕信息
    pub fn update_screen_info(
        &mut self,
        width: u32,
        height: u32,
        pixel_ratio: f64,
        user_agent: &str,
        is_touch: bool,
    ) {
        let device_type = self.detect_device_type(user_agent);
        let orientation = self.get_screen_orientation(width, height);
        let current_breakpoint = self.get_current_breakpoint(width);
        let (browser_name, browser_version) = self.detect_browser(user_agent);
        let os = self.detect_os(user_agent);
        let font_scale = if self.config.enable_font_scaling {
            self.config.font_scaling_ratio
        } else {
            1.0
        };

        self.current_screen = Some(ScreenInfo {
            width,
            height,
            pixel_ratio,
            device_type,
            orientation,
            current_breakpoint,
            is_touch_device: is_touch,
            browser_name,
            browser_version,
            os,
            font_scale,
        });
    }

    /// 检测浏览器
    pub fn detect_browser(&self, user_agent: &str) -> (String, String) {
        // 简化的浏览器检测，实际应用中可能需要更复杂的逻辑
        if user_agent.contains("Chrome") && !user_agent.contains("Edg") {
            ("Chrome".to_string(), "unknown".to_string())
        } else if user_agent.contains("Firefox") {
            ("Firefox".to_string(), "unknown".to_string())
        } else if user_agent.contains("Safari") && !user_agent.contains("Chrome") {
            ("Safari".to_string(), "unknown".to_string())
        } else if user_agent.contains("Edg") {
            ("Edge".to_string(), "unknown".to_string())
        } else if user_agent.contains("MSIE") || user_agent.contains("Trident") {
            ("Internet Explorer".to_string(), "unknown".to_string())
        } else {
            ("Unknown".to_string(), "unknown".to_string())
        }
    }

    /// 检测操作系统
    pub fn detect_os(&self, user_agent: &str) -> String {
        // 简化的操作系统检测，实际应用中可能需要更复杂的逻辑
        if user_agent.contains("Windows") {
            "Windows".to_string()
        } else if user_agent.contains("Macintosh") {
            "macOS".to_string()
        } else if user_agent.contains("Linux") && !user_agent.contains("Android") {
            "Linux".to_string()
        } else if user_agent.contains("Android") {
            "Android".to_string()
        } else if user_agent.contains("iPhone") || user_agent.contains("iPad") {
            "iOS".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// 获取当前屏幕信息
    pub fn get_current_screen(&self) -> Option<&ScreenInfo> {
        self.current_screen.as_ref()
    }

    /// 生成响应式样式
    pub fn generate_responsive_styles(&self, base_styles: &Value) -> Value {
        let mut responsive_styles = Value::Object(serde_json::Map::new());

        // 添加基础样式
        if let Value::Object(base_obj) = base_styles {
            for (key, value) in base_obj {
                responsive_styles[key.clone()] = value.clone();
            }
        }

        // 添加响应式样式
        if let Some(screen) = &self.current_screen {
            let breakpoint = &screen.current_breakpoint;

            // 根据断点添加特定样式
            match breakpoint.as_str() {
                "xs" => {
                    // 移动端样式
                    if let Value::Object(styles_obj) = &mut responsive_styles {
                        styles_obj
                            .insert("font-size".to_string(), Value::String("14px".to_string()));
                        styles_obj.insert("padding".to_string(), Value::String("8px".to_string()));
                    }
                }
                "sm" => {
                    // 小平板样式
                    if let Value::Object(styles_obj) = &mut responsive_styles {
                        styles_obj
                            .insert("font-size".to_string(), Value::String("14px".to_string()));
                        styles_obj.insert("padding".to_string(), Value::String("12px".to_string()));
                    }
                }
                "md" => {
                    // 平板样式
                    if let Value::Object(styles_obj) = &mut responsive_styles {
                        styles_obj
                            .insert("font-size".to_string(), Value::String("16px".to_string()));
                        styles_obj.insert("padding".to_string(), Value::String("16px".to_string()));
                    }
                }
                "lg" => {
                    // 桌面样式
                    if let Value::Object(styles_obj) = &mut responsive_styles {
                        styles_obj
                            .insert("font-size".to_string(), Value::String("16px".to_string()));
                        styles_obj.insert("padding".to_string(), Value::String("20px".to_string()));
                    }
                }
                "xl" | "xxl" => {
                    // 大屏桌面样式
                    if let Value::Object(styles_obj) = &mut responsive_styles {
                        styles_obj
                            .insert("font-size".to_string(), Value::String("18px".to_string()));
                        styles_obj.insert("padding".to_string(), Value::String("24px".to_string()));
                    }
                }
                _ => {}
            }
        }

        responsive_styles
    }

    /// 生成自适应布局
    pub fn generate_adaptive_layout(&self, layout: &Value) -> Value {
        let mut adaptive_layout = layout.clone();

        if let Some(screen) = &self.current_screen {
            let device_type = &screen.device_type;

            // 根据设备类型调整布局
            match device_type {
                DeviceType::Mobile => {
                    // 移动端布局调整
                    self.adjust_for_mobile(&mut adaptive_layout);
                }
                DeviceType::Tablet => {
                    // 平板布局调整
                    self.adjust_for_tablet(&mut adaptive_layout);
                }
                DeviceType::Tv => {
                    // TV布局调整
                    self.adjust_for_tv(&mut adaptive_layout);
                }
                _ => {}
            }
        }

        adaptive_layout
    }

    /// 为移动端调整布局
    fn adjust_for_mobile(&self, layout: &mut Value) {
        // 简化布局，减少列数
        if let Value::Object(layout_obj) = layout {
            // 调整网格布局
            if layout_obj.contains_key("grid")
                && let Some(Value::Object(grid_obj)) = layout_obj.get_mut("grid")
            {
                grid_obj.insert("columns".to_string(), Value::Number(2.into()));
                grid_obj.insert("gap".to_string(), Value::String("8px".to_string()));
            }

            // 调整导航
            if layout_obj.contains_key("navigation")
                && let Some(Value::Object(nav_obj)) = layout_obj.get_mut("navigation")
            {
                nav_obj.insert("type".to_string(), Value::String("hamburger".to_string()));
            }
        }
    }

    /// 为平板调整布局
    fn adjust_for_tablet(&self, layout: &mut Value) {
        // 适度调整布局
        if let Value::Object(layout_obj) = layout {
            // 调整网格布局
            if layout_obj.contains_key("grid")
                && let Some(Value::Object(grid_obj)) = layout_obj.get_mut("grid")
            {
                grid_obj.insert("columns".to_string(), Value::Number(3.into()));
                grid_obj.insert("gap".to_string(), Value::String("12px".to_string()));
            }
        }
    }

    /// 为TV调整布局
    fn adjust_for_tv(&self, layout: &mut Value) {
        // 为TV优化布局
        if let Value::Object(layout_obj) = layout {
            // 增大字体和间距
            if layout_obj.contains_key("typography")
                && let Some(Value::Object(typography_obj)) = layout_obj.get_mut("typography")
            {
                typography_obj.insert("font-size".to_string(), Value::String("20px".to_string()));
                typography_obj.insert("line-height".to_string(), Value::String("1.4".to_string()));
            }

            // 增大导航项
            if layout_obj.contains_key("navigation")
                && let Some(Value::Object(nav_obj)) = layout_obj.get_mut("navigation")
            {
                nav_obj.insert("item-height".to_string(), Value::String("60px".to_string()));
            }
        }
    }
}

/// 全局响应式工具实例
static RESPONSIVE_UTILS: OnceCell<Arc<RwLock<ResponsiveUtils>>> = OnceCell::const_new();

/// 初始化响应式工具
pub async fn initialize_responsive() {
    RESPONSIVE_UTILS
        .get_or_init(|| async { Arc::new(RwLock::new(ResponsiveUtils::default())) })
        .await;
}

/// 获取响应式工具实例
pub async fn get_responsive_utils() -> Arc<RwLock<ResponsiveUtils>> {
    RESPONSIVE_UTILS
        .get_or_init(|| async { Arc::new(RwLock::new(ResponsiveUtils::default())) })
        .await
        .clone()
}

/// 更新屏幕信息
pub async fn update_screen_info(width: u32, height: u32, pixel_ratio: f64, user_agent: &str, is_touch: bool) {
    let utils = get_responsive_utils().await;
    let mut utils_guard = utils.write().await;
    utils_guard.update_screen_info(width, height, pixel_ratio, user_agent, is_touch);
}

/// 获取当前屏幕信息
pub async fn get_current_screen() -> Option<ScreenInfo> {
    let utils = get_responsive_utils().await;
    let utils_guard = utils.read().await;
    utils_guard.get_current_screen().cloned()
}

/// 生成响应式样式
pub async fn generate_responsive_styles(base_styles: &Value) -> Value {
    let utils = get_responsive_utils().await;
    let utils_guard = utils.read().await;
    utils_guard.generate_responsive_styles(base_styles)
}

/// 生成自适应布局
pub async fn generate_adaptive_layout(layout: &Value) -> Value {
    let utils = get_responsive_utils().await;
    let utils_guard = utils.read().await;
    utils_guard.generate_adaptive_layout(layout)
}

/// 设备信息结构
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// 设备类型
    pub device_type: DeviceType,
    /// 屏幕宽度
    pub screen_width: u32,
    /// 屏幕高度
    pub screen_height: u32,
    /// 是否移动设备
    pub is_mobile: bool,
}

/// 获取设备信息
pub async fn get_device_info() -> DeviceInfo {
    // 模拟设备信息，实际应用中应该通过浏览器API或用户代理检测
    DeviceInfo {
        device_type: DeviceType::Desktop,
        screen_width: 1920,
        screen_height: 1080,
        is_mobile: false,
    }
}
