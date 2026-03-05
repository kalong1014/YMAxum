use serde::{Deserialize, Serialize}; use std::fs::File; use std::io::{Read}; use std::path::Path;

/// 存储配置
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 数据存储目录
    pub data_dir: String,
    /// 交易记录存储文件
    pub transactions_file: String,
    /// 仲裁记录存储文件
    pub arbitrations_file: String,
    /// 风险规则存储文件
    pub risk_rules_file: String,
}

/// 防欺诈保障存储模块
#[derive(Debug)]
pub struct FraudStorage {
    /// 存储配置
    config: StorageConfig,
}

impl FraudStorage {
    /// 创建新的存储模块
    pub fn new() -> Self {
        Self {
            config: StorageConfig {
                data_dir: "./data/fraud".to_string(),
                transactions_file: "transactions.json".to_string(),
                arbitrations_file: "arbitrations.json".to_string(),
                risk_rules_file: "risk_rules.json".to_string(),
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
        
        // 创建交易记录存储文件
        let transactions_path = data_path.join(&self.config.transactions_file);
        if !transactions_path.exists() {
            let file = File::create(transactions_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建仲裁记录存储文件
        let arbitrations_path = data_path.join(&self.config.arbitrations_file);
        if !arbitrations_path.exists() {
            let file = File::create(arbitrations_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建风险规则存储文件
        let risk_rules_path = data_path.join(&self.config.risk_rules_file);
        if !risk_rules_path.exists() {
            let file = File::create(risk_rules_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        Ok(())
    }
    
    /// 保存交易记录
    pub async fn save_transaction(&self, transaction: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let transactions_path = data_path.join(&self.config.transactions_file);
        
        // 读取现有数据
        let mut file = File::open(&transactions_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut transactions: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let transaction_id = transaction["id"].as_str().unwrap_or_default();
        if let Some(index) = transactions.iter().position(|t| t["id"].as_str().unwrap_or_default() == transaction_id) {
            transactions[index] = transaction;
        } else {
            transactions.push(transaction);
        }
        
        // 写回文件
        let mut file = File::create(&transactions_path)?;
        serde_json::to_writer_pretty(&mut file, &transactions)?;
        
        Ok(())
    }
    
    /// 保存仲裁记录
    pub async fn save_arbitration(&self, arbitration: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let arbitrations_path = data_path.join(&self.config.arbitrations_file);
        
        // 读取现有数据
        let mut file = File::open(&arbitrations_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut arbitrations: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let arbitration_id = arbitration["id"].as_str().unwrap_or_default();
        if let Some(index) = arbitrations.iter().position(|a| a["id"].as_str().unwrap_or_default() == arbitration_id) {
            arbitrations[index] = arbitration;
        } else {
            arbitrations.push(arbitration);
        }
        
        // 写回文件
        let mut file = File::create(&arbitrations_path)?;
        serde_json::to_writer_pretty(&mut file, &arbitrations)?;
        
        Ok(())
    }
    
    /// 保存风险规则
    pub async fn save_risk_rule(&self, rule: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let risk_rules_path = data_path.join(&self.config.risk_rules_file);
        
        // 读取现有数据
        let mut file = File::open(&risk_rules_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut rules: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let rule_id = rule["id"].as_str().unwrap_or_default();
        if let Some(index) = rules.iter().position(|r| r["id"].as_str().unwrap_or_default() == rule_id) {
            rules[index] = rule;
        } else {
            rules.push(rule);
        }
        
        // 写回文件
        let mut file = File::create(&risk_rules_path)?;
        serde_json::to_writer_pretty(&mut file, &rules)?;
        
        Ok(())
    }
    
    /// 加载交易记录
    pub async fn load_transaction(&self, transaction_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let transactions_path = data_path.join(&self.config.transactions_file);
        
        // 读取现有数据
        let mut file = File::open(&transactions_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let transactions: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找交易
        let transaction = transactions.into_iter().find(|t| t["id"].as_str().unwrap_or_default() == transaction_id);
        
        Ok(transaction)
    }
    
    /// 加载用户交易记录
    pub async fn load_user_transactions(
        &self,
        user_id: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let transactions_path = data_path.join(&self.config.transactions_file);
        
        // 读取现有数据
        let mut file = File::open(&transactions_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let transactions: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出用户的交易
        let user_transactions: Vec<serde_json::Value> = transactions
            .into_iter()
            .filter(|t| {
                t["from_user_id"].as_str().unwrap_or_default() == user_id || 
                t["to_user_id"].as_str().unwrap_or_default() == user_id
            })
            .collect();
        
        // 应用分页
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > user_transactions.len() {
            user_transactions.len()
        } else {
            end
        };
        
        Ok(user_transactions[start..end].to_vec())
    }
    
    /// 加载仲裁记录
    pub async fn load_arbitration(&self, arbitration_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let arbitrations_path = data_path.join(&self.config.arbitrations_file);
        
        // 读取现有数据
        let mut file = File::open(&arbitrations_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let arbitrations: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找仲裁
        let arbitration = arbitrations.into_iter().find(|a| a["id"].as_str().unwrap_or_default() == arbitration_id);
        
        Ok(arbitration)
    }
    
    /// 加载用户仲裁记录
    pub async fn load_user_arbitrations(
        &self,
        user_id: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let arbitrations_path = data_path.join(&self.config.arbitrations_file);
        
        // 读取现有数据
        let mut file = File::open(&arbitrations_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let arbitrations: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出用户的仲裁
        let user_arbitrations: Vec<serde_json::Value> = arbitrations
            .into_iter()
            .filter(|a| {
                a["from_user_id"].as_str().unwrap_or_default() == user_id || 
                a["to_user_id"].as_str().unwrap_or_default() == user_id || 
                a["initiator_id"].as_str().unwrap_or_default() == user_id
            })
            .collect();
        
        // 应用分页
        let start = offset as usize;
        let end = (offset + limit) as usize;
        let end = if end > user_arbitrations.len() {
            user_arbitrations.len()
        } else {
            end
        };
        
        Ok(user_arbitrations[start..end].to_vec())
    }
    
    /// 加载风险规则
    pub async fn load_risk_rules(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let risk_rules_path = data_path.join(&self.config.risk_rules_file);
        
        // 读取现有数据
        let mut file = File::open(&risk_rules_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let rules: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(rules)
    }
    
    /// 批量保存交易记录
    pub async fn batch_save_transactions(
        &self,
        transactions: Vec<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let transactions_path = data_path.join(&self.config.transactions_file);
        
        // 读取现有数据
        let mut file = File::open(&transactions_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut existing_transactions: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 合并交易记录
        for transaction in transactions {
            let transaction_id = transaction["id"].as_str().unwrap_or_default();
            if let Some(index) = existing_transactions.iter().position(|t| t["id"].as_str().unwrap_or_default() == transaction_id) {
                existing_transactions[index] = transaction;
            } else {
                existing_transactions.push(transaction);
            }
        }
        
        // 写回文件
        let mut file = File::create(&transactions_path)?;
        serde_json::to_writer_pretty(&mut file, &existing_transactions)?;
        
        Ok(())
    }
}
