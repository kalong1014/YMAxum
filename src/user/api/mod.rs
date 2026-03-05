use axum::{Router}; use serde::{Deserialize, Serialize}; use crate::user::core::{UserCore, ExpType}; use crate::user::storage::UserStorage;

/// API请求和响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddExpRequest {
    pub user_id: String,
    pub amount: u64,
    pub exp_type: ExpType,
    pub business_id: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPermissionRequest {
    pub user_id: String,
    pub permission: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLevelsRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTasksRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteTaskRequest {
    pub user_id: String,
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetExpRecordsRequest {
    pub user_id: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLoginRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 用户成长与权限API模块
#[derive(Debug)]
pub struct UserApi {
    core: UserCore,
    storage: UserStorage,
    router: Router,
}

#[allow(dead_code)]
impl UserApi {
    /// 创建新的API模块
    pub fn new() -> Self {
        let core = UserCore::new();
        let storage = UserStorage::new();
        
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
    
    /// 获取用户信息
    async fn get_user_info(
        req: GetUserInfoRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let user = api.core.get_user_info(req.user_id).await?;
        let user_json = serde_json::to_value(user)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User info retrieved successfully".to_string(),
            data: Some(user_json),
        })
    }
    
    /// 增加用户经验值
    async fn add_exp(
        req: AddExpRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let (user, exp_record) = api.core.add_exp(
            req.user_id,
            req.amount,
            req.exp_type,
            req.business_id,
            req.description,
        ).await?;
        
        let user_json = serde_json::to_value(user)?;
        let exp_record_json = serde_json::to_value(exp_record)?;
        
        api.storage.save_user(user_json.clone()).await?;
        api.storage.save_exp_record(exp_record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Experience added successfully".to_string(),
            data: Some(serde_json::json!({
                "user": user_json,
                "exp_record": exp_record_json,
            })),
        })
    }
    
    /// 检查用户权限
    async fn check_permission(
        req: CheckPermissionRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let has_permission = api.core.check_permission(req.user_id, req.permission).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Permission checked successfully".to_string(),
            data: Some(serde_json::json!({
                "has_permission": has_permission,
            })),
        })
    }
    
    /// 获取所有等级配置
    async fn get_levels(
        _req: GetLevelsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let levels = api.core.get_all_levels().await?;
        let levels_json = serde_json::to_value(levels)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Levels retrieved successfully".to_string(),
            data: Some(levels_json),
        })
    }
    
    /// 获取所有成长任务
    async fn get_tasks(
        _req: GetTasksRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let tasks = api.core.get_all_growth_tasks().await?;
        let tasks_json = serde_json::to_value(tasks)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Tasks retrieved successfully".to_string(),
            data: Some(tasks_json),
        })
    }
    
    /// 完成成长任务
    async fn complete_task(
        req: CompleteTaskRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let (user, task) = api.core.complete_growth_task(req.user_id, req.task_id).await?;
        
        let user_json = serde_json::to_value(user)?;
        let task_json = serde_json::to_value(task)?;
        
        api.storage.save_user(user_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Task completed successfully".to_string(),
            data: Some(serde_json::json!({
                "user": user_json,
                "task": task_json,
            })),
        })
    }
    
    /// 获取用户经验值记录
    async fn get_exp_records(
        req: GetExpRecordsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let records = api.core.get_user_exp_records(req.user_id, req.offset, req.limit).await?;
        let records_json = serde_json::to_value(records)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Exp records retrieved successfully".to_string(),
            data: Some(records_json),
        })
    }
    
    /// 更新用户登录时间
    async fn update_login(
        req: UpdateLoginRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = UserApi::new();
        let user = api.core.update_login_time(req.user_id).await?;
        let user_json = serde_json::to_value(user)?;
        
        api.storage.save_user(user_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Login time updated successfully".to_string(),
            data: Some(user_json),
        })
    }
}
