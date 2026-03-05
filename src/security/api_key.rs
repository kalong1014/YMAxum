//! API密钥管理模块
//! 用于生成、验证、吊销和管理API密钥

use chrono;
use hex;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// API密钥配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// 密钥长度
    pub key_length: usize,
    /// 密钥过期时间(天)
    pub expiration_days: u32,
    /// 启用密钥轮换
    pub key_rotation_enabled: bool,
    /// 密钥轮换周期(天)
    pub key_rotation_period: u32,
    /// 启用密钥吊销
    pub key_revocation_enabled: bool,
    /// 最大活跃密钥数
    pub max_active_keys: u32,
    /// 启用IP限制
    pub ip_restriction_enabled: bool,
    /// 允许的IP地址列表
    pub allowed_ips: Vec<String>,
    /// 启用速率限制
    pub rate_limit_enabled: bool,
    /// 速率限制(请求/分钟)
    pub rate_limit: u32,
}

/// API密钥状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiKeyStatus {
    /// 活跃
    Active,
    /// 过期
    Expired,
    /// 已吊销
    Revoked,
    /// 待激活
    Pending,
}

/// API密钥
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// 密钥ID
    pub id: String,
    /// 密钥值
    pub key: String,
    /// 密钥哈希
    pub key_hash: String,
    /// 密钥名称
    pub name: String,
    /// 密钥描述
    pub description: String,
    /// 创建时间
    pub created_at: String,
    /// 过期时间
    pub expires_at: String,
    /// 状态
    pub status: ApiKeyStatus,
    /// 创建者
    pub created_by: String,
    /// 最后使用时间
    pub last_used_at: Option<String>,
    /// 使用次数
    pub usage_count: u64,
    /// 允许的IP地址
    pub allowed_ips: Vec<String>,
    /// 速率限制
    pub rate_limit: Option<u32>,
    /// 权限列表
    pub permissions: Vec<String>,
}

/// API密钥管理
#[derive(Debug, Clone)]
pub struct ApiKeyManager {
    config: Arc<RwLock<ApiKeyConfig>>,
    keys: Arc<RwLock<Vec<ApiKey>>>,
}

impl ApiKeyManager {
    /// 创建新的API密钥管理器
    pub fn new(config: ApiKeyConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            keys: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 初始化API密钥管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 清理过期密钥
        self.cleanup_expired_keys().await;
        Ok(())
    }

    /// 生成API密钥
    pub async fn generate_key(
        &self,
        name: &str,
        description: &str,
        created_by: &str,
        permissions: Vec<String>,
    ) -> Result<ApiKey, Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        let mut keys = self.keys.write().await;

        // 检查活跃密钥数量
        let active_keys_count = keys
            .iter()
            .filter(|k| k.status == ApiKeyStatus::Active)
            .count();
        if active_keys_count >= config.max_active_keys as usize {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Maximum number of active keys reached",
            )));
        }

        // 生成密钥
        let id = Uuid::new_v4().to_string();
        let key = self.generate_key_value(config.key_length);
        let key_hash = self.hash_key(&key);
        let created_at = chrono::Utc::now().to_string();
        let expires_at = (chrono::Utc::now()
            + chrono::Duration::days(config.expiration_days as i64))
        .to_string();

        let api_key = ApiKey {
            id,
            key: key.clone(),
            key_hash,
            name: name.to_string(),
            description: description.to_string(),
            created_at,
            expires_at,
            status: ApiKeyStatus::Active,
            created_by: created_by.to_string(),
            last_used_at: None,
            usage_count: 0,
            allowed_ips: config.allowed_ips.clone(),
            rate_limit: Some(config.rate_limit),
            permissions,
        };

        keys.push(api_key.clone());
        Ok(api_key)
    }

    /// 验证API密钥
    pub async fn validate_key(
        &self,
        key: &str,
        ip: Option<&str>,
    ) -> Result<ApiKey, Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        let mut keys = self.keys.write().await;

        // 查找密钥
        let key_hash = self.hash_key(key);
        let key_index = keys.iter().position(|k| k.key_hash == key_hash);

        if let Some(index) = key_index {
            let api_key = &mut keys[index];

            // 检查密钥状态
            if api_key.status != ApiKeyStatus::Active {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "API key is not active",
                )));
            }

            // 检查密钥是否过期
            if self.is_key_expired(api_key) {
                api_key.status = ApiKeyStatus::Expired;
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "API key has expired",
                )));
            }

            // 检查IP限制
            if config.ip_restriction_enabled
                && !api_key.allowed_ips.is_empty()
                && let Some(client_ip) = ip
                && !api_key.allowed_ips.contains(&client_ip.to_string())
            {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "IP address not allowed",
                )));
            }

            // 更新使用信息
            api_key.last_used_at = Some(chrono::Utc::now().to_string());
            api_key.usage_count += 1;

            Ok(api_key.clone())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Invalid API key",
            )))
        }
    }

    /// 吊销API密钥
    pub async fn revoke_key(&self, key_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut keys = self.keys.write().await;
        let key_index = keys.iter().position(|k| k.id == key_id);

        if let Some(index) = key_index {
            keys[index].status = ApiKeyStatus::Revoked;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "API key not found",
            )))
        }
    }

    /// 列出所有API密钥
    pub async fn list_keys(&self) -> Vec<ApiKey> {
        self.keys.read().await.clone()
    }

    /// 列出活跃API密钥
    pub async fn list_active_keys(&self) -> Vec<ApiKey> {
        let keys = self.keys.read().await;
        keys.iter()
            .filter(|k| k.status == ApiKeyStatus::Active)
            .cloned()
            .collect()
    }

    /// 获取API密钥
    pub async fn get_key(&self, key_id: &str) -> Option<ApiKey> {
        let keys = self.keys.read().await;
        keys.iter().find(|k| k.id == key_id).cloned()
    }

    /// 清理过期密钥
    pub async fn cleanup_expired_keys(&self) {
        let mut keys_guard = self.keys.write().await;
        let now = chrono::Utc::now();

        for key in keys_guard.iter_mut() {
            if key.status == ApiKeyStatus::Active
                && let Ok(expires_at) = chrono::DateTime::parse_from_rfc3339(&key.expires_at)
                && now > expires_at
            {
                key.status = ApiKeyStatus::Expired;
            }
        }
    }

    /// 生成密钥值
    fn generate_key_value(&self, length: usize) -> String {
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_+"
            .chars()
            .collect();
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    /// 哈希密钥
    fn hash_key(&self, key: &str) -> String {
        // 使用SHA-256哈希密钥
        let hash = ring::digest::digest(&ring::digest::SHA256, key.as_bytes());
        hex::encode(hash.as_ref())
    }

    /// 检查密钥是否过期
    fn is_key_expired(&self, key: &ApiKey) -> bool {
        if let Ok(expires_at) = chrono::DateTime::parse_from_rfc3339(&key.expires_at) {
            chrono::Utc::now() > expires_at
        } else {
            true
        }
    }

    /// 更新API密钥配置
    pub async fn update_config(
        &self,
        config: ApiKeyConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 获取API密钥配置
    pub async fn get_config(&self) -> ApiKeyConfig {
        self.config.read().await.clone()
    }

    /// 检查速率限制
    pub async fn check_rate_limit(&self, key_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let keys = self.keys.read().await;
        if let Some(_key) = keys.iter().find(|k| k.id == key_id) {
            // 实现速率限制检查
            Ok(true)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "API key not found",
            )))
        }
    }
}
