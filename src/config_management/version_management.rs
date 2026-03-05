//! 配置版本管理模块
//! 用于管理配置文件的版本控制和变更历史

use serde::{Deserialize, Serialize};
use std::path::Path;

/// 版本配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConfig {
    /// 配置ID
    pub config_id: String,
    /// 操作类型
    pub operation_type: String,
    /// 版本号
    pub version: String,
    /// 配置文件路径
    pub config_file_path: String,
    /// 操作描述
    pub operation_description: String,
    /// 操作参数
    pub parameters: serde_json::Value,
}

/// 版本结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResult {
    /// 操作状态
    pub status: String,
    /// 结果ID
    pub result_id: String,
    /// 版本信息
    pub version_info: VersionInfo,
    /// 操作时间
    pub operation_time: String,
    /// 操作描述
    pub operation_description: String,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// 版本号
    pub version: String,
    /// 配置文件路径
    pub config_file_path: String,
    /// 版本描述
    pub version_description: String,
    /// 创建时间
    pub created_at: String,
    /// 创建者
    pub created_by: String,
    /// 版本状态
    pub status: String,
    /// 版本标签
    pub tags: Vec<String>,
    /// 配置内容快照
    pub config_snapshot: serde_json::Value,
}

/// 版本历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistory {
    /// 历史ID
    pub history_id: String,
    /// 版本信息
    pub version_info: VersionInfo,
    /// 变更内容
    pub change_content: serde_json::Value,
    /// 变更时间
    pub change_time: String,
    /// 变更者
    pub changed_by: String,
}

/// 版本差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    /// 版本1
    pub version1: String,
    /// 版本2
    pub version2: String,
    /// 增加的键
    pub added: Vec<String>,
    /// 修改的键
    pub modified: Vec<String>,
    /// 删除的键
    pub removed: Vec<String>,
    /// 具体变更
    pub changes: serde_json::Value,
}

/// 版本管理器
#[derive(Debug, Clone)]
pub struct VersionManager {
    /// 版本结果列表
    version_results: std::sync::Arc<tokio::sync::RwLock<Vec<VersionResult>>>,
    /// 版本历史列表
    version_history: std::sync::Arc<tokio::sync::RwLock<Vec<VersionHistory>>>,
    /// 版本存储路径
    version_storage_path: String,
}

impl VersionManager {
    /// 创建新的版本管理器
    pub fn new() -> Self {
        let version_storage_path = "config/versions".to_string();
        // 确保版本存储目录存在
        std::fs::create_dir_all(&version_storage_path).ok();
        
        Self {
            version_results: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            version_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            version_storage_path,
        }
    }

    /// 初始化版本管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 加载已存储的版本历史
        self.load_version_history().await?;
        
