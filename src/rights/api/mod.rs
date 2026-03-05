use axum::{Router, Json, response::IntoResponse}; use serde::{Deserialize, Serialize}; use crate::rights::core::RightsCore; use crate::rights::blockchain::BlockchainClient; use crate::rights::storage::RightsStorage;

/// API请求和响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishWorkRequest {
    pub creator_id: String,
    pub creator_name: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRightsRequest {
    pub work_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeOwnershipRequest {
    pub work_id: String,
    pub from_holder_id: String,
    pub from_holder_name: String,
    pub to_holder_id: String,
    pub to_holder_name: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigureBenefitsRequest {
    pub work_id: String,
    pub points_bonus: f64,
    pub exposure_weight: f64,
    pub exchange_priority: u32,
    pub transfer_permission: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWorksRequest {
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWorkRequest {
    pub work_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnershipChangesRequest {
    pub work_id: String,
}

/// API响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 去中心化确权API模块
#[derive(Debug)]
pub struct RightsApi {
    core: RightsCore,
    blockchain: BlockchainClient,
    storage: RightsStorage,
    router: Router,
}

#[allow(dead_code)]
impl RightsApi {
    /// 创建新的API模块
    pub fn new() -> Self {
        let core = RightsCore::new();
        let blockchain = BlockchainClient::new();
        let storage = RightsStorage::new();
        
        let router = Router::new();
        
        Self {
            core,
            blockchain,
            storage,
            router,
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.core.init().await?;
        self.blockchain.init().await?;
        self.storage.init().await?;
        Ok(())
    }
    
    /// 获取路由器
    pub fn router(&self) -> &Router {
        &self.router
    }
    
    /// 发布成果
    async fn publish_work(
        Json(req): Json<PublishWorkRequest>,
    ) -> impl IntoResponse {
        let api = RightsApi::new();
        match api.core.publish_work(
            req.creator_id,
            req.creator_name,
            req.title,
            req.description,
        ).await {
            Ok(work) => {
                let work_json = serde_json::to_value(work).unwrap();
                let _ = api.storage.save_work(work_json.clone()).await;
                
                Json(ApiResponse {
                    success: true,
                    message: "Work published successfully".to_string(),
                    data: Some(work_json),
                })
            }
            Err(e) => {
                Json(ApiResponse {
                    success: false,
                    message: format!("Failed to publish work: {}", e),
                    data: None,
                })
            }
        }
    }
    
    /// 申请确权
    async fn apply_rights(
        req: ApplyRightsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let work = api.core.apply_rights(req.work_id.clone()).await?;
        
        // 存证到区块链
        let deposit_request = crate::rights::blockchain::DepositRequest {
            creator_id: work.creator_id.clone(),
            creator_name: work.creator_name.clone(),
            work_id: work.id.clone(),
            work_title: work.title.clone(),
            work_description: work.description.clone(),
            created_at: work.created_at.to_string(),
        };
        
        let deposit_response = api.blockchain.deposit_work(deposit_request).await?;
        
        // 更新存证信息
        let updated_work = api.core.update_deposit_info(
            work.id.clone(),
            deposit_response.tx_hash,
            deposit_response.block_height,
        ).await?;
        
        let work_json = serde_json::to_value(updated_work)?;
        api.storage.save_work(work_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Rights applied successfully".to_string(),
            data: Some(work_json),
        })
    }
    
    /// 变更权属
    async fn change_ownership(
        req: ChangeOwnershipRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let (work, ownership_change) = api.core.change_ownership(
            req.work_id.clone(),
            req.from_holder_id,
            req.from_holder_name,
            req.to_holder_id,
            req.to_holder_name,
            req.reason,
        ).await?;
        
        // 存证到区块链
        let ownership_change_request = crate::rights::blockchain::OwnershipChangeRequest {
            work_id: work.id.clone(),
            from_holder_id: ownership_change.from_holder_id.clone(),
            from_holder_name: ownership_change.from_holder_name.clone(),
            to_holder_id: ownership_change.to_holder_id.clone(),
            to_holder_name: ownership_change.to_holder_name.clone(),
            changed_at: ownership_change.changed_at.to_string(),
            reason: ownership_change.reason.clone(),
        };
        
        let deposit_response = api.blockchain.deposit_ownership_change(ownership_change_request).await?;
        
        // 更新存证信息
        let updated_ownership_change = api.core.update_ownership_deposit_info(
            ownership_change.id.clone(),
            deposit_response.tx_hash,
            deposit_response.block_height,
        ).await?;
        
        let work_json = serde_json::to_value(work)?;
        let ownership_change_json = serde_json::to_value(updated_ownership_change)?;
        
        api.storage.save_work(work_json.clone()).await?;
        api.storage.save_ownership_change(ownership_change_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Ownership changed successfully".to_string(),
            data: Some(serde_json::json!({
                "work": work_json,
                "ownership_change": ownership_change_json,
            })),
        })
    }
    
    /// 配置权益
    async fn configure_benefits(
        req: ConfigureBenefitsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let benefits = crate::rights::core::WorkBenefits {
            points_bonus: req.points_bonus,
            exposure_weight: req.exposure_weight,
            exchange_priority: req.exchange_priority,
            transfer_permission: req.transfer_permission,
        };
        
        let work = api.core.configure_benefits(req.work_id, benefits).await?;
        let work_json = serde_json::to_value(work)?;
        api.storage.save_work(work_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Benefits configured successfully".to_string(),
            data: Some(work_json),
        })
    }
    
    /// 获取成果
    async fn get_work(
        req: GetWorkRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let work = api.core.get_work(req.work_id).await?;
        let work_json = serde_json::to_value(work)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Work retrieved successfully".to_string(),
            data: Some(work_json),
        })
    }
    
    /// 获取成果列表
    async fn get_works(
        req: GetWorksRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let works = api.core.get_works(req.offset, req.limit).await?;
        let works_json = serde_json::to_value(works)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Works retrieved successfully".to_string(),
            data: Some(works_json),
        })
    }
    
    /// 获取权属变更记录
    async fn get_ownership_changes(
        req: GetOwnershipChangesRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = RightsApi::new();
        let changes = api.core.get_ownership_changes(req.work_id).await?;
        let changes_json = serde_json::to_value(changes)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Ownership changes retrieved successfully".to_string(),
            data: Some(changes_json),
        })
    }
}
