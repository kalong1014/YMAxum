// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Security models module
//! Provides data models for security scanning and intrusion detection

use serde::{Deserialize, Serialize};

/// Security scan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanConfig {
    /// Scan types
    pub scan_types: Vec<ScanType>,
    /// Scan scope
    pub scan_scope: ScanScope,
    /// Enable deep scan
    pub enable_deep_scan: bool,
    /// Scan timeout (seconds)
    pub scan_timeout: u64,
}

impl Default for SecurityScanConfig {
    fn default() -> Self {
        Self {
            scan_types: vec![
                ScanType::SqlInjection,
                ScanType::Xss,
                ScanType::Csrf,
                ScanType::Authentication,
                ScanType::Authorization,
            ],
            scan_scope: ScanScope::Full,
            enable_deep_scan: false,
            scan_timeout: 300,
        }
    }
}

/// Scan type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScanType {
    /// SQL injection
    SqlInjection,
    /// Cross-site scripting
    Xss,
    /// Cross-site request forgery
    Csrf,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// Information disclosure
    InformationDisclosure,
    /// File inclusion
    FileInclusion,
    /// Command injection
    CommandInjection,
    /// Insecure deserialization
    InsecureDeserialization,
    /// Insecure direct object reference
    InsecureDirectObjectReference,
    /// Security misconfiguration
    SecurityMisconfiguration,
    /// Using known vulnerable components
    UsingKnownVulnerableComponents,
    /// XML external entity (XXE)
    XmlExternalEntity,
    /// Server-side request forgery (SSRF)
    ServerSideRequestForgery,
    /// Clickjacking
    Clickjacking,
    /// Insecure HTTP methods
    InsecureHttpMethods,
    /// Missing security headers
    MissingSecurityHeaders,
    /// Session management
    SessionManagement,
    /// Rate limiting
    RateLimiting,
    /// Input validation
    InputValidation,
    /// Output encoding
    OutputEncoding,
    /// Buffer overflow
    BufferOverflow,
    /// Race condition
    RaceCondition,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Network security
    NetworkSecurity,
    /// Cryptographic issues
    CryptographicIssues,
    /// DoS vulnerabilities
    DoSVulnerabilities,
    /// API security
    ApiSecurity,
    /// Container security
    ContainerSecurity,
    /// Cloud security
    CloudSecurity,
    /// Dependency vulnerabilities
    DependencyVulnerabilities,
    /// Secure coding practices
    SecureCodingPractices,
    /// Logging and monitoring
    LoggingAndMonitoring,
    /// Incident response
    IncidentResponse,
    /// Zero-day vulnerability
    ZeroDayVulnerability,
    /// Supply chain attack
    SupplyChainAttack,
    /// AI-driven attack
    AiDrivenAttack,
    /// Blockchain security
    BlockchainSecurity,
    /// Quantum computing threats
    QuantumComputingThreats,
    /// Edge computing security
    EdgeComputingSecurity,
    /// IoT security
    IoTSecurity,
    /// Cloud native security
    CloudNativeSecurity,
    /// DevSecOps integration
    DevSecOpsIntegration,
    /// AI-generated malicious code
    AiGeneratedMaliciousCode,
    /// Container escape attack
    ContainerEscape,
    /// Cloud service misconfiguration
    CloudServiceMisconfiguration,
    /// API abuse
    ApiAbuse,
    /// Serverless security
    ServerlessSecurity,
    /// Edge computing vulnerability
    EdgeComputingVulnerability,
    /// IoT device compromise
    IoTDeviceCompromise,
}

/// Scan scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanScope {
    /// Full scan
    Full,
    /// API only
    ApiOnly,
    /// Web pages only
    WebOnly,
    /// Database only
    DatabaseOnly,
}

/// Security vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityVulnerability {
    /// Vulnerability ID
    pub id: String,
    /// Vulnerability name
    pub name: String,
    /// Vulnerability type
    pub vulnerability_type: ScanType,
    /// Severity level
    pub severity: VulnerabilitySeverity,
    /// Vulnerability description
    pub description: String,
    /// Affected scope
    pub affected_components: Vec<String>,
    /// Vulnerability location
    pub location: String,
    /// Remediation suggestions
    pub remediation: Vec<String>,
    /// CVSS score
    pub cvss_score: Option<f64>,
    /// CWE ID
    pub cwe_id: Option<String>,
}

/// Vulnerability severity level
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
    /// Info
    Info,
}

/// Intrusion detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionConfig {
    /// Enable intrusion detection
    pub enabled: bool,
    /// Detection rules
    pub detection_rules: Vec<DetectionRule>,
    /// Alert threshold
    pub alert_threshold: u32,
    /// Detection window (seconds)
    pub detection_window: u64,
}

impl Default for IntrusionDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_rules: vec![
                DetectionRule::BruteForce,
                DetectionRule::DoSAttempt,
                DetectionRule::SuspiciousIP,
                DetectionRule::AnomalyDetection,
                DetectionRule::UnauthorizedAccess,
            ],
            alert_threshold: 5,
            detection_window: 3600,
        }
    }
}

