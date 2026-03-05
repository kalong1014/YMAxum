use async_trait::async_trait;
use std::hash::Hash;
use std::sync::Arc;

/// GUF版本信息
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GufVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl GufVersion {
    /// 解析版本字符串
    pub fn parse(version: &str) -> Option<Self> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;

        Some(Self {
            major,
            minor,
            patch,
        })
    }

    /// 比较版本
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        if self.major != other.major {
            return self.major.cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.cmp(&other.minor);
        }
        self.patch.cmp(&other.patch)
    }
}

/// GUF适配器接口
#[async_trait]
pub trait GufAdapter: Send + Sync {
    /// 获取适配器支持的最低版本
    fn min_version(&self) -> GufVersion;

    /// 获取适配器支持的最高版本
    fn max_version(&self) -> GufVersion;

    /// 检查是否支持指定版本
    fn supports_version(&self, version: &GufVersion) -> bool;

    /// 初始化适配器
    async fn initialize(&self) -> Result<(), crate::error::Error>;

    /// 创建UI组件
    async fn create_component(
        &self,
        component_type: &str,
        props: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error>;

    /// 更新UI组件
    async fn update_component(
        &self,
        component_id: &str,
        props: serde_json::Value,
    ) -> Result<(), crate::error::Error>;

    /// 销毁UI组件
    async fn destroy_component(&self, component_id: &str) -> Result<(), crate::error::Error>;

    /// 触发组件事件
    async fn trigger_event(
        &self,
        component_id: &str,
        event_name: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error>;
}

/// GUF适配器注册表
pub struct GufAdapterRegistry {
    adapters: Vec<Arc<dyn GufAdapter>>,
}

impl GufAdapterRegistry {
    /// 创建新的适配器注册表
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }

    /// 注册适配器
    pub fn register(&mut self, adapter: Arc<dyn GufAdapter>) {
        self.adapters.push(adapter);
    }

    /// 根据版本获取合适的适配器
    pub fn get_adapter(&self, version: &GufVersion) -> Option<Arc<dyn GufAdapter>> {
        self.adapters
            .iter()
            .find(|adapter| adapter.supports_version(version))
            .cloned()
    }

    /// 获取所有适配器
    pub fn get_all_adapters(&self) -> &Vec<Arc<dyn GufAdapter>> {
        &self.adapters
    }
}

/// 全局适配器注册表
static ADAPTER_REGISTRY: tokio::sync::OnceCell<GufAdapterRegistry> =
    tokio::sync::OnceCell::const_new();

/// 获取适配器注册表
pub async fn get_adapter_registry() -> &'static GufAdapterRegistry {
    ADAPTER_REGISTRY
        .get_or_init(|| async {
            let mut registry = GufAdapterRegistry::new();
            // 注册默认适配器
            registry.register(Arc::new(
                crate::ui::core::adapter::default::DefaultGufAdapter::new(),
            ));
            // 注册v4.3适配器
            registry.register(Arc::new(
                crate::ui::core::adapter::v4_3::GufAdapterV4_3::new(),
            ));
            // 注册v4.4适配器
            registry.register(Arc::new(
                crate::ui::core::adapter::v4_4::GufAdapterV4_4::new(),
            ));
            registry
        })
        .await
}

/// 初始化适配器系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化适配器注册表
    let registry = get_adapter_registry().await;

    // 初始化所有适配器
    for adapter in registry.get_all_adapters() {
        adapter.initialize().await?;
    }

    Ok(())
}

/// 适配器子模块
pub mod default;
pub mod v4_3;
pub mod v4_4;
