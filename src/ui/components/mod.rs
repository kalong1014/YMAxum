/// UI组件模块
///
/// 包含可复用的UI组件，分为基础组件、布局组件、导航组件和仪表板组件
pub mod base;
pub mod dashboard;
pub mod layout;
pub mod navigation;

/// 组件类型枚举
pub enum ComponentType {
    // 基础组件
    Button,
    Input,
    Checkbox,
    Radio,
    Select,
    Textarea,

    // 布局组件
    Container,
    Grid,
    Flex,
    Stack,

    // 导航组件
    Menu,
    Tab,
    Breadcrumb,
    Pagination,

    // 仪表板组件
    PerformanceDashboard,
    SystemStatus,
    MetricsDisplay,
    AlertPanel,
}

impl ComponentType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentType::Button => "button",
            ComponentType::Input => "input",
            ComponentType::Checkbox => "checkbox",
            ComponentType::Radio => "radio",
            ComponentType::Select => "select",
            ComponentType::Textarea => "textarea",
            ComponentType::Container => "container",
            ComponentType::Grid => "grid",
            ComponentType::Flex => "flex",
            ComponentType::Stack => "stack",
            ComponentType::Menu => "menu",
            ComponentType::Tab => "tab",
            ComponentType::Breadcrumb => "breadcrumb",
            ComponentType::Pagination => "pagination",
            ComponentType::PerformanceDashboard => "performance_dashboard",
            ComponentType::SystemStatus => "system_status",
            ComponentType::MetricsDisplay => "metrics_display",
            ComponentType::AlertPanel => "alert_panel",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "button" => Some(ComponentType::Button),
            "input" => Some(ComponentType::Input),
            "checkbox" => Some(ComponentType::Checkbox),
            "radio" => Some(ComponentType::Radio),
            "select" => Some(ComponentType::Select),
            "textarea" => Some(ComponentType::Textarea),
            "container" => Some(ComponentType::Container),
            "grid" => Some(ComponentType::Grid),
            "flex" => Some(ComponentType::Flex),
            "stack" => Some(ComponentType::Stack),
            "menu" => Some(ComponentType::Menu),
            "tab" => Some(ComponentType::Tab),
            "breadcrumb" => Some(ComponentType::Breadcrumb),
            "pagination" => Some(ComponentType::Pagination),
            "performance_dashboard" => Some(ComponentType::PerformanceDashboard),
            "system_status" => Some(ComponentType::SystemStatus),
            "metrics_display" => Some(ComponentType::MetricsDisplay),
            "alert_panel" => Some(ComponentType::AlertPanel),
            _ => None,
        }
    }
}
