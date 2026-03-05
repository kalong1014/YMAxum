use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// 存储配置
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 数据存储目录
    pub data_dir: String,
    /// 成果存储文件
    pub works_file: String,
    /// 权属变更记录存储文件
    pub ownership_changes_file: String,
}

/// 去中心化确权存储模块
#[derive(Debug)]
pub struct RightsStorage {
    /// 存储配置
    config: StorageConfig,
}

impl RightsStorage {
    /// 创建新的存储模块
    pub fn new() -> Self {
        Self {
            config: StorageConfig {
                data_dir: "./data/rights".to_string(),
                works_file: "works.json".to_string(),
                ownership_changes_file: "ownership_changes.json".to_string(),
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
        
        // 创建成果存储文件
        let works_path = data_path.join(&self.config.works_file);
        if !works_path.exists() {
            let file = File::create(works_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建权属变更记录存储文件
        let changes_path = data_path.join(&self.config.ownership_changes_file);
        if !changes_path.exists() {
            let file = File::create(changes_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        Ok(())
    }
    
    /// 保存成果
    pub async fn save_work(&self, work: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let works_path = data_path.join(&self.config.works_file);
        
        // 读取现有数据
        let mut file = File::open(&works_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut works: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let work_id = work["id"].as_str().unwrap_or_default();
        if let Some(index) = works.iter().position(|w| w["id"].as_str().unwrap_or_default() == work_id) {
            works[index] = work;
        } else {
            works.push(work);
        }
        
        // 写回文件
        let mut file = File::create(&works_path)?;
        serde_json::to_writer_pretty(&mut file, &works)?;
        
        Ok(())
    }
    
    /// 保存权属变更记录
    pub async fn save_ownership_change(
        &self,
        change: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let changes_path = data_path.join(&self.config.ownership_changes_file);
        
        // 读取现有数据
        let mut file = File::open(&changes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut changes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let change_id = change["id"].as_str().unwrap_or_default();
        if let Some(index) = changes.iter().position(|c| c["id"].as_str().unwrap_or_default() == change_id) {
            changes[index] = change;
        } else {
            changes.push(change);
        }
        
        // 写回文件
        let mut file = File::create(&changes_path)?;
        serde_json::to_writer_pretty(&mut file, &changes)?;
        
        Ok(())
    }
    
    /// 加载成果
    pub async fn load_work(&self, work_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let works_path = data_path.join(&self.config.works_file);
        
        // 读取现有数据
        let mut file = File::open(&works_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let works: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找成果
        let work = works.into_iter().find(|w| w["id"].as_str().unwrap_or_default() == work_id);
        
        Ok(work)
    }
    
    /// 加载所有成果
    pub async fn load_all_works(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let works_path = data_path.join(&self.config.works_file);
        
        // 读取现有数据
        let mut file = File::open(&works_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let works: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(works)
    }
    
    /// 加载权属变更记录
    pub async fn load_ownership_changes(
        &self,
        work_id: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let changes_path = data_path.join(&self.config.ownership_changes_file);
        
        // 读取现有数据
        let mut file = File::open(&changes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let changes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 过滤出指定成果的变更记录
        let filtered_changes = changes
            .into_iter()
            .filter(|c| c["work_id"].as_str().unwrap_or_default() == work_id)
            .collect();
        
        Ok(filtered_changes)
    }
    
    /// 加载所有权属变更记录
    pub async fn load_all_ownership_changes(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let changes_path = data_path.join(&self.config.ownership_changes_file);
        
        // 读取现有数据
        let mut file = File::open(&changes_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let changes: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(changes)
    }
}
