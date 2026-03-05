use serde::{Deserialize, Serialize}; use std::fs::File; use std::io::Read; use std::path::Path;

/// 存储配置
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 数据存储目录
    pub data_dir: String,
    /// 邀请码存储文件
    pub invite_codes_file: String,
    /// 推广记录存储文件
    pub referral_records_file: String,
    /// 推广活动存储文件
    pub campaigns_file: String,
    /// 团队信息存储文件
    pub teams_file: String,
}

/// 推广引流和刺激裂变存储模块
#[derive(Debug)]
pub struct ReferralStorage {
    /// 存储配置
    config: StorageConfig,
}

impl ReferralStorage {
    /// 创建新的存储模块
    pub fn new() -> Self {
        Self {
            config: StorageConfig {
                data_dir: "./data/referral".to_string(),
                invite_codes_file: "invite_codes.json".to_string(),
                referral_records_file: "referral_records.json".to_string(),
                campaigns_file: "campaigns.json".to_string(),
                teams_file: "teams.json".to_string(),
            },
        }
    }
    
    /// 初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 创建数据目录
        let data_path = Path::new(&self.config.data_dir);
        if !data_path.exists() {
            std::fs::create_dir_all(data_path)?;
        }
        
        // 创建邀请码存储文件
        let invite_codes_path = data_path.join(&self.config.invite_codes_file);
        if !invite_codes_path.exists() {
            let file = File::create(invite_codes_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建推广记录存储文件
        let referral_records_path = data_path.join(&self.config.referral_records_file);
        if !referral_records_path.exists() {
            let file = File::create(referral_records_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建推广活动存储文件
        let campaigns_path = data_path.join(&self.config.campaigns_file);
        if !campaigns_path.exists() {
            let file = File::create(campaigns_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建团队信息存储文件
        let teams_path = data_path.join(&self.config.teams_file);
        if !teams_path.exists() {
            let file = File::create(teams_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        Ok(())
    }
    
    /// 保存邀请码
    pub async fn save_invite_code(&self, invite_code: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let invite_codes_path = data_path.join(&self.config.invite_codes_file);
        
        // 读取现有数据
        let mut file = File::open(&invite_codes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut invite_codes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let code_id = invite_code["id"].as_str().unwrap_or_default();
        if let Some(index) = invite_codes.iter().position(|c| c["id"].as_str().unwrap_or_default() == code_id) {
            invite_codes[index] = invite_code;
        } else {
            invite_codes.push(invite_code);
        }
        
        // 写回文件
        let mut file = File::create(&invite_codes_path)?;
        serde_json::to_writer_pretty(&mut file, &invite_codes)?;
        
        Ok(())
    }
    
    /// 保存推广记录
    pub async fn save_referral_record(&self, record: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let referral_records_path = data_path.join(&self.config.referral_records_file);
        
        // 读取现有数据
        let mut file = File::open(&referral_records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let record_id = record["id"].as_str().unwrap_or_default();
        if let Some(index) = records.iter().position(|r| r["id"].as_str().unwrap_or_default() == record_id) {
            records[index] = record;
        } else {
            records.push(record);
        }
        
        // 写回文件
        let mut file = File::create(&referral_records_path)?;
        serde_json::to_writer_pretty(&mut file, &records)?;
        
        Ok(())
    }
    
    /// 保存推广活动
    pub async fn save_campaign(&self, campaign: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let campaigns_path = data_path.join(&self.config.campaigns_file);
        
        // 读取现有数据
        let mut file = File::open(&campaigns_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut campaigns: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let campaign_id = campaign["id"].as_str().unwrap_or_default();
        if let Some(index) = campaigns.iter().position(|c| c["id"].as_str().unwrap_or_default() == campaign_id) {
            campaigns[index] = campaign;
        } else {
            campaigns.push(campaign);
        }
        
        // 写回文件
        let mut file = File::create(&campaigns_path)?;
        serde_json::to_writer_pretty(&mut file, &campaigns)?;
        
        Ok(())
    }
    
    /// 保存团队信息
    pub async fn save_team(&self, team: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let teams_path = data_path.join(&self.config.teams_file);
        
        // 读取现有数据
        let mut file = File::open(&teams_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut teams: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let team_id = team["id"].as_str().unwrap_or_default();
        if let Some(index) = teams.iter().position(|t| t["id"].as_str().unwrap_or_default() == team_id) {
            teams[index] = team;
        } else {
            teams.push(team);
        }
        
        // 写回文件
        let mut file = File::create(&teams_path)?;
        serde_json::to_writer_pretty(&mut file, &teams)?;
        
        Ok(())
    }
    
    /// 加载邀请码
    pub async fn load_invite_code(&self, code: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let invite_codes_path = data_path.join(&self.config.invite_codes_file);
        
        // 读取现有数据
        let mut file = File::open(&invite_codes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let invite_codes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找邀请码
        let invite_code = invite_codes.into_iter().find(|c| c["code"].as_str().unwrap_or_default() == code);
        
        Ok(invite_code)
    }
    
    /// 加载用户的邀请码
    pub async fn load_user_invite_codes(&self, user_id: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let invite_codes_path = data_path.join(&self.config.invite_codes_file);
        
        // 读取现有数据
        let mut file = File::open(&invite_codes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let invite_codes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出用户的邀请码
        let user_codes: Vec<serde_json::Value> = invite_codes
            .into_iter()
            .filter(|c| c["user_id"].as_str().unwrap_or_default() == user_id)
            .collect();
        
        Ok(user_codes)
    }
    
    /// 加载用户的推广记录
    pub async fn load_user_referrals(&self, user_id: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let referral_records_path = data_path.join(&self.config.referral_records_file);
        
        // 读取现有数据
        let mut file = File::open(&referral_records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出用户的推广记录
        let user_records: Vec<serde_json::Value> = records
            .into_iter()
            .filter(|r| r["referrer_id"].as_str().unwrap_or_default() == user_id)
            .collect();
        
        Ok(user_records)
    }
    
    /// 加载用户作为被推广者的记录
    pub async fn load_user_referred_by(&self, user_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let referral_records_path = data_path.join(&self.config.referral_records_file);
        
        // 读取现有数据
        let mut file = File::open(&referral_records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找用户作为被推广者的记录
        let record = records
            .into_iter()
            .find(|r| r["referee_id"].as_str().unwrap_or_default() == user_id);
        
        Ok(record)
    }
    
    /// 加载所有推广活动
    pub async fn load_campaigns(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let campaigns_path = data_path.join(&self.config.campaigns_file);
        
        // 读取现有数据
        let mut file = File::open(&campaigns_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let campaigns: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(campaigns)
    }
    
    /// 加载所有团队
    pub async fn load_teams(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let teams_path = data_path.join(&self.config.teams_file);
        
        // 读取现有数据
        let mut file = File::open(&teams_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let teams: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(teams)
    }
    
    /// 批量保存邀请码
    pub async fn batch_save_invite_codes(
        &self,
        invite_codes: Vec<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let invite_codes_path = data_path.join(&self.config.invite_codes_file);
        
        // 读取现有数据
        let mut file = File::open(&invite_codes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut existing_codes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 合并邀请码
        for code in invite_codes {
            let code_id = code["id"].as_str().unwrap_or_default();
            if let Some(index) = existing_codes.iter().position(|c| c["id"].as_str().unwrap_or_default() == code_id) {
                existing_codes[index] = code;
            } else {
                existing_codes.push(code);
            }
        }
        
        // 写回文件
        let mut file = File::create(&invite_codes_path)?;
        serde_json::to_writer_pretty(&mut file, &existing_codes)?;
        
        Ok(())
    }
}
