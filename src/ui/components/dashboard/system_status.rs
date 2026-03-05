//! 系统状态面板组件
//! 用于显示系统的整体状态和健康状况

use serde::{Deserialize, Serialize};

/// 系统状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemStatusLevel {
    Normal,
    Warning,
    Error,
    Critical,
    Unknown,
}

/// 系统组件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// 组件名称
    pub name: String,
    /// 状态
    pub status: SystemStatusLevel,
    /// 状态描述
    pub description: String,
    /// 最后检查时间
    pub last_check: String,
    /// 响应时间(ms)
    pub response_time: Option<u64>,
    /// 版本信息
    pub version: Option<String>,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 系统状态信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusInfo {
    /// 系统名称
    pub system_name: String,
    /// 系统版本
    pub system_version: String,
    /// 整体状态
    pub overall_status: SystemStatusLevel,
    /// 组件状态列表
    pub components: Vec<ComponentStatus>,
    /// 启动时间
    pub uptime: String,
    /// 最后更新时间
    pub last_updated: String,
    /// 系统负载
    pub system_load: f64,
    /// 内存使用情况
    pub memory_usage: f64,
    /// CPU使用情况
    pub cpu_usage: f64,
    /// 磁盘使用情况
    pub disk_usage: f64,
    /// 网络连接状态
    pub network_status: bool,
    /// 数据库连接状态
    pub database_status: bool,
    /// 插件系统状态
    pub plugin_system_status: bool,
    /// GUF集成状态
    pub guf_integration_status: bool,
}

/// 系统状态面板
#[derive(Debug, Clone)]
pub struct SystemStatusPanel {
    /// 系统状态信息
    status_info: std::sync::Arc<tokio::sync::RwLock<SystemStatusInfo>>,
}

impl SystemStatusPanel {
    /// 创建新的系统状态面板
    pub fn new() -> Self {
        let status_info = SystemStatusInfo {
            system_name: "YMAxum".to_string(),
            system_version: "1.0.0".to_string(),
            overall_status: SystemStatusLevel::Normal,
            components: vec![
                ComponentStatus {
                    name: "API Server".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "API服务器运行正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(10),
                    version: Some("1.0.0".to_string()),
                    error_message: None,
                },
                ComponentStatus {
                    name: "Database".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "数据库连接正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(5),
                    version: Some("MySQL 8.0".to_string()),
                    error_message: None,
                },
                ComponentStatus {
                    name: "Plugin System".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "插件系统运行正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(2),
                    version: Some("1.0.0".to_string()),
                    error_message: None,
                },
                ComponentStatus {
                    name: "GUF Integration".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "GUF集成正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(8),
                    version: Some("2.0.0".to_string()),
                    error_message: None,
                },
                ComponentStatus {
                    name: "Cache System".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "缓存系统运行正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(1),
                    version: Some("Redis 7.0".to_string()),
                    error_message: None,
                },
                ComponentStatus {
                    name: "Security System".to_string(),
                    status: SystemStatusLevel::Normal,
                    description: "安全系统运行正常".to_string(),
                    last_check: chrono::Utc::now().to_string(),
                    response_time: Some(3),
                    version: Some("1.0.0".to_string()),
                    error_message: None,
                },
            ],
            uptime: "0d 0h 0m 0s".to_string(),
            last_updated: chrono::Utc::now().to_string(),
            system_load: 0.0,
            memory_usage: 0.0,
            cpu_usage: 0.0,
            disk_usage: 0.0,
            network_status: true,
            database_status: true,
            plugin_system_status: true,
            guf_integration_status: true,
        };

        Self {
            status_info: std::sync::Arc::new(tokio::sync::RwLock::new(status_info)),
        }
    }

    /// 初始化系统状态面板
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化系统状态面板
        println!("Initializing system status panel...");

        // 更新系统状态
        self.update_status().await?;

