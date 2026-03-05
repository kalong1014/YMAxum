// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Intrusion detection module
//! Provides intrusion detection capabilities

use log::info;
use std::collections::{HashMap, HashSet};
use std::time::{Instant, SystemTime};

use crate::security::models::{DetectionRule, IntrusionDetectionConfig, IntrusionEvent, IntrusionStatistics, VulnerabilitySeverity};

/// Intrusion detection engine
    #[derive(Clone)]
    pub struct IntrusionDetectionEngine {
        /// Configuration
        config: IntrusionDetectionConfig,
        /// Intrusion events
        intrusion_events: Vec<IntrusionEvent>,
        /// Intrusion detection statistics
        intrusion_stats: IntrusionStatistics,
        /// IP activity tracker
        ip_activity: HashMap<String, Vec<Instant>>,
        /// Suspicious IPs
        suspicious_ips: HashSet<String>,
        /// Blocked IPs
        blocked_ips: HashSet<String>,
        /// Real-time alerts
        alerts: Vec<String>,
        /// IP behavior patterns
        ip_behavior_patterns: HashMap<String, HashMap<String, Vec<u64>>>,
        /// Adaptive thresholds
        adaptive_thresholds: HashMap<String, u32>,
        /// Machine learning model data
        ml_model_data: HashMap<String, Vec<f64>>,
        /// IP behavior baselines
        ip_behavior_baselines: HashMap<String, HashMap<String, f64>>,
        /// Multi-dimensional anomaly scores
        multi_dimensional_scores: HashMap<String, HashMap<String, f64>>,
        /// False positive history
        false_positive_history: HashMap<String, Vec<u64>>,
        /// True positive history
        true_positive_history: HashMap<String, Vec<u64>>,
    }

