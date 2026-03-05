//! 大数据处理模块
//! 用于数据采集和存储、数据分析和可视化、机器学习模型训练

pub mod data_acquisition;
pub mod data_storage;
pub mod data_analysis;
pub mod machine_learning;

/// 大数据处理管理器
#[derive(Debug, Clone)]
pub struct BigDataManager {
    data_acquisition: data_acquisition::DataAcquisitionService,
    data_storage: data_storage::DataStorageService,
    data_analysis: data_analysis::DataAnalysisService,
    machine_learning: machine_learning::MachineLearningService,
}

impl BigDataManager {
    /// 创建新的大数据处理管理器
    pub fn new() -> Self {
        Self {
            data_acquisition: data_acquisition::DataAcquisitionService::new(),
            data_storage: data_storage::DataStorageService::new(),
            data_analysis: data_analysis::DataAnalysisService::new(),
            machine_learning: machine_learning::MachineLearningService::new(),
        }
    }

    /// 初始化大数据处理
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.data_acquisition.initialize().await?;
        self.data_storage.initialize().await?;
        self.data_analysis.initialize().await?;
        self.machine_learning.initialize().await?;
        Ok(())
    }

    /// 采集数据
    pub async fn acquire_data(&self, config: data_acquisition::AcquisitionConfig) -> Result<data_acquisition::AcquisitionResult, Box<dyn std::error::Error>> {
        self.data_acquisition.acquire_data(config).await
    }

    /// 存储数据
    pub async fn store_data(&self, data: data_storage::StorageData) -> Result<data_storage::StorageResult, Box<dyn std::error::Error>> {
        self.data_storage.store_data(data).await
    }

    /// 分析数据
    pub async fn analyze_data(&self, request: data_analysis::AnalysisRequest) -> Result<data_analysis::AnalysisResult, Box<dyn std::error::Error>> {
        self.data_analysis.analyze_data(request).await
    }

    /// 训练机器学习模型
    pub async fn train_model(&self, request: machine_learning::TrainingRequest) -> Result<machine_learning::TrainingResult, Box<dyn std::error::Error>> {
        self.machine_learning.train_model(request).await
    }
}
