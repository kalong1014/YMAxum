use serde::{Deserialize, Serialize}; use std::sync::Arc; use tokio::sync::RwLock;

/// 用户等级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserLevel {
    /// 等级ID
    pub id: u32,
    /// 等级名称
    pub name: String,
    /// 所需经验值
    pub required_exp: u64,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 等级特权描述
    pub privileges: Vec<String>,
}

/// 用户经验值类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExpType {
    /// 登录经验
    Login,
    /// 创作经验
    Creation,
    /// 分享经验
    Share,
    /// 交易经验
    Transaction,
    /// 其他经验
    Other,
}

/// 用户经验值记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpRecord {
    /// 记录ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 经验值数量
    pub amount: u64,
    /// 经验值类型
    pub exp_type: ExpType,
    /// 相关业务ID
    pub business_id: Option<String>,
    /// 描述
    pub description: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 用户信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub username: String,
    /// 当前等级
    pub current_level: u32,
    /// 当前经验值
    pub current_exp: u64,
    /// 总经验值
    pub total_exp: u64,
    /// 注册时间
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// 最后登录时间
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 成长任务完成情况
    pub completed_tasks: Vec<String>,
}

/// 成长任务
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrowthTask {
    /// 任务ID
    pub id: String,
    /// 任务名称
    pub name: String,
    /// 任务描述
    pub description: String,
    /// 所需完成次数
    pub required_count: u32,
    /// 经验值奖励
    pub exp_reward: u64,
    /// 积分奖励
    pub points_reward: f64,
    /// 任务类型
    pub task_type: String,
    /// 等级限制
    pub level_requirement: Option<u32>,
    /// 是否可重复
    pub repeatable: bool,
}

/// 用户成长与权限核心逻辑
#[derive(Debug)]
pub struct UserCore {
    /// 用户等级配置
    levels: Arc<RwLock<Vec<UserLevel>>>,
    /// 用户信息
    users: Arc<RwLock<Vec<UserInfo>>>,
    /// 成长任务
    growth_tasks: Arc<RwLock<Vec<GrowthTask>>>,
    /// 经验值记录
    exp_records: Arc<RwLock<Vec<ExpRecord>>>,
}

