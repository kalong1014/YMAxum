// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::config::ConfigManager;
use super::domain::DomainManager;
use super::site::SiteManager;
use crate::scene::SceneAdapter;
use log::{error, info};
use std::sync::Arc;

/// SaaS scene adapter
pub struct SaasScene {
    /// Site manager
    site_manager: Option<Arc<SiteManager>>,
    /// Domain manager
    domain_manager: Option<Arc<DomainManager>>,
    /// Config manager
    config_manager: Option<Arc<ConfigManager>>,
    /// Scene name
    scene_name: &'static str,
    /// Is initialized
    initialized: bool,
    /// Is started
    started: bool,
    /// Maximum number of sites
    max_sites: u32,
}

impl SaasScene {
    /// Create new SaaS scene adapter
    pub fn new(max_sites: u32) -> Self {
        Self {
            site_manager: None,
            domain_manager: None,
            config_manager: None,
            scene_name: "saas",
            initialized: false,
            started: false,
            max_sites,
        }
    }
}

impl SceneAdapter for SaasScene {
    /// Get scene name
    fn name(&self) -> &'static str {
        self.scene_name
    }

    /// Initialize scene
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            info!("SaaS scene already initialized");
            return Ok(());
        }

        // Initialize site manager
        let site_manager = Arc::new(SiteManager::new(self.max_sites));
        self.site_manager = Some(site_manager.clone());
        info!(
            "SaaS site manager initialized, maximum sites: {}",
            self.max_sites
        );

        // Initialize domain manager
        let domain_manager = Arc::new(DomainManager::new());
        self.domain_manager = Some(domain_manager.clone());
        info!("SaaS domain manager initialized");

        // Initialize config manager
        let config_manager = Arc::new(ConfigManager::new());
        self.config_manager = Some(config_manager.clone());
        info!("SaaS config manager initialized");

        self.initialized = true;
        info!("SaaS scene initialization completed");
        Ok(())
    }

    /// Start scene
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("SaaS scene not initialized, please call init() first".into());
        }

        if self.started {
            info!("SaaS scene already started");
            return Ok(());
        }

        // Start domain route refresh task
        let _domain_manager_clone = self.domain_manager.clone().unwrap();
        tokio::spawn(async move {
            loop {
                // Refresh domain route cache every 5 minutes
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                info!("Refresh SaaS domain route cache");
                // Here you can add route cache refresh logic
            }
        });

        // Start config verification task
        let config_manager_clone = self.config_manager.clone().unwrap();
        let site_manager_clone = self.site_manager.clone().unwrap();
        tokio::spawn(async move {
            loop {
                // Verify all site configurations every hour
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                info!("Verify all site configurations");

                // Get all sites
                let sites = site_manager_clone.list_sites(None).await;
                for site in sites {
                    if let Err(e) = config_manager_clone.validate_config(&site.id).await {
                        error!("Site {} config verification failed: {}", site.id, e);
                    }
                }
            }
        });

        self.started = true;
        info!("SaaS scene startup completed");
        Ok(())
    }

    /// Stop scene
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            info!("SaaS scene already stopped");
            return Ok(());
        }

        // Release resources
        self.site_manager = None;
        self.domain_manager = None;
        self.config_manager = None;

        self.started = false;
        info!("SaaS scene stop completed");
        Ok(())
    }
}

