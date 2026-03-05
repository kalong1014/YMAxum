pub mod core; pub mod storage; pub mod api;

/// 用户成长与权限系统模块
/// 
/// 负责管理用户的等级体系、权限控制和成长路径
/// 支持等级配置、权限管理、成长任务、等级晋升等功能
#[derive(Debug)]
pub struct UserModule {
    core: core::UserCore,
    storage: storage::UserStorage,
    api: api::UserApi,
}

impl UserModule {
    /// 创建新的用户成长与权限系统模块
    pub fn new() -> Self {
        Self {
            core: core::UserCore::new(),
            storage: storage::UserStorage::new(),
            api: api::UserApi::new(),
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
    pub fn core(&self) -> &core::UserCore {
        &self.core
    }
    
    /// 获取存储模块
    pub fn storage(&self) -> &storage::UserStorage {
        &self.storage
    }
    
    /// 获取API模块
    pub fn api(&self) -> &api::UserApi {
        &self.api
    }
}
