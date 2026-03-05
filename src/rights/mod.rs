pub mod core;
pub mod blockchain;
pub mod storage;
pub mod api;

/// 去中心化确权系统模块
/// 
/// 负责为各类数字资产提供安全、可靠的权属证明方案
/// 支持成果发布、一键确权申请、链上存证、确权展示、权属变更、权益配置等功能
#[derive(Debug)]
pub struct RightsModule {
    core: core::RightsCore,
    blockchain: blockchain::BlockchainClient,
    storage: storage::RightsStorage,
    api: api::RightsApi,
}

impl RightsModule {
    /// 创建新的去中心化确权系统模块
    pub fn new() -> Self {
        Self {
            core: core::RightsCore::new(),
            blockchain: blockchain::BlockchainClient::new(),
            storage: storage::RightsStorage::new(),
            api: api::RightsApi::new(),
        }
    }
    
    /// 初始化模块
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.core.init().await?;
        self.blockchain.init().await?;
        self.storage.init().await?;
        self.api.init().await?;
        Ok(())
    }
    
    /// 获取核心逻辑
    pub fn core(&self) -> &core::RightsCore {
        &self.core
    }
    
    /// 获取区块链客户端
    pub fn blockchain(&self) -> &blockchain::BlockchainClient {
        &self.blockchain
    }
    
    /// 获取存储模块
    pub fn storage(&self) -> &storage::RightsStorage {
        &self.storage
    }
    
    /// 获取API模块
    pub fn api(&self) -> &api::RightsApi {
        &self.api
    }
}
