// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Security scanner command module
//! Provides security scanning commands for the command line interface

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::command::executor::ExecuteResult;
use crate::security::{IntrusionDetectionConfig, SecurityScanConfig, SecurityScanner};
use crate::security::models::{ScanType, ScanScope};

/// Security scanner command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScannerCommand {
    /// Command name
    pub name: String,
    /// Target to scan
    pub target: String,
    /// Scan types
    pub scan_types: Option<Vec<String>>,
    /// Enable deep scan
    pub deep_scan: Option<bool>,
    /// Scan timeout
    pub timeout: Option<u64>,
    /// Enable intrusion detection
    pub enable_intrusion_detection: Option<bool>,
}

impl SecurityScannerCommand {
    pub fn execute(&self) -> ExecuteResult {
        info!("Executing security scanner command: {}", self.name);

        // Create scan configuration
        let mut scan_types = Vec::new();
        if let Some(types) = &self.scan_types {
            for type_str in types {
                match type_str.to_lowercase().as_str() {
                    "sql" => scan_types.push(ScanType::SqlInjection),
                    "xss" => scan_types.push(ScanType::Xss),
                    "csrf" => scan_types.push(ScanType::Csrf),
                    "auth" => scan_types.push(ScanType::Authentication),
                    "authz" => scan_types.push(ScanType::Authorization),
                    "info" => scan_types.push(ScanType::InformationDisclosure),
                    "file" => scan_types.push(ScanType::FileInclusion),
                    "cmd" => scan_types.push(ScanType::CommandInjection),
                    "deser" => scan_types.push(ScanType::InsecureDeserialization),
                    "idor" => scan_types.push(ScanType::InsecureDirectObjectReference),
                    "config" => scan_types.push(ScanType::SecurityMisconfiguration),
                    "vuln" => scan_types.push(ScanType::UsingKnownVulnerableComponents),
                    "xxe" => scan_types.push(ScanType::XmlExternalEntity),
                    "ssrf" => scan_types.push(ScanType::ServerSideRequestForgery),
                    "click" => scan_types.push(ScanType::Clickjacking),
                    "http" => scan_types.push(ScanType::InsecureHttpMethods),
                    "headers" => scan_types.push(ScanType::MissingSecurityHeaders),
                    "session" => scan_types.push(ScanType::SessionManagement),
                    "rate" => scan_types.push(ScanType::RateLimiting),
                    "input" => scan_types.push(ScanType::InputValidation),
                    "output" => scan_types.push(ScanType::OutputEncoding),
                    "buffer" => scan_types.push(ScanType::BufferOverflow),
                    "race" => scan_types.push(ScanType::RaceCondition),
                    "priv" => scan_types.push(ScanType::PrivilegeEscalation),
                    "network" => scan_types.push(ScanType::NetworkSecurity),
                    "crypto" => scan_types.push(ScanType::CryptographicIssues),
                    "dos" => scan_types.push(ScanType::DoSVulnerabilities),
                    "api" => scan_types.push(ScanType::ApiSecurity),
                    "container" => scan_types.push(ScanType::ContainerSecurity),
                    "cloud" => scan_types.push(ScanType::CloudSecurity),
                    "dependency" => scan_types.push(ScanType::DependencyVulnerabilities),
                    "code" => scan_types.push(ScanType::SecureCodingPractices),
                    "log" => scan_types.push(ScanType::LoggingAndMonitoring),
                    "incident" => scan_types.push(ScanType::IncidentResponse),
                    _ => info!("Unknown scan type: {}", type_str),
                }
            }
        } else {
            // Default scan types
            scan_types = vec![
                ScanType::SqlInjection,
                ScanType::Xss,
                ScanType::Csrf,
                ScanType::Authentication,
                ScanType::Authorization,
                ScanType::InformationDisclosure,
                ScanType::FileInclusion,
                ScanType::CommandInjection,
                ScanType::InsecureDeserialization,
                ScanType::InsecureDirectObjectReference,
                ScanType::SecurityMisconfiguration,
                ScanType::UsingKnownVulnerableComponents,
                ScanType::XmlExternalEntity,
                ScanType::ServerSideRequestForgery,
                ScanType::Clickjacking,
                ScanType::InsecureHttpMethods,
                ScanType::MissingSecurityHeaders,
                ScanType::SessionManagement,
                ScanType::RateLimiting,
                ScanType::InputValidation,
                ScanType::OutputEncoding,
                ScanType::BufferOverflow,
                ScanType::RaceCondition,
                ScanType::PrivilegeEscalation,
                ScanType::NetworkSecurity,
                ScanType::CryptographicIssues,
                ScanType::DoSVulnerabilities,
                ScanType::ApiSecurity,
                ScanType::ContainerSecurity,
                ScanType::CloudSecurity,
                ScanType::DependencyVulnerabilities,
                ScanType::SecureCodingPractices,
                ScanType::LoggingAndMonitoring,
                ScanType::IncidentResponse,
            ];
        }

        let config = SecurityScanConfig {
            scan_types,
            scan_scope: ScanScope::Full,
            enable_deep_scan: self.deep_scan.unwrap_or(false),
            scan_timeout: self.timeout.unwrap_or(300),
        };

        let intrusion_config = IntrusionDetectionConfig {
            enabled: self.enable_intrusion_detection.unwrap_or(true),
            detection_rules: vec![
                crate::security::models::DetectionRule::BruteForce,
                crate::security::models::DetectionRule::DoSAttempt,
                crate::security::models::DetectionRule::SuspiciousIP,
                crate::security::models::DetectionRule::AnomalyDetection,
                crate::security::models::DetectionRule::UnauthorizedAccess,
                crate::security::models::DetectionRule::SqlInjectionAttempt,
                crate::security::models::DetectionRule::XssAttempt,
                crate::security::models::DetectionRule::CommandInjectionAttempt,
                crate::security::models::DetectionRule::CsrfAttempt,
                crate::security::models::DetectionRule::InformationDisclosureAttempt,
                crate::security::models::DetectionRule::FileInclusionAttempt,
                crate::security::models::DetectionRule::InsecureDeserializationAttempt,
                crate::security::models::DetectionRule::InsecureDirectObjectReferenceAttempt,
                crate::security::models::DetectionRule::SecurityMisconfigurationAttempt,
                crate::security::models::DetectionRule::XmlExternalEntityAttempt,
                crate::security::models::DetectionRule::ServerSideRequestForgeryAttempt,
                crate::security::models::DetectionRule::ClickjackingAttempt,
                crate::security::models::DetectionRule::InsecureHttpMethodsAttempt,
                crate::security::models::DetectionRule::MissingSecurityHeadersAttempt,
                crate::security::models::DetectionRule::SessionManagementAttempt,
                crate::security::models::DetectionRule::RateLimitingAttempt,
                crate::security::models::DetectionRule::InputValidationAttempt,
                crate::security::models::DetectionRule::OutputEncodingAttempt,
                crate::security::models::DetectionRule::BufferOverflowAttempt,
                crate::security::models::DetectionRule::RaceConditionAttempt,
                crate::security::models::DetectionRule::PrivilegeEscalationAttempt,
                crate::security::models::DetectionRule::NetworkSecurityAttempt,
                crate::security::models::DetectionRule::CryptographicIssuesAttempt,
                crate::security::models::DetectionRule::DoSVulnerabilitiesAttempt,
                crate::security::models::DetectionRule::ApiSecurityAttempt,
            ],
            alert_threshold: 5,
            detection_window: 3600,
        };

        let mut scanner = SecurityScanner::new(config, intrusion_config);

        // Run scan
        let scan_stats = scanner.scan(&self.target);

        // Get scan results
        let scan_results = scanner.get_scan_results();

        // Get intrusion events
        let intrusion_events = scanner.get_intrusion_events();

        // Prepare result
        let data = json!({
            "target": self.target,
            "scan_stats": scan_stats,
            "vulnerabilities": scan_results,
            "intrusion_events": intrusion_events
        });

        ExecuteResult::Success {
            message: format!("Security scan completed for {}", self.target),
            data,
        }
    }
}

/// Parse security scanner command
pub fn parse_security_scanner_command(args: &HashMap<String, String>) -> Result<SecurityScannerCommand, Box<dyn std::error::Error>> {
    let name = args.get("name").unwrap_or(&"scan".to_string()).clone();
    let target = args.get("target").ok_or("Target is required")?.clone();
    
    let scan_types = args.get("types").map(|types| {
        types.split(',').map(|t| t.trim().to_string()).collect()
    });
    
    let deep_scan = args.get("deep").map(|s| s.parse().unwrap_or(false));
    let timeout = args.get("timeout").map(|s| s.parse().unwrap_or(300));
    let enable_intrusion_detection = args.get("intrusion").map(|s| s.parse().unwrap_or(true));

    Ok(SecurityScannerCommand {
        name,
        target,
        scan_types,
        deep_scan,
        timeout,
        enable_intrusion_detection,
    })
}
