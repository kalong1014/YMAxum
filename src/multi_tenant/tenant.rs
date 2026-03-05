//! 租户管理模块
//! 
//! 提供租户的创建、管理、删除等功能

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono;

/// 租户状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    /// 活跃
    Active,
    /// 已暂停
    Suspended,
    /// 已删除
    Deleted,
    /// 待激活
    Pending,
}

/// 租户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    /// 租户ID
    pub id: String,
    /// 租户名称
    pub name: String,
    /// 租户描述
    pub description: String,
    /// 租户状态
    pub status: TenantStatus,
    /// 创建时间
    pub created_at: String,
    /// 最后更新时间
    pub updated_at: String,
    /// 过期时间
    pub expires_at: Option<String>,
    /// 联系人
    pub contact_person: String,
    /// 联系邮箱
    pub contact_email: String,
    /// 联系电话
    pub contact_phone: String,
    /// 租户域名
    pub domain: Option<String>,
    /// 租户子域名
    pub subdomain: Option<String>,
    /// 数据库配置
    pub database_config: serde_json::Value,
    /// 缓存配置
    pub cache_config: serde_json::Value,
    /// 存储配置
    pub storage_config: serde_json::Value,
    /// 权限配置
    pub permission_config: serde_json::Value,
    /// 其他配置
    pub other_config: serde_json::Value,
}

/// 租户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// 租户ID
    pub id: String,
    /// 租户配置
    pub config: TenantConfig,
    /// 租户资源使用情况
    pub resource_usage: serde_json::Value,
    /// 租户账单信息
    pub billing_info: serde_json::Value,
    /// 租户统计信息
    pub statistics: serde_json::Value,
}

/// 租户管理器
#[derive(Debug, Clone)]
pub struct TenantManager {
    tenants: Arc<RwLock<Vec<Tenant>>>,
    tenant_map: Arc<RwLock<std::collections::HashMap<String, Tenant>>>,
}

impl TenantManager {
    /// 创建新的租户管理器
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(Vec::new())),
            tenant_map: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 初始化租户管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化租户管理器
        Ok(())
    }

    /// 创建租户
    pub async fn create_tenant(&self, config: TenantConfig) -> Result<Tenant, Box<dyn std::error::Error>> {
        let mut tenants = self.tenants.write().await;
        let mut tenant_map = self.tenant_map.write().await;

        // 检查租户ID是否已存在
        if tenant_map.contains_key(&config.id) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Tenant with ID {} already exists", config.id),
            )));
        }

        // 创建租户
        let tenant = Tenant {
            id: config.id.clone(),
            config,
            resource_usage: serde_json::json!({}),
            billing_info: serde_json::json!({}),
            statistics: serde_json::json!({}),
        };

        // 添加租户
        tenants.push(tenant.clone());
        tenant_map.insert(tenant.id.clone(), tenant.clone());

        Ok(tenant)
    }

    /// 获取租户
    pub async fn get_tenant(&self, tenant_id: &str) -> Option<Tenant> {
        let tenant_map = self.tenant_map.read().await;
        tenant_map.get(tenant_id).cloned()
    }

    /// 更新租户
    pub async fn update_tenant(&self, tenant: Tenant) -> Result<Tenant, Box<dyn std::error::Error>> {
        let mut tenants = self.tenants.write().await;
        let mut tenant_map = self.tenant_map.write().await;

        // 检查租户是否存在
        if !tenant_map.contains_key(&tenant.id) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Tenant with ID {} not found", tenant.id),
            )));
        }

        // 更新租户
        if let Some(index) = tenants.iter().position(|t| t.id == tenant.id) {
            tenants[index] = tenant.clone();
        }
        tenant_map.insert(tenant.id.clone(), tenant.clone());

        Ok(tenant)
    }

    /// 删除租户
    pub async fn delete_tenant(&self, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut tenants = self.tenants.write().await;
        let mut tenant_map = self.tenant_map.write().await;

        // 检查租户是否存在
        if !tenant_map.contains_key(tenant_id) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Tenant with ID {} not found", tenant_id),
            )));
        }

        // 删除租户
        tenants.retain(|t| t.id != tenant_id);
        tenant_map.remove(tenant_id);

        Ok(())
    }

    /// 暂停租户
    pub async fn suspend_tenant(&self, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tenant = self.get_tenant(tenant_id).await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Tenant with ID {} not found", tenant_id)))?;

        let mut updated_tenant = tenant;
        updated_tenant.config.status = TenantStatus::Suspended;
        updated_tenant.config.updated_at = chrono::Utc::now().to_string();

        self.update_tenant(updated_tenant).await?;
        Ok(())
    }

    /// 激活租户
    pub async fn activate_tenant(&self, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tenant = self.get_tenant(tenant_id).await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Tenant with ID {} not found", tenant_id)))?;

        let mut updated_tenant = tenant;
        updated_tenant.config.status = TenantStatus::Active;
        updated_tenant.config.updated_at = chrono::Utc::now().to_string();

        self.update_tenant(updated_tenant).await?;
        Ok(())
    }

    /// 列出所有租户
    pub async fn list_tenants(&self) -> Vec<Tenant> {
        self.tenants.read().await.clone()
    }

    /// 列出活跃租户
    pub async fn list_active_tenants(&self) -> Vec<Tenant> {
        let tenants = self.tenants.read().await;
        tenants.iter().filter(|t| t.config.status == TenantStatus::Active).cloned().collect()
    }

    /// 检查租户是否存在
    pub async fn tenant_exists(&self, tenant_id: &str) -> bool {
        let tenant_map = self.tenant_map.read().await;
        tenant_map.contains_key(tenant_id)
    }

    /// 获取租户资源使用情况
    pub async fn get_tenant_resource_usage(&self, tenant_id: &str) -> Option<serde_json::Value> {
        let tenant = self.get_tenant(tenant_id).await;
        tenant.map(|t| t.resource_usage)
    }

    /// 更新租户资源使用情况
    pub async fn update_tenant_resource_usage(&self, tenant_id: &str, usage: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let tenant = self.get_tenant(tenant_id).await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Tenant with ID {} not found", tenant_id)))?;

        let mut updated_tenant = tenant;
        updated_tenant.resource_usage = usage;

        self.update_tenant(updated_tenant).await?;
        Ok(())
    }
}
