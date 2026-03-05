// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Security audit module
//! Provides security audit functionality for applications

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

use crate::security::{IntrusionDetectionConfig, SecurityScanConfig, SecurityScanner, VulnerabilitySeverity};
use crate::security::models::{DetectionRule, IntrusionEvent, SecurityVulnerability};

/// Security audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditConfig {
    /// Enable security audit
    pub enabled: bool,
    /// Audit log retention period (days)
    pub log_retention_days: u32,
    /// Enable real-time alerting
    pub enable_real_time_alerting: bool,
    /// Alert threshold
    pub alert_threshold: u32,
    /// Audit scope
    pub audit_scope: AuditScope,
}

impl Default for SecurityAuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_retention_days: 30,
            enable_real_time_alerting: true,
            alert_threshold: 5,
            audit_scope: AuditScope::Full,
        }
    }
}

/// Audit scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditScope {
    /// Full audit
    Full,
    /// Scan only
    ScanOnly,
    /// Intrusion detection only
    IntrusionOnly,
    /// Custom scope
    Custom(Vec<AuditItem>),
}

/// Audit item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditItem {
    /// Vulnerability scan
    VulnerabilityScan,
    /// Intrusion detection
    IntrusionDetection,
    /// Access control
    AccessControl,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// Data protection
    DataProtection,
    /// Network security
    NetworkSecurity,
    /// System configuration
    SystemConfiguration,
}

/// Security audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditLog {
    /// Log ID
    pub id: String,
    /// Audit type
    pub audit_type: AuditType,
    /// Audit timestamp
    pub timestamp: u64,
    /// Audit source
    pub source: String,
    /// Audit target
    pub target: String,
    /// Audit result
    pub result: AuditResult,
    /// Detailed message
    pub message: String,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Audit type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(PartialEq)]