/// Detection rule type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetectionRule {
    /// Brute force attack detection
    BruteForce,
    /// DoS attempt detection
    DoSAttempt,
    /// Suspicious IP detection
    SuspiciousIP,
    /// Anomaly detection
    AnomalyDetection,
    /// Unauthorized access detection
    UnauthorizedAccess,
    /// SQL injection attempt
    SqlInjectionAttempt,
    /// XSS attempt
    XssAttempt,
    /// Command injection attempt
    CommandInjectionAttempt,
    /// CSRF attempt
    CsrfAttempt,
    /// Information disclosure attempt
    InformationDisclosureAttempt,
    /// File inclusion attempt
    FileInclusionAttempt,
    /// Insecure deserialization attempt
    InsecureDeserializationAttempt,
    /// Insecure direct object reference attempt
    InsecureDirectObjectReferenceAttempt,
    /// Security misconfiguration attempt
    SecurityMisconfigurationAttempt,
    /// XML external entity attempt
    XmlExternalEntityAttempt,
    /// Server-side request forgery attempt
    ServerSideRequestForgeryAttempt,
    /// Clickjacking attempt
    ClickjackingAttempt,
    /// Insecure HTTP methods attempt
    InsecureHttpMethodsAttempt,
    /// Missing security headers attempt
    MissingSecurityHeadersAttempt,
    /// Session management attempt
    SessionManagementAttempt,
    /// Rate limiting attempt
    RateLimitingAttempt,
    /// Input validation attempt
    InputValidationAttempt,
    /// Output encoding attempt
    OutputEncodingAttempt,
    /// Buffer overflow attempt
    BufferOverflowAttempt,
    /// Race condition attempt
    RaceConditionAttempt,
    /// Privilege escalation attempt
    PrivilegeEscalationAttempt,
    /// Network security attempt
    NetworkSecurityAttempt,
    /// Cryptographic issues attempt
    CryptographicIssuesAttempt,
    /// DoS vulnerabilities attempt
    DoSVulnerabilitiesAttempt,
    /// API security attempt
    ApiSecurityAttempt,
    /// Zero-day vulnerability attempt
    ZeroDayVulnerabilityAttempt,
    /// Supply chain attack attempt
    SupplyChainAttackAttempt,
    /// AI-driven attack attempt
    AiDrivenAttackAttempt,
    /// Blockchain security attempt
    BlockchainSecurityAttempt,
    /// Quantum computing threats attempt
    QuantumComputingThreatsAttempt,
    /// Edge computing security attempt
    EdgeComputingSecurityAttempt,
    /// IoT security attempt
    IoTSecurityAttempt,
    /// Cloud native security attempt
    CloudNativeSecurityAttempt,
    /// DevSecOps integration attempt
    DevSecOpsIntegrationAttempt,
    /// AI-generated malicious code attempt
    AiGeneratedMaliciousCodeAttempt,
    /// Container escape attempt
    ContainerEscapeAttempt,
    /// Cloud service misconfiguration attempt
    CloudServiceMisconfigurationAttempt,
    /// API abuse attempt
    ApiAbuseAttempt,
    /// Serverless security attempt
    ServerlessSecurityAttempt,
    /// Edge computing vulnerability attempt
    EdgeComputingVulnerabilityAttempt,
    /// IoT device compromise attempt
    IoTDeviceCompromiseAttempt,
}

