//! 缓存插件
//! 提供内存缓存和Redis分布式缓存功能

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 缓存类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheType {
    Memory,
    Redis,
}

/// 缓存请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl: Option<u64>,
    pub cache_type: Option<CacheType>,
}

/// 缓存获取请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCacheRequest {
    pub key: String,
    pub cache_type: Option<CacheType>,
}

/// 缓存删除请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCacheRequest {
    pub key: String,
    pub cache_type: Option<CacheType>,
}

/// 缓存响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheResponse {
    pub success: bool,
    pub value: Option<serde_json::Value>,
    pub message: Option<String>,
}

/// 内存缓存服务
pub struct MemoryCacheService {
    cache: moka::sync::Cache<String, serde_json::Value>,
}

impl MemoryCacheService {
    pub fn new(max_size: u64) -> Self {
        Self {
            cache: moka::sync::Cache::new(max_size),
        }
    }

    pub fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) {
        if let Some(ttl) = ttl {
            self.cache.insert_with_expiry(key, value, std::time::Duration::from_secs(ttl));
        } else {
            self.cache.insert(key, value);
        }
    }

    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.cache.get(key).cloned()
    }

    pub fn delete(&self, key: &str) {
        self.cache.remove(key);
    }

    pub fn clear(&self) {
        self.cache.invalidate_all();
    }

    pub fn size(&self) -> usize {
        self.cache.entry_count()
    }
}

/// Redis缓存服务
#[cfg(feature = "redis")]
pub struct RedisCacheService {
    client: redis::Client,
}

#[cfg(feature = "redis")]
impl RedisCacheService {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let value_str = serde_json::to_string(&value)?;
        
        if let Some(ttl) = ttl {
            redis::cmd("SET").arg(&key).arg(&value_str).arg("EX").arg(ttl).query_async(&mut conn).await?;
        } else {
            redis::cmd("SET").arg(&key).arg(&value_str).query_async(&mut conn).await?;
        }
        
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<serde_json::Value>, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let value_str: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        
        match value_str {
            Some(s) => {
                let value = serde_json::from_str(&s)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn clear(&self) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        redis::cmd("FLUSHDB").query_async(&mut conn).await?;
        Ok(())
    }
}

/// 缓存服务
pub struct CacheService {
    memory_cache: Arc<MemoryCacheService>,
    #[cfg(feature = "redis")]
    redis_cache: Option<Arc<RedisCacheService>>,
    default_cache_type: CacheType,
}

impl CacheService {
    pub fn new(max_size: u64, default_cache_type: CacheType) -> Self {
        Self {
            memory_cache: Arc::new(MemoryCacheService::new(max_size)),
            #[cfg(feature = "redis")]
            redis_cache: None,
            default_cache_type,
        }
    }

    #[cfg(feature = "redis")]
    pub fn with_redis(&mut self, redis_url: &str) -> Result<(), redis::RedisError> {
        let redis_cache = RedisCacheService::new(redis_url)?;
        self.redis_cache = Some(Arc::new(redis_cache));
        Ok(())
    }

    pub async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>, cache_type: Option<CacheType>) -> Result<(), String> {
        let cache_type = cache_type.unwrap_or(self.default_cache_type.clone());
        
        match cache_type {
            CacheType::Memory => {
                self.memory_cache.set(key, value, ttl);
                Ok(())
            }
            CacheType::Redis => {
                #[cfg(feature = "redis")]
                {
                    if let Some(redis_cache) = &self.redis_cache {
                        redis_cache.set(key, value, ttl).await.map_err(|e| e.to_string())
                    } else {
                        Err("Redis cache not initialized".to_string())
                    }
                }
                #[cfg(not(feature = "redis"))]
                Err("Redis cache not enabled".to_string())
            }
        }
    }

    pub async fn get(&self, key: &str, cache_type: Option<CacheType>) -> Result<Option<serde_json::Value>, String> {
        let cache_type = cache_type.unwrap_or(self.default_cache_type.clone());
        
        match cache_type {
            CacheType::Memory => {
                Ok(self.memory_cache.get(key))
            }
            CacheType::Redis => {
                #[cfg(feature = "redis")]
                {
                    if let Some(redis_cache) = &self.redis_cache {
                        redis_cache.get(key).await.map_err(|e| e.to_string())
                    } else {
                        Err("Redis cache not initialized".to_string())
                    }
                }
                #[cfg(not(feature = "redis"))]
                Err("Redis cache not enabled".to_string())
            }
        }
    }

