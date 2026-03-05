// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Domain type
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DomainType {
    /// Main domain
    Main,
    /// Backup domain
    Backup,
    /// Subdomain
    Subdomain,
    /// Wildcard domain
    Wildcard,
}

/// Domain configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Domain ID
    pub id: String,
    /// Domain
    pub domain: String,
    /// Domain type
    pub domain_type: DomainType,
    /// Associated site ID
    pub site_id: String,
    /// Is enabled
    pub enabled: bool,
    /// Remark
    pub remark: Option<String>,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
}

/// Domain route mapping
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DomainRoute {
    /// Route ID
    pub id: String,
    /// Domain
    pub domain: String,
    /// Path prefix
    pub path_prefix: String,
    /// Target service
    pub target_service: String,
    /// Target path
    pub target_path: String,
    /// Is enabled
    pub enabled: bool,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
}

/// Domain manager
pub struct DomainManager {
    /// Domain configuration map
    domains: Arc<RwLock<HashMap<String, DomainConfig>>>,
    /// Domain to site ID mapping
    domain_to_site: Arc<RwLock<HashMap<String, String>>>,
    /// Domain route map
    routes: Arc<RwLock<HashMap<String, DomainRoute>>>,
    /// Domain + path prefix to route mapping
    domain_path_to_route: Arc<RwLock<HashMap<(String, String), String>>>,
}

