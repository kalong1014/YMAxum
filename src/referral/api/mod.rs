use axum::{Router}; use serde::{Deserialize, Serialize}; use crate::referral::core::{ReferralCore, RewardType, CampaignStatus}; use crate::referral::storage::ReferralStorage;

/// API请求和响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateInviteCodeRequest {
    pub user_id: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseInviteCodeRequest {
    pub code: String,
    pub referee_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueRewardRequest {
    pub referral_id: String,
    pub reward_amount: f64,
    pub reward_type: RewardType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: String,
    pub start_at: String,
    pub end_at: String,
    pub reward_type: RewardType,
    pub reward_amount: f64,
    pub max_rewards: Option<u32>,
    pub rules: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCampaignStatusRequest {
    pub campaign_id: String,
    pub status: CampaignStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub leader_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInviteCodesRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserReferralsRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserReferredByRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserStatsRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCampaignsRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTeamsRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 推广引流和刺激裂变API模块
#[derive(Debug)]
#[allow(dead_code)]
pub struct ReferralApi {
    core: ReferralCore,
    storage: ReferralStorage,
    router: Router,
}

#[allow(dead_code)]
impl ReferralApi {
    /// 创建新的API模块
    pub fn new() -> Self {
        let core = ReferralCore::new();
        let storage = ReferralStorage::new();
        
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
    
    /// 生成邀请码
    async fn generate_invite_code(
        req: GenerateInviteCodeRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        
        let expires_at = req.expires_at.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }).transpose()?;
        
        let invite_code = api.core.generate_invite_code(req.user_id, expires_at).await?;
        let invite_code_json = serde_json::to_value(invite_code)?;
        
        api.storage.save_invite_code(invite_code_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Invite code generated successfully".to_string(),
            data: Some(invite_code_json),
        })
    }
    
    /// 使用邀请码
    async fn use_invite_code(
        req: UseInviteCodeRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let (invite_code, referral_record) = api.core.use_invite_code(req.code, req.referee_id).await?;
        
        let invite_code_json = serde_json::to_value(invite_code)?;
        let referral_record_json = serde_json::to_value(referral_record)?;
        
        api.storage.save_invite_code(invite_code_json.clone()).await?;
        api.storage.save_referral_record(referral_record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Invite code used successfully".to_string(),
            data: Some(serde_json::json!({
                "invite_code": invite_code_json,
                "referral_record": referral_record_json,
            })),
        })
    }
    
    /// 发放推广奖励
    async fn issue_reward(
        req: IssueRewardRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let referral_record = api.core.issue_reward(req.referral_id, req.reward_amount, req.reward_type).await?;
        
        let referral_record_json = serde_json::to_value(referral_record)?;
        api.storage.save_referral_record(referral_record_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Reward issued successfully".to_string(),
            data: Some(referral_record_json),
        })
    }
    
    /// 创建推广活动
    async fn create_campaign(
        req: CreateCampaignRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        
        let start_at = chrono::DateTime::parse_from_rfc3339(&req.start_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))?;
        let end_at = chrono::DateTime::parse_from_rfc3339(&req.end_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))?;
        
        let campaign = api.core.create_campaign(
            req.name,
            req.description,
            start_at,
            end_at,
            req.reward_type,
            req.reward_amount,
            req.max_rewards,
            req.rules,
        ).await?;
        
        let campaign_json = serde_json::to_value(campaign)?;
        api.storage.save_campaign(campaign_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Campaign created successfully".to_string(),
            data: Some(campaign_json),
        })
    }
    
    /// 更新活动状态
    async fn update_campaign_status(
        req: UpdateCampaignStatusRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let campaign = api.core.update_campaign_status(req.campaign_id, req.status).await?;
        
        let campaign_json = serde_json::to_value(campaign)?;
        api.storage.save_campaign(campaign_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Campaign status updated successfully".to_string(),
            data: Some(campaign_json),
        })
    }
    
    /// 创建团队
    async fn create_team(
        req: CreateTeamRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let team = api.core.create_team(req.name, req.leader_id).await?;
        
        let team_json = serde_json::to_value(team)?;
        api.storage.save_team(team_json.clone()).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Team created successfully".to_string(),
            data: Some(team_json),
        })
    }
    
    /// 获取用户的邀请码
    async fn get_user_invite_codes(
        req: GetUserInviteCodesRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let invite_codes = api.core.get_user_invite_codes(req.user_id).await?;
        let invite_codes_json = serde_json::to_value(invite_codes)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User invite codes retrieved successfully".to_string(),
            data: Some(invite_codes_json),
        })
    }
    
    /// 获取用户的推广记录
    async fn get_user_referrals(
        req: GetUserReferralsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let referrals = api.core.get_user_referrals(req.user_id).await?;
        let referrals_json = serde_json::to_value(referrals)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User referrals retrieved successfully".to_string(),
            data: Some(referrals_json),
        })
    }
    
    /// 获取用户作为被推广者的记录
    async fn get_user_referred_by(
        req: GetUserReferredByRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let referred_by = api.core.get_user_referred_by(req.user_id).await?;
        let referred_by_json = serde_json::to_value(referred_by)?;
        
        Ok(ApiResponse {
            success: true,
            message: "User referred by information retrieved successfully".to_string(),
            data: Some(referred_by_json),
        })
    }
    
    /// 获取用户推广统计
    async fn get_user_stats(
        req: GetUserStatsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let stats = api.core.get_user_stats(req.user_id).await?;
        
        Ok(ApiResponse {
            success: true,
            message: "User stats retrieved successfully".to_string(),
            data: Some(stats),
        })
    }
    
    /// 获取所有推广活动
    async fn get_campaigns(
        _req: GetCampaignsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let campaigns = api.core.get_campaigns().await?;
        let campaigns_json = serde_json::to_value(campaigns)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Campaigns retrieved successfully".to_string(),
            data: Some(campaigns_json),
        })
    }
    
    /// 获取所有团队
    async fn get_teams(
        _req: GetTeamsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        let teams = api.core.get_teams().await?;
        let teams_json = serde_json::to_value(teams)?;
        
        Ok(ApiResponse {
            success: true,
            message: "Teams retrieved successfully".to_string(),
            data: Some(teams_json),
        })
    }
    
    /// 检查和更新活动状态
    async fn check_campaigns(
        _req: (),
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error>> {
        let api = ReferralApi::new();
        api.core.check_and_update_campaigns().await?;
        
        Ok(ApiResponse {
            success: true,
            message: "Campaigns checked and updated successfully".to_string(),
            data: None,
        })
    }
}