impl UserCore {
    /// 创建新的核心逻辑实例
    pub fn new() -> Self {
        let mut levels = Vec::new();
        // 默认等级配置
        levels.push(UserLevel {
            id: 1,
            name: "新手".to_string(),
            required_exp: 0,
            permissions: vec!["basic_access".to_string()],
            privileges: vec!["基础访问权限".to_string()],
        });
        levels.push(UserLevel {
            id: 2,
            name: "进阶".to_string(),
            required_exp: 1000,
            permissions: vec!["basic_access".to_string(), "creation_access".to_string()],
            privileges: vec!["基础访问权限".to_string(), "创作权限".to_string()],
        });
        levels.push(UserLevel {
            id: 3,
            name: "专业".to_string(),
            required_exp: 5000,
            permissions: vec!["basic_access".to_string(), "creation_access".to_string(), "advanced_access".to_string()],
            privileges: vec!["基础访问权限".to_string(), "创作权限".to_string(), "高级功能访问".to_string()],
        });
        levels.push(UserLevel {
            id: 4,
            name: "专家".to_string(),
            required_exp: 20000,
            permissions: vec!["basic_access".to_string(), "creation_access".to_string(), "advanced_access".to_string(), "expert_access".to_string()],
            privileges: vec!["基础访问权限".to_string(), "创作权限".to_string(), "高级功能访问".to_string(), "专家特权".to_string()],
        });
        levels.push(UserLevel {
            id: 5,
            name: "大师".to_string(),
            required_exp: 100000,
            permissions: vec!["basic_access".to_string(), "creation_access".to_string(), "advanced_access".to_string(), "expert_access".to_string(), "master_access".to_string()],
            privileges: vec!["基础访问权限".to_string(), "创作权限".to_string(), "高级功能访问".to_string(), "专家特权".to_string(), "大师特权".to_string()],
        });
        
        let mut growth_tasks = Vec::new();
        // 默认成长任务
        growth_tasks.push(GrowthTask {
            id: "first_login".to_string(),
            name: "首次登录".to_string(),
            description: "完成首次登录".to_string(),
            required_count: 1,
            exp_reward: 100,
            points_reward: 10.0,
            task_type: "login".to_string(),
            level_requirement: None,
            repeatable: false,
        });
        growth_tasks.push(GrowthTask {
            id: "first_creation".to_string(),
            name: "首次创作".to_string(),
            description: "完成首次创作".to_string(),
            required_count: 1,
            exp_reward: 500,
            points_reward: 50.0,
            task_type: "creation".to_string(),
            level_requirement: None,
            repeatable: false,
        });
        growth_tasks.push(GrowthTask {
            id: "daily_login".to_string(),
            name: "每日登录".to_string(),
            description: "每天登录一次".to_string(),
            required_count: 1,
            exp_reward: 20,
            points_reward: 2.0,
            task_type: "login".to_string(),
            level_requirement: None,
            repeatable: true,
        });
        
        Self {
            levels: Arc::new(RwLock::new(levels)),
            users: Arc::new(RwLock::new(Vec::new())),
            growth_tasks: Arc::new(RwLock::new(growth_tasks)),
            exp_records: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化逻辑
        Ok(())
    }
    
    /// 获取用户信息
    pub async fn get_user_info(
        &self,
        user_id: String,
    ) -> Result<UserInfo, Box<dyn std::error::Error>> {
        let users = self.users.read().await;
        
        if let Some(user) = users.iter().find(|u| u.user_id == user_id) {
            Ok(user.clone())
        } else {
            // 如果用户不存在，创建默认用户
            let default_user = UserInfo {
                user_id: user_id.clone(),
                username: format!("user_{}", user_id),
                current_level: 1,
                current_exp: 0,
                total_exp: 0,
                registered_at: chrono::Utc::now(),
                last_login_at: None,
                permissions: vec!["basic_access".to_string()],
                completed_tasks: Vec::new(),
            };
            
            let mut users_write = self.users.write().await;
            users_write.push(default_user.clone());
            
            Ok(default_user)
        }
    }
    
    /// 增加用户经验值
    pub async fn add_exp(
        &self,
        user_id: String,
        amount: u64,
        exp_type: ExpType,
        business_id: Option<String>,
        description: String,
    ) -> Result<(UserInfo, ExpRecord), Box<dyn std::error::Error>> {
        if amount == 0 {
            return Err("Amount must be positive".into());
        }
        
        let mut users = self.users.write().await;
        let levels = self.levels.read().await;
        
        let user_index = users
            .iter_mut()
            .position(|u| u.user_id == user_id)
            .unwrap_or_else(|| {
                // 如果用户不存在，添加默认用户
                users.push(UserInfo {
                    user_id: user_id.clone(),
                    username: format!("user_{}", user_id),
                    current_level: 1,
                    current_exp: 0,
                    total_exp: 0,
                    registered_at: chrono::Utc::now(),
                    last_login_at: None,
                    permissions: vec!["basic_access".to_string()],
                    completed_tasks: Vec::new(),
                });
                users.len() - 1
            });
        
        let user = &mut users[user_index];
        user.current_exp += amount;
        user.total_exp += amount;
        
        // 检查是否需要升级
        let mut new_level = user.current_level;
        for level in levels.iter().rev() {
            if user.total_exp >= level.required_exp {
                new_level = level.id;
                break;
            }
        }
        
        if new_level > user.current_level {
            user.current_level = new_level;
            // 更新权限
            if let Some(level) = levels.iter().find(|l| l.id == new_level) {
                user.permissions = level.permissions.clone();
            }
        }
        
        let exp_record = ExpRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            amount,
            exp_type,
            business_id,
            description,
            created_at: chrono::Utc::now(),
        };
        
        let mut exp_records = self.exp_records.write().await;
        exp_records.push(exp_record.clone());
        
        Ok((user.clone(), exp_record))
    }
    
    /// 检查用户权限
    pub async fn check_permission(
        &self,
        user_id: String,
        permission: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let user = self.get_user_info(user_id).await?;
        Ok(user.permissions.contains(&permission))
    }
    
    /// 获取用户等级信息
    pub async fn get_user_level(
        &self,
        level_id: u32,
    ) -> Result<UserLevel, Box<dyn std::error::Error>> {
        let levels = self.levels.read().await;
        levels
            .iter()
            .find(|l| l.id == level_id)
            .cloned()
            .ok_or("Level not found".into())
    }
    
    /// 获取所有等级信息
    pub async fn get_all_levels(
        &self,
    ) -> Result<Vec<UserLevel>, Box<dyn std::error::Error>> {
        let levels = self.levels.read().await;
        Ok(levels.clone())
    }
    
