use serde::{Deserialize, Serialize}; use std::sync::Arc; use tokio::sync::RwLock;

/// 推广活动状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CampaignStatus {
    /// 未开始
    Pending,
    /// 进行中
    Active,
    /// 已结束
    Ended,
    /// 已暂停
    Paused,
}

/// 奖励类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RewardType {
    /// 积分奖励
    Points,
    /// 经验值奖励
    Experience,
    /// 现金奖励
    Cash,
    /// 其他奖励
    Other,
}

/// 邀请码
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InviteCode {
    /// 邀请码ID
    pub id: String,
    /// 邀请码
    pub code: String,
    /// 生成用户ID
    pub user_id: String,
    /// 生成时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 使用状态
    pub used: bool,
    /// 使用用户ID
    pub used_by: Option<String>,
    /// 使用时间
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 推广记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReferralRecord {
    /// 记录ID
    pub id: String,
    /// 推广者ID
    pub referrer_id: String,
    /// 被推广者ID
    pub referee_id: String,
    /// 推广时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 奖励状态
    pub reward_status: bool,
    /// 奖励金额/数量
    pub reward_amount: f64,
    /// 奖励类型
    pub reward_type: RewardType,
    /// 推广活动ID
    pub campaign_id: Option<String>,
    /// 邀请码
    pub invite_code: String,
}

/// 推广活动
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReferralCampaign {
    /// 活动ID
    pub id: String,
    /// 活动名称
    pub name: String,
    /// 活动描述
    pub description: String,
    /// 开始时间
    pub start_at: chrono::DateTime<chrono::Utc>,
    /// 结束时间
    pub end_at: chrono::DateTime<chrono::Utc>,
    /// 活动状态
    pub status: CampaignStatus,
    /// 奖励类型
    pub reward_type: RewardType,
    /// 奖励金额/数量
    pub reward_amount: f64,
    /// 最大奖励次数
    pub max_rewards: Option<u32>,
    /// 活动规则
    pub rules: String,
}

/// 团队信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    /// 团队ID
    pub id: String,
    /// 团队名称
    pub name: String,
    /// 团队 leader ID
    pub leader_id: String,
    /// 团队成员数量
    pub member_count: u32,
    /// 团队总推广数
    pub total_referrals: u32,
    /// 团队总奖励
    pub total_rewards: f64,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 推广引流和刺激裂变核心逻辑
#[derive(Debug)]
pub struct ReferralCore {
    /// 邀请码
    invite_codes: Arc<RwLock<Vec<InviteCode>>>,
    /// 推广记录
    referral_records: Arc<RwLock<Vec<ReferralRecord>>>,
    /// 推广活动
    campaigns: Arc<RwLock<Vec<ReferralCampaign>>>,
    /// 团队信息
    teams: Arc<RwLock<Vec<TeamInfo>>>,
}

impl ReferralCore {
    /// 创建新的核心逻辑实例
    pub fn new() -> Self {
        Self {
            invite_codes: Arc::new(RwLock::new(Vec::new())),
            referral_records: Arc::new(RwLock::new(Vec::new())),
            campaigns: Arc::new(RwLock::new(Vec::new())),
            teams: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化逻辑
        Ok(())
    }
    
    /// 生成邀请码
    pub async fn generate_invite_code(
        &self,
        user_id: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<InviteCode, Box<dyn std::error::Error>> {
        // 生成唯一邀请码
        let code = format!("{}{}", 
            user_id.chars().take(4).collect::<String>().to_uppercase(),
            uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>().to_uppercase()
        );
        
        let invite_code = InviteCode {
            id: uuid::Uuid::new_v4().to_string(),
            code,
            user_id,
            created_at: chrono::Utc::now(),
            expires_at,
            used: false,
            used_by: None,
            used_at: None,
        };
        
        let mut invite_codes = self.invite_codes.write().await;
        invite_codes.push(invite_code.clone());
        
        Ok(invite_code)
    }
    
    /// 使用邀请码
    pub async fn use_invite_code(
        &self,
        code: String,
        referee_id: String,
    ) -> Result<(InviteCode, ReferralRecord), Box<dyn std::error::Error>> {
        let mut invite_codes = self.invite_codes.write().await;
        
        // 查找邀请码
        let code_index = invite_codes
            .iter_mut()
            .position(|c| c.code == code)
            .ok_or("Invite code not found")?;
        
        let invite_code = &mut invite_codes[code_index];
        
        // 检查邀请码是否已使用
        if invite_code.used {
            return Err("Invite code already used".into());
        }
        
        // 检查邀请码是否过期
        if let Some(expires_at) = invite_code.expires_at {
            if expires_at < chrono::Utc::now() {
                return Err("Invite code expired".into());
            }
        }
        
        // 标记邀请码为已使用
        invite_code.used = true;
        invite_code.used_by = Some(referee_id.clone());
        invite_code.used_at = Some(chrono::Utc::now());
        
        // 创建推广记录
        let referral_record = ReferralRecord {
            id: uuid::Uuid::new_v4().to_string(),
            referrer_id: invite_code.user_id.clone(),
            referee_id,
            created_at: chrono::Utc::now(),
            reward_status: false,
            reward_amount: 0.0,
            reward_type: RewardType::Points,
            campaign_id: None,
            invite_code: code,
        };
        
        let mut referral_records = self.referral_records.write().await;
        referral_records.push(referral_record.clone());
        
        Ok((invite_code.clone(), referral_record))
    }
    
    /// 发放推广奖励
    pub async fn issue_reward(
        &self,
        referral_id: String,
        reward_amount: f64,
        reward_type: RewardType,
    ) -> Result<ReferralRecord, Box<dyn std::error::Error>> {
        let mut referral_records = self.referral_records.write().await;
        
        let record_index = referral_records
            .iter_mut()
            .position(|r| r.id == referral_id)
            .ok_or("Referral record not found")?;
        
        let record = &mut referral_records[record_index];
        record.reward_status = true;
        record.reward_amount = reward_amount;
        record.reward_type = reward_type;
        
        Ok(record.clone())
    }
    
    /// 创建推广活动
    pub async fn create_campaign(
        &self,
        name: String,
        description: String,
        start_at: chrono::DateTime<chrono::Utc>,
        end_at: chrono::DateTime<chrono::Utc>,
        reward_type: RewardType,
        reward_amount: f64,
        max_rewards: Option<u32>,
        rules: String,
    ) -> Result<ReferralCampaign, Box<dyn std::error::Error>> {
        let campaign = ReferralCampaign {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            start_at,
            end_at,
            status: CampaignStatus::Pending,
            reward_type,
            reward_amount,
            max_rewards,
            rules,
        };
        
        let mut campaigns = self.campaigns.write().await;
        campaigns.push(campaign.clone());
        
        Ok(campaign)
    }
    
    /// 更新活动状态
    pub async fn update_campaign_status(
        &self,
        campaign_id: String,
        status: CampaignStatus,
    ) -> Result<ReferralCampaign, Box<dyn std::error::Error>> {
        let mut campaigns = self.campaigns.write().await;
        
        let campaign_index = campaigns
            .iter_mut()
            .position(|c| c.id == campaign_id)
            .ok_or("Campaign not found")?;
        
        let campaign = &mut campaigns[campaign_index];
        campaign.status = status;
        
        Ok(campaign.clone())
    }
    
    /// 创建团队
    pub async fn create_team(
        &self,
        name: String,
        leader_id: String,
    ) -> Result<TeamInfo, Box<dyn std::error::Error>> {
        let team = TeamInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            leader_id,
            member_count: 1, // 初始只有leader
            total_referrals: 0,
            total_rewards: 0.0,
            created_at: chrono::Utc::now(),
        };
        
        let mut teams = self.teams.write().await;
        teams.push(team.clone());
        
        Ok(team)
    }
    
    /// 获取用户的邀请码
    pub async fn get_user_invite_codes(
        &self,
        user_id: String,
    ) -> Result<Vec<InviteCode>, Box<dyn std::error::Error>> {
        let invite_codes = self.invite_codes.read().await;
        
        let user_codes: Vec<InviteCode> = invite_codes
            .iter()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect();
        
        Ok(user_codes)
    }
    
    /// 获取用户的推广记录
    pub async fn get_user_referrals(
        &self,
        user_id: String,
    ) -> Result<Vec<ReferralRecord>, Box<dyn std::error::Error>> {
        let referral_records = self.referral_records.read().await;
        
        let user_records: Vec<ReferralRecord> = referral_records
            .iter()
            .filter(|r| r.referrer_id == user_id)
            .cloned()
            .collect();
        
        Ok(user_records)
    }
    
    /// 获取用户作为被推广者的记录
    pub async fn get_user_referred_by(
        &self,
        user_id: String,
    ) -> Result<Option<ReferralRecord>, Box<dyn std::error::Error>> {
        let referral_records = self.referral_records.read().await;
        
        let record = referral_records
            .iter()
            .find(|r| r.referee_id == user_id)
            .cloned();
        
        Ok(record)
    }
    
    /// 获取所有推广活动
    pub async fn get_campaigns(
        &self,
    ) -> Result<Vec<ReferralCampaign>, Box<dyn std::error::Error>> {
        let campaigns = self.campaigns.read().await;
        Ok(campaigns.clone())
    }
    
    /// 获取所有团队
    pub async fn get_teams(
        &self,
    ) -> Result<Vec<TeamInfo>, Box<dyn std::error::Error>> {
        let teams = self.teams.read().await;
        Ok(teams.clone())
    }
    
    /// 获取团队详情
    pub async fn get_team(
        &self,
        team_id: String,
    ) -> Result<TeamInfo, Box<dyn std::error::Error>> {
        let teams = self.teams.read().await;
        
        teams
            .iter()
            .find(|t| t.id == team_id)
            .cloned()
            .ok_or("Team not found".into())
    }
    
    /// 统计用户推广数据
    pub async fn get_user_stats(
        &self,
        user_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let referral_records = self.referral_records.read().await;
        
        let user_records: Vec<ReferralRecord> = referral_records
            .iter()
            .filter(|r| r.referrer_id == user_id)
            .cloned()
            .collect();
        
        let total_referrals = user_records.len() as u32;
        let total_rewards = user_records
            .iter()
            .filter(|r| r.reward_status)
            .map(|r| r.reward_amount)
            .sum::<f64>();
        let pending_rewards = user_records
            .iter()
            .filter(|r| !r.reward_status)
            .count() as u32;
        
        let stats = serde_json::json!({
            "user_id": user_id,
            "total_referrals": total_referrals,
            "total_rewards": total_rewards,
            "pending_rewards": pending_rewards,
            "last_referral": user_records
                .last()
                .map(|r| r.created_at.to_string())
                .unwrap_or_else(|| "None".to_string()),
        });
        
        Ok(stats)
    }
    
    /// 检测和更新活动状态
    pub async fn check_and_update_campaigns(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut campaigns = self.campaigns.write().await;
        let now = chrono::Utc::now();
        
        for campaign in campaigns.iter_mut() {
            if campaign.status == CampaignStatus::Pending && campaign.start_at <= now {
                campaign.status = CampaignStatus::Active;
            } else if campaign.status == CampaignStatus::Active && campaign.end_at < now {
                campaign.status = CampaignStatus::Ended;
            }
        }
        
        Ok(())
    }
}
