pub mod core; pub mod storage; pub mod api;

/// 防欺诈保障系统模块
/// 
/// 负责保障交易安全，防止欺诈行为，提供仲裁机制
/// 支持风险评估、交易监控、异常检测、仲裁管理等功能
#[derive(Debug)]
pub struct FraudModule {
    core: core::FraudCore,
    storage: storage::FraudStorage,
    api: api::FraudApi,
}

impl FraudModule {
    /// 创建新的防欺诈保障系统模块
    pub fn new() -> Self {
        Self {
            core: core::FraudCore::new(),
            storage: storage::FraudStorage::new(),
            api: api::FraudApi::new(),
        }
    }
    
    /// 初始化模块
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.core.init().await?;
        self.storage.init().await?;
        self.api.init().await?;
        Ok(())
    }
    
    /// 获取核心逻辑
    pub fn core(&self) -> &core::FraudCore {
        &self.core
    }
    
    /// 获取存储模块
    pub fn storage(&self) -> &storage::FraudStorage {
        &self.storage
    }
    
    /// 获取API模块
    pub fn api(&self) -> &api::FraudApi {
        &self.api
    }
}
