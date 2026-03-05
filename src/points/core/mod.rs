use serde::{Deserialize, Serialize}; use std::sync::Arc; use tokio::sync::RwLock;

/// 积分类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PointsType {
    /// 任务奖励
    TaskReward,
    /// 创作奖励
    CreationReward,
    /// 分享奖励
    ShareReward,
    /// 交易奖励
    TransactionReward,
    /// 其他奖励
    OtherReward,
    /// 交易消耗
    TransactionConsumption,
    /// 兑换消耗
    ExchangeConsumption,
    /// 转让
    Transfer,
}

/// 积分记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointsRecord {
    /// 记录ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 积分数量
    pub amount: f64,
    /// 积分类型
    pub points_type: PointsType,
    /// 相关业务ID
    pub business_id: Option<String>,
    /// 描述
    pub description: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 交易哈希（用于区块链存证）
    pub tx_hash: Option<String>,
}

/// 用户积分余额
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPointsBalance {
    /// 用户ID
    pub user_id: String,
    /// 总积分
    pub total_points: f64,
    /// 可用积分
    pub available_points: f64,
    /// 冻结积分
    pub frozen_points: f64,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 积分核心逻辑
#[derive(Debug)]
pub struct PointsCore {
    /// 用户积分余额
    user_balances: Arc<RwLock<Vec<UserPointsBalance>>>,
    /// 积分记录
    points_records: Arc<RwLock<Vec<PointsRecord>>>,
}

impl PointsCore {
    /// 创建新的积分核心逻辑实例
    pub fn new() -> Self {
        Self {
            user_balances: Arc::new(RwLock::new(Vec::new())),
            points_records: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化逻辑
        Ok(())
    }
    
    /// 获取用户积分余额
    pub async fn get_user_balance(
        &self,
        user_id: String,
    ) -> Result<UserPointsBalance, Box<dyn std::error::Error>> {
        let user_balances = self.user_balances.read().await;
        
        if let Some(balance) = user_balances.iter().find(|b| b.user_id == user_id) {
            Ok(balance.clone())
        } else {
            // 如果用户不存在，创建默认余额
            let default_balance = UserPointsBalance {
                user_id: user_id.clone(),
                total_points: 0.0,
                available_points: 0.0,
                frozen_points: 0.0,
                updated_at: chrono::Utc::now(),
            };
            
            let mut user_balances_write = self.user_balances.write().await;
            user_balances_write.push(default_balance.clone());
            
            Ok(default_balance)
        }
    }
    
    /// 增加积分
    pub async fn add_points(
        &self,
        user_id: String,
        amount: f64,
        points_type: PointsType,
        business_id: Option<String>,
        description: String,
    ) -> Result<(UserPointsBalance, PointsRecord), Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        let mut user_balances = self.user_balances.write().await;
        
        let balance_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == user_id)
            .unwrap_or_else(|| {
                // 如果用户不存在，添加默认余额
                user_balances.push(UserPointsBalance {
                    user_id: user_id.clone(),
                    total_points: 0.0,
                    available_points: 0.0,
                    frozen_points: 0.0,
                    updated_at: chrono::Utc::now(),
                });
                user_balances.len() - 1
            });
        
        let balance = &mut user_balances[balance_index];
        balance.total_points += amount;
        balance.available_points += amount;
        balance.updated_at = chrono::Utc::now();
        
        let points_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            amount: amount,
            points_type,
            business_id,
            description,
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        let mut points_records = self.points_records.write().await;
        points_records.push(points_record.clone());
        
        Ok((balance.clone(), points_record))
    }
    
    /// 消耗积分
    pub async fn consume_points(
        &self,
        user_id: String,
        amount: f64,
        points_type: PointsType,
        business_id: Option<String>,
        description: String,
    ) -> Result<(UserPointsBalance, PointsRecord), Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        let mut user_balances = self.user_balances.write().await;
        
        let balance_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == user_id)
            .ok_or("User not found")?;
        
        let balance = &mut user_balances[balance_index];
        
        if balance.available_points < amount {
            return Err("Insufficient available points".into());
        }
        
        balance.total_points -= amount;
        balance.available_points -= amount;
        balance.updated_at = chrono::Utc::now();
        
        let points_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            amount: -amount, // 负值表示消耗
            points_type,
            business_id,
            description,
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        let mut points_records = self.points_records.write().await;
        points_records.push(points_record.clone());
        
        Ok((balance.clone(), points_record))
    }
    
    /// 转让积分
    pub async fn transfer_points(
        &self,
        from_user_id: String,
        to_user_id: String,
        amount: f64,
        _description: String,
    ) -> Result<(UserPointsBalance, UserPointsBalance, PointsRecord, PointsRecord), Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        if from_user_id == to_user_id {
            return Err("Cannot transfer points to yourself".into());
        }
        
        // 首先检查转出用户是否存在并获取余额信息
        let (from_exists, from_available_points) = {
            let user_balances = self.user_balances.read().await;
            match user_balances.iter().find(|b| b.user_id == from_user_id) {
                Some(balance) => (true, balance.available_points),
                None => (false, 0.0)
            }
        };
        
        if !from_exists {
            return Err("From user not found".into());
        }
        
        if from_available_points < amount {
            return Err("Insufficient available points".into());
        }
        
        // 执行转让操作
        let mut user_balances = self.user_balances.write().await;
        
        // 查找或创建转出用户
        let from_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == from_user_id)
            .unwrap();
        
        // 查找或创建转入用户
        let to_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == to_user_id)
            .unwrap_or_else(|| {
                user_balances.push(UserPointsBalance {
                    user_id: to_user_id.clone(),
                    total_points: 0.0,
                    available_points: 0.0,
                    frozen_points: 0.0,
                    updated_at: chrono::Utc::now(),
                });
                user_balances.len() - 1
            });
        
        // 执行转让并获取更新后的余额
        let (from_balance, to_balance) = if from_index < to_index {
            let (first_half, second_half) = user_balances.split_at_mut(to_index);
            let from_balance = &mut first_half[from_index];
            let to_balance = &mut second_half[0];
            
            from_balance.total_points -= amount;
            from_balance.available_points -= amount;
            from_balance.updated_at = chrono::Utc::now();
            
            to_balance.total_points += amount;
            to_balance.available_points += amount;
            to_balance.updated_at = chrono::Utc::now();
            
            (from_balance.clone(), to_balance.clone())
        } else {
            let (first_half, second_half) = user_balances.split_at_mut(from_index);
            let to_balance = &mut first_half[to_index];
            let from_balance = &mut second_half[0];
            
            from_balance.total_points -= amount;
            from_balance.available_points -= amount;
            from_balance.updated_at = chrono::Utc::now();
            
            to_balance.total_points += amount;
            to_balance.available_points += amount;
            to_balance.updated_at = chrono::Utc::now();
            
            (from_balance.clone(), to_balance.clone())
        };
        
        // 创建转出记录
        let from_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: from_user_id.clone(),
            amount: -amount,
            points_type: PointsType::Transfer,
            business_id: Some(to_user_id.clone()),
            description: format!("Transfer to {}", to_user_id),
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        // 创建转入记录
        let to_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: to_user_id.clone(),
            amount: amount,
            points_type: PointsType::Transfer,
            business_id: Some(from_user_id.clone()),
            description: format!("Transfer from {}", from_user_id),
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        let mut points_records = self.points_records.write().await;
        points_records.push(from_record.clone());
        points_records.push(to_record.clone());
        
        Ok((from_balance, to_balance, from_record, to_record))
    }
    
    /// 获取用户积分记录
    pub async fn get_user_points_records(
        &self,
        user_id: String,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<PointsRecord>, Box<dyn std::error::Error>> {
        let points_records = self.points_records.read().await;
        
        let filtered_records: Vec<PointsRecord> = points_records
            .iter()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect();
        
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > filtered_records.len() {
            filtered_records.len()
        } else {
            end
        };
        
        Ok(filtered_records[start..end].to_vec())
    }
    
    /// 更新积分记录的交易哈希
    pub async fn update_record_tx_hash(
        &self,
        record_id: String,
        tx_hash: String,
    ) -> Result<PointsRecord, Box<dyn std::error::Error>> {
        let mut points_records = self.points_records.write().await;
        
        let record_index = points_records
            .iter_mut()
            .position(|r| r.id == record_id)
            .ok_or("Record not found")?;
        
        let record = &mut points_records[record_index];
        record.tx_hash = Some(tx_hash);
        
        Ok(record.clone())
    }
    
    /// 冻结积分
    pub async fn freeze_points(
        &self,
        user_id: String,
        amount: f64,
        business_id: String,
        description: String,
    ) -> Result<UserPointsBalance, Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        let mut user_balances = self.user_balances.write().await;
        
        let balance_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == user_id)
            .ok_or("User not found")?;
        
        let balance = &mut user_balances[balance_index];
        
        if balance.available_points < amount {
            return Err("Insufficient available points".into());
        }
        
        balance.available_points -= amount;
        balance.frozen_points += amount;
        balance.updated_at = chrono::Utc::now();
        
        // 创建冻结记录
        let freeze_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            amount: 0.0, // 冻结不改变总积分
            points_type: PointsType::OtherReward, // 使用其他类型表示冻结
            business_id: Some(business_id),
            description: format!("Freeze: {}", description),
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        let mut points_records = self.points_records.write().await;
        points_records.push(freeze_record);
        
        Ok(balance.clone())
    }
    
    /// 解冻积分
    pub async fn unfreeze_points(
        &self,
        user_id: String,
        amount: f64,
        business_id: String,
        description: String,
    ) -> Result<UserPointsBalance, Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        let mut user_balances = self.user_balances.write().await;
        
        let balance_index = user_balances
            .iter_mut()
            .position(|b| b.user_id == user_id)
            .ok_or("User not found")?;
        
        let balance = &mut user_balances[balance_index];
        
        if balance.frozen_points < amount {
            return Err("Insufficient frozen points".into());
        }
        
        balance.frozen_points -= amount;
        balance.available_points += amount;
        balance.updated_at = chrono::Utc::now();
        
        // 创建解冻记录
        let unfreeze_record = PointsRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            amount: 0.0, // 解冻不改变总积分
            points_type: PointsType::OtherReward, // 使用其他类型表示解冻
            business_id: Some(business_id),
            description: format!("Unfreeze: {}", description),
            created_at: chrono::Utc::now(),
            tx_hash: None,
        };
        
        let mut points_records = self.points_records.write().await;
        points_records.push(unfreeze_record);
        
        Ok(balance.clone())
    }
}