pub enum AuditType {
    /// Vulnerability scan
    VulnerabilityScan,
    /// Intrusion detection
    IntrusionDetection,
    /// Access control
    AccessControl,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// Data protection
    DataProtection,
    /// Network security
    NetworkSecurity,
    /// System configuration
    SystemConfiguration,
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditResult {
    /// Success
    Success,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Security audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditReport {
    /// Report ID
    pub id: String,
    /// Report generation timestamp
    pub generated_at: u64,
    /// Audit duration (seconds)
    pub duration: u64,
    /// Total audit items
    pub total_items: usize,
    /// Failed items
    pub failed_items: usize,
    /// Warning items
    pub warning_items: usize,
    /// Critical items
    pub critical_items: usize,
    /// Scan results
    pub scan_results: Vec<SecurityVulnerability>,
    /// Intrusion events
    pub intrusion_events: Vec<IntrusionEvent>,
    /// Audit logs
    pub audit_logs: Vec<SecurityAuditLog>,
    /// Summary
    pub summary: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Security audit engine
#[derive(Clone)]
pub struct SecurityAuditEngine {
    /// Configuration
    config: SecurityAuditConfig,
    /// Security scanner
    scanner: SecurityScanner,
    /// Audit logs
    audit_logs: Vec<SecurityAuditLog>,
    /// Last audit time
    last_audit_time: Option<Instant>,
}

impl SecurityAuditEngine {
    /// Create new security audit engine
    pub fn new(config: SecurityAuditConfig) -> Self {
        let scanner_config = SecurityScanConfig {
            scan_types: vec![
                crate::security::models::ScanType::SqlInjection,
                crate::security::models::ScanType::Xss,
                crate::security::models::ScanType::Csrf,
                crate::security::models::ScanType::Authentication,
                crate::security::models::ScanType::Authorization,
                crate::security::models::ScanType::InformationDisclosure,
                crate::security::models::ScanType::FileInclusion,
                crate::security::models::ScanType::CommandInjection,
                crate::security::models::ScanType::InsecureDeserialization,
                crate::security::models::ScanType::InsecureDirectObjectReference,
                crate::security::models::ScanType::SecurityMisconfiguration,
                crate::security::models::ScanType::UsingKnownVulnerableComponents,
                crate::security::models::ScanType::XmlExternalEntity,
                crate::security::models::ScanType::ServerSideRequestForgery,
                crate::security::models::ScanType::Clickjacking,
                crate::security::models::ScanType::InsecureHttpMethods,
                crate::security::models::ScanType::MissingSecurityHeaders,
                crate::security::models::ScanType::SessionManagement,
                crate::security::models::ScanType::RateLimiting,
                crate::security::models::ScanType::InputValidation,
                crate::security::models::ScanType::OutputEncoding,
                crate::security::models::ScanType::BufferOverflow,
                crate::security::models::ScanType::RaceCondition,
                crate::security::models::ScanType::PrivilegeEscalation,
                crate::security::models::ScanType::NetworkSecurity,
                crate::security::models::ScanType::CryptographicIssues,
                crate::security::models::ScanType::DoSVulnerabilities,
                crate::security::models::ScanType::ApiSecurity,
                crate::security::models::ScanType::ContainerSecurity,
                crate::security::models::ScanType::CloudSecurity,
                crate::security::models::ScanType::DependencyVulnerabilities,
                crate::security::models::ScanType::SecureCodingPractices,
                crate::security::models::ScanType::LoggingAndMonitoring,
                crate::security::models::ScanType::IncidentResponse,
            ],
            scan_scope: crate::security::models::ScanScope::Full,
            enable_deep_scan: true,
            scan_timeout: 600,
        };

        let intrusion_config = IntrusionDetectionConfig::default();
        let scanner = SecurityScanner::new(scanner_config, intrusion_config);

        Self {
            config,
            scanner,
            audit_logs: Vec::new(),
            last_audit_time: None,
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: SecurityAuditConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &SecurityAuditConfig {
        &self.config
    }

    /// Run security audit
    pub async fn run_audit(&mut self, target: &str) -> SecurityAuditReport {
        info!("Starting security audit for: {}", target);

        let start_time = Instant::now();

        // Clear previous audit logs
        self.audit_logs.clear();

        // Run vulnerability scan
        let _scan_stats = self.scanner.scan_async(target).await;

        // Get scan results
        let scan_results = self.scanner.get_scan_results().to_vec();

        // Get intrusion events
        let intrusion_events = self.scanner.get_intrusion_events().to_vec();

        // Generate audit logs
        self.generate_audit_logs(target, &scan_results, &intrusion_events);

        // Calculate audit duration
        let duration = start_time.elapsed().as_secs();

        // Generate report
        let report = self.generate_report(target, duration, &scan_results, &intrusion_events);

        // Update last audit time
        self.last_audit_time = Some(Instant::now());

        info!("Security audit completed for: {}", target);

        report
    }

    /// Generate audit logs
    fn generate_audit_logs(&mut self, target: &str, scan_results: &Vec<SecurityVulnerability>, intrusion_events: &Vec<IntrusionEvent>) {
        // Generate scan logs
        for vuln in scan_results {
            let log = SecurityAuditLog {
                id: format!("LOG-{}-{}", vuln.id, self.audit_logs.len() + 1),
                audit_type: AuditType::VulnerabilityScan,
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                source: "SecurityAuditEngine".to_string(),
                target: target.to_string(),
                result: match vuln.severity {
                    VulnerabilitySeverity::Critical => AuditResult::Critical,
                    VulnerabilitySeverity::High => AuditResult::Error,
                    VulnerabilitySeverity::Medium => AuditResult::Warning,
                    VulnerabilitySeverity::Low => AuditResult::Warning,
                    VulnerabilitySeverity::Info => AuditResult::Success,
                },
                message: format!("Vulnerability found: {} ({:?})", vuln.name, vuln.severity),
                details: HashMap::from([
                    ("vulnerability_id".to_string(), vuln.id.clone()),
                    ("vulnerability_type".to_string(), format!("{:?}", vuln.vulnerability_type)),
                    ("severity".to_string(), format!("{:?}", vuln.severity)),
                    ("location".to_string(), vuln.location.clone()),
                ]),
            };

            self.audit_logs.push(log);
        }

        // Generate intrusion logs
        for event in intrusion_events {
            let log = SecurityAuditLog {
                id: format!("LOG-EVENT-{}-{}", event.id, self.audit_logs.len() + 1),
                audit_type: AuditType::IntrusionDetection,
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                source: "SecurityAuditEngine".to_string(),
                target: event.target.clone(),
                result: match event.severity {
                    VulnerabilitySeverity::Critical => AuditResult::Critical,
                    VulnerabilitySeverity::High => AuditResult::Error,
                    VulnerabilitySeverity::Medium => AuditResult::Warning,
                    VulnerabilitySeverity::Low => AuditResult::Warning,
                    VulnerabilitySeverity::Info => AuditResult::Success,
                },
                message: format!("Intrusion detected: {} from IP {}", event.event_type.to_string(), event.source_ip),
                details: event.details.clone(),
            };

            self.audit_logs.push(log);
        }
    }

    /// Generate audit report
    fn generate_report(&self, target: &str, duration: u64, scan_results: &Vec<SecurityVulnerability>, intrusion_events: &Vec<IntrusionEvent>) -> SecurityAuditReport {
        // Calculate statistics
        let total_items = scan_results.len() + intrusion_events.len();
        let failed_items = scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Critical | VulnerabilitySeverity::High)).count();
        let warning_items = scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Medium)).count();
        let critical_items = scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Critical)).count();

