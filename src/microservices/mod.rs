//! 微服务架构模块
//! 用于服务发现和负载均衡、分布式追踪、配置中心

pub mod service_discovery;
pub mod load_balancing;
pub mod distributed_tracing;
pub mod config_center;

/// 微服务架构管理器
#[derive(Debug, Clone)]
pub struct MicroservicesManager {
    service_discovery: service_discovery::ServiceDiscovery,
    load_balancing: load_balancing::LoadBalancing,
    distributed_tracing: distributed_tracing::DistributedTracing,
    config_center: config_center::ConfigCenter,
}

impl MicroservicesManager {
    /// 创建新的微服务架构管理器
    pub fn new() -> Self {
        Self {
            service_discovery: service_discovery::ServiceDiscovery::new(),
            load_balancing: load_balancing::LoadBalancing::new(),
            distributed_tracing: distributed_tracing::DistributedTracing::new(),
            config_center: config_center::ConfigCenter::new(),
        }
    }

    /// 初始化微服务架构
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.service_discovery.initialize().await?;
        self.load_balancing.initialize().await?;
        self.distributed_tracing.initialize().await?;
        self.config_center.initialize().await?;
        Ok(())
    }

    /// 注册服务
    pub async fn register_service(&self, service: service_discovery::ServiceInfo) -> Result<(), Box<dyn std::error::Error>> {
        self.service_discovery.register_service(service).await
    }

    /// 发现服务
    pub async fn discover_service(&self, service_name: &str) -> Result<Vec<service_discovery::ServiceInfo>, Box<dyn std::error::Error>> {
        self.service_discovery.discover_service(service_name).await
    }

    /// 负载均衡
    pub async fn balance_load(&self, service_name: &str, request: serde_json::Value) -> Result<load_balancing::LoadBalancingResult, Box<dyn std::error::Error>> {
        self.load_balancing.balance_load(service_name, request).await
    }

    /// 开始追踪
    pub async fn start_tracing(&self, service_name: &str, operation: &str) -> Result<distributed_tracing::TraceSpan, Box<dyn std::error::Error>> {
        self.distributed_tracing.start_span(service_name, operation).await
    }

    /// 获取配置
    pub async fn get_config(&self, key: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.config_center.get_config(key).await
    }

    /// 设置配置
    pub async fn set_config(&self, key: &str, value: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        self.config_center.set_config(key, value).await
    }
}
