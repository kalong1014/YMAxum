// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Merchant status
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MerchantStatus {
    /// Pending
    Pending,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Frozen
    Frozen,
    /// Cancelled
    Cancelled,
}

/// Merchant permissions
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MerchantPermission {
    /// Product management
    ProductManage,
    /// Order management
    OrderManage,
    /// Finance management
    FinanceManage,
    /// Member management
    MemberManage,
    /// Marketing management
    MarketingManage,
}

/// Merchant information
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Merchant {
    /// Merchant ID
    pub id: String,
    /// Merchant name
    pub name: String,
    /// Merchant type
    pub merchant_type: String,
    /// Contact person name
    pub contact_name: String,
    /// Contact phone
    pub contact_phone: String,
    /// Contact email
    pub contact_email: String,
    /// Business license
    pub business_license: String,
    /// Tax registration number
    pub tax_registration: String,
    /// Merchant status
    pub status: MerchantStatus,
    /// Permission list
    pub permissions: Vec<MerchantPermission>,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
    /// Approved at
    pub approved_at: Option<u64>,
    /// Approved by
    pub approved_by: Option<String>,
    /// Approval remark
    pub approval_remark: Option<String>,
}

/// Merchant application
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MerchantApplication {
    /// Application ID
    pub application_id: String,
    /// Merchant name
    pub merchant_name: String,
    /// Merchant type
    pub merchant_type: String,
    /// Contact person name
    pub contact_name: String,
    /// Contact phone
    pub contact_phone: String,
    /// Contact email
    pub contact_email: String,
    /// Business license
    pub business_license: String,
    /// Tax registration number
    pub tax_registration: String,
    /// Applied at
    pub applied_at: u64,
    /// Approval status
    pub status: MerchantStatus,
    /// Approved at
    pub approved_at: Option<u64>,
    /// Approved by
    pub approved_by: Option<String>,
    /// Approval remark
    pub approval_remark: Option<String>,
}

/// Merchant manager
pub struct MerchantManager {
    /// Merchant map
    merchants: Arc<RwLock<HashMap<String, Merchant>>>,
    /// Application map
    applications: Arc<RwLock<HashMap<String, MerchantApplication>>>,
    /// Maximum number of merchants
    max_merchants: u32,
}

impl MerchantManager {
    /// Create new merchant manager
    pub fn new(max_merchants: u32) -> Self {
        Self {
            merchants: Arc::new(RwLock::new(HashMap::new())),
            applications: Arc::new(RwLock::new(HashMap::new())),
            max_merchants,
        }
    }

    /// Submit merchant application
    pub async fn submit_application(&self, application: MerchantApplication) -> Result<(), String> {
        let mut apps = self.applications.write().await;
        if apps.contains_key(&application.application_id) {
            return Err("Application already exists".to_string());
        }

        let app_id = application.application_id.clone();
        apps.insert(app_id.clone(), application);
        info!("Merchant application submitted: {}", app_id);
        Ok(())
    }

    /// Approve merchant application
    pub async fn approve_application(
        &self,
        application_id: &str,
        status: MerchantStatus,
        approved_by: &str,
        remark: Option<&str>,
    ) -> Result<(), String> {
        let mut apps = self.applications.write().await;
        let mut merchants = self.merchants.write().await;

        let application = apps
            .get_mut(application_id)
            .ok_or("Application does not exist".to_string())?;

        application.status = status.clone();
        application.approved_at = Some(chrono::Utc::now().timestamp() as u64);
        application.approved_by = Some(approved_by.to_string());
        application.approval_remark = remark.map(|r| r.to_string());

        if status == MerchantStatus::Approved {
            // Check if merchant count exceeds limit
            if merchants.len() >= self.max_merchants as usize {
                return Err("Merchant count has reached limit".to_string());
            }

            // Create merchant
            let merchant = Merchant {
                id: application.application_id.clone(),
                name: application.merchant_name.clone(),
                merchant_type: application.merchant_type.clone(),
                contact_name: application.contact_name.clone(),
                contact_phone: application.contact_phone.clone(),
                contact_email: application.contact_email.clone(),
                business_license: application.business_license.clone(),
                tax_registration: application.tax_registration.clone(),
                status: MerchantStatus::Approved,
                permissions: vec![
                    MerchantPermission::ProductManage,
                    MerchantPermission::OrderManage,
                    MerchantPermission::FinanceManage,
                ],
                created_at: application.applied_at,
                updated_at: chrono::Utc::now().timestamp() as u64,
                approved_at: application.approved_at,
                approved_by: application.approved_by.clone(),
                approval_remark: application.approval_remark.clone(),
            };

            merchants.insert(merchant.id.clone(), merchant);
            info!(
                "Merchant application approved and merchant created: {}",
                application.application_id
            );
        }

        Ok(())
    }