    /// 更新用户等级信息
    pub async fn update_level(
        &self,
        level: UserLevel,
    ) -> Result<UserLevel, Box<dyn std::error::Error>> {
        let mut levels = self.levels.write().await;
        
        let level_index = levels
            .iter_mut()
            .position(|l| l.id == level.id)
            .unwrap_or_else(|| {
                levels.push(level.clone());
                levels.len() - 1
            });
        
        levels[level_index] = level.clone();
        
        Ok(level)
    }
    
    /// 获取成长任务
    pub async fn get_growth_task(
        &self,
        task_id: String,
    ) -> Result<GrowthTask, Box<dyn std::error::Error>> {
        let growth_tasks = self.growth_tasks.read().await;
        growth_tasks
            .iter()
            .find(|t| t.id == task_id)
            .cloned()
            .ok_or("Task not found".into())
    }
    
    /// 获取所有成长任务
    pub async fn get_all_growth_tasks(
        &self,
    ) -> Result<Vec<GrowthTask>, Box<dyn std::error::Error>> {
        let growth_tasks = self.growth_tasks.read().await;
        Ok(growth_tasks.clone())
    }
    
    /// 完成成长任务
    pub async fn complete_growth_task(
        &self,
        user_id: String,
        task_id: String,
    ) -> Result<(UserInfo, GrowthTask), Box<dyn std::error::Error>> {
        let growth_tasks = self.growth_tasks.read().await;
        let task = growth_tasks
            .iter()
            .find(|t| t.id == task_id)
            .cloned()
            .ok_or("Task not found")?;
        
        let mut users = self.users.write().await;
        
        let user_index = users
            .iter_mut()
            .position(|u| u.user_id == user_id)
            .unwrap_or_else(|| {
                users.push(UserInfo {
                    user_id: user_id.clone(),
                    username: format!("user_{}", user_id),
                    current_level: 1,
                    current_exp: 0,
                    total_exp: 0,
                    registered_at: chrono::Utc::now(),
                    last_login_at: None,
                    permissions: vec!["basic_access".to_string()],
                    completed_tasks: Vec::new(),
                });
                users.len() - 1
            });
        
        let user = &mut users[user_index];
        
        // 检查任务是否已完成
        if !task.repeatable && user.completed_tasks.contains(&task_id) {
            return Err("Task already completed".into());
        }
        
        // 检查等级限制
        if let Some(level_req) = task.level_requirement {
            if user.current_level < level_req {
                return Err("Level requirement not met".into());
            }
        }
        
        // 增加经验值和积分
        user.current_exp += task.exp_reward;
        user.total_exp += task.exp_reward;
        
        // 标记任务为完成
        if !task.repeatable {
            user.completed_tasks.push(task_id.clone());
        }
        
        // 检查是否需要升级
        let levels = self.levels.read().await;
        let mut new_level = user.current_level;
        for level in levels.iter().rev() {
            if user.total_exp >= level.required_exp {
                new_level = level.id;
                break;
            }
        }
        
        if new_level > user.current_level {
            user.current_level = new_level;
            // 更新权限
            if let Some(level) = levels.iter().find(|l| l.id == new_level) {
                user.permissions = level.permissions.clone();
            }
        }
        
        Ok((user.clone(), task))
    }
    
    /// 获取用户经验值记录
    pub async fn get_user_exp_records(
        &self,
        user_id: String,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<ExpRecord>, Box<dyn std::error::Error>> {
        let exp_records = self.exp_records.read().await;
        
        let filtered_records: Vec<ExpRecord> = exp_records
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
    
    /// 更新用户登录时间
    pub async fn update_login_time(
        &self,
        user_id: String,
    ) -> Result<UserInfo, Box<dyn std::error::Error>> {
        let mut users = self.users.write().await;
        
        let user_index = users
            .iter_mut()
            .position(|u| u.user_id == user_id)
            .unwrap_or_else(|| {
                // 如果用户不存在，添加默认用户
                users.push(UserInfo {
                    user_id: user_id.clone(),
                    username: format!("user_{}", user_id),
                    current_level: 1,
                    current_exp: 0,
                    total_exp: 0,
                    registered_at: chrono::Utc::now(),
                    last_login_at: Some(chrono::Utc::now()),
                    permissions: vec!["basic_access".to_string()],
                    completed_tasks: Vec::new(),
                });
                users.len() - 1
            });
        
        let user = &mut users[user_index];
        user.last_login_at = Some(chrono::Utc::now());
        
        Ok(user.clone())
    }
}
