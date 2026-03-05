pub mod core; pub mod storage; pub mod api;

/// 积分生态系统模块
/// 
/// 负责管理用户积分的获取、消耗、转让等功能
/// 支持任务奖励、创作奖励、交易消耗、积分兑换等场景
#[derive(Debug)]
pub struct PointsModule {
    core: core::PointsCore,
    storage: storage::PointsStorage,
    api: api::PointsApi,
}

impl PointsModule {
    /// 创建新的积分生态系统模块
    pub fn new() -> Self {
        Self {
            core: core::PointsCore::new(),
            storage: storage::PointsStorage::new(),
            api: api::PointsApi::new(),
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
    pub fn core(&self) -> &core::PointsCore {
        &self.core
    }
    
    /// 获取存储模块
    pub fn storage(&self) -> &storage::PointsStorage {
        &self.storage
    }
    
    /// 获取API模块
    pub fn api(&self) -> &api::PointsApi {
        &self.api
    }
}
