use axum::{Router}; use serde::{Deserialize, Serialize}; use crate::fraud::core::{FraudCore, TransactionType, ArbitrationResult}; use crate::fraud::storage::FraudStorage;

/// API请求和响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct AssessRiskRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub description: String,
    pub business_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkAbnormalRequest {
    pub transaction_id: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessTransactionRequest {
    pub transaction_id: String,
    pub approve: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitiateArbitrationRequest {
    pub transaction_id: String,
    pub initiator_id: String,
    pub reason: String,
    pub evidences: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessArbitrationRequest {
    pub arbitration_id: String,
    pub result: ArbitrationResult,
    pub decision_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionRequest {
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserTransactionsRequest {
    pub user_id: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetArbitrationRequest {
    pub arbitration_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserArbitrationsRequest {
    pub user_id: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRiskRulesRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 防欺诈保障API模块
#[derive(Debug)]
#[allow(dead_code)]
pub struct FraudApi {
    core: FraudCore,
    storage: FraudStorage,
    router: Router,
}

#[allow(dead_code)]
impl FraudApi {
    /// 创建新的API模块
    pub fn new() -> Self {
        let core = FraudCore::new();
        let storage = FraudStorage::new();
        
        let router = Router::new();
        
        Self {
            core,
            storage,
            router,
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.core.init().await?;
        self.storage.init().await?;
        Ok(())
    }
    
    /// 获取路由器
    pub fn router(&self) -> &Router {
        &self.router
    }
    
    /// 评估交易风险
    async fn assess_risk(
        req: AssessRiskRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let (transaction, risk_level) = api.core.assess_transaction_risk(
            req.from_user_id,
            req.to_user_id,
            req.amount,
            req.transaction_type,
            req.description,
            req.business_id,
        ).await?;
        
        let transaction_json = serde_json::to_value(transaction)?;
        let risk_level_json = serde_json::to_value(risk_level)?;
        
        api.storage.save_transaction(transaction_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Risk assessed successfully".to_string(),
            data: Some(serde_json::json!({
                "transaction": transaction_json,
                "risk_level": risk_level_json,
            })),
        })
    }
    
    /// 标记交易异常
    async fn mark_abnormal(
        req: MarkAbnormalRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let transaction = api.core.mark_transaction_abnormal(req.transaction_id, req.reason).await?;
        
        let transaction_json = serde_json::to_value(transaction)?;
        api.storage.save_transaction(transaction_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Transaction marked as abnormal".to_string(),
            data: Some(transaction_json),
        })
    }
    
    /// 处理交易
    async fn process_transaction(
        req: ProcessTransactionRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let transaction = api.core.process_transaction(req.transaction_id, req.approve).await?;
        
        let transaction_json = serde_json::to_value(transaction)?;
        api.storage.save_transaction(transaction_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Transaction processed successfully".to_string(),
            data: Some(transaction_json),
        })
    }
    
    /// 发起仲裁
    async fn initiate_arbitration(
        req: InitiateArbitrationRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let arbitration = api.core.initiate_arbitration(
            req.transaction_id,
            req.initiator_id,
            req.reason,
            req.evidences,
        ).await?;
        
        let arbitration_json = serde_json::to_value(arbitration)?;
        api.storage.save_arbitration(arbitration_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Arbitration initiated successfully".to_string(),
            data: Some(arbitration_json),
        })
    }
    
    /// 处理仲裁
    async fn process_arbitration(
        req: ProcessArbitrationRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let arbitration = api.core.process_arbitration(
            req.arbitration_id,
            req.result,
            req.decision_reason,
        ).await?;
        
        let arbitration_json = serde_json::to_value(arbitration)?;
        api.storage.save_arbitration(arbitration_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Arbitration processed successfully".to_string(),
            data: Some(arbitration_json),
        })
    }
    
    /// 获取交易信息
    async fn get_transaction(
        req: GetTransactionRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let transaction = api.core.get_transaction(req.transaction_id).await?;
        let transaction_json = serde_json::to_value(transaction)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Transaction retrieved successfully".to_string(),
            data: Some(transaction_json),
        })
    }
    
    /// 获取用户交易列表
    async fn get_user_transactions(
        req: GetUserTransactionsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let transactions = api.core.get_user_transactions(req.user_id, req.offset, req.limit).await?;
        let transactions_json = serde_json::to_value(transactions)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User transactions retrieved successfully".to_string(),
            data: Some(transactions_json),
        })
    }
    
    /// 获取仲裁信息
    async fn get_arbitration(
        req: GetArbitrationRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let arbitration = api.core.get_arbitration(req.arbitration_id).await?;
        let arbitration_json = serde_json::to_value(arbitration)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Arbitration retrieved successfully".to_string(),
            data: Some(arbitration_json),
        })
    }
    
    /// 获取用户仲裁列表
    async fn get_user_arbitrations(
        req: GetUserArbitrationsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let arbitrations = api.core.get_user_arbitrations(req.user_id, req.offset, req.limit).await?;
        let arbitrations_json = serde_json::to_value(arbitrations)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User arbitrations retrieved successfully".to_string(),
            data: Some(arbitrations_json),
        })
    }
    
    /// 获取风险规则
    async fn get_risk_rules(
        _req: GetRiskRulesRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let rules = api.core.get_risk_rules().await?;
        let rules_json = serde_json::to_value(rules)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Risk rules retrieved successfully".to_string(),
            data: Some(rules_json),
        })
    }
    
    /// 检测异常交易
    async fn detect_abnormal(
        _req: (),
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = FraudApi::new();
        let abnormal_transactions = api.core.detect_abnormal_transactions().await?;
        let abnormal_transactions_json = serde_json::to_value(abnormal_transactions)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Abnormal transactions detected successfully".to_string(),
            data: Some(abnormal_transactions_json),
        })
    }
}
