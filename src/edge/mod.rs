//! 边缘计算支持模块
//! 用于边缘节点部署、边缘缓存和计算、边缘设备管理

pub mod node_deployment;
pub mod edge_cache;
pub mod device_management;

/// 边缘计算管理器
#[derive(Debug, Clone)]
pub struct EdgeComputingManager {
    node_deployment: node_deployment::NodeDeployment,
    edge_cache: edge_cache::EdgeCacheManager,
    device_management: device_management::DeviceManager,
}

impl EdgeComputingManager {
    /// 创建新的边缘计算管理器
    pub fn new() -> Self {
        Self {
            node_deployment: node_deployment::NodeDeployment::new(),
            edge_cache: edge_cache::EdgeCacheManager::new(),
            device_management: device_management::DeviceManager::new(),
        }
    }

    /// 初始化边缘计算
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.node_deployment.initialize().await?;
        self.edge_cache.initialize().await?;
        self.device_management.initialize().await?;
        Ok(())
    }

    /// 部署边缘节点
    pub async fn deploy_node(&self, node_config: node_deployment::NodeConfig) -> Result<node_deployment::NodeDeploymentResult, Box<dyn std::error::Error>> {
        self.node_deployment.deploy_node(node_config).await
    }

    /// 管理边缘缓存
    pub async fn manage_cache(&self, cache_config: edge_cache::CacheConfig) -> Result<edge_cache::CacheOperationResult, Box<dyn std::error::Error>> {
        self.edge_cache.manage_cache(cache_config).await
    }

    /// 管理边缘设备
    pub async fn manage_device(&self, device_op: device_management::DeviceOperation) -> Result<device_management::DeviceOperationResult, Box<dyn std::error::Error>> {
        self.device_management.manage_device(device_op).await
    }
}
