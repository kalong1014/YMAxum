use serde::{Deserialize, Serialize}; use std::sync::Arc; use tokio::sync::RwLock;

/// 交易类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionType {
    /// 积分转让
    PointsTransfer,
    /// 成果交易
    WorkTransaction,
    /// 任务完成
    TaskCompletion,
    /// 其他交易
    Other,
}

/// 交易状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionStatus {
    /// 待处理
    Pending,
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 异常
    Abnormal,
    /// 仲裁中
    Arbitrating,
}

/// 风险等级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum RiskLevel {
    /// 低风险
    Low,
    /// 中风险
    Medium,
    /// 高风险
    High,
    /// 极高风险
    Critical,
}

/// 交易信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    /// 交易ID
    pub id: String,
    /// 交易类型
    pub transaction_type: TransactionType,
    /// 发起方ID
    pub from_user_id: String,
    /// 接收方ID
    pub to_user_id: String,
    /// 交易金额/数量
    pub amount: f64,
    /// 交易时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 交易状态
    pub status: TransactionStatus,
    /// 风险等级
    pub risk_level: RiskLevel,
    /// 交易描述
    pub description: String,
    /// 相关业务ID
    pub business_id: Option<String>,
    /// 异常原因
    pub abnormal_reason: Option<String>,
    /// 仲裁ID
    pub arbitration_id: Option<String>,
}

/// 仲裁状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ArbitrationStatus {
    /// 待受理
    Pending,
    /// 处理中
    Processing,
    /// 已裁决
    Decided,
    /// 已撤销
    Canceled,
}

/// 仲裁结果
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ArbitrationResult {
    /// 支持发起方
    SupportFrom,
    /// 支持接收方
    SupportTo,
    /// 双方各担责任
    SharedResponsibility,
    /// 无法裁决
    Undecidable,
}

/// 仲裁信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Arbitration {
    /// 仲裁ID
    pub id: String,
    /// 交易ID
    pub transaction_id: String,
    /// 发起方ID
    pub from_user_id: String,
    /// 接收方ID
    pub to_user_id: String,
    /// 仲裁发起方ID
    pub initiator_id: String,
    /// 仲裁时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 仲裁状态
    pub status: ArbitrationStatus,
    /// 仲裁结果
    pub result: Option<ArbitrationResult>,
    /// 仲裁理由
    pub reason: String,
    /// 证据列表
    pub evidences: Vec<String>,
    /// 裁决时间
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 裁决理由
    pub decision_reason: Option<String>,
}

/// 风险规则
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskRule {
    /// 规则ID
    pub id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则阈值
    pub threshold: f64,
    /// 风险等级
    pub risk_level: RiskLevel,
    /// 规则类型
    pub rule_type: String,
    /// 是否启用
    pub enabled: bool,
}

/// 防欺诈保障核心逻辑
#[derive(Debug)]
pub struct FraudCore {
    /// 交易记录
    transactions: Arc<RwLock<Vec<Transaction>>>,
    /// 仲裁记录
    arbitrations: Arc<RwLock<Vec<Arbitration>>>,
    /// 风险规则
    risk_rules: Arc<RwLock<Vec<RiskRule>>>,
}

