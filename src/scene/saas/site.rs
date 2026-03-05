// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Site status
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SiteStatus {
    /// Pending
    Pending,
    /// Active
    Active,
    /// Frozen
    Frozen,
    /// Cancelled
    Cancelled,
}

/// Site information
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Site {
    /// Site ID
    pub id: String,
    /// Site name
    pub name: String,
    /// Main domain name
    pub main_domain: String,
    /// Backup domain list
    pub backup_domains: Vec<String>,
    /// Site status
    pub status: SiteStatus,
    /// Merchant ID
    pub merchant_id: Option<String>,
    /// Template ID
    pub template_id: String,
    /// Database connection information
    pub db_connection: String,
    /// Configuration information JSON
    pub config_json: String,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
    /// Activated at
    pub activated_at: Option<u64>,
    /// Frozen at
    pub frozen_at: Option<u64>,
    /// Cancelled at
    pub cancelled_at: Option<u64>,
}

/// Site manager
pub struct SiteManager {
    /// Site map
    sites: Arc<RwLock<HashMap<String, Site>>>,
    /// Domain to site ID mapping
    domain_to_site: Arc<RwLock<HashMap<String, String>>>,
    /// Maximum number of sites
    max_sites: u32,
}

impl SiteManager {
    /// Create new site manager
    pub fn new(max_sites: u32) -> Self {
        Self {
            sites: Arc::new(RwLock::new(HashMap::new())),
            domain_to_site: Arc::new(RwLock::new(HashMap::new())),
            max_sites,
        }
    }

    /// Create site
    pub async fn create_site(&self, site: Site) -> Result<Site, String> {
        let mut sites = self.sites.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if site count exceeds limit
        if sites.len() >= self.max_sites as usize {
            return Err(format!("Site count has reached limit {}", self.max_sites));
        }

        // Check if site ID already exists
        if sites.contains_key(&site.id) {
            return Err("Site ID already exists".to_string());
        }

        // Check if main domain is already occupied
        if domain_to_site.contains_key(&site.main_domain) {
            return Err(format!("Main domain {} already occupied", site.main_domain));
        }

        // Check if backup domains are already occupied
        for domain in &site.backup_domains {
            if domain_to_site.contains_key(domain) {
                return Err(format!("Backup domain {} already occupied", domain));
            }
        }

        // Generate new site
        let mut new_site = site.clone();
        new_site.created_at = chrono::Utc::now().timestamp() as u64;
        new_site.updated_at = new_site.created_at;

        // Save site information
        sites.insert(new_site.id.clone(), new_site.clone());

        // Register domain mapping
        domain_to_site.insert(new_site.main_domain.clone(), new_site.id.clone());
        for domain in &new_site.backup_domains {
            domain_to_site.insert(domain.clone(), new_site.id.clone());
        }

        info!(
            "Site created: {}, domain: {}",
            new_site.id, new_site.main_domain
        );
        Ok(new_site)
    }

    /// Get site information
    pub async fn get_site(&self, site_id: &str) -> Option<Site> {
        let sites = self.sites.read().await;
        sites.get(site_id).cloned()
    }

    /// Get site information by domain
    pub async fn get_site_by_domain(&self, domain: &str) -> Option<Site> {
        let domain_to_site = self.domain_to_site.read().await;
        if let Some(site_id) = domain_to_site.get(domain) {
            self.get_site(site_id).await
        } else {
            None
        }
    }

    /// Update site information
    pub async fn update_site(&self, site: Site) -> Result<Site, String> {
        let mut sites = self.sites.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if site exists
        if !sites.contains_key(&site.id) {
            return Err("Site does not exist".to_string());
        }

        let old_site = sites.get(&site.id).unwrap().clone();

        // Check if main domain is modified
        if old_site.main_domain != site.main_domain {
            // Check if new main domain is already occupied
            if domain_to_site.contains_key(&site.main_domain) {
                return Err(format!("Main domain {} already occupied", site.main_domain));
            }

            // Remove old main domain mapping
            domain_to_site.remove(&old_site.main_domain);
            // Add new main domain mapping
            domain_to_site.insert(site.main_domain.clone(), site.id.clone());
        }

        // Check if backup domains are modified
        let old_domains: HashSet<String> = old_site.backup_domains.iter().cloned().collect();
        let new_domains: HashSet<String> = site.backup_domains.iter().cloned().collect();

        // Remove unused domain mappings
        for domain in old_domains.difference(&new_domains) {
            domain_to_site.remove(domain);
        }

        // Add new backup domain mappings
        for domain in new_domains.difference(&old_domains) {
            if domain_to_site.contains_key(domain) {
                return Err(format!("Backup domain {} already occupied", domain));
            }
            domain_to_site.insert(domain.clone(), site.id.clone());
        }

        // Update site information
        let mut updated_site = site.clone();
        updated_site.updated_at = chrono::Utc::now().timestamp() as u64;

        sites.insert(updated_site.id.clone(), updated_site.clone());
        info!("Site updated: {}", updated_site.id);
        Ok(updated_site)
    }

    /// Update site status
    pub async fn update_site_status(
        &self,
        site_id: &str,
        status: SiteStatus,
    ) -> Result<Site, String> {
        let mut sites = self.sites.write().await;
        let site = sites
            .get_mut(site_id)
            .ok_or("Site does not exist".to_string())?;

        let old_status = site.status.clone();
        site.status = status.clone();
        site.updated_at = chrono::Utc::now().timestamp() as u64;

        // Update activation time, freeze time or cancellation time
        match status {
            SiteStatus::Active => {
                site.activated_at = Some(site.updated_at);
                site.frozen_at = None;
                site.cancelled_at = None;
            }
            SiteStatus::Frozen => {
                site.frozen_at = Some(site.updated_at);
            }
            SiteStatus::Cancelled => {
                site.cancelled_at = Some(site.updated_at);
            }
            _ => {}
        }

        info!(
            "Site status updated: {} -> {:?} -> {:?}",
            site_id, old_status, status
        );
        Ok(site.clone())
    }

    /// Delete site
    pub async fn delete_site(&self, site_id: &str) -> Result<(), String> {
        let mut sites = self.sites.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if site exists
        let site = sites
            .remove(site_id)
            .ok_or("Site does not exist".to_string())?;

        // Remove domain mapping
        domain_to_site.remove(&site.main_domain);
        for domain in &site.backup_domains {
            domain_to_site.remove(domain);
        }

        info!("Site deleted: {}, domain: {}", site_id, site.main_domain);
        Ok(())
    }

    /// Get site list
    pub async fn list_sites(&self, status: Option<SiteStatus>) -> Vec<Site> {
        let sites = self.sites.read().await;
        let mut result = Vec::new();

        for site in sites.values() {
            if let Some(ref site_status) = status {
                if site.status == *site_status {
                    result.push(site.clone());
                }
            } else {
                result.push(site.clone());
            }
        }

        result
    }

    /// Get merchant's site list
    pub async fn list_merchant_sites(&self, merchant_id: &str) -> Vec<Site> {
        let sites = self.sites.read().await;
        let mut result = Vec::new();

        for site in sites.values() {
            if let Some(site_merchant_id) = &site.merchant_id
                && site_merchant_id == merchant_id
            {
                result.push(site.clone());
            }
        }

        result
    }

    /// Check if domain is available
    pub async fn is_domain_available(&self, domain: &str) -> bool {
        let domain_to_site = self.domain_to_site.read().await;
        !domain_to_site.contains_key(domain)
    }
}

