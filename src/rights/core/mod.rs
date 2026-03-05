use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 成果信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Work {
    /// 成果ID
    pub id: String,
    /// 创作者ID
    pub creator_id: String,
    /// 创作者名称
    pub creator_name: String,
    /// 成果标题
    pub title: String,
    /// 成果描述
    pub description: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 确权状态
    pub confirmed: bool,
    /// 持有人ID
    pub holder_id: String,
    /// 持有人名称
    pub holder_name: String,
    /// 存证交易哈希
    pub tx_hash: Option<String>,
    /// 区块高度
    pub block_height: Option<u64>,
    /// 权益配置
    pub benefits: WorkBenefits,
}

/// 成果权益配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkBenefits {
    /// 积分奖励加成
    pub points_bonus: f64,
    /// 曝光权重
    pub exposure_weight: f64,
    /// 兑换优先级
    pub exchange_priority: u32,
    /// 转让权限
    pub transfer_permission: bool,
}

/// 权属变更记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OwnershipChange {
    /// 变更ID
    pub id: String,
    /// 成果ID
    pub work_id: String,
    /// 原持有人ID
    pub from_holder_id: String,
    /// 原持有人名称
    pub from_holder_name: String,
    /// 新持有人ID
    pub to_holder_id: String,
    /// 新持有人名称
    pub to_holder_name: String,
    /// 变更时间
    pub changed_at: chrono::DateTime<chrono::Utc>,
    /// 变更原因
    pub reason: String,
    /// 存证交易哈希
    pub tx_hash: Option<String>,
    /// 区块高度
    pub block_height: Option<u64>,
}

/// 去中心化确权核心逻辑
#[derive(Debug)]
pub struct RightsCore {
    /// 成果存储
    works: Arc<RwLock<Vec<Work>>>,
    /// 权属变更记录
    ownership_changes: Arc<RwLock<Vec<OwnershipChange>>>,
}

impl RightsCore {
    /// 创建新的核心逻辑实例
    pub fn new() -> Self {
        Self {
            works: Arc::new(RwLock::new(Vec::new())),
            ownership_changes: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化逻辑
        Ok(())
    }
    
    /// 发布成果
    pub async fn publish_work(
        &self,
        creator_id: String,
        creator_name: String,
        title: String,
        description: String,
    ) -> Result<Work, Box<dyn std::error::Error>> {
        let work = Work {
            id: uuid::Uuid::new_v4().to_string(),
            creator_id: creator_id.clone(),
            creator_name: creator_name.clone(),
            title,
            description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            confirmed: false,
            holder_id: creator_id,
            holder_name: creator_name,
            tx_hash: None,
            block_height: None,
            benefits: WorkBenefits {
                points_bonus: 1.0,
                exposure_weight: 1.0,
                exchange_priority: 1,
                transfer_permission: true,
            },
        };
        
        let mut works = self.works.write().await;
        works.push(work.clone());
        
        Ok(work)
    }
    
    /// 申请确权
    pub async fn apply_rights(
        &self,
        work_id: String,
    ) -> Result<Work, Box<dyn std::error::Error>> {
        let mut works = self.works.write().await;
        
        let work_index = works
            .iter_mut()
            .position(|w| w.id == work_id)
            .ok_or("Work not found")?;
        
        let work = &mut works[work_index];
        work.confirmed = true;
        work.updated_at = chrono::Utc::now();
        
        Ok(work.clone())
    }
    
    /// 更新存证信息
    pub async fn update_deposit_info(
        &self,
        work_id: String,
        tx_hash: String,
        block_height: u64,
    ) -> Result<Work, Box<dyn std::error::Error>> {
        let mut works = self.works.write().await;
        
        let work_index = works
            .iter_mut()
            .position(|w| w.id == work_id)
            .ok_or("Work not found")?;
        
        let work = &mut works[work_index];
        work.tx_hash = Some(tx_hash);
        work.block_height = Some(block_height);
        work.updated_at = chrono::Utc::now();
        
        Ok(work.clone())
    }
    
    /// 变更权属
    pub async fn change_ownership(
        &self,
        work_id: String,
        from_holder_id: String,
        from_holder_name: String,
        to_holder_id: String,
        to_holder_name: String,
        reason: String,
    ) -> Result<(Work, OwnershipChange), Box<dyn std::error::Error>> {
        let mut works = self.works.write().await;
        
        let work_index = works
            .iter_mut()
            .position(|w| w.id == work_id)
            .ok_or("Work not found")?;
        
        let work = &mut works[work_index];
        
        let ownership_change = OwnershipChange {
            id: uuid::Uuid::new_v4().to_string(),
            work_id: work_id.clone(),
            from_holder_id,
            from_holder_name,
            to_holder_id: to_holder_id.clone(),
            to_holder_name: to_holder_name.clone(),
            changed_at: chrono::Utc::now(),
            reason,
            tx_hash: None,
            block_height: None,
        };
        
        work.holder_id = to_holder_id;
        work.holder_name = to_holder_name;
        work.updated_at = chrono::Utc::now();
        
        let mut ownership_changes = self.ownership_changes.write().await;
        ownership_changes.push(ownership_change.clone());
        
        Ok((work.clone(), ownership_change))
    }
    
    /// 更新权属变更的存证信息
    pub async fn update_ownership_deposit_info(
        &self,
        change_id: String,
        tx_hash: String,
        block_height: u64,
    ) -> Result<OwnershipChange, Box<dyn std::error::Error>> {
        let mut ownership_changes = self.ownership_changes.write().await;
        
        let change_index = ownership_changes
            .iter_mut()
            .position(|c| c.id == change_id)
            .ok_or("Ownership change not found")?;
        
        let change = &mut ownership_changes[change_index];
        change.tx_hash = Some(tx_hash);
        change.block_height = Some(block_height);
        
        Ok(change.clone())
    }
    
    /// 配置成果权益
    pub async fn configure_benefits(
        &self,
        work_id: String,
        benefits: WorkBenefits,
    ) -> Result<Work, Box<dyn std::error::Error>> {
        let mut works = self.works.write().await;
        
        let work_index = works
            .iter_mut()
            .position(|w| w.id == work_id)
            .ok_or("Work not found")?;
        
        let work = &mut works[work_index];
        work.benefits = benefits;
        work.updated_at = chrono::Utc::now();
        
        Ok(work.clone())
    }
    
    /// 获取成果
    pub async fn get_work(&self, work_id: String) -> Result<Work, Box<dyn std::error::Error>> {
        let works = self.works.read().await;
        
        let work = works
            .iter()
            .find(|w| w.id == work_id)
            .ok_or("Work not found")?;
        
        Ok(work.clone())
    }
    
    /// 获取成果列表
    pub async fn get_works(
        &self,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<Work>, Box<dyn std::error::Error>> {
        let works = self.works.read().await;
        
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > works.len() {
            works.len()
        } else {
            end
        };
        
        Ok(works[start..end].to_vec())
    }
    
    /// 获取权属变更记录
    pub async fn get_ownership_changes(
        &self,
        work_id: String,
    ) -> Result<Vec<OwnershipChange>, Box<dyn std::error::Error>> {
        let ownership_changes = self.ownership_changes.read().await;
        
        let changes = ownership_changes
            .iter()
            .filter(|c| c.work_id == work_id)
            .cloned()
            .collect();
        
        Ok(changes)
    }
}