impl FraudCore {
    /// 创建新的核心逻辑实例
    pub fn new() -> Self {
        let mut risk_rules = Vec::new();
        // 默认风险规则
        risk_rules.push(RiskRule {
            id: "large_amount".to_string(),
            name: "大额交易".to_string(),
            description: "交易金额超过阈值".to_string(),
            threshold: 10000.0,
            risk_level: RiskLevel::High,
            rule_type: "amount".to_string(),
            enabled: true,
        });
        risk_rules.push(RiskRule {
            id: "rapid_transactions".to_string(),
            name: "快速连续交易".to_string(),
            description: "短时间内多次交易".to_string(),
            threshold: 5.0,
            risk_level: RiskLevel::Medium,
            rule_type: "frequency".to_string(),
            enabled: true,
        });
        risk_rules.push(RiskRule {
            id: "new_user".to_string(),
            name: "新用户交易".to_string(),
            description: "注册时间短的用户交易".to_string(),
            threshold: 24.0,
            risk_level: RiskLevel::Medium,
            rule_type: "user_age".to_string(),
            enabled: true,
        });
        
        Self {
            transactions: Arc::new(RwLock::new(Vec::new())),
            arbitrations: Arc::new(RwLock::new(Vec::new())),
            risk_rules: Arc::new(RwLock::new(risk_rules)),
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化逻辑
        Ok(())
    }
    
    /// 评估交易风险
    pub async fn assess_transaction_risk(
        &self,
        from_user_id: String,
        to_user_id: String,
        amount: f64,
        transaction_type: TransactionType,
        description: String,
        business_id: Option<String>,
    ) -> Result<(Transaction, RiskLevel), Box<dyn std::error::Error>> {
        let risk_rules = self.risk_rules.read().await;
        
        // 评估风险等级
        let mut risk_level = RiskLevel::Low;
        
        // 检查大额交易规则
        if let Some(rule) = risk_rules.iter().find(|r| r.id == "large_amount" && r.enabled) {
            if amount > rule.threshold {
                risk_level = rule.risk_level.clone();
            }
        }
        
        // 检查快速连续交易规则
        let transactions = self.transactions.read().await;
        let recent_transactions = transactions
            .iter()
            .filter(|t| t.from_user_id == from_user_id)
            .filter(|t| {
                let time_diff = chrono::Utc::now().signed_duration_since(t.created_at);
                time_diff.num_hours() < 1
            })
            .count();
        
        if let Some(rule) = risk_rules.iter().find(|r| r.id == "rapid_transactions" && r.enabled) {
            if recent_transactions as f64 > rule.threshold {
                if risk_level < rule.risk_level {
                    risk_level = rule.risk_level.clone();
                }
            }
        }
        
        // 创建交易记录
        let transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_type,
            from_user_id,
            to_user_id,
            amount,
            created_at: chrono::Utc::now(),
            status: TransactionStatus::Pending,
            risk_level: risk_level.clone(),
            description,
            business_id,
            abnormal_reason: None,
            arbitration_id: None,
        };
        
        let mut transactions_write = self.transactions.write().await;
        transactions_write.push(transaction.clone());
        
        Ok((transaction, risk_level))
    }
    
    /// 标记交易异常
    pub async fn mark_transaction_abnormal(
        &self,
        transaction_id: String,
        reason: String,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let mut transactions = self.transactions.write().await;
        
        let transaction_index = transactions
            .iter_mut()
            .position(|t| t.id == transaction_id)
            .ok_or("Transaction not found")?;
        
        let transaction = &mut transactions[transaction_index];
        transaction.status = TransactionStatus::Abnormal;
        transaction.abnormal_reason = Some(reason);
        
        Ok(transaction.clone())
    }
    
    /// 处理交易
    pub async fn process_transaction(
        &self,
        transaction_id: String,
        approve: bool,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let mut transactions = self.transactions.write().await;
        
        let transaction_index = transactions
            .iter_mut()
            .position(|t| t.id == transaction_id)
            .ok_or("Transaction not found")?;
        
        let transaction = &mut transactions[transaction_index];
        if approve {
            transaction.status = TransactionStatus::Success;
        } else {
            transaction.status = TransactionStatus::Failed;
        }
        
        Ok(transaction.clone())
    }
    
    /// 发起仲裁
    pub async fn initiate_arbitration(
        &self,
        transaction_id: String,
        initiator_id: String,
        reason: String,
        evidences: Vec<String>,
    ) -> Result<Arbitration, Box<dyn std::error::Error>> {
        let transactions = self.transactions.read().await;
        let transaction = transactions
            .iter()
            .find(|t| t.id == transaction_id)
            .ok_or("Transaction not found")?;
        
        // 创建仲裁记录
        let arbitration = Arbitration {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_id: transaction_id.clone(),
            from_user_id: transaction.from_user_id.clone(),
            to_user_id: transaction.to_user_id.clone(),
            initiator_id,
            created_at: chrono::Utc::now(),
            status: ArbitrationStatus::Pending,
            result: None,
            reason,
            evidences,
            decided_at: None,
            decision_reason: None,
        };
        
        let mut arbitrations = self.arbitrations.write().await;
        arbitrations.push(arbitration.clone());
        
        // 更新交易状态为仲裁中
        let mut transactions_write = self.transactions.write().await;
        if let Some(t) = transactions_write.iter_mut().find(|t| t.id == transaction_id) {
            t.status = TransactionStatus::Arbitrating;
            t.arbitration_id = Some(arbitration.id.clone());
        }
        
        Ok(arbitration)
    }
    
