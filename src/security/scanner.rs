// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Security scanner module
//! Provides security vulnerability scanning capabilities for applications

use crate::security::intrusion_detection::IntrusionDetectionEngine;
use crate::security::models::{IntrusionDetectionConfig, SecurityScanConfig};
use crate::security::scanner_core::SecurityScannerCore;

/// Security scanner builder
#[derive(Default)]
pub struct SecurityScannerBuilder {
    scan_config: SecurityScanConfig,
    intrusion_config: IntrusionDetectionConfig,
}

impl SecurityScannerBuilder {
    /// Create new security scanner builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set scan configuration
    pub fn with_scan_config(mut self, config: SecurityScanConfig) -> Self {
        self.scan_config = config;
        self
    }

    /// Set intrusion detection configuration
    pub fn with_intrusion_config(mut self, config: IntrusionDetectionConfig) -> Self {
        self.intrusion_config = config;
        self
    }

    /// Build security scanner
    pub fn build(self) -> SecurityScanner {
        SecurityScanner::new(self.scan_config, self.intrusion_config)
    }
}

/// Security scanner
#[derive(Clone)]
pub struct SecurityScanner {
    /// Security scanner core
    scanner_core: SecurityScannerCore,
    /// Intrusion detection engine
    intrusion_engine: IntrusionDetectionEngine,
}

impl SecurityScanner {
    /// Create new security scanner
    pub fn new(scan_config: SecurityScanConfig, intrusion_config: IntrusionDetectionConfig) -> Self {
        Self {
            scanner_core: SecurityScannerCore::new(scan_config),
            intrusion_engine: IntrusionDetectionEngine::new(intrusion_config),
        }
    }

    /// Create new security scanner with default configuration (简洁API)
    pub fn builder() -> SecurityScannerBuilder {
        SecurityScannerBuilder::new()
    }

    /// Set intrusion detection configuration
    pub fn set_intrusion_config(&mut self, config: IntrusionDetectionConfig) {
        self.intrusion_engine.set_config(config);
    }

    /// Get intrusion detection configuration
    pub fn get_intrusion_config(&self) -> &IntrusionDetectionConfig {
        self.intrusion_engine.get_config()
    }

    /// Detect intrusion
    pub fn detect_intrusion(&mut self, source_ip: &str, target: &str, event_type: crate::security::models::DetectionRule, details: std::collections::HashMap<String, String>) {
        self.intrusion_engine.detect_intrusion(source_ip, target, event_type, details);
    }

    /// Get intrusion events
    pub fn get_intrusion_events(&self) -> &Vec<crate::security::models::IntrusionEvent> {
        self.intrusion_engine.get_intrusion_events()
    }

    /// Get intrusion events by severity
    pub fn get_intrusion_events_by_severity(&self, severity: crate::security::models::VulnerabilitySeverity) -> Vec<&crate::security::models::IntrusionEvent> {
        self.intrusion_engine.get_intrusion_events_by_severity(severity)
    }

    /// Get intrusion statistics
    pub fn get_intrusion_stats(&self) -> &crate::security::models::IntrusionStatistics {
        self.intrusion_engine.get_intrusion_stats()
    }

    /// Clear intrusion events
    pub fn clear_intrusion_events(&mut self) {
        self.intrusion_engine.clear_intrusion_events();
    }

    /// Execute security scan asynchronously (异步扫描支持)
    pub async fn scan_async(&mut self, target: &str) -> crate::security::models::ScanStatistics {
        self.scanner_core.scan_async(target).await
    }

    /// Get vulnerabilities by severity level
    pub fn get_vulnerabilities_by_severity(
        &self,
        severity: crate::security::models::VulnerabilitySeverity,
    ) -> Vec<&crate::security::models::SecurityVulnerability> {
        self.scanner_core.get_vulnerabilities_by_severity(severity)
    }

    /// Get critical vulnerabilities only
    pub fn get_critical_vulnerabilities(&self) -> Vec<&crate::security::models::SecurityVulnerability> {
        self.scanner_core.get_critical_vulnerabilities()
    }

    /// Get high severity vulnerabilities only
    pub fn get_high_vulnerabilities(&self) -> Vec<&crate::security::models::SecurityVulnerability> {
        self.scanner_core.get_high_vulnerabilities()
    }

    /// Export scan results as JSON
    pub fn export_results_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.scanner_core.export_results_json()
    }

    /// Clear previous scan results
    pub fn clear_results(&mut self) {
        self.scanner_core.clear_results();
        self.clear_intrusion_events();
    }

    /// Get scan results
    pub fn get_scan_results(&self) -> &Vec<crate::security::models::SecurityVulnerability> {
        self.scanner_core.get_scan_results()
    }

    /// Get scan statistics
    pub fn get_scan_stats(&self) -> &crate::security::models::ScanStatistics {
        self.scanner_core.get_scan_stats()
    }

    /// Execute security scan
    pub fn scan(&mut self, target: &str) -> crate::security::models::ScanStatistics {
        self.scanner_core.scan(target)
    }
}
