//! 物联网支持模块
//! 用于设备管理和监控、数据采集和分析、边缘计算和云协同

pub mod device_management;
pub mod data_collection;
pub mod edge_cloud;

/// 物联网支持管理器
#[derive(Debug, Clone)]
pub struct IotManager {
    device_management: device_management::IotDeviceManager,
    data_collection: data_collection::DataCollector,
    edge_cloud: edge_cloud::EdgeCloudCoordinator,
}

impl IotManager {
    /// 创建新的物联网支持管理器
    pub fn new() -> Self {
        Self {
            device_management: device_management::IotDeviceManager::new(),
            data_collection: data_collection::DataCollector::new(),
            edge_cloud: edge_cloud::EdgeCloudCoordinator::new(),
        }
    }

    /// 初始化物联网支持
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.device_management.initialize().await?;
        self.data_collection.initialize().await?;
        self.edge_cloud.initialize().await?;
        Ok(())
    }

    /// 管理物联网设备
    pub async fn manage_device(&self, device_op: device_management::DeviceOperation) -> Result<device_management::DeviceOperationResult, Box<dyn std::error::Error>> {
        self.device_management.manage_device(device_op).await
    }

    /// 采集物联网数据
    pub async fn collect_data(&self, collection_config: data_collection::CollectionConfig) -> Result<data_collection::CollectionResult, Box<dyn std::error::Error>> {
        self.data_collection.collect_data(collection_config).await
    }

    /// 协调边缘计算和云服务
    pub async fn coordinate_edge_cloud(&self, coordination_config: edge_cloud::CoordinationConfig) -> Result<edge_cloud::CoordinationResult, Box<dyn std::error::Error>> {
        self.edge_cloud.coordinate(coordination_config).await
    }
}