        // Generate summary
        let summary = format!(
            "Security audit completed for {}. Found {} vulnerabilities ({} critical, {} high, {} medium, {} low, {} info) and {} intrusion events in {} seconds.",
            target,
            scan_results.len(),
            scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Critical)).count(),
            scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::High)).count(),
            scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Medium)).count(),
            scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Low)).count(),
            scan_results.iter().filter(|v| matches!(v.severity, VulnerabilitySeverity::Info)).count(),
            intrusion_events.len(),
            duration
        );

        // Generate recommendations
        let mut recommendations: Vec<String> = Vec::new();

        // Add recommendations based on scan results
        if !scan_results.is_empty() {
            recommendations.push("Fix identified vulnerabilities according to the remediation suggestions".to_string());
        }

        if !intrusion_events.is_empty() {
            recommendations.push("Investigate and address detected intrusion attempts".to_string());
        }

        if critical_items > 0 {
            recommendations.push("Prioritize fixing critical vulnerabilities immediately".to_string());
        }

        // Create report
        SecurityAuditReport {
            id: format!("REPORT-{}-{}", target.replace("/", "-"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            generated_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            duration,
            total_items,
            failed_items,
            warning_items,
            critical_items,
            scan_results: scan_results.to_vec(),
            intrusion_events: intrusion_events.to_vec(),
            audit_logs: self.audit_logs.clone(),
            summary,
            recommendations,
        }
    }

    /// Get audit logs
    pub fn get_audit_logs(&self) -> &Vec<SecurityAuditLog> {
        &self.audit_logs
    }

    /// Get audit logs by type
    pub fn get_audit_logs_by_type(&self, audit_type: AuditType) -> Vec<&SecurityAuditLog> {
        self.audit_logs
            .iter()
            .filter(|&log| log.audit_type == audit_type)
            .collect()
    }

    /// Get audit logs by result
    pub fn get_audit_logs_by_result(&self, result: AuditResult) -> Vec<&SecurityAuditLog> {
        self.audit_logs
            .iter()
            .filter(|&log| log.result == result)
            .collect()
    }

    /// Clear audit logs
    pub fn clear_audit_logs(&mut self) {
        self.audit_logs.clear();
    }

    /// Get last audit time
    pub fn get_last_audit_time(&self) -> Option<Instant> {
        self.last_audit_time
    }

    /// Export audit report as JSON
    pub fn export_report_json(&self, report: &SecurityAuditReport) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(report)?;
        Ok(json)
    }

    /// Export audit logs as JSON
    pub fn export_logs_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.audit_logs)?;
        Ok(json)
    }

    /// Detect intrusion
    pub fn detect_intrusion(&mut self, source_ip: &str, target: &str, event_type: DetectionRule, details: HashMap<String, String>) {
        self.scanner.detect_intrusion(source_ip, target, event_type, details);
    }

    /// Get intrusion events
    pub fn get_intrusion_events(&self) -> &Vec<IntrusionEvent> {
        self.scanner.get_intrusion_events()
    }

    /// Get scan results
    pub fn get_scan_results(&self) -> &Vec<SecurityVulnerability> {
        self.scanner.get_scan_results()
    }

    /// Clear results
    pub fn clear_results(&mut self) {
        self.scanner.clear_results();
        self.clear_audit_logs();
    }

    /// Generate compliance report
    pub fn generate_compliance_report(&self, standard: &ComplianceStandard) -> ComplianceReport {
        // Generate compliance report based on the specified standard
        let mut report = ComplianceReport {
            id: format!("COMPLIANCE-REPORT-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            generated_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            standard: format!("{:?}", standard),
            compliance_score: 0.0,
            passed_requirements: Vec::new(),
            failed_requirements: Vec::new(),
            recommendations: Vec::new(),
        };

        // Calculate compliance score based on scan results
        let scan_results = self.scanner.get_scan_results();
        let total_requirements = self.get_standard_requirements(standard).len();
        let passed_requirements = self.get_passed_requirements(standard, &scan_results);

        report.passed_requirements = passed_requirements;
        report.failed_requirements = self.get_failed_requirements(standard, &scan_results);
        report.compliance_score = (report.passed_requirements.len() as f64 / total_requirements as f64) * 100.0;

        // Generate recommendations
        report.recommendations = self.get_compliance_recommendations(standard, &scan_results);

        report
    }

    /// Get standard requirements
    fn get_standard_requirements(&self, standard: &ComplianceStandard) -> Vec<String> {
        match standard {
            ComplianceStandard::PciDss => vec![
                "Maintain a firewall configuration".to_string(),
                "Do not use vendor-supplied defaults for system passwords and other security parameters".to_string(),
                "Protect stored cardholder data".to_string(),
                "Encrypt transmission of cardholder data across open, public networks".to_string(),
                "Use and regularly update anti-virus software or programs".to_string(),
                "Develop and maintain secure systems and applications".to_string(),
                "Restrict access to cardholder data by business need-to-know".to_string(),
                "Assign a unique ID to each person with computer access".to_string(),
                "Restrict physical access to cardholder data".to_string(),
                "Track and monitor all access to network resources and cardholder data".to_string(),
                "Regularly test security systems and processes".to_string(),
                "Maintain a policy that addresses information security for all personnel".to_string(),
            ],
            ComplianceStandard::GDPR => vec![
                "Process personal data lawfully, fairly and in a transparent manner".to_string(),
                "Collect personal data for specified, explicit and legitimate purposes".to_string(),
                "Ensure personal data is adequate, relevant and limited to what is necessary".to_string(),
                "Ensure personal data is accurate and, where necessary, kept up to date".to_string(),
                "Keep personal data in a form which permits identification of data subjects for no longer than is necessary".to_string(),
                "Process personal data in a manner that ensures appropriate security of the personal data".to_string(),
            ],
            ComplianceStandard::Iso27001 => vec![
                "Information security policy".to_string(),
                "Information security risk assessment".to_string(),
                "Information security risk treatment".to_string(),
                "Information security objectives".to_string(),
                "Information security awareness, education and training".to_string(),
                "Asset management".to_string(),
                "Access control".to_string(),
                "Cryptography".to_string(),
                "Physical and environmental security".to_string(),
                "Operations security".to_string(),
                "Communications security".to_string(),
                "System acquisition, development and maintenance".to_string(),
                "Supplier relationships".to_string(),
                "Information security incident management".to_string(),
                "Business continuity management".to_string(),
            ],
        }
    }

    /// Get passed requirements
    fn get_passed_requirements(&self, standard: &ComplianceStandard, scan_results: &Vec<SecurityVulnerability>) -> Vec<String> {
        let all_requirements = self.get_standard_requirements(standard);
        let failed_requirements = self.get_failed_requirements(standard, scan_results);

        all_requirements
            .into_iter()
            .filter(|req| !failed_requirements.contains(req))
            .collect()
    }

    /// Get failed requirements
    fn get_failed_requirements(&self, standard: &ComplianceStandard, scan_results: &Vec<SecurityVulnerability>) -> Vec<String> {
        let mut failed_requirements: Vec<String> = Vec::new();

        match standard {
            ComplianceStandard::PciDss => {
                // Check for vulnerabilities related to PCI DSS requirements
                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::SqlInjection | crate::security::models::ScanType::Xss | crate::security::models::ScanType::CommandInjection)) {
                    failed_requirements.push("Develop and maintain secure systems and applications".to_string());
                }

                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::Authentication | crate::security::models::ScanType::Authorization)) {
                    failed_requirements.push("Restrict access to cardholder data by business need-to-know".to_string());
                    failed_requirements.push("Assign a unique ID to each person with computer access".to_string());
                }

                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::NetworkSecurity | crate::security::models::ScanType::CryptographicIssues)) {
                    failed_requirements.push("Encrypt transmission of cardholder data across open, public networks".to_string());
                }
            },
            ComplianceStandard::GDPR => {
                // Check for vulnerabilities related to GDPR requirements
                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::InformationDisclosure)) {
                    failed_requirements.push("Process personal data in a manner that ensures appropriate security of the personal data".to_string());
                }
            },
            ComplianceStandard::Iso27001 => {
                // Check for vulnerabilities related to ISO 27001 requirements
                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::Authentication | crate::security::models::ScanType::Authorization)) {
                    failed_requirements.push("Access control".to_string());
                }

                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::ServerSideRequestForgery)) {
                    failed_requirements.push("Cryptography".to_string());
                }

                if scan_results.iter().any(|v| matches!(v.vulnerability_type, crate::security::models::ScanType::ServerSideRequestForgery)) {
                    failed_requirements.push("Communications security".to_string());
                }
            },
        }

        failed_requirements
    }

    /// Get compliance recommendations
    fn get_compliance_recommendations(&self, standard: &ComplianceStandard, _scan_results: &Vec<SecurityVulnerability>) -> Vec<String> {
        let mut recommendations: Vec<String> = Vec::new();

        match standard {
            ComplianceStandard::PciDss => {
                recommendations.push("Implement secure coding practices to prevent SQL injection, XSS, and command injection vulnerabilities".to_string());
                recommendations.push("Implement strong authentication and authorization controls".to_string());
                recommendations.push("Encrypt all cardholder data in transit".to_string());
                recommendations.push("Regularly scan for vulnerabilities and apply security patches".to_string());
            },
            ComplianceStandard::GDPR => {
                recommendations.push("Implement data protection measures to prevent information disclosure".to_string());
                recommendations.push("Ensure personal data is processed lawfully, fairly and transparently".to_string());
                recommendations.push("Implement appropriate security measures to protect personal data".to_string());
            },
            ComplianceStandard::Iso27001 => {
                recommendations.push("Implement strong access control measures".to_string());
                recommendations.push("Use strong cryptography to protect sensitive data".to_string());
                recommendations.push("Implement secure network and communications security controls".to_string());
                recommendations.push("Regularly assess and treat information security risks".to_string());
            },
        }

        recommendations
    }
}

