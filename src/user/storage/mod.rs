use serde::{Deserialize, Serialize}; use std::fs::File; use std::io::Read; use std::path::Path;

/// 存储配置
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 数据存储目录
    pub data_dir: String,
    /// 用户信息存储文件
    pub users_file: String,
    /// 等级配置存储文件
    pub levels_file: String,
    /// 成长任务存储文件
    pub tasks_file: String,
    /// 经验值记录存储文件
    pub exp_records_file: String,
}

/// 用户成长与权限存储模块
#[derive(Debug)]
pub struct UserStorage {
    /// 存储配置
    config: StorageConfig,
}

impl UserStorage {
    /// 创建新的存储模块
    pub fn new() -> Self {
        Self {
            config: StorageConfig {
                data_dir: "./data/user".to_string(),
                users_file: "users.json".to_string(),
                levels_file: "levels.json".to_string(),
                tasks_file: "tasks.json".to_string(),
                exp_records_file: "exp_records.json".to_string(),
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
        
        // 创建用户信息存储文件
        let users_path = data_path.join(&self.config.users_file);
        if !users_path.exists() {
            let file = File::create(users_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建等级配置存储文件
        let levels_path = data_path.join(&self.config.levels_file);
        if !levels_path.exists() {
            let file = File::create(levels_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建成长任务存储文件
        let tasks_path = data_path.join(&self.config.tasks_file);
        if !tasks_path.exists() {
            let file = File::create(tasks_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        // 创建经验值记录存储文件
        let exp_records_path = data_path.join(&self.config.exp_records_file);
        if !exp_records_path.exists() {
            let file = File::create(exp_records_path)?;
            serde_json::to_writer(file, &Vec::<serde_json::Value>::new())?;
        }
        
        Ok(())
    }
    
    /// 保存用户信息
    pub async fn save_user(&self, user: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let users_path = data_path.join(&self.config.users_file);
        
        // 读取现有数据
        let mut file = File::open(&users_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut users: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let user_id = user["user_id"].as_str().unwrap_or_default();
        if let Some(index) = users.iter().position(|u| u["user_id"].as_str().unwrap_or_default() == user_id) {
            users[index] = user;
        } else {
            users.push(user);
        }
        
        // 写回文件
        let mut file = File::create(&users_path)?;
        serde_json::to_writer_pretty(&mut file, &users)?;
        
        Ok(())
    }
    
    /// 保存等级配置
    pub async fn save_level(&self, level: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let levels_path = data_path.join(&self.config.levels_file);
        
        // 读取现有数据
        let mut file = File::open(&levels_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut levels: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let level_id = level["id"].as_u64().unwrap_or_default() as u32;
        if let Some(index) = levels.iter().position(|l| l["id"].as_u64().unwrap_or_default() as u32 == level_id) {
            levels[index] = level;
        } else {
            levels.push(level);
        }
        
        // 写回文件
        let mut file = File::create(&levels_path)?;
        serde_json::to_writer_pretty(&mut file, &levels)?;
        
        Ok(())
    }
    
    /// 保存成长任务
    pub async fn save_task(&self, task: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let tasks_path = data_path.join(&self.config.tasks_file);
        
        // 读取现有数据
        let mut file = File::open(&tasks_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut tasks: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 检查是否已存在
        let task_id = task["id"].as_str().unwrap_or_default();
        if let Some(index) = tasks.iter().position(|t| t["id"].as_str().unwrap_or_default() == task_id) {
            tasks[index] = task;
        } else {
            tasks.push(task);
        }
        
        // 写回文件
        let mut file = File::create(&tasks_path)?;
        serde_json::to_writer_pretty(&mut file, &tasks)?;
        
        Ok(())
    }
    
    /// 保存经验值记录
    pub async fn save_exp_record(&self, record: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let exp_records_path = data_path.join(&self.config.exp_records_file);
        
        // 读取现有数据
        let mut file = File::open(&exp_records_path)?;
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
        let mut file = File::create(&exp_records_path)?;
        serde_json::to_writer_pretty(&mut file, &records)?;
        
        Ok(())
    }
    
    /// 加载用户信息
    pub async fn load_user(&self, user_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let users_path = data_path.join(&self.config.users_file);
        
        // 读取现有数据
        let mut file = File::open(&users_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let users: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找用户
        let user = users.into_iter().find(|u| u["user_id"].as_str().unwrap_or_default() == user_id);
        
        Ok(user)
    }
    
    /// 加载所有用户信息
    pub async fn load_all_users(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let users_path = data_path.join(&self.config.users_file);
        
        // 读取现有数据
        let mut file = File::open(&users_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let users: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(users)
    }
    
    /// 加载等级配置
    pub async fn load_level(&self, level_id: u32) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let levels_path = data_path.join(&self.config.levels_file);
        
        // 读取现有数据
        let mut file = File::open(&levels_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let levels: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找等级
        let level = levels.into_iter().find(|l| l["id"].as_u64().unwrap_or_default() as u32 == level_id);
        
        Ok(level)
    }
    
    /// 加载所有等级配置
    pub async fn load_all_levels(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let levels_path = data_path.join(&self.config.levels_file);
        
        // 读取现有数据
        let mut file = File::open(&levels_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let levels: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(levels)
    }
    
    /// 加载成长任务
    pub async fn load_task(&self, task_id: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let tasks_path = data_path.join(&self.config.tasks_file);
        
        // 读取现有数据
        let mut file = File::open(&tasks_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let tasks: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 查找任务
        let task = tasks.into_iter().find(|t| t["id"].as_str().unwrap_or_default() == task_id);
        
        Ok(task)
    }
    
    /// 加载所有成长任务
    pub async fn load_all_tasks(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let tasks_path = data_path.join(&self.config.tasks_file);
        
        // 读取现有数据
        let mut file = File::open(&tasks_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let tasks: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        Ok(tasks)
    }
    
    /// 加载用户经验值记录
    pub async fn load_user_exp_records(
        &self,
        user_id: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let exp_records_path = data_path.join(&self.config.exp_records_file);
        
        // 读取现有数据
        let mut file = File::open(&exp_records_path)?;
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
    
    /// 批量保存用户信息
    pub async fn batch_save_users(
        &self,
        users: Vec<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_path = Path::new(&self.config.data_dir);
        let users_path = data_path.join(&self.config.users_file);
        
        // 读取现有数据
        let mut file = File::open(&users_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let mut existing_users: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        
        // 合并用户信息
        for user in users {
            let user_id = user["user_id"].as_str().unwrap_or_default();
            if let Some(index) = existing_users.iter().position(|u| u["user_id"].as_str().unwrap_or_default() == user_id) {
                existing_users[index] = user;
            } else {
                existing_users.push(user);
            }
        }
        
        // 写回文件
        let mut file = File::create(&users_path)?;
        serde_json::to_writer_pretty(&mut file, &existing_users)?;
        
        Ok(())
    }
}
