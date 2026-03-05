/// DAPP前端组件模块
///
/// 包含与区块链DAPP相关的前端组件，如钱包连接、交易、合约调用等
pub mod wallet;
pub mod transaction;
pub mod contract;
pub mod nft;

/// DAPP组件类型枚举
pub enum DappComponentType {
    // 钱包相关组件
    WalletConnect,
    WalletBalance,
    WalletTransactions,
    
    // 交易相关组件
    TransactionSender,
    TransactionHistory,
    TransactionStatus,
    
    // 合约相关组件
    ContractCaller,
    ContractEvents,
    ContractDeployer,
    
    // NFT相关组件
    NftGallery,
    NftMinter,
    NftTransfer,
}

impl DappComponentType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            DappComponentType::WalletConnect => "wallet_connect",
            DappComponentType::WalletBalance => "wallet_balance",
            DappComponentType::WalletTransactions => "wallet_transactions",
            DappComponentType::TransactionSender => "transaction_sender",
            DappComponentType::TransactionHistory => "transaction_history",
            DappComponentType::TransactionStatus => "transaction_status",
            DappComponentType::ContractCaller => "contract_caller",
            DappComponentType::ContractEvents => "contract_events",
            DappComponentType::ContractDeployer => "contract_deployer",
            DappComponentType::NftGallery => "nft_gallery",
            DappComponentType::NftMinter => "nft_minter",
            DappComponentType::NftTransfer => "nft_transfer",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "wallet_connect" => Some(DappComponentType::WalletConnect),
            "wallet_balance" => Some(DappComponentType::WalletBalance),
            "wallet_transactions" => Some(DappComponentType::WalletTransactions),
            "transaction_sender" => Some(DappComponentType::TransactionSender),
            "transaction_history" => Some(DappComponentType::TransactionHistory),
            "transaction_status" => Some(DappComponentType::TransactionStatus),
            "contract_caller" => Some(DappComponentType::ContractCaller),
            "contract_events" => Some(DappComponentType::ContractEvents),
            "contract_deployer" => Some(DappComponentType::ContractDeployer),
            "nft_gallery" => Some(DappComponentType::NftGallery),
            "nft_minter" => Some(DappComponentType::NftMinter),
            "nft_transfer" => Some(DappComponentType::NftTransfer),
            _ => None,
        }
    }
}

/// DAPP组件管理器
pub struct DappComponentManager {
    /// 已注册的DAPP组件
    components: std::collections::HashMap<String, serde_json::Value>,
}

impl DappComponentManager {
    /// 创建新的DAPP组件管理器
    pub fn new() -> Self {
        Self {
            components: std::collections::HashMap::new(),
        }
    }

    /// 注册DAPP组件
    pub fn register_component(&mut self, component_id: &str, component: serde_json::Value) {
        self.components.insert(component_id.to_string(), component);
    }

    /// 获取DAPP组件
    pub fn get_component(&self, component_id: &str) -> Option<&serde_json::Value> {
        self.components.get(component_id)
    }

    /// 更新DAPP组件
    pub fn update_component(&mut self, component_id: &str, component: serde_json::Value) -> bool {
        if self.components.contains_key(component_id) {
            self.components.insert(component_id.to_string(), component);
            true
        } else {
            false
        }
    }

    /// 移除DAPP组件
    pub fn remove_component(&mut self, component_id: &str) -> bool {
        self.components.remove(component_id).is_some()
    }

    /// 获取所有DAPP组件
    pub fn get_all_components(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.components
    }
}

/// 全局DAPP组件管理器
static DAPP_COMPONENT_MANAGER: tokio::sync::OnceCell<tokio::sync::RwLock<DappComponentManager>> = 
    tokio::sync::OnceCell::const_new();

/// 获取DAPP组件管理器
pub async fn get_dapp_component_manager() -> &'static tokio::sync::RwLock<DappComponentManager> {
    DAPP_COMPONENT_MANAGER
        .get_or_init(|| async { tokio::sync::RwLock::new(DappComponentManager::new()) })
        .await
}

/// 初始化DAPP组件管理器
pub async fn initialize() -> Result<(), crate::error::Error> {
    let manager = get_dapp_component_manager().await;
    log::info!("DAPP component manager initialized");
    Ok(())
}

/// 注册DAPP组件
pub async fn register_dapp_component(component_id: &str, component: serde_json::Value) {
    let manager = get_dapp_component_manager().await;
    let mut dapp_manager = manager.write().await;
    dapp_manager.register_component(component_id, component);
}

/// 获取DAPP组件
pub async fn get_dapp_component(component_id: &str) -> Option<serde_json::Value> {
    let manager = get_dapp_component_manager().await;
    let dapp_manager = manager.read().await;
    dapp_manager.get_component(component_id).cloned()
}

/// 更新DAPP组件
pub async fn update_dapp_component(component_id: &str, component: serde_json::Value) -> bool {
    let manager = get_dapp_component_manager().await;
    let mut dapp_manager = manager.write().await;
    dapp_manager.update_component(component_id, component)
}

/// 移除DAPP组件
pub async fn remove_dapp_component(component_id: &str) -> bool {
    let manager = get_dapp_component_manager().await;
    let mut dapp_manager = manager.write().await;
    dapp_manager.remove_component(component_id)
}

/// 获取所有DAPP组件
pub async fn get_all_dapp_components() -> std::collections::HashMap<String, serde_json::Value> {
    let manager = get_dapp_component_manager().await;
    let dapp_manager = manager.read().await;
    dapp_manager.get_all_components().clone()
}