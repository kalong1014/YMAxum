use axum::{Router}; use serde::{Deserialize, Serialize}; use crate::points::core::{PointsCore, PointsType}; use crate::points::storage::PointsStorage;

/// API请求和响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct AddPointsRequest {
    pub user_id: String,
    pub amount: f64,
    pub points_type: PointsType,
    pub business_id: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumePointsRequest {
    pub user_id: String,
    pub amount: f64,
    pub points_type: PointsType,
    pub business_id: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferPointsRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRecordsRequest {
    pub user_id: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezePointsRequest {
    pub user_id: String,
    pub amount: f64,
    pub business_id: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfreezePointsRequest {
    pub user_id: String,
    pub amount: f64,
    pub business_id: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 积分生态API模块
#[derive(Debug)]
#[allow(dead_code)]
pub struct PointsApi {
    core: PointsCore,
    storage: PointsStorage,
    router: Router,
}

#[allow(dead_code)]
impl PointsApi {
    /// 创建新的API模块
    pub fn new() -> Self {
        let core = PointsCore::new();
        let storage = PointsStorage::new();
        
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
    
    /// 增加积分
    async fn add_points(
        req: AddPointsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let (balance, record) = api.core.add_points(
            req.user_id,
            req.amount,
            req.points_type,
            req.business_id,
            req.description,
        ).await?;
        
        let balance_json = serde_json::to_value(balance)?;
        let record_json = serde_json::to_value(record)?;
        
        api.storage.save_balance(balance_json.clone()).await?;
        api.storage.save_record(record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Points added successfully".to_string(),
            data: Some(serde_json::json!({
                "balance": balance_json,
                "record": record_json,
            })),
        })
    }
    
    /// 消耗积分
    async fn consume_points(
        req: ConsumePointsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let (balance, record) = api.core.consume_points(
            req.user_id,
            req.amount,
            req.points_type,
            req.business_id,
            req.description,
        ).await?;
        
        let balance_json = serde_json::to_value(balance)?;
        let record_json = serde_json::to_value(record)?;
        
        api.storage.save_balance(balance_json.clone()).await?;
        api.storage.save_record(record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Points consumed successfully".to_string(),
            data: Some(serde_json::json!({
                "balance": balance_json,
                "record": record_json,
            })),
        })
    }
    
    /// 转让积分
    async fn transfer_points(
        req: TransferPointsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let (from_balance, to_balance, from_record, to_record) = api.core.transfer_points(
            req.from_user_id,
            req.to_user_id,
            req.amount,
            req.description,
        ).await?;
        
        let from_balance_json = serde_json::to_value(from_balance)?;
        let to_balance_json = serde_json::to_value(to_balance)?;
        let from_record_json = serde_json::to_value(from_record)?;
        let to_record_json = serde_json::to_value(to_record)?;
        
        api.storage.save_balance(from_balance_json.clone()).await?;
        api.storage.save_balance(to_balance_json.clone()).await?;
        api.storage.save_record(from_record_json.clone()).await?;
        api.storage.save_record(to_record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Points transferred successfully".to_string(),
            data: Some(serde_json::json!({
                "from_balance": from_balance_json,
                "to_balance": to_balance_json,
                "from_record": from_record_json,
                "to_record": to_record_json,
            })),
        })
    }
    
    /// 获取用户积分余额
    async fn get_balance(
        req: GetBalanceRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let balance = api.core.get_user_balance(req.user_id).await?;
        let balance_json = serde_json::to_value(balance)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Balance retrieved successfully".to_string(),
            data: Some(balance_json),
        })
    }
    
    /// 获取用户积分记录
    async fn get_records(
        req: GetRecordsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let records = api.core.get_user_points_records(
            req.user_id,
            req.offset,
            req.limit,
        ).await?;
        let records_json = serde_json::to_value(records)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Records retrieved successfully".to_string(),
            data: Some(records_json),
        })
    }
    
    /// 冻结积分
    async fn freeze_points(
        req: FreezePointsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let balance = api.core.freeze_points(
            req.user_id,
            req.amount,
            req.business_id,
            req.description,
        ).await?;
        let balance_json = serde_json::to_value(balance)?;
        
        api.storage.save_balance(balance_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Points frozen successfully".to_string(),
            data: Some(balance_json),
        })
    }
    
    /// 解冻积分
    async fn unfreeze_points(
        req: UnfreezePointsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = PointsApi::new();
        let balance = api.core.unfreeze_points(
            req.user_id,
            req.amount,
            req.business_id,
            req.description,
        ).await?;
        let balance_json = serde_json::to_value(balance)?;
        
        api.storage.save_balance(balance_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Points unfrozen successfully".to_string(),
            data: Some(balance_json),
        })
    }
}