    /// 处理仲裁
    pub async fn process_arbitration(
        &self,
        arbitration_id: String,
        result: ArbitrationResult,
        decision_reason: String,
    ) -> Result<Arbitration, Box<dyn std::error::Error>> {
        let mut arbitrations = self.arbitrations.write().await;
        
        let arbitration_index = arbitrations
            .iter_mut()
            .position(|a| a.id == arbitration_id)
            .ok_or("Arbitration not found")?;
        
        let arbitration = &mut arbitrations[arbitration_index];
        arbitration.status = ArbitrationStatus::Decided;
        arbitration.result = Some(result);
        arbitration.decided_at = Some(chrono::Utc::now());
        arbitration.decision_reason = Some(decision_reason);
        
        // 更新交易状态
        let mut transactions = self.transactions.write().await;
        let arbitration_id_clone = arbitration_id.clone();
        if let Some(t) = transactions.iter_mut().find(|t| t.arbitration_id == Some(arbitration_id_clone.clone())) {
            t.status = TransactionStatus::Success;
        }
        
        Ok(arbitration.clone())
    }
    
    /// 获取交易信息
    pub async fn get_transaction(
        &self,
        transaction_id: String,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let transactions = self.transactions.read().await;
        transactions
            .iter()
            .find(|t| t.id == transaction_id)
            .cloned()
            .ok_or("Transaction not found".into())
    }
    
    /// 获取用户交易列表
    pub async fn get_user_transactions(
        &self,
        user_id: String,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let transactions = self.transactions.read().await;
        
        let user_transactions: Vec<Transaction> = transactions
            .iter()
            .filter(|t| t.from_user_id == user_id || t.to_user_id == user_id)
            .cloned()
            .collect();
        
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > user_transactions.len() {
            user_transactions.len()
        } else {
            end
        };
        
        Ok(user_transactions[start..end].to_vec())
    }
    
    /// 获取仲裁信息
    pub async fn get_arbitration(
        &self,
        arbitration_id: String,
    ) -> Result<Arbitration, Box<dyn std::error::Error>> {
        let arbitrations = self.arbitrations.read().await;
        arbitrations
            .iter()
            .find(|a| a.id == arbitration_id)
            .cloned()
            .ok_or("Arbitration not found".into())
    }
    
    /// 获取用户仲裁列表
    pub async fn get_user_arbitrations(
        &self,
        user_id: String,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<Arbitration>, Box<dyn std::error::Error>> {
        let arbitrations = self.arbitrations.read().await;
        
        let user_arbitrations: Vec<Arbitration> = arbitrations
            .iter()
            .filter(|a| a.from_user_id == user_id || a.to_user_id == user_id || a.initiator_id == user_id)
            .cloned()
            .collect();
        
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > user_arbitrations.len() {
            user_arbitrations.len()
        } else {
            end
        };
        
        Ok(user_arbitrations[start..end].to_vec())
    }
    
    /// 获取风险规则
    pub async fn get_risk_rules(
        &self,
    ) -> Result<Vec<RiskRule>, Box<dyn std::error::Error>> {
        let risk_rules = self.risk_rules.read().await;
        Ok(risk_rules.clone())
    }
    
    /// 更新风险规则
    pub async fn update_risk_rule(
        &self,
        rule: RiskRule,
    ) -> Result<RiskRule, Box<dyn std::error::Error>> {
        let mut risk_rules = self.risk_rules.write().await;
        
        let rule_index = risk_rules
            .iter_mut()
            .position(|r| r.id == rule.id)
            .unwrap_or_else(|| {
                risk_rules.push(rule.clone());
                risk_rules.len() - 1
            });
        
        risk_rules[rule_index] = rule.clone();
        
        Ok(rule)
    }
    
    /// 检测异常交易
    pub async fn detect_abnormal_transactions(
        &self,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let transactions = self.transactions.read().await;
        
        // 检测异常交易
        let abnormal_transactions: Vec<Transaction> = transactions
            .iter()
            .filter(|t| t.risk_level >= RiskLevel::High)
            .filter(|t| t.status == TransactionStatus::Pending)
            .cloned()
            .collect();
        
        Ok(abnormal_transactions)
    }
}