/// Compliance standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStandard {
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// General Data Protection Regulation
    GDPR,
    /// ISO/IEC 27001
    Iso27001,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub id: String,
    /// Report generation timestamp
    pub generated_at: u64,
    /// Compliance standard
    pub standard: String,
    /// Compliance score (0-100)
    pub compliance_score: f64,
    /// Passed requirements
    pub passed_requirements: Vec<String>,
    /// Failed requirements
    pub failed_requirements: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Get severity level from string
pub fn get_severity_from_string(severity: &str) -> VulnerabilitySeverity {
    match severity.to_lowercase().as_str() {
        "critical" => VulnerabilitySeverity::Critical,
        "high" => VulnerabilitySeverity::High,
        "medium" => VulnerabilitySeverity::Medium,
        "low" => VulnerabilitySeverity::Low,
        _ => VulnerabilitySeverity::Info,
    }
}

/// Get string from severity level
pub fn get_string_from_severity(severity: VulnerabilitySeverity) -> String {
    match severity {
        VulnerabilitySeverity::Critical => "严重".to_string(),
        VulnerabilitySeverity::High => "高危".to_string(),
        VulnerabilitySeverity::Medium => "中危".to_string(),
        VulnerabilitySeverity::Low => "低危".to_string(),
        VulnerabilitySeverity::Info => "信息".to_string(),
    }
}