    pub async fn delete(&self, key: &str, cache_type: Option<CacheType>) -> Result<(), String> {
        let cache_type = cache_type.unwrap_or(self.default_cache_type.clone());
        
        match cache_type {
            CacheType::Memory => {
                self.memory_cache.delete(key);
                Ok(())
            }
            CacheType::Redis => {
                #[cfg(feature = "redis")]
                {
                    if let Some(redis_cache) = &self.redis_cache {
                        redis_cache.delete(key).await.map_err(|e| e.to_string())
                    } else {
                        Err("Redis cache not initialized".to_string())
                    }
                }
                #[cfg(not(feature = "redis"))]
                Err("Redis cache not enabled".to_string())
            }
        }
    }

    pub async fn clear(&self, cache_type: Option<CacheType>) -> Result<(), String> {
        let cache_type = cache_type.unwrap_or(self.default_cache_type.clone());
        
        match cache_type {
            CacheType::Memory => {
                self.memory_cache.clear();
                Ok(())
            }
            CacheType::Redis => {
                #[cfg(feature = "redis")]
                {
                    if let Some(redis_cache) = &self.redis_cache {
                        redis_cache.clear().await.map_err(|e| e.to_string())
                    } else {
                        Err("Redis cache not initialized".to_string())
                    }
                }
                #[cfg(not(feature = "redis"))]
                Err("Redis cache not enabled".to_string())
            }
        }
    }

    pub fn get_memory_cache_size(&self) -> usize {
        self.memory_cache.size()
    }
}

/// 插件生命周期实现
pub struct CachePlugin {
    cache_service: Option<CacheService>,
    initialized: bool,
    started: bool,
}

impl CachePlugin {
    pub fn new() -> Self {
        Self {
            cache_service: None,
            initialized: false,
            started: false,
        }
    }
}

/// 插件生命周期接口实现
impl crate::plugin::runtime::PluginLifecycle for CachePlugin {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            return Ok(());
        }
        
        // 初始化缓存服务
        self.cache_service = Some(CacheService::new(10000, CacheType::Memory));
        self.initialized = true;
        
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("插件未初始化".into());
        }
        
        if self.started {
            return Ok(());
        }
        
        self.started = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            return Ok(());
        }
        
        self.started = false;
        Ok(())
    }
}

/// 设置缓存处理器
pub async fn set_cache_handler(req: axum::extract::Json<SetCacheRequest>) -> axum::response::Json<CacheResponse> {
    // 实际应用中应该从应用状态中获取CacheService
    let mut cache_service = CacheService::new(10000, CacheType::Memory);
    
    match cache_service.set(req.0.key, req.0.value, req.0.ttl, req.0.cache_type).await {
        Ok(_) => {
            axum::response::Json(CacheResponse {
                success: true,
                value: None,
                message: Some("缓存设置成功".to_string()),
            })
        }
        Err(err) => {
            axum::response::Json(CacheResponse {
                success: false,
                value: None,
                message: Some(err),
            })
        }
    }
}

/// 获取缓存处理器
pub async fn get_cache_handler(req: axum::extract::Json<GetCacheRequest>) -> axum::response::Json<CacheResponse> {
    // 实际应用中应该从应用状态中获取CacheService
    let cache_service = CacheService::new(10000, CacheType::Memory);
    
    match cache_service.get(&req.0.key, req.0.cache_type).await {
        Ok(value) => {
            axum::response::Json(CacheResponse {
                success: true,
                value,
                message: None,
            })
        }
        Err(err) => {
            axum::response::Json(CacheResponse {
                success: false,
                value: None,
                message: Some(err),
            })
        }
    }
}

/// 删除缓存处理器
pub async fn delete_cache_handler(req: axum::extract::Json<DeleteCacheRequest>) -> axum::response::Json<CacheResponse> {
    // 实际应用中应该从应用状态中获取CacheService
    let cache_service = CacheService::new(10000, CacheType::Memory);
    
    match cache_service.delete(&req.0.key, req.0.cache_type).await {
        Ok(_) => {
            axum::response::Json(CacheResponse {
                success: true,
                value: None,
                message: Some("缓存删除成功".to_string()),
            })
        }
        Err(err) => {
            axum::response::Json(CacheResponse {
                success: false,
                value: None,
                message: Some(err),
            })
        }
    }
}

/// 清空缓存处理器
pub async fn clear_cache_handler() -> axum::response::Json<CacheResponse> {
    // 实际应用中应该从应用状态中获取CacheService
    let cache_service = CacheService::new(10000, CacheType::Memory);
    
    match cache_service.clear(None).await {
        Ok(_) => {
            axum::response::Json(CacheResponse {
                success: true,
                value: None,
                message: Some("缓存清空成功".to_string()),
            })
        }
        Err(err) => {
            axum::response::Json(CacheResponse {
                success: false,
                value: None,
                message: Some(err),
            })
        }
    }
}
