// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use log::{debug, info};
use std::time::Instant;
use tokio::runtime::Runtime;
use crate::security::scanner_core::SecurityScannerCore;
use crate::security::intrusion_detection::IntrusionDetectionEngine;
use crate::security::vulnerability_db::{VulnerabilityDbConfig, VulnerabilityDatabase};

pub fn run_security_benchmarks() {
    info!("=== Security Module Benchmarks ===");
    
    // Benchmark security scanner
    benchmark_security_scanner();
    
    // Benchmark intrusion detection
    benchmark_intrusion_detection();
    
    // Benchmark vulnerability database
    benchmark_vulnerability_database();
    
    info!("=== Benchmarks Completed ===");
}

fn benchmark_security_scanner() {
    info!("\n1. Security Scanner Benchmark");
    
    let mut scanner = SecurityScannerCore::new(SecurityScanConfig::default());
    
    // Warm-up
    scanner.scan("http://example.com");
    scanner.clear_results();
    
    // Benchmark full scan
    let start_time = Instant::now();
    let stats = scanner.scan("http://example.com");
    let duration = start_time.elapsed();
    
    debug!("   Full scan time: {:?}", duration);
    debug!("   Vulnerabilities found: {}", stats.vulnerabilities_found);
    debug!("   Total scanned: {}", stats.total_scanned);
    
    // Benchmark incremental scan
    let start_time = Instant::now();
    let stats = scanner.scan("http://example.com");
    let duration = start_time.elapsed();
    
    debug!("   Incremental scan time: {:?}", duration);
    debug!("   Vulnerabilities found: {}", stats.vulnerabilities_found);
    debug!("   Total scanned: {}", stats.total_scanned);
}

fn benchmark_intrusion_detection() {
    info!("\n2. Intrusion Detection Benchmark");
    
    let config = IntrusionDetectionConfig::default();
    let mut engine = IntrusionDetectionEngine::new(config);
    
    let mut details = std::collections::HashMap::new();
    details.insert("user_agent".to_string(), "Mozilla/5.0".to_string());
    
    // Benchmark detection performance
    let start_time = Instant::now();
    for i in 0..1000 {
        engine.detect_intrusion(
            &format!("192.168.1.{}", i % 255),
            "/login",
            DetectionRule::BruteForce,
            details.clone()
        );
    }
    let duration = start_time.elapsed();
    
    let stats = engine.get_intrusion_stats();
    debug!("   Detection time for 1000 events: {:?}", duration);
    debug!("   Average detection time per event: {:?}", duration / 1000);
    debug!("   Total events detected: {}", stats.total_events);
}

fn benchmark_vulnerability_database() {
    info!("\n3. Vulnerability Database Benchmark");
    
    let config = VulnerabilityDbConfig::default();
    let mut db = VulnerabilityDatabase::new(config);
    
    // Benchmark update
    let start_time = Instant::now();
    Runtime::new().unwrap().block_on(async {
        let result = db.update().await;
        assert!(result.is_ok());
    });
    let duration = start_time.elapsed();
    
    debug!("   Database update time: {:?}", duration);
    
    // Benchmark query performance
    let all_vulns = db.get_all_vulnerabilities();
    debug!("   Total vulnerabilities: {}", all_vulns.len());
    
    let start_time = Instant::now();
    for vuln in all_vulns {
        let _ = db.get_vulnerability_by_cve(&vuln.cve_id);
    }
    let duration = start_time.elapsed();
    
    debug!("   Query time for all vulnerabilities: {:?}", duration);
    debug!("   Average query time per vulnerability: {:?}", duration / all_vulns.len() as u32);
}