impl std::fmt::Display for DetectionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectionRule::BruteForce => write!(f, "BruteForce"),
            DetectionRule::DoSAttempt => write!(f, "DoSAttempt"),
            DetectionRule::SuspiciousIP => write!(f, "SuspiciousIP"),
            DetectionRule::AnomalyDetection => write!(f, "AnomalyDetection"),
            DetectionRule::UnauthorizedAccess => write!(f, "UnauthorizedAccess"),
            DetectionRule::SqlInjectionAttempt => write!(f, "SqlInjectionAttempt"),
            DetectionRule::XssAttempt => write!(f, "XssAttempt"),
            DetectionRule::CommandInjectionAttempt => write!(f, "CommandInjectionAttempt"),
            DetectionRule::CsrfAttempt => write!(f, "CsrfAttempt"),
            DetectionRule::InformationDisclosureAttempt => write!(f, "InformationDisclosureAttempt"),
            DetectionRule::FileInclusionAttempt => write!(f, "FileInclusionAttempt"),
            DetectionRule::InsecureDeserializationAttempt => write!(f, "InsecureDeserializationAttempt"),
            DetectionRule::InsecureDirectObjectReferenceAttempt => write!(f, "InsecureDirectObjectReferenceAttempt"),
            DetectionRule::SecurityMisconfigurationAttempt => write!(f, "SecurityMisconfigurationAttempt"),
            DetectionRule::XmlExternalEntityAttempt => write!(f, "XmlExternalEntityAttempt"),
            DetectionRule::ServerSideRequestForgeryAttempt => write!(f, "ServerSideRequestForgeryAttempt"),
            DetectionRule::ClickjackingAttempt => write!(f, "ClickjackingAttempt"),
            DetectionRule::InsecureHttpMethodsAttempt => write!(f, "InsecureHttpMethodsAttempt"),
            DetectionRule::MissingSecurityHeadersAttempt => write!(f, "MissingSecurityHeadersAttempt"),
            DetectionRule::SessionManagementAttempt => write!(f, "SessionManagementAttempt"),
            DetectionRule::RateLimitingAttempt => write!(f, "RateLimitingAttempt"),
            DetectionRule::InputValidationAttempt => write!(f, "InputValidationAttempt"),
            DetectionRule::OutputEncodingAttempt => write!(f, "OutputEncodingAttempt"),
            DetectionRule::BufferOverflowAttempt => write!(f, "BufferOverflowAttempt"),
            DetectionRule::RaceConditionAttempt => write!(f, "RaceConditionAttempt"),
            DetectionRule::PrivilegeEscalationAttempt => write!(f, "PrivilegeEscalationAttempt"),
            DetectionRule::NetworkSecurityAttempt => write!(f, "NetworkSecurityAttempt"),
            DetectionRule::CryptographicIssuesAttempt => write!(f, "CryptographicIssuesAttempt"),
            DetectionRule::DoSVulnerabilitiesAttempt => write!(f, "DoSVulnerabilitiesAttempt"),
            DetectionRule::ApiSecurityAttempt => write!(f, "ApiSecurityAttempt"),
            DetectionRule::ZeroDayVulnerabilityAttempt => write!(f, "ZeroDayVulnerabilityAttempt"),
            DetectionRule::SupplyChainAttackAttempt => write!(f, "SupplyChainAttackAttempt"),
            DetectionRule::AiDrivenAttackAttempt => write!(f, "AiDrivenAttackAttempt"),
            DetectionRule::BlockchainSecurityAttempt => write!(f, "BlockchainSecurityAttempt"),
            DetectionRule::QuantumComputingThreatsAttempt => write!(f, "QuantumComputingThreatsAttempt"),
            DetectionRule::EdgeComputingSecurityAttempt => write!(f, "EdgeComputingSecurityAttempt"),
            DetectionRule::IoTSecurityAttempt => write!(f, "IoTSecurityAttempt"),
            DetectionRule::CloudNativeSecurityAttempt => write!(f, "CloudNativeSecurityAttempt"),
            DetectionRule::DevSecOpsIntegrationAttempt => write!(f, "DevSecOpsIntegrationAttempt"),
            DetectionRule::AiGeneratedMaliciousCodeAttempt => write!(f, "AiGeneratedMaliciousCodeAttempt"),
            DetectionRule::ContainerEscapeAttempt => write!(f, "ContainerEscapeAttempt"),
            DetectionRule::CloudServiceMisconfigurationAttempt => write!(f, "CloudServiceMisconfigurationAttempt"),
            DetectionRule::ApiAbuseAttempt => write!(f, "ApiAbuseAttempt"),
            DetectionRule::ServerlessSecurityAttempt => write!(f, "ServerlessSecurityAttempt"),
            DetectionRule::EdgeComputingVulnerabilityAttempt => write!(f, "EdgeComputingVulnerabilityAttempt"),
            DetectionRule::IoTDeviceCompromiseAttempt => write!(f, "IoTDeviceCompromiseAttempt"),
        }
    }
}

/// Intrusion event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: DetectionRule,
    /// Severity level
    pub severity: VulnerabilitySeverity,
    /// Event description
    pub description: String,
    /// Source IP
    pub source_ip: String,
    /// Target resource
    pub target: String,
    /// Event timestamp
    pub timestamp: u64,
    /// Additional details
    pub details: std::collections::HashMap<String, String>,
}

/// Scan statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    /// Total scanned items
    pub total_scanned: usize,
    /// Vulnerabilities found
    pub vulnerabilities_found: usize,
    /// Critical vulnerabilities count
    pub critical_count: usize,
    /// High vulnerabilities count
    pub high_count: usize,
    /// Medium vulnerabilities count
    pub medium_count: usize,
    /// Low vulnerabilities count
    pub low_count: usize,
    /// Info vulnerabilities count
    pub info_count: usize,
    /// Scan duration (seconds)
    pub scan_duration: u64,
}

/// Intrusion detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionStatistics {
    /// Total events detected
    pub total_events: usize,
    /// Critical events count
    pub critical_count: usize,
    /// High events count
    pub high_count: usize,
    /// Medium events count
    pub medium_count: usize,
    /// Low events count
    pub low_count: usize,
    /// Info events count
    pub info_count: usize,
    /// Suspicious IPs count
    pub suspicious_ips_count: usize,
    /// Blocked IPs count
    pub blocked_ips_count: usize,
}