impl DomainManager {
    /// Create new domain manager
    pub fn new() -> Self {
        Self {
            domains: Arc::new(RwLock::new(HashMap::new())),
            domain_to_site: Arc::new(RwLock::new(HashMap::new())),
            routes: Arc::new(RwLock::new(HashMap::new())),
            domain_path_to_route: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add domain configuration
    pub async fn add_domain(&self, domain_config: DomainConfig) -> Result<DomainConfig, String> {
        let mut domains = self.domains.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if domain ID already exists
        if domains.contains_key(&domain_config.id) {
            return Err("Domain configuration ID already exists".to_string());
        }

        // Check if domain is already used by other sites
        if domain_to_site.contains_key(&domain_config.domain) {
            return Err(format!(
                "Domain {} already used by other sites",
                domain_config.domain
            ));
        }

        // Generate new domain configuration
        let mut new_domain = domain_config.clone();
        new_domain.created_at = chrono::Utc::now().timestamp() as u64;
        new_domain.updated_at = new_domain.created_at;

        // Save domain configuration
        domains.insert(new_domain.id.clone(), new_domain.clone());

        // Create domain to site mapping
        domain_to_site.insert(new_domain.domain.clone(), new_domain.site_id.clone());

        info!(
            "Domain configuration added: {} -> {}",
            new_domain.domain, new_domain.site_id
        );
        Ok(new_domain)
    }

    /// Get domain configuration
    pub async fn get_domain(&self, domain_id: &str) -> Option<DomainConfig> {
        let domains = self.domains.read().await;
        domains.get(domain_id).cloned()
    }

    /// Get domain configuration by domain name
    pub async fn get_domain_by_name(&self, domain: &str) -> Option<DomainConfig> {
        let domains = self.domains.read().await;
        domains.values().find(|d| d.domain == domain).cloned()
    }

    /// Update domain configuration
    pub async fn update_domain(&self, domain_config: DomainConfig) -> Result<DomainConfig, String> {
        let mut domains = self.domains.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if domain configuration exists
        let old_domain = domains
            .get(&domain_config.id)
            .ok_or("Domain configuration does not exist".to_string())?;

        let mut updated_domain = domain_config.clone();
        updated_domain.updated_at = chrono::Utc::now().timestamp() as u64;

        // If domain name changes, need to update mapping
        if old_domain.domain != updated_domain.domain {
            // Check if new domain is already used by other sites
            if let Some(existing_site_id) = domain_to_site.get(&updated_domain.domain)
                && existing_site_id != &updated_domain.site_id
            {
                return Err(format!(
                    "Domain {} already used by other sites",
                    updated_domain.domain
                ));
            }

            // Update mapping
            domain_to_site.remove(&old_domain.domain);
            domain_to_site.insert(
                updated_domain.domain.clone(),
                updated_domain.site_id.clone(),
            );
        }

        // Save updated domain configuration
        domains.insert(updated_domain.id.clone(), updated_domain.clone());

        info!("Domain configuration updated: {}", updated_domain.domain);
        Ok(updated_domain)
    }

    /// Delete domain configuration
    pub async fn delete_domain(&self, domain_id: &str) -> Result<(), String> {
        let mut domains = self.domains.write().await;
        let mut domain_to_site = self.domain_to_site.write().await;

        // Check if domain configuration exists
        let domain = domains
            .remove(domain_id)
            .ok_or("Domain configuration does not exist".to_string())?;

        // Remove mapping
        domain_to_site.remove(&domain.domain);

        info!("Domain configuration deleted: {}", domain.domain);
        Ok(())
    }

    /// Enable/disable domain
    pub async fn toggle_domain(
        &self,
        domain_id: &str,
        enabled: bool,
    ) -> Result<DomainConfig, String> {
        let mut domains = self.domains.write().await;
        let domain = domains
            .get_mut(domain_id)
            .ok_or("Domain configuration does not exist".to_string())?;

        domain.enabled = enabled;
        domain.updated_at = chrono::Utc::now().timestamp() as u64;

        info!(
            "Domain {} {}",
            domain.domain,
            if enabled { "enabled" } else { "disabled" }
        );
        Ok(domain.clone())
    }

    /// Add domain route
    pub async fn add_route(&self, route: DomainRoute) -> Result<DomainRoute, String> {
        let mut routes = self.routes.write().await;
        let mut domain_path_to_route = self.domain_path_to_route.write().await;

        // Check if route ID already exists
        if routes.contains_key(&route.id) {
            return Err("Route ID already exists".to_string());
        }

        // Check if domain + path prefix is already occupied
        let key = (route.domain.clone(), route.path_prefix.clone());
        if domain_path_to_route.contains_key(&key) {
            return Err(format!(
                "Domain {} route prefix {} already occupied",
                route.domain, route.path_prefix
            ));
        }

        // Generate new route
        let mut new_route = route.clone();
        new_route.created_at = chrono::Utc::now().timestamp() as u64;
        new_route.updated_at = new_route.created_at;

        // Save route
        routes.insert(new_route.id.clone(), new_route.clone());
        domain_path_to_route.insert(key, new_route.id.clone());

        info!(
            "Domain route added: {} -> {}{}",
            new_route.domain, new_route.path_prefix, new_route.target_path
        );
        Ok(new_route)
    }

    /// Get route configuration
    pub async fn get_route(&self, route_id: &str) -> Option<DomainRoute> {
        let routes = self.routes.read().await;
        routes.get(route_id).cloned()
    }

    /// Find route by domain and path
    pub async fn find_route(&self, domain: &str, path: &str) -> Option<DomainRoute> {
        let routes = self.routes.read().await;
        let domain_path_to_route = self.domain_path_to_route.read().await;

        // Exact match
        let exact_key = (domain.to_string(), path.to_string());
        if let Some(route_id) = domain_path_to_route.get(&exact_key) {
            return routes.get(route_id).cloned();
        }

        // Prefix match (from long to short)
        let mut path_parts: Vec<String> = path
            .split('/')
            .filter(|&p| !p.is_empty())
            .map(|p| p.to_string())
            .collect();

        while !path_parts.is_empty() {
            let prefix = format!("/{}", path_parts.join("/"));
            let key = (domain.to_string(), prefix.clone());
            if let Some(route_id) = domain_path_to_route.get(&key) {
                return routes.get(route_id).cloned();
            }
            path_parts.pop();
        }

        // Default route (root path)
        let default_key = (domain.to_string(), "/".to_string());
        if let Some(route_id) = domain_path_to_route.get(&default_key) {
            return routes.get(route_id).cloned();
        }

        None
    }

    /// Update route configuration
    pub async fn update_route(&self, route: DomainRoute) -> Result<DomainRoute, String> {
        let mut routes = self.routes.write().await;
        let mut domain_path_to_route = self.domain_path_to_route.write().await;

        // Check if route exists
        let old_route = routes
            .get(&route.id)
            .ok_or("Route does not exist".to_string())?;

        let old_key = (old_route.domain.clone(), old_route.path_prefix.clone());
        let new_key = (route.domain.clone(), route.path_prefix.clone());

        // If domain or path prefix changes, need to update mapping
        if old_key != new_key {
            // Check if new domain + path prefix is already occupied
            if let Some(existing_route_id) = domain_path_to_route.get(&new_key)
                && existing_route_id != &route.id
            {
                return Err(format!(
                    "Domain {} route prefix {} already occupied",
                    route.domain, route.path_prefix
                ));
            }

            // Update mapping
            domain_path_to_route.remove(&old_key);
            domain_path_to_route.insert(new_key, route.id.clone());
        }

        // Generate updated route
        let mut updated_route = route.clone();
        updated_route.updated_at = chrono::Utc::now().timestamp() as u64;

        // Save updated route
        routes.insert(updated_route.id.clone(), updated_route.clone());

        info!("Domain route updated: {}", updated_route.id);
        Ok(updated_route)
    }

    /// Delete route
    pub async fn delete_route(&self, route_id: &str) -> Result<(), String> {
        let mut routes = self.routes.write().await;
        let mut domain_path_to_route = self.domain_path_to_route.write().await;

        // Check if route exists
        let route = routes
            .remove(route_id)
            .ok_or("Route does not exist".to_string())?;

        // Remove mapping
        let key = (route.domain.clone(), route.path_prefix.clone());
        domain_path_to_route.remove(&key);

        info!("Domain route deleted: {}", route.id);
        Ok(())
    }

    /// Enable/disable route
    pub async fn toggle_route(&self, route_id: &str, enabled: bool) -> Result<DomainRoute, String> {
        let mut routes = self.routes.write().await;
        let route = routes
            .get_mut(route_id)
            .ok_or("Route does not exist".to_string())?;

        route.enabled = enabled;
        route.updated_at = chrono::Utc::now().timestamp() as u64;

        info!(
            "Domain route {} {}",
            route.id,
            if enabled { "enabled" } else { "disabled" }
        );
        Ok(route.clone())
    }

    /// Get all domain configurations
    pub async fn list_domains(&self, site_id: Option<&str>) -> Vec<DomainConfig> {
        let domains = self.domains.read().await;
        let mut result = Vec::new();

        for domain in domains.values() {
            if let Some(site_id_filter) = site_id {
                if domain.site_id == site_id_filter {
                    result.push(domain.clone());
                }
            } else {
                result.push(domain.clone());
            }
        }

        result
    }

    /// Get all route configurations
    pub async fn list_routes(&self, domain: Option<&str>) -> Vec<DomainRoute> {
        let routes = self.routes.read().await;
        let mut result = Vec::new();

        for route in routes.values() {
            if let Some(domain_filter) = domain {
                if route.domain == domain_filter {
                    result.push(route.clone());
                }
            } else {
                result.push(route.clone());
            }
        }

        result
    }
}

impl Default for DomainManager {
    fn default() -> Self {
        Self::new()
    }
}