        Ok(())
    }

    /// 更新系统状态
    pub async fn update_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟系统状态更新
        println!("Updating system status...");

        let mut status_info = self.status_info.write().await;

        // 更新系统资源使用情况
        status_info.system_load = 0.1 + (rand::random::<f64>() * 0.5);
        status_info.memory_usage = 30.0 + (rand::random::<f64>() * 20.0);
        status_info.cpu_usage = 20.0 + (rand::random::<f64>() * 30.0);
        status_info.disk_usage = 15.0 + (rand::random::<f64>() * 10.0);

        // 随机模拟组件状态变化
        for component in &mut status_info.components {
            if rand::random::<f64>() > 0.95 {
                // 5%的概率状态变化
                component.status = match rand::random::<f64>() {
                    x if x < 0.7 => SystemStatusLevel::Normal,
                    x if x < 0.9 => SystemStatusLevel::Warning,
                    _ => SystemStatusLevel::Error,
                };

                component.description = match component.status {
                    SystemStatusLevel::Normal => format!("{}运行正常", component.name),
                    SystemStatusLevel::Warning => format!("{}警告", component.name),
                    SystemStatusLevel::Error => format!("{}错误", component.name),
                    SystemStatusLevel::Critical => format!("{}严重错误", component.name),
                    SystemStatusLevel::Unknown => format!("{}状态未知", component.name),
                };

                if component.status != SystemStatusLevel::Normal {
                    component.error_message = Some(format!("模拟错误: {}", rand::random::<u32>()));
                } else {
                    component.error_message = None;
                }
            }

            component.last_check = chrono::Utc::now().to_string();
            component.response_time = Some(1 + rand::random::<u64>() % 20);
        }

        // 更新整体状态
        status_info.overall_status = self.calculate_overall_status(&status_info.components);

        // 更新网络和数据库状态
        status_info.network_status = rand::random::<f64>() > 0.05; // 95%的概率正常
        status_info.database_status = rand::random::<f64>() > 0.03; // 97%的概率正常
        status_info.plugin_system_status = rand::random::<f64>() > 0.02; // 98%的概率正常
        status_info.guf_integration_status = rand::random::<f64>() > 0.04; // 96%的概率正常

        // 更新最后更新时间
        status_info.last_updated = chrono::Utc::now().to_string();

        Ok(())
    }

    /// 计算整体系统状态
    fn calculate_overall_status(&self, components: &Vec<ComponentStatus>) -> SystemStatusLevel {
        if components.is_empty() {
            return SystemStatusLevel::Unknown;
        }

        let has_critical = components
            .iter()
            .any(|c| c.status == SystemStatusLevel::Critical);
        if has_critical {
            return SystemStatusLevel::Critical;
        }

        let has_error = components
            .iter()
            .any(|c| c.status == SystemStatusLevel::Error);
        if has_error {
            return SystemStatusLevel::Error;
        }

        let has_warning = components
            .iter()
            .any(|c| c.status == SystemStatusLevel::Warning);
        if has_warning {
            return SystemStatusLevel::Warning;
        }

        let all_normal = components
            .iter()
            .all(|c| c.status == SystemStatusLevel::Normal);
        if all_normal {
            return SystemStatusLevel::Normal;
        }

        SystemStatusLevel::Unknown
    }

    /// 获取系统状态信息
    pub async fn get_status(&self) -> Result<SystemStatusInfo, Box<dyn std::error::Error>> {
        let status_info = self.status_info.read().await;
        Ok(status_info.clone())
    }

    /// 获取组件状态
    pub async fn get_component_status(
        &self,
        component_name: &str,
    ) -> Result<Option<ComponentStatus>, Box<dyn std::error::Error>> {
        let status_info = self.status_info.read().await;
        Ok(status_info
            .components
            .iter()
            .find(|c| c.name == component_name)
            .cloned())
    }

    /// 获取整体系统状态
    pub async fn get_overall_status(
        &self,
    ) -> Result<SystemStatusLevel, Box<dyn std::error::Error>> {
        let status_info = self.status_info.read().await;
        Ok(status_info.overall_status.clone())
    }

    /// 检查系统健康状况
    pub async fn check_health(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status_info = self.status_info.read().await;
        Ok(status_info.overall_status == SystemStatusLevel::Normal
            || status_info.overall_status == SystemStatusLevel::Warning)
    }

    /// 获取系统资源使用情况
    pub async fn get_resource_usage(
        &self,
    ) -> Result<(f64, f64, f64, f64), Box<dyn std::error::Error>> {
        let status_info = self.status_info.read().await;
        Ok((
            status_info.cpu_usage,
            status_info.memory_usage,
            status_info.disk_usage,
            status_info.system_load,
        ))
    }
}