impl IntrusionDetectionEngine {
    /// Create new intrusion detection engine
    pub fn new(config: IntrusionDetectionConfig) -> Self {
        Self {
            config,
            intrusion_events: Vec::new(),
            intrusion_stats: IntrusionStatistics {
                total_events: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                low_count: 0,
                info_count: 0,
                suspicious_ips_count: 0,
                blocked_ips_count: 0,
            },
            ip_activity: HashMap::new(),
            suspicious_ips: HashSet::new(),
            blocked_ips: HashSet::new(),
            alerts: Vec::new(),
            ip_behavior_patterns: HashMap::new(),
            adaptive_thresholds: HashMap::new(),
            ml_model_data: HashMap::new(),
            ip_behavior_baselines: HashMap::new(),
            multi_dimensional_scores: HashMap::new(),
            false_positive_history: HashMap::new(),
            true_positive_history: HashMap::new(),
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: IntrusionDetectionConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &IntrusionDetectionConfig {
        &self.config
    }

    /// Detect intrusion
    pub fn detect_intrusion(&mut self, source_ip: &str, target: &str, event_type: DetectionRule, details: HashMap<String, String>) {
        if !self.config.enabled {
            return;
        }

        // Check if the rule is enabled
        if !self.config.detection_rules.contains(&event_type) {
            return;
        }

        // Track IP activity
        self.track_ip_activity(source_ip);

        // Analyze IP behavior patterns
        self.analyze_behavior_patterns(source_ip, &event_type, target);

        // Determine severity based on event type and behavior analysis
        let severity = self.determine_severity(&event_type, source_ip);

        // Create intrusion event
        let event_type_clone = event_type.clone();
        let event = IntrusionEvent {
            id: format!("EVENT-{}-{}", event_type.to_string().to_uppercase().replace(" ", "-"), self.intrusion_events.len() + 1),
            event_type,
            severity,
            description: format!("{} detected from IP {}", event_type_clone.to_string(), source_ip),
            source_ip: source_ip.to_string(),
            target: target.to_string(),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            details,
        };

        // Add event to collection
        self.intrusion_events.push(event.clone());

        // Update intrusion statistics
        self.update_intrusion_stats();

        // Check if IP should be marked as suspicious using adaptive threshold
        self.check_suspicious_ip_with_adaptive_threshold(source_ip);

        info!("Intrusion detected: {} from IP {}", event_type_clone.to_string(), source_ip);
    }

    /// Track IP activity
    fn track_ip_activity(&mut self, ip: &str) {
        let now = Instant::now();
        let activities = self.ip_activity.entry(ip.to_string()).or_insert(Vec::new());
        
        // Remove old activities outside the detection window
        activities.retain(|&time| now.duration_since(time).as_secs() < self.config.detection_window);
        
        // Add new activity
        activities.push(now);
    }

    /// Check if IP is suspicious
    #[allow(dead_code)]
    fn check_suspicious_ip(&mut self, ip: &str) {
        let empty_vec = vec![];
        let activities = self.ip_activity.get(ip).unwrap_or(&empty_vec);
        
        if activities.len() >= self.config.alert_threshold as usize && !self.suspicious_ips.contains(ip) {
            self.suspicious_ips.insert(ip.to_string());
            self.intrusion_stats.suspicious_ips_count += 1;
            info!("IP {} marked as suspicious due to high activity", ip);
        }
    }

    /// Analyze IP behavior patterns
    fn analyze_behavior_patterns(&mut self, ip: &str, event_type: &DetectionRule, target: &str) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        
        // Get or create behavior pattern for this IP
        let patterns = self.ip_behavior_patterns.entry(ip.to_string()).or_insert(HashMap::new());
        
        // Record event type occurrence
        let event_type_key = event_type.to_string();
        let event_times = patterns.entry(event_type_key).or_insert(Vec::new());
        event_times.push(now);
        
        // Record target access
        let target_key = format!("target_{}", target);
        let target_times = patterns.entry(target_key).or_insert(Vec::new());
        target_times.push(now);
        
        // Keep only recent events (last 24 hours)
        let twenty_four_hours = 24 * 60 * 60;
        for (_, times) in patterns.iter_mut() {
            times.retain(|&time| now - time < twenty_four_hours);
        }
        
        // Update behavior baselines
        self.update_behavior_baselines(ip);
        
        // Calculate multi-dimensional anomaly scores
        self.calculate_multi_dimensional_scores(ip);
        
        // Update machine learning model
        self.update_ml_model(ip);
    }

    /// Update behavior baselines for IP
    fn update_behavior_baselines(&mut self, ip: &str) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let one_hour = 60 * 60;
        
        if let Some(patterns) = self.ip_behavior_patterns.get(ip) {
            let baselines = self.ip_behavior_baselines.entry(ip.to_string()).or_insert(HashMap::new());
            
            for (key, times) in patterns {
                // Calculate average event rate per hour
                let recent_times = times.iter().filter(|&&time| now - time < one_hour).count();
                let avg_rate = recent_times as f64 / one_hour as f64;
                
                // Update baseline with exponential moving average
                if let Some(current_baseline) = baselines.get(key) {
                    let new_baseline = 0.7 * current_baseline + 0.3 * avg_rate;
                    baselines.insert(key.to_string(), new_baseline);
                } else {
                    baselines.insert(key.to_string(), avg_rate);
                }
            }
        }
    }

    /// Calculate multi-dimensional anomaly scores
    fn calculate_multi_dimensional_scores(&mut self, ip: &str) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let one_hour = 60 * 60;
        
