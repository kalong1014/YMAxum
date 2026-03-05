use serde::{Deserialize, Serialize}; use std::fs::File; use std::io::{Read}; use std::path::Path;

/// 存储配置
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 数据存储目录
    pub data_dir: String,
    /// 用户积分余额存储文件
    pub balances_file: String,
    /// 积分记录存储文件
    pub records_file: String,
}

/// 积分生态存储模块
#[derive(Debug)]
pub struct PointsStorage {
    /// 存储配置
    config: StorageConfig,
}

impl PointsStorage {
    /// 创建新的存储模块
    pub fn new() -> Self {
        Self {
            config: StorageConfig {
                data_dir: "./data/points".to_string(),
                balances_file: "balances.json".to_string(),
                records_file: "records.json".to_string(),
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
        
        // 创建用户积分余额存储文件
        let balances_path = data_path.join(&self.config.balances_file);
        if !balances_path.exists() {
            let file = File::create(balances_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建积分记录存储文件
        let records_path = data_path.join(&self.config.records_file);
        if !records_path.exists() {
            let file = File::create(records_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        Ok(())
    }
    
    /// 保存用户积分余额
    pub async fn save_balance(&self, balance: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let balances_path = data_path.join(&self.config.balances_file);
        
        // 读取现有数据
        let mut file = File::open(&balances_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut balances: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let user_id = balance["user_id"].as_str().unwrap_or_default();
        if let Some(index) = balances.iter().position(|b| b["user_id"].as_str().unwrap_or_default() == user_id) {
            balances[index] = balance;
        } else {
            balances.push(balance);
        }
        
        // 写回文件
        let mut file = File::create(&balances_path)?;
        serde_json::to_writer_pretty(&mut file, &balances)?;
        
        Ok(())
    }
    
    /// 保存积分记录
    pub async fn save_record(&self, record: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let records_path = data_path.join(&self.config.records_file);
        
        // 读取现有数据
        let mut file = File::open(&records_path)?;
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
        let mut file = File::create(&records_path)?;
        serde_json::to_writer_pretty(&mut file, &records)?;
        
        Ok(())
    }
    
    /// 加载用户积分余额
    pub async fn load_balance(&self, user_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let balances_path = data_path.join(&self.config.balances_file);
        
        // 读取现有数据
        let mut file = File::open(&balances_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let balances: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找用户余额
        let balance = balances.into_iter().find(|b| b["user_id"].as_str().unwrap_or_default() == user_id);
        
        Ok(balance)
    }
    
    /// 加载所有用户积分余额
    pub async fn load_all_balances(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let balances_path = data_path.join(&self.config.balances_file);
        
        // 读取现有数据
        let mut file = File::open(&balances_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let balances: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(balances)
    }
    
    /// 加载用户积分记录
    pub async fn load_user_records(
        &self,
        user_id: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let records_path = data_path.join(&self.config.records_file);
        
        // 读取现有数据
        let mut file = File::open(&records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出用户的记录
        let user_records: Vec<serde_json::Value> = records
            .into_iter()
            .filter(|r| r["user_id"].as_str().unwrap_or_default() == user_id)
            .collect();
        
        // 应用分页
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > user_records.len() {
            user_records.len()
        } else {
            end
        };
        
        Ok(user_records[start..end].to_vec())
    }
    
    /// 加载所有积分记录
    pub async fn load_all_records(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let records_path = data_path.join(&self.config.records_file);
        
        // 读取现有数据
        let mut file = File::open(&records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(records)
    }
    
    /// 批量保存积分记录
    pub async fn batch_save_records(
        &self,
        records: Vec<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let records_path = data_path.join(&self.config.records_file);
        
        // 读取现有数据
        let mut file = File::open(&records_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut existing_records: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 合并记录
        for record in records {
            let record_id = record["id"].as_str().unwrap_or_default();
            if let Some(index) = existing_records.iter().position(|r| r["id"].as_str().unwrap_or_default() == record_id) {
                existing_records[index] = record;
            } else {
                existing_records.push(record);
            }
        }
        
        // 写回文件
        let mut file = File::create(&records_path)?;
        serde_json::to_writer_pretty(&mut file, &existing_records)?;
        
        Ok(())
    }
}