        Ok(())
    }

    /// 管理配置版本
    pub async fn manage_version(
        &self,
        config: VersionConfig,
    ) -> Result<VersionResult, Box<dyn std::error::Error>> {

        // 读取配置文件内容作为快照
        let config_snapshot = match tokio::fs::read_to_string(&config.config_file_path).await {
            Ok(content) => {
                let extension = config.config_file_path.split('.').last().unwrap_or("");
                match extension {
                    "toml" => {
                        let toml_value: toml::Value = toml::from_str(&content)?;
                        serde_json::to_value(toml_value)?
                    }
                    "json" => {
                        serde_json::from_str(&content)?
                    }
                    "yaml" | "yml" => {
                        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
                        serde_json::to_value(yaml_value)?
                    }
                    _ => serde_json::Value::Null
                }
            }
            Err(_) => serde_json::Value::Null
        };

        // 提取版本标签
        let tags = if let Some(tags_value) = config.parameters.get("tags") {
            if let Some(array) = tags_value.as_array() {
                array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // 生成版本信息
        let version_info = VersionInfo {
            version: config.version,
            config_file_path: config.config_file_path,
            version_description: config.operation_description.clone(),
            created_at: chrono::Utc::now().to_string(),
            created_by: "system".to_string(),
            status: "active".to_string(),
            tags,
            config_snapshot,
        };

        // 生成版本历史
        let version_history = VersionHistory {
            history_id: format!(
                "hist_{}_{}",
                config.config_id,
                chrono::Utc::now().timestamp()
            ),
            version_info: version_info.clone(),
            change_content: serde_json::json!({
                "operation": config.operation_type,
                "parameters": config.parameters
            }),
            change_time: chrono::Utc::now().to_string(),
            changed_by: "system".to_string(),
        };

        // 添加到版本历史列表
        let mut version_history_list = self.version_history.write().await;
        version_history_list.push(version_history.clone());

        // 持久化版本历史
        self.save_version_history(&*version_history_list).await?;

        // 生成版本结果
        let result = VersionResult {
            status: "completed".to_string(),
            result_id: format!(
                "ver_{}_{}",
                config.config_id,
                chrono::Utc::now().timestamp()
            ),
            version_info,
            operation_time: chrono::Utc::now().to_string(),
            operation_description: config.operation_description,
        };

        // 添加到版本结果列表
        let mut version_results = self.version_results.write().await;
        version_results.push(result.clone());

        Ok(result)
    }

    /// 获取版本历史
    pub async fn get_version_history(
        &self,
        config_file_path: String,
    ) -> Result<Vec<VersionHistory>, Box<dyn std::error::Error>> {
        let version_history = self.version_history.read().await;
        let filtered_history = (&*version_history)
            .iter()
            .filter(|h| h.version_info.config_file_path == config_file_path)
            .cloned()
            .collect();
        Ok(filtered_history)
    }

    /// 获取版本结果列表
    pub async fn get_version_results(
        &self,
    ) -> Result<Vec<VersionResult>, Box<dyn std::error::Error>> {
        let version_results = self.version_results.read().await;
        Ok((&*version_results).clone())
    }

    /// 比较两个版本
    pub async fn compare_versions(
        &self,
        version1: String,
        version2: String,
        config_file_path: String,
    ) -> Result<VersionDiff, Box<dyn std::error::Error>> {
        // 获取两个版本的历史记录
        let version_history = self.version_history.read().await;
        
        let version1_history = (&*version_history)
            .iter()
            .find(|h| h.version_info.version == version1 && h.version_info.config_file_path == config_file_path);
        
        let version2_history = (&*version_history)
            .iter()
            .find(|h| h.version_info.version == version2 && h.version_info.config_file_path == config_file_path);
        
        if version1_history.is_none() || version2_history.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "One or both versions not found",
            )));
        }
        
        let version1_snapshot = &version1_history.unwrap().version_info.config_snapshot;
        let version2_snapshot = &version2_history.unwrap().version_info.config_snapshot;
        
        // 比较两个版本的配置内容
        let (added, modified, removed, changes) = self.compare_configs(version1_snapshot, version2_snapshot, "");
        
        let diff = VersionDiff {
            version1,
            version2,
            added,
            modified,
            removed,
            changes,
        };
        Ok(diff)
    }
    
    /// 比较两个配置对象
    fn compare_configs(&self, old: &serde_json::Value, new: &serde_json::Value, prefix: &str) -> (Vec<String>, Vec<String>, Vec<String>, serde_json::Value) {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut removed = Vec::new();
        let mut changes = serde_json::Map::new();
        
        match (old, new) {
            (serde_json::Value::Object(old_map), serde_json::Value::Object(new_map)) => {
                // 检查新增的键
                for (key, new_value) in new_map {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    
                    if !old_map.contains_key(key) {
                        added.push(full_key.clone());
                        changes.insert(full_key, serde_json::json!({
                            "type": "added",
                            "value": new_value
                        }));
                    } else {
                        let old_value = old_map.get(key).unwrap();
                        if old_value != new_value {
                            let (nested_added, nested_modified, nested_removed, nested_changes) = 
                                self.compare_configs(old_value, new_value, &full_key);
                            
                            if !nested_added.is_empty() || !nested_modified.is_empty() || !nested_removed.is_empty() {
                                modified.push(full_key.clone());
                                changes.insert(full_key, nested_changes);
                            }
                        }
                    }
                }
                
                // 检查删除的键
                for (key, old_value) in old_map {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    
                    if !new_map.contains_key(key) {
                        removed.push(full_key.clone());
                        changes.insert(full_key, serde_json::json!({
                            "type": "removed",
                            "value": old_value
                        }));
                    }
                }
            }
            (serde_json::Value::Array(old_array), serde_json::Value::Array(new_array)) => {
                // 简单比较数组长度
                if old_array.len() != new_array.len() {
                    let full_key = prefix.to_string();
                    modified.push(full_key.clone());
                    changes.insert(full_key, serde_json::json!({
                        "type": "modified",
                        "old_length": old_array.len(),
                        "new_length": new_array.len()
                    }));
                }
            }
            (old_value, new_value) => {
                if old_value != new_value {
                    let full_key = prefix.to_string();
                    modified.push(full_key.clone());
                    changes.insert(full_key, serde_json::json!({
                        "type": "modified",
                        "old_value": old_value,
                        "new_value": new_value
                    }));
                }
            }
        }
        
        (added, modified, removed, serde_json::Value::Object(changes))
    }

    /// 回滚到指定版本
    pub async fn rollback_to_version(
        &self,
        version: String,
        config_file_path: String,
    ) -> Result<VersionResult, Box<dyn std::error::Error>> {
        // 获取指定版本的历史记录
        let target_version = {
            let version_history = self.version_history.read().await;
            (&*version_history)
                .iter()
                .find(|h| h.version_info.version == version && h.version_info.config_file_path == config_file_path)
                .cloned()
        };
        
        if target_version.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Version {} not found for config file {}", version, config_file_path),
            )));
        }
        
        let target_version = target_version.unwrap();
        let config_snapshot = target_version.version_info.config_snapshot;
        
        // 将配置快照写回文件
        if !config_snapshot.is_null() {
            let extension = config_file_path.split('.').last().unwrap_or("");
            let content = match extension {
                "toml" => {
                    let toml_value: toml::Value = serde_json::from_value(config_snapshot.clone())?;
                    toml::to_string(&toml_value)?
                }
                "json" => {
                    serde_json::to_string_pretty(&config_snapshot)?
                }
                "yaml" | "yml" => {
                    let yaml_value: serde_yaml::Value = serde_json::from_value(config_snapshot.clone())?;
                    serde_yaml::to_string(&yaml_value)?
                }
                _ => return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unsupported file format",
                ))),
            };
            
            tokio::fs::write(&config_file_path, content).await?;
        }
        
        // 生成回滚版本配置
        let version_clone = version.clone();
        let config = VersionConfig {
            config_id: format!("rollback_{}", chrono::Utc::now().timestamp()),
            operation_type: "rollback".to_string(),
            version: format!("{}-rollback-{}", version, chrono::Utc::now().timestamp()),
            config_file_path: config_file_path.clone(),
            operation_description: format!("Rollback to version {}", version_clone),
            parameters: serde_json::json!({
                "original_version": version_clone,
                "rollback_time": chrono::Utc::now().to_string()
            }),
        };
        
        self.manage_version(config).await
    }

    /// 保存版本历史到文件
    async fn save_version_history(&self, version_history: &[VersionHistory]) -> Result<(), Box<dyn std::error::Error>> {
        let history_path = Path::new(&self.version_storage_path).join("version_history.json");
        let content = serde_json::to_string_pretty(version_history)?;
        tokio::fs::write(history_path, content).await?;
        Ok(())
    }

    /// 从文件加载版本历史
    async fn load_version_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let history_path = Path::new(&self.version_storage_path).join("version_history.json");
        if tokio::fs::try_exists(history_path.clone()).await? {
            let content = tokio::fs::read_to_string(history_path).await?;
            let history: Vec<VersionHistory> = serde_json::from_str(&content)?;
            let mut version_history_list = self.version_history.write().await;
            version_history_list.extend(history);
        }
        Ok(())
    }
}
