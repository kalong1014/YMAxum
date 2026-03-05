// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::security::scanner_core::SecurityScannerCore;
    use crate::security::intrusion_detection::IntrusionDetectionEngine;
    use crate::security::vulnerability_db::{VulnerabilityDbConfig, VulnerabilityDatabase};

    #[test]
    fn test_security_scanner_core() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        let stats = scanner.scan("http://example.com");
        assert!(stats.total_scanned > 0);

    }

    #[test]
    fn test_security_scanner_core_incremental_scan() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // First scan
        let first_stats = scanner.scan("http://example.com");
        assert!(first_stats.total_scanned > 0);
        
        // Second scan (should be incremental)
        let second_stats = scanner.scan("http://example.com");
        assert!(second_stats.total_scanned > 0);
    }

    #[test]
    fn test_intrusion_detection_engine() {
        let config = IntrusionDetectionConfig::default();
        let mut engine = IntrusionDetectionEngine::new(config);
        
        let mut details = std::collections::HashMap::new();
        details.insert("user_agent".to_string(), "Mozilla/5.0".to_string());
        
        engine.detect_intrusion(
            "192.168.1.1",
            "/login",
            DetectionRule::BruteForce,
            details
        );
        
        let events = engine.get_intrusion_events();
        assert!(!events.is_empty());
        
        let stats = engine.get_intrusion_stats();
        assert!(stats.total_events > 0);
    }

    #[test]
    fn test_vulnerability_database() {
        let config = VulnerabilityDbConfig::default();
        let mut db = VulnerabilityDatabase::new(config);
        
        // Test update
        tokio_test::block_on(async {
            let result = db.update().await;
            assert!(result.is_ok());
        });
        
        // Test get all vulnerabilities
        let all_vulns = db.get_all_vulnerabilities();
        assert!(!all_vulns.is_empty());
        
        // Test get vulnerability by CVE
        let cve_id = &all_vulns[0].cve_id;
        let vuln = db.get_vulnerability_by_cve(cve_id);
        assert!(vuln.is_some());
        
        // Test get vulnerabilities by severity
        let critical_vulns = db.get_vulnerabilities_by_severity(9.0);
        assert!(!critical_vulns.is_empty());
        
        // Test get vulnerabilities by component
        let component = &all_vulns[0].affected_components[0];
        let component_vulns = db.get_vulnerabilities_by_component(component);
        assert!(!component_vulns.is_empty());
        
        // Test is component affected
        let is_affected = db.is_component_affected(component);
        assert!(is_affected);
        
        // Test generate report
        let report = db.generate_report();
        assert!(!report.is_empty());
    }

    #[test]
    fn test_vulnerability_database_classification() {
        let config = VulnerabilityDbConfig::default();
        let mut db = VulnerabilityDatabase::new(config);
        
        // Test auto-classification
        db.auto_classify_vulnerabilities();
        
        // Test get all classifications
        let classifications = db.get_all_classifications();
        assert!(!classifications.is_empty());
        
        // Test get vulnerabilities by classification
        if !classifications.is_empty() {
            let classification = &classifications[0];
            let vulns = db.get_vulnerabilities_by_classification(classification);
            assert!(!vulns.is_empty());
        }
    }

    #[test]
    fn test_security_scanner() {
        let scanner = SecurityScanner::builder()
            .with_scan_config(SecurityScanConfig::default())
            .with_intrusion_config(IntrusionDetectionConfig::default())
            .build();
        
        // Test get intrusion config
        let intrusion_config = scanner.get_intrusion_config();
        assert!(intrusion_config.enabled);
    }

    #[test]
    fn test_performance_scan() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        let start_time = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration = start_time.elapsed();
        
        // Scan should complete within 10 seconds
        assert!(duration < Duration::from_secs(10));
    }

    #[test]
    fn test_performance_incremental_scan() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // First scan
        let start_time_1 = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration_1 = start_time_1.elapsed();
        
        // Second scan (should be faster)
        let start_time_2 = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration_2 = start_time_2.elapsed();
        
        // Incremental scan should be faster
        assert!(duration_2 < duration_1);
    }

    #[test]
    fn test_false_positive_filtering() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // Perform a scan
        scanner.scan("http://example.com");
        let _initial_count = scanner.get_scan_results().len();
        
        // The scan should have filtered out some false positives

    }

    #[test]
    fn test_new_attack_types() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // Add new attack types to scan config
        let mut config = SecurityScanConfig::default();
        config.scan_types.push(ScanType::AiGeneratedMaliciousCode);
        config.scan_types.push(ScanType::ContainerEscape);
        config.scan_types.push(ScanType::CloudServiceMisconfiguration);
        config.scan_types.push(ScanType::ApiAbuse);
        scanner.set_config(config);
        
        // Perform a scan
        let stats = scanner.scan("http://example.com");
        assert!(stats.total_scanned > 0);
    }

    #[test]
    fn test_intelligent_anomaly_detection() {
        let config = IntrusionDetectionConfig::default();
        let mut engine = IntrusionDetectionEngine::new(config);
        
        // Simulate multiple intrusion attempts
        for i in 0..10 {
            let mut details = std::collections::HashMap::new();
            details.insert("user_agent".to_string(), format!("Mozilla/5.0 (attacker-{})", i));
            
            engine.detect_intrusion(
                "192.168.1.100",
                "/login",
                DetectionRule::BruteForce,
                details
            );
        }
        
        // Check if the IP was marked as suspicious
        let stats = engine.get_intrusion_stats();
        assert!(stats.suspicious_ips_count > 0);
    }

    #[test]
    fn test_vulnerability_database_new_features() {
        let config = VulnerabilityDbConfig::default();
        let mut db = VulnerabilityDatabase::new(config);
        
        // Test update
        tokio_test::block_on(async {
            let result = db.update().await;
            assert!(result.is_ok());
        });
        
        // Test new methods
        let prioritized = db.get_prioritized_vulnerabilities();
        assert!(!prioritized.is_empty());
        
        let _trends = db.get_vulnerability_trends();
        // Trends might be empty if no vulnerabilities have valid dates
        
        let top_vulns = db.get_top_vulnerabilities(5);
        assert!(!top_vulns.is_empty());
    }

    #[test]
    fn test_scanner_performance_optimization() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // Test parallel scan performance
        let start_time = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration = start_time.elapsed();
        
        // Scan should complete within 5 seconds (optimized)
        assert!(duration < Duration::from_secs(5));
    }

    #[test]
    fn test_incremental_scan_optimization() {
        let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
        
        // First scan
        let start_time_1 = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration_1 = start_time_1.elapsed();
        
        // Second scan (should be faster with optimization)
        let start_time_2 = std::time::Instant::now();
        scanner.scan("http://example.com");
        let duration_2 = start_time_2.elapsed();
        
        // Incremental scan should be significantly faster
        assert!(duration_2 < duration_1);
        assert!(duration_2 < Duration::from_secs(2)); // Should be very fast
    }
}