    /// Get merchant information
    pub async fn get_merchant(&self, merchant_id: &str) -> Option<Merchant> {
        let merchants = self.merchants.read().await;
        merchants.get(merchant_id).cloned()
    }

    /// Update merchant information
    pub async fn update_merchant(&self, merchant: Merchant) -> Result<(), String> {
        let mut merchants = self.merchants.write().await;
        let merchant_id = merchant.id.clone();
        if !merchants.contains_key(&merchant_id) {
            return Err("Merchant does not exist".to_string());
        }

        merchants.insert(merchant_id.clone(), merchant);
        info!("Merchant information updated: {}", merchant_id);
        Ok(())
    }

    /// Update merchant status
    pub async fn update_merchant_status(
        &self,
        merchant_id: &str,
        status: MerchantStatus,
    ) -> Result<(), String> {
        let mut merchants = self.merchants.write().await;
        let merchant = merchants
            .get_mut(merchant_id)
            .ok_or("Merchant does not exist".to_string())?;

        let status_clone = status.clone();
        merchant.status = status;
        merchant.updated_at = chrono::Utc::now().timestamp() as u64;

        info!(
            "Merchant status updated: {} -> {:?}",
            merchant_id, status_clone
        );
        Ok(())
    }

    /// Grant merchant permission
    pub async fn grant_permission(
        &self,
        merchant_id: &str,
        permission: MerchantPermission,
    ) -> Result<(), String> {
        let mut merchants = self.merchants.write().await;
        let merchant = merchants
            .get_mut(merchant_id)
            .ok_or("Merchant does not exist".to_string())?;

        if !merchant.permissions.contains(&permission) {
            merchant.permissions.push(permission.clone());
            merchant.updated_at = chrono::Utc::now().timestamp() as u64;
            info!(
                "Merchant permission granted: {} -> {:?}",
                merchant_id, permission
            );
        }

        Ok(())
    }

    /// Revoke merchant permission
    pub async fn revoke_permission(
        &self,
        merchant_id: &str,
        permission: MerchantPermission,
    ) -> Result<(), String> {
        let mut merchants = self.merchants.write().await;
        let merchant = merchants
            .get_mut(merchant_id)
            .ok_or("Merchant does not exist".to_string())?;

        if let Some(index) = merchant.permissions.iter().position(|p| p == &permission) {
            merchant.permissions.remove(index);
            merchant.updated_at = chrono::Utc::now().timestamp() as u64;
            info!(
                "Merchant permission revoked: {} -> {:?}",
                merchant_id, permission
            );
        }

        Ok(())
    }

    /// Get merchant list
    pub async fn list_merchants(&self, status: Option<MerchantStatus>) -> Vec<Merchant> {
        let merchants = self.merchants.read().await;
        let mut result = Vec::new();

        for merchant in merchants.values() {
            if let Some(ref merchant_status) = status {
                if merchant.status == *merchant_status {
                    result.push(merchant.clone());
                }
            } else {
                result.push(merchant.clone());
            }
        }

        result
    }

    /// Get application list
    pub async fn list_applications(
        &self,
        status: Option<MerchantStatus>,
    ) -> Vec<MerchantApplication> {
        let apps = self.applications.read().await;
        let mut result = Vec::new();

        for app in apps.values() {
            if let Some(ref app_status) = status {
                if app.status == *app_status {
                    result.push(app.clone());
                }
            } else {
                result.push(app.clone());
            }
        }

        result
    }
}

