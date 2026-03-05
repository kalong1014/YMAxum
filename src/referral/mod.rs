pub mod core; pub mod storage; pub mod api;

/// 推广引流和刺激裂变系统模块
/// 
/// 负责促进用户增长，通过推广和裂变机制吸引新用户
/// 支持邀请码生成、推广奖励、裂变追踪、团队管理等功能
#[derive(Debug)]
pub struct ReferralModule {
    core: core::ReferralCore,
    storage: storage::ReferralStorage,
    api: api::ReferralApi,
}

impl ReferralModule {
    /// 创建新的推广引流和刺激裂变系统模块
    pub fn new() -> Self {
        Self {
            core: core::ReferralCore::new(),
            storage: storage::ReferralStorage::new(),
            api: api::ReferralApi::new(),
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
    pub fn core(&self) -> &core::ReferralCore {
        &self.core
    }
    
    /// 获取存储模块
    pub fn storage(&self) -> &storage::ReferralStorage {
        &self.storage
    }
    
    /// 获取API模块
    pub fn api(&self) -> &api::ReferralApi {
        &self.api
    }
}
