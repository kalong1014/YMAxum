/// 钱包相关组件
///
/// 包含钱包连接、余额显示和交易记录等组件

use serde::{Deserialize, Serialize};

/// 钱包连接组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectComponent {
    /// 组件ID
    pub id: String,
    /// 组件类型
    pub component_type: String,
    /// 组件属性
    pub props: WalletConnectProps,
}

/// 钱包连接属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectProps {
    /// 支持的钱包列表
    pub supported_wallets: Vec<String>,
    /// 连接状态
    pub connected: bool,
    /// 当前钱包地址
    pub current_address: Option<String>,
    /// 网络名称
    pub network: String,
    /// 回调函数
    pub on_connect: Option<String>,
    pub on_disconnect: Option<String>,
}

/// 钱包余额组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalanceComponent {
    /// 组件ID
    pub id: String,
    /// 组件类型
    pub component_type: String,
    /// 组件属性
    pub props: WalletBalanceProps,
}

/// 钱包余额属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalanceProps {
    /// 钱包地址
    pub address: String,
    /// 余额
    pub balance: String,
    /// 代币符号
    pub token_symbol: String,
    /// 网络名称
    pub network: String,
    /// 刷新间隔（秒）
    pub refresh_interval: u64,
}

/// 钱包交易记录组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionsComponent {
    /// 组件ID
    pub id: String,
    /// 组件类型
    pub component_type: String,
    /// 组件属性
    pub props: WalletTransactionsProps,
}

/// 钱包交易记录属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionsProps {
    /// 钱包地址
    pub address: String,
    /// 交易列表
    pub transactions: Vec<TransactionItem>,
    /// 网络名称
    pub network: String,
    /// 分页大小
    pub page_size: u32,
    /// 当前页码
    pub current_page: u32,
}

/// 交易项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionItem {
    /// 交易哈希
    pub hash: String,
    /// 交易类型
    pub transaction_type: String,
    /// 金额
    pub amount: String,
    /// 时间戳
    pub timestamp: String,
    /// 状态
    pub status: String,
    /// 对方地址
    pub to_address: Option<String>,
    /// 来源地址
    pub from_address: Option<String>,
}

/// 创建钱包连接组件
pub fn create_wallet_connect_component(
    id: &str,
    supported_wallets: Vec<String>,
    network: &str
) -> WalletConnectComponent {
    WalletConnectComponent {
        id: id.to_string(),
        component_type: "wallet_connect".to_string(),
        props: WalletConnectProps {
            supported_wallets,
            connected: false,
            current_address: None,
            network: network.to_string(),
            on_connect: None,
            on_disconnect: None,
        },
    }
}

/// 创建钱包余额组件
pub fn create_wallet_balance_component(
    id: &str,
    address: &str,
    network: &str
) -> WalletBalanceComponent {
    WalletBalanceComponent {
        id: id.to_string(),
        component_type: "wallet_balance".to_string(),
        props: WalletBalanceProps {
            address: address.to_string(),
            balance: "0".to_string(),
            token_symbol: "ETH".to_string(),
            network: network.to_string(),
            refresh_interval: 30,
        },
    }
}

/// 创建钱包交易记录组件
pub fn create_wallet_transactions_component(
    id: &str,
    address: &str,
    network: &str
) -> WalletTransactionsComponent {
    WalletTransactionsComponent {
        id: id.to_string(),
        component_type: "wallet_transactions".to_string(),
        props: WalletTransactionsProps {
            address: address.to_string(),
            transactions: Vec::new(),
            network: network.to_string(),
            page_size: 10,
            current_page: 1,
        },
    }
}

/// 模拟钱包连接
pub async fn mock_wallet_connect(address: &str) -> Result<String, String> {
    // 模拟钱包连接过程
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(address.to_string())
}

/// 模拟获取钱包余额
pub async fn mock_get_wallet_balance(address: &str) -> Result<String, String> {
    // 模拟获取余额过程
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(format!("{:.4}", rand::random::<f64>() * 10))
}

/// 模拟获取交易记录
pub async fn mock_get_transactions(address: &str, page: u32, page_size: u32) -> Result<Vec<TransactionItem>, String> {
    // 模拟获取交易记录过程
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    let mut transactions = Vec::new();
    for i in 0..page_size {
        let tx_hash = format!("0x{:x}", rand::random::<u128>());
        let amount = format!("{:.4}", rand::random::<f64>() * 1);
        let timestamp = chrono::Utc::now().to_string();
        
        transactions.push(TransactionItem {
            hash: tx_hash,
            transaction_type: if rand::random::<bool>() { "send".to_string() } else { "receive".to_string() },
            amount,
            timestamp,
            status: "completed".to_string(),
            to_address: Some(format!("0x{:x}", rand::random::<u64>())),
            from_address: Some(address.to_string()),
        });
    }
    
    Ok(transactions)
}