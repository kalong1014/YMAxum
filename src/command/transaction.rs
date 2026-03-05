use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct TransactionManager {
    transactions: Arc<RwLock<Vec<Transaction>>>,
    current_transaction: Arc<RwLock<Option<usize>>>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    _id: usize,
    operations: Vec<TransactionOperation>,
    status: TransactionStatus,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Active,
    Committed,
    RolledBack,
}

#[derive(Debug, Clone)]
pub enum TransactionOperation {
    CreateFile(String),
    DeleteFile(String, Option<String>), // 第二个参数是文件备份内容
    UpdateConfig(String, String, String), // key, old_value, new_value
                                        // 其他操作类型
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(Vec::new())),
            current_transaction: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn begin_transaction(&self) -> Result<usize> {
        let mut transactions = self.transactions.write().await;
        let mut current_transaction = self.current_transaction.write().await;

        let transaction_id = transactions.len();
        transactions.push(Transaction {
            _id: transaction_id,
            operations: Vec::new(),
            status: TransactionStatus::Active,
        });

        *current_transaction = Some(transaction_id);
        Ok(transaction_id)
    }

    pub async fn commit_transaction(&self) -> Result<()> {
        let mut transactions = self.transactions.write().await;
        let mut current_transaction = self.current_transaction.write().await;

        if let Some(tx_id) = *current_transaction {
            if tx_id < transactions.len() {
                transactions[tx_id].status = TransactionStatus::Committed;
            }
            *current_transaction = None;
        }

        Ok(())
    }

    pub async fn rollback_transaction(&self) -> Result<()> {
        let mut transactions = self.transactions.write().await;
        let mut current_transaction = self.current_transaction.write().await;

        if let Some(tx_id) = *current_transaction {
            if tx_id < transactions.len() {
                let transaction = &mut transactions[tx_id];

                // 执行回滚操作
                for op in transaction.operations.iter().rev() {
                    self.rollback_operation(op).await?;
                }

                transaction.status = TransactionStatus::RolledBack;
            }
            *current_transaction = None;
        }

        Ok(())
    }

    pub async fn add_operation(&self, operation: TransactionOperation) -> Result<()> {
        let mut transactions = self.transactions.write().await;
        let current_transaction = self.current_transaction.read().await;

        if let Some(tx_id) = *current_transaction
            && tx_id < transactions.len()
        {
            transactions[tx_id].operations.push(operation);
        }

        Ok(())
    }

    async fn rollback_operation(&self, operation: &TransactionOperation) -> Result<()> {
        // 实现具体的回滚逻辑
        match operation {
            TransactionOperation::CreateFile(path) => {
                if std::path::Path::new(path).exists() {
                    std::fs::remove_file(path)
                        .context(format!("Failed to rollback create file: {}", path))?;
                }
            }
            TransactionOperation::DeleteFile(path, backup_content) => {
                // 恢复文件
                if let Some(content) = backup_content {
                    std::fs::write(path, content)
                        .context(format!("Failed to rollback delete file: {}", path))?;
                }
            }
            TransactionOperation::UpdateConfig(key, old_value, _new_value) => {
                // 恢复配置到旧值
                // 这里需要实现具体的配置更新逻辑
                // 例如，写入配置文件或更新内存中的配置
                println!("Rolling back config {} to {}", key, old_value);
            }
        }

        Ok(())
    }

    pub async fn get_transaction_status(&self, tx_id: usize) -> Option<TransactionStatus> {
        let transactions = self.transactions.read().await;
        if tx_id < transactions.len() {
            Some(transactions[tx_id].status.clone())
        } else {
            None
        }
    }
}
