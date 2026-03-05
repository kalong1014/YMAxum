//! Security scanner binary
//! Performs security vulnerability scanning

use clap::Parser;
use log::{error, info};
use ymaxum::security::{ScanType, ScanScope};
use ymaxum::security::models::{SecurityScanConfig, IntrusionDetectionConfig};
use ymaxum::security::scanner::SecurityScanner;

/// Security scanner command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target to scan
    #[arg(short, long, default_value = ".")]
    target: String,

    /// Output format (json, text)
    #[arg(short, long, default_value = "text")]
    output: String,

    /// Enable deep scan
    #[arg(long, default_value = "false")]
    deep_scan: bool,

    /// Scan timeout in seconds
    #[arg(long, default_value = "300")]
    timeout: u64,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let target = args.target;

    info!("Starting security scan for: {}", target);
    info!("Output format: {}", args.output);
    info!("Deep scan: {}", args.deep_scan);
    info!("Timeout: {} seconds", args.timeout);
    info!("=");

    // Create security scanner
    let config = SecurityScanConfig {
        scan_types: vec![
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
        ],
        scan_scope: ScanScope::Full,
        enable_deep_scan: args.deep_scan,
        scan_timeout: args.timeout,
    };

    let intrusion_config = IntrusionDetectionConfig {
        enabled: true,
        detection_rules: vec![
            ymaxum::security::models::DetectionRule::BruteForce,
            ymaxum::security::models::DetectionRule::DoSAttempt,
            ymaxum::security::models::DetectionRule::SuspiciousIP,
            ymaxum::security::models::DetectionRule::AnomalyDetection,
            ymaxum::security::models::DetectionRule::UnauthorizedAccess,
        ],
        alert_threshold: 5,
        detection_window: 3600,
    };

    let mut scanner = SecurityScanner::new(config, intrusion_config);

    // Run scan
    let stats = scanner.scan(&target);

    // Generate report based on output format
    match args.output.as_str() {
        "json" => {
            if let Ok(json) = scanner.export_results_json() {
                println!("{}", json);
            } else {
                error!("Failed to generate JSON report");
            }
        }
        "text" | _ => {
            println!("=== Security Scan Report ===");
            println!("Target: {}", target);
            println!(
                "Total vulnerabilities found: {}",
                stats.vulnerabilities_found
            );
            println!("Critical: {}", stats.critical_count);
            println!("High: {}", stats.high_count);
            println!("Medium: {}", stats.medium_count);
            println!("Low: {}", stats.low_count);
            println!("Info: {}", stats.info_count);
            println!("Scan duration: {} seconds", stats.scan_duration);
            println!("=");

            // Print critical vulnerabilities
            let critical_vulns = scanner.get_critical_vulnerabilities();
            if !critical_vulns.is_empty() {
                println!("Critical vulnerabilities:");
                for vuln in critical_vulns {
                    println!("- [{}] {}", vuln.id, vuln.name);
                    println!("  Description: {}", vuln.description);
                    println!("  Location: {}", vuln.location);
                    println!("  Remediation:");
                    for remediation in &vuln.remediation {
                        println!("    - {}", remediation);
                    }
                    println!();
                }
            }

            // Print high vulnerabilities
            let high_vulns = scanner.get_high_vulnerabilities();
            if !high_vulns.is_empty() {
                println!("High vulnerabilities:");
                for vuln in high_vulns {
                    println!("- [{}] {}", vuln.id, vuln.name);
                    println!("  Description: {}", vuln.description);
                    println!("  Location: {}", vuln.location);
                    println!();
                }
            }
        }
    }

    println!("Security scan completed.");
}
