//! 区块链集成模块
//! 用于智能合约部署和管理、区块链数据存储、去中心化身份验证

pub mod smart_contract;
pub mod data_storage;
pub mod identity;

/// 区块链集成管理器
#[derive(Debug, Clone)]
pub struct BlockchainManager {
    smart_contract: smart_contract::SmartContractManager,
    data_storage: data_storage::BlockchainStorage,
    identity: identity::DecentralizedIdentity,
}

impl BlockchainManager {
    /// 创建新的区块链集成管理器
    pub fn new() -> Self {
        Self {
            smart_contract: smart_contract::SmartContractManager::new(),
            data_storage: data_storage::BlockchainStorage::new(),
            identity: identity::DecentralizedIdentity::new(),
        }
    }

    /// 初始化区块链集成
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.smart_contract.initialize().await?;
        self.data_storage.initialize().await?;
        self.identity.initialize().await?;
        Ok(())
    }

    /// 部署智能合约
    pub async fn deploy_contract(&self, contract: smart_contract::ContractInfo) -> Result<smart_contract::ContractDeploymentResult, Box<dyn std::error::Error>> {
        self.smart_contract.deploy_contract(contract).await
    }

    /// 存储数据到区块链
    pub async fn store_data(&self, data: data_storage::BlockchainData) -> Result<data_storage::StorageResult, Box<dyn std::error::Error>> {
        self.data_storage.store_data(data).await
    }

    /// 验证去中心化身份
    pub async fn verify_identity(&self, identity_info: identity::IdentityInfo) -> Result<identity::IdentityVerificationResult, Box<dyn std::error::Error>> {
        self.identity.verify_identity(identity_info).await
    }
}