        if let Some(patterns) = self.ip_behavior_patterns.get(ip) {
            if let Some(baselines) = self.ip_behavior_baselines.get(ip) {
                let scores = self.multi_dimensional_scores.entry(ip.to_string()).or_insert(HashMap::new());
                
                for (key, times) in patterns {
                    // Calculate current rate
                    let recent_times = times.iter().filter(|&&time| now - time < one_hour).count();
                    let current_rate = recent_times as f64 / one_hour as f64;
                    
                    // Calculate anomaly score
                    if let Some(baseline) = baselines.get(key) {
                        let score = if *baseline > 0.0 {
                            (current_rate - *baseline).abs() / *baseline
                        } else {
                            if current_rate > 0.0 {
                                1.0
                            } else {
                                0.0
                            }
                        };
                        
                        scores.insert(key.to_string(), score);
                    }
                }
            }
        }
    }

    /// Determine severity based on event type and behavior analysis
    fn determine_severity(&self, event_type: &DetectionRule, ip: &str) -> VulnerabilitySeverity {
        // Base severity based on event type
        let base_severity = match event_type {
            DetectionRule::BruteForce => VulnerabilitySeverity::Critical,
            DetectionRule::DoSAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::SuspiciousIP => VulnerabilitySeverity::High,
            DetectionRule::AnomalyDetection => VulnerabilitySeverity::Medium,
            DetectionRule::UnauthorizedAccess => VulnerabilitySeverity::High,
            DetectionRule::SqlInjectionAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::XssAttempt => VulnerabilitySeverity::High,
            DetectionRule::CommandInjectionAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::CsrfAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::InformationDisclosureAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::FileInclusionAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::InsecureDeserializationAttempt => VulnerabilitySeverity::High,
            DetectionRule::InsecureDirectObjectReferenceAttempt => VulnerabilitySeverity::High,
            DetectionRule::SecurityMisconfigurationAttempt => VulnerabilitySeverity::High,
            DetectionRule::XmlExternalEntityAttempt => VulnerabilitySeverity::High,
            DetectionRule::ServerSideRequestForgeryAttempt => VulnerabilitySeverity::High,
            DetectionRule::ClickjackingAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::InsecureHttpMethodsAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::MissingSecurityHeadersAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::SessionManagementAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::RateLimitingAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::InputValidationAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::OutputEncodingAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::BufferOverflowAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::RaceConditionAttempt => VulnerabilitySeverity::High,
            DetectionRule::PrivilegeEscalationAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::NetworkSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::CryptographicIssuesAttempt => VulnerabilitySeverity::High,
            DetectionRule::DoSVulnerabilitiesAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::ApiSecurityAttempt => VulnerabilitySeverity::High,
            DetectionRule::ZeroDayVulnerabilityAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::SupplyChainAttackAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::AiDrivenAttackAttempt => VulnerabilitySeverity::High,
            DetectionRule::BlockchainSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::QuantumComputingThreatsAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::EdgeComputingSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::IoTSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::CloudNativeSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::DevSecOpsIntegrationAttempt => VulnerabilitySeverity::Low,
            DetectionRule::AiGeneratedMaliciousCodeAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::ContainerEscapeAttempt => VulnerabilitySeverity::Critical,
            DetectionRule::CloudServiceMisconfigurationAttempt => VulnerabilitySeverity::High,
            DetectionRule::ApiAbuseAttempt => VulnerabilitySeverity::High,
            DetectionRule::ServerlessSecurityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::EdgeComputingVulnerabilityAttempt => VulnerabilitySeverity::Medium,
            DetectionRule::IoTDeviceCompromiseAttempt => VulnerabilitySeverity::High,
        };
        
        // Adjust severity based on behavior analysis
        let attack_probability = self.predict_attack_probability(ip);
        
        if attack_probability > 0.8 {
            // Increase severity if high attack probability
            match base_severity {
                VulnerabilitySeverity::Low => VulnerabilitySeverity::Medium,
                VulnerabilitySeverity::Medium => VulnerabilitySeverity::High,
                VulnerabilitySeverity::High => VulnerabilitySeverity::Critical,
                _ => base_severity,
            }
        } else if attack_probability < 0.3 {
            // Decrease severity if low attack probability
            match base_severity {
                VulnerabilitySeverity::Critical => VulnerabilitySeverity::High,
                VulnerabilitySeverity::High => VulnerabilitySeverity::Medium,
                VulnerabilitySeverity::Medium => VulnerabilitySeverity::Low,
                _ => base_severity,
            }
        } else {
            base_severity
        }
    }

    /// Check if IP is suspicious using adaptive threshold
    fn check_suspicious_ip_with_adaptive_threshold(&mut self, ip: &str) {
        let threshold = self.calculate_adaptive_threshold(ip);
        
        let empty_vec = vec![];
        let activities = self.ip_activity.get(ip).unwrap_or(&empty_vec);
        
        if activities.len() >= threshold as usize && !self.suspicious_ips.contains(ip) {
            self.suspicious_ips.insert(ip.to_string());
            self.intrusion_stats.suspicious_ips_count += 1;
            info!("IP {} marked as suspicious due to high activity (adaptive threshold: {})", ip, threshold);
        }
    }

    /// Calculate adaptive threshold for IP
    fn calculate_adaptive_threshold(&mut self, ip: &str) -> u32 {
        // Get or create adaptive threshold for this IP
        let threshold = self.adaptive_thresholds.entry(ip.to_string()).or_insert(self.config.alert_threshold);
        
        // Analyze behavior patterns to adjust threshold
        if let Some(patterns) = self.ip_behavior_patterns.get(ip) {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let one_hour = 60 * 60;
            
            // Count events in the last hour
            let recent_events = patterns.values()
                .flat_map(|times| times.iter().filter(|&&time| now - time < one_hour))
                .count();
            
            // Check for anomaly scores
            let mut anomaly_score = 0.0;
            if let Some(scores) = self.multi_dimensional_scores.get(ip) {
                if !scores.is_empty() {
                    anomaly_score = scores.values().fold(0.0, |sum, &score| sum + score) / scores.len() as f64;
                }
            }
            
            // Adjust threshold based on recent activity and anomaly score
            if recent_events > *threshold as usize * 2 || anomaly_score > 1.5 {
                // High activity or high anomaly, lower threshold
                *threshold = (*threshold as f32 * 0.7).max(2.0) as u32;
            } else if recent_events < *threshold as usize / 2 && anomaly_score < 0.5 {
                // Low activity and low anomaly, raise threshold
                *threshold = (*threshold as f32 * 1.3).min(self.config.alert_threshold as f32 * 2.0) as u32;
            } else if anomaly_score > 1.0 {
                // Moderate anomaly, slightly lower threshold
                *threshold = (*threshold as f32 * 0.9).max(2.0) as u32;
            }
        }
        
        *threshold
    }

    /// Update machine learning model data
    fn update_ml_model(&mut self, ip: &str) {
        // Extract features from behavior patterns
        let features = self.extract_features(ip);
        
        // Store features for future analysis
        self.ml_model_data.insert(ip.to_string(), features);
    }

    /// Extract features from IP behavior
    fn extract_features(&self, ip: &str) -> Vec<f64> {
        let mut features = Vec::new();
        
        // Get behavior patterns for this IP
        if let Some(patterns) = self.ip_behavior_patterns.get(ip) {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let one_hour = 60 * 60;
            let six_hours = 6 * one_hour;
            let twenty_four_hours = 24 * one_hour;
            
            // Feature 1: Total events in last hour
            let events_last_hour = patterns.values()
                .flat_map(|times| times.iter().filter(|&&time| now - time < one_hour))
                .count() as f64;
            features.push(events_last_hour);
            
            // Feature 2: Total events in last 6 hours
            let events_last_six_hours = patterns.values()
                .flat_map(|times| times.iter().filter(|&&time| now - time < six_hours))
                .count() as f64;
            features.push(events_last_six_hours);
            
            // Feature 3: Total events in last 24 hours
            let events_last_twenty_four_hours = patterns.values()
                .flat_map(|times| times.iter().filter(|&&time| now - time < twenty_four_hours))
                .count() as f64;
            features.push(events_last_twenty_four_hours);
            
            // Feature 4: Number of different event types
            let event_types = patterns.keys()
                .filter(|key| key.starts_with("DetectionRule::"))
                .count() as f64;
            features.push(event_types);
            
            // Feature 5: Number of different targets
            let targets = patterns.keys()
                .filter(|key| key.starts_with("target_"))
                .count() as f64;
            features.push(targets);
        } else {
            // Default features for new IPs
            features.extend(vec![0.0; 5]);
        }
        
        features
    }

    /// Predict attack probability using machine learning model
    fn predict_attack_probability(&self, ip: &str) -> f64 {
        // In a real implementation, this would use a trained machine learning model
        // For now, we'll use a heuristic based on features, baselines, and multi-dimensional scores
        
        let mut probability = 0.1; // Default probability for new IPs
        
        // Use machine learning features
        if let Some(features) = self.ml_model_data.get(ip) {
            let total_events = features[0] + features[1] + features[2];
            let event_types = features[3];
            let targets = features[4];
            
            // Base probability from features
            let feature_probability = (total_events / 100.0) * (1.0 + event_types / 10.0) * (1.0 + targets / 5.0);
            probability = feature_probability;
        }
        
        // Adjust based on multi-dimensional anomaly scores
        if let Some(scores) = self.multi_dimensional_scores.get(ip) {
            let high_scores = scores.values().filter(|&&score| score > 1.0).count();
            let avg_score = scores.values().fold(0.0, |sum, &score| sum + score) / scores.len() as f64;
            
            // Increase probability based on anomaly scores
            probability += high_scores as f64 * 0.1 + avg_score * 0.2;
        }
        
        // Adjust based on behavior history
        let false_positives = self.false_positive_history.get(ip).map(|v| v.len()).unwrap_or(0);
        let true_positives = self.true_positive_history.get(ip).map(|v| v.len()).unwrap_or(0);
        
        if false_positives + true_positives > 0 {
            let false_positive_rate = false_positives as f64 / (false_positives + true_positives) as f64;
            // Decrease probability if high false positive rate
            probability *= 1.0 - false_positive_rate * 0.5;
        }
        
        probability.min(1.0).max(0.0)
    }

    /// Get intrusion events
    pub fn get_intrusion_events(&self) -> &Vec<IntrusionEvent> {
        &self.intrusion_events
    }

    /// Get intrusion events by severity
    pub fn get_intrusion_events_by_severity(&self, severity: VulnerabilitySeverity) -> Vec<&IntrusionEvent> {
        self.intrusion_events
            .iter()
            .filter(|&event| event.severity == severity)
            .collect()
    }

    /// Get intrusion statistics
    pub fn get_intrusion_stats(&self) -> &IntrusionStatistics {
        &self.intrusion_stats
    }

    /// Clear intrusion events
    pub fn clear_intrusion_events(&mut self) {
        self.intrusion_events.clear();
        self.intrusion_stats = IntrusionStatistics {
            total_events: 0,
            critical_count: 0,
            high_count: 0,
            medium_count: 0,
            low_count: 0,
            info_count: 0,
            suspicious_ips_count: self.suspicious_ips.len(),
            blocked_ips_count: 0,
        };
        // Clear other temporary data
        self.alerts.clear();
        self.ip_behavior_patterns.clear();
        self.adaptive_thresholds.clear();
        self.ml_model_data.clear();
        self.ip_behavior_baselines.clear();
        self.multi_dimensional_scores.clear();
        self.false_positive_history.clear();
        self.true_positive_history.clear();
    }

    /// Update intrusion statistics
    fn update_intrusion_stats(&mut self) {
        self.intrusion_stats.total_events = self.intrusion_events.len();
        self.intrusion_stats.critical_count = self.get_intrusion_events_by_severity(VulnerabilitySeverity::Critical).len();
        self.intrusion_stats.high_count = self.get_intrusion_events_by_severity(VulnerabilitySeverity::High).len();
        self.intrusion_stats.medium_count = self.get_intrusion_events_by_severity(VulnerabilitySeverity::Medium).len();
        self.intrusion_stats.low_count = self.get_intrusion_events_by_severity(VulnerabilitySeverity::Low).len();
        self.intrusion_stats.info_count = self.get_intrusion_events_by_severity(VulnerabilitySeverity::Info).len();
        self.intrusion_stats.suspicious_ips_count = self.suspicious_ips.len();
        self.intrusion_stats.blocked_ips_count = self.blocked_ips.len();
    }

    /// Check if IP is blocked
    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        self.blocked_ips.contains(ip)
    }

    /// Block IP
    pub fn block_ip(&mut self, ip: &str) {
        if !self.blocked_ips.contains(ip) {
            self.blocked_ips.insert(ip.to_string());
            self.update_intrusion_stats();
            let alert = format!("IP {} has been blocked due to suspicious activity", ip);
            self.alerts.push(alert.clone());
            info!("{}", alert);
        }
    }

    /// Unblock IP
    pub fn unblock_ip(&mut self, ip: &str) {
        if self.blocked_ips.remove(ip) {
            self.update_intrusion_stats();
            let alert = format!("IP {} has been unblocked", ip);
            self.alerts.push(alert.clone());
            info!("{}", alert);
        }
    }

    /// Get all alerts
    pub fn get_alerts(&self) -> &Vec<String> {
        &self.alerts
    }

    /// Clear alerts
    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    /// Generate security report
    pub fn generate_security_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Security Intrusion Detection Report ===\n\n");

        // Summary
        report.push_str("Summary:\n");
        report.push_str(&format!("Total intrusion events: {}\n", self.intrusion_stats.total_events));
        report.push_str(&format!("Critical events: {}\n", self.intrusion_stats.critical_count));
        report.push_str(&format!("High severity events: {}\n", self.intrusion_stats.high_count));
        report.push_str(&format!("Medium severity events: {}\n", self.intrusion_stats.medium_count));
        report.push_str(&format!("Low severity events: {}\n", self.intrusion_stats.low_count));
        report.push_str(&format!("Info events: {}\n", self.intrusion_stats.info_count));
        report.push_str(&format!("Suspicious IPs: {}\n", self.intrusion_stats.suspicious_ips_count));
        report.push_str(&format!("Blocked IPs: {}\n\n", self.intrusion_stats.blocked_ips_count));

        // Recent alerts
        if !self.alerts.is_empty() {
            report.push_str("Recent Alerts:\n");
            for alert in self.alerts.iter().take(10) {
                report.push_str(&format!("- {}\n", alert));
            }
            report.push_str("\n");
        }

        // Top suspicious IPs
        if !self.suspicious_ips.is_empty() {
            report.push_str("Top Suspicious IPs:\n");
            for ip in self.suspicious_ips.iter().take(10) {
                let activity_count = self.ip_activity.get(ip).map(|v| v.len()).unwrap_or(0);
                report.push_str(&format!("- {} ({} activities)\n", ip, activity_count));
            }
            report.push_str("\n");
        }

        // Blocked IPs
        if !self.blocked_ips.is_empty() {
            report.push_str("Blocked IPs:\n");
            for ip in self.blocked_ips.iter() {
                report.push_str(&format!("- {}\n", ip));
            }
            report.push_str("\n");
        }

        report
    }

    /// Process real-time security event
    pub fn process_security_event(&mut self, source_ip: &str, target: &str, event_type: DetectionRule, details: HashMap<String, String>) {
        // Detect intrusion
        self.detect_intrusion(source_ip, target, event_type, details);

        // Check if IP should be blocked
        let empty_vec = vec![];
        let activities = self.ip_activity.get(source_ip).unwrap_or(&empty_vec);
        
        if activities.len() >= self.config.alert_threshold as usize && !self.blocked_ips.contains(source_ip) {
            self.block_ip(source_ip);
        }
    }
}
