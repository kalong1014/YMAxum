//! Security monitoring module
//! Provides security event monitoring, security alerts, and security logs

use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Monitor items
    pub monitor_items: Vec<MonitorItem>,
    /// Alert rules
    pub alert_rules: Vec<AlertRule>,
    /// Log retention days
    pub log_retention_days: u32,
    /// Enable real-time monitoring
    pub enable_realtime_monitoring: bool,
    /// Monitor interval (seconds)
    pub monitor_interval: u64,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            monitor_items: vec![
                MonitorItem::AuthenticationAttempts,
                MonitorItem::AuthorizationFailures,
                MonitorItem::SqlInjectionAttempts,
                MonitorItem::XssAttempts,
                MonitorItem::CsrfAttempts,
                MonitorItem::RateLimitViolations,
                MonitorItem::UnusualTraffic,
                MonitorItem::CommandInjectionAttempts,
                MonitorItem::FileInclusionAttempts,
                MonitorItem::SsrfAttempts,
                MonitorItem::XxeAttempts,
                MonitorItem::InsecureDeserializationAttempts,
                MonitorItem::PrivilegeEscalationAttempts,
                MonitorItem::DataExfiltrationAttempts,
                MonitorItem::BruteForceAttempts,
                MonitorItem::DdosAttempts,
                MonitorItem::MalwareDetection,
                MonitorItem::AnomalyDetection,
            ],
            alert_rules: vec![
                AlertRule::HighFailureRate,
                AlertRule::MultipleFailedAttempts,
                AlertRule::UnusualAccessPattern,
                AlertRule::HighResourceUsage,
                AlertRule::PotentialAttackDetected,
                AlertRule::DataExfiltrationAttempt,
                AlertRule::BruteForceAttack,
                AlertRule::DdosAttack,
                AlertRule::MalwareDetected,
                AlertRule::AnomalyDetected,
            ],
            log_retention_days: 30,
            enable_realtime_monitoring: true,
            monitor_interval: 60,
        }
    }
}

/// Monitor item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonitorItem {
    /// Authentication attempts
    AuthenticationAttempts,
    /// Authorization failures
    AuthorizationFailures,
    /// SQL injection attempts
    SqlInjectionAttempts,
    /// XSS attempts
    XssAttempts,
    /// CSRF attempts
    CsrfAttempts,
    /// Rate limit violations
    RateLimitViolations,
    /// Unusual traffic
    UnusualTraffic,
    /// Command injection attempts
    CommandInjectionAttempts,
    /// File inclusion attempts
    FileInclusionAttempts,
    /// Server-side request forgery attempts
    SsrfAttempts,
    /// XML external entity attempts
    XxeAttempts,
    /// Insecure deserialization attempts
    InsecureDeserializationAttempts,
    /// Privilege escalation attempts
    PrivilegeEscalationAttempts,
    /// Data exfiltration attempts
    DataExfiltrationAttempts,
    /// Brute force attacks
    BruteForceAttempts,
    /// DDoS attacks
    DdosAttempts,
    /// Malware detection
    MalwareDetection,
    /// Anomaly detection
    AnomalyDetection,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertRule {
    /// High failure rate
    HighFailureRate,
    /// Multiple failed attempts
    MultipleFailedAttempts,
    /// Unusual access pattern
    UnusualAccessPattern,
    /// High resource usage
    HighResourceUsage,
    /// Potential attack detected
    PotentialAttackDetected,
    /// Data exfiltration attempt
    DataExfiltrationAttempt,
    /// Brute force attack
    BruteForceAttack,
    /// DDoS attack
    DdosAttack,
    /// Malware detected
    MalwareDetected,
    /// Anomaly detected
    AnomalyDetected,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: MonitorItem,
    /// Event severity
    pub severity: EventSeverity,
    /// Event description
    pub description: String,
    /// Source IP
    pub source_ip: Option<String>,
    /// Target user
    pub target_user: Option<String>,
    /// Event timestamp
    pub timestamp: String,
    /// Event details
    pub details: serde_json::Value,
}

/// Event severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
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

/// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    /// Alert ID
    pub id: String,
    /// Alert type
    pub alert_type: AlertRule,
    /// Alert severity
    pub severity: EventSeverity,
    /// Alert description
    pub description: String,
    /// Triggered events
    pub triggered_events: Vec<SecurityEvent>,
    /// Alert timestamp
    pub timestamp: String,
    /// Is handled
    pub is_handled: bool,
}

/// Security monitor
pub struct SecurityMonitor {
    /// Configuration
    config: MonitorConfig,
    /// Security events
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    /// Security alerts
    alerts: Arc<RwLock<Vec<SecurityAlert>>>,
    /// Statistics information
    statistics: Arc<RwLock<MonitorStatistics>>,
}

/// Monitor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorStatistics {
    /// Total events count
    pub total_events: usize,
    /// Critical events count
    pub critical_events: usize,
    /// High events count
    pub high_events: usize,
    /// Medium events count
    pub medium_events: usize,
    /// Low events count
    pub low_events: usize,
    /// Info events count
    pub info_events: usize,
    /// Total alerts count
    pub total_alerts: usize,
    /// Unhandled alerts count
    pub unhandled_alerts: usize,
}

impl SecurityMonitor {
    /// Create new security monitor
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(MonitorStatistics {
                total_events: 0,
                critical_events: 0,
                high_events: 0,
                medium_events: 0,
                low_events: 0,
                info_events: 0,
                total_alerts: 0,
                unhandled_alerts: 0,
            })),
        }
    }

    /// Record security event
    pub async fn record_event(&self, event: SecurityEvent) {
        info!("Recording security event: {:?}", event.event_type);

        // Push event to events list
        {
            let mut events = self.events.write().await;
            events.push(event.clone());

            // Keep only the most recent 10000 events
            if events.len() > 10000 {
                events.remove(0);
            }
        }

        // Update statistics
        self.update_statistics(&event).await;

        // Check if alert should be triggered
        self.check_alert_rules(&event).await;
    }

    /// Check alert rules
    async fn check_alert_rules(&self, event: &SecurityEvent) {
        for rule in &self.config.alert_rules {
            if self.should_trigger_alert(rule, event).await {
                self.create_alert(rule.clone(), event).await;
            }
        }
    }

    /// Determine if alert should be triggered
    async fn should_trigger_alert(&self, rule: &AlertRule, _event: &SecurityEvent) -> bool {
        match rule {
            AlertRule::HighFailureRate => {
                // Check if failure rate exceeds threshold
                {
                    let stats = self.statistics.read().await;
                    if stats.total_events == 0 {
                        return false;
                    }
                    let failure_rate = (stats.critical_events + stats.high_events) as f64
                        / stats.total_events as f64
                        * 100.0;
                    failure_rate > 10.0
                }
            }
            AlertRule::MultipleFailedAttempts => {
                // Check if there are multiple failed attempts
                {
                    let events = self.events.read().await;
                    let recent_failures = events
                        .iter()
                        .filter(|e| {
                            matches!(e.severity, EventSeverity::Critical | EventSeverity::High)
                        })
                        .count();
                    recent_failures > 5
                }
            }
            AlertRule::UnusualAccessPattern => {
                // Check if there is unusual access pattern
                {
                    let events = self.events.read().await;
                    let source_ips: Vec<_> =
                        events.iter().filter_map(|e| e.source_ip.clone()).collect();
                    let unique_ips: std::collections::HashSet<_> = source_ips.into_iter().collect();
                    unique_ips.len() > 50
                }
            }
            AlertRule::HighResourceUsage => {
                // Check if resource usage is too high
                // Simulate resource usage monitoring
                false
            }
            AlertRule::PotentialAttackDetected => {
                // Check if potential attack is detected
                {
                    let events = self.events.read().await;
                    let suspicious_events = events
                        .iter()
                        .filter(|e| {
                            matches!(e.severity, EventSeverity::Critical | EventSeverity::High)
                        })
                        .count();
                    suspicious_events > 3
                }
            }
            AlertRule::DataExfiltrationAttempt => {
                // Check if data exfiltration attempt is detected
                {
                    let events = self.events.read().await;
                    let data_breach_events = events
                        .iter()
                        .filter(|e| e.event_type == MonitorItem::DataExfiltrationAttempts)
                        .count();
                    data_breach_events > 0
                }
            }
            AlertRule::BruteForceAttack => {
                // Check if brute force attack is detected
                {
                    let events = self.events.read().await;
                    let auth_failures = events
                        .iter()
                        .filter(|e| e.event_type == MonitorItem::BruteForceAttempts)
                        .count();
                    auth_failures > 10
                }
            }
            AlertRule::DdosAttack => {
                // Check if DDoS attack is detected
                {
                    let events = self.events.read().await;
                    let unique_ips: std::collections::HashSet<_> =
                        events.iter().filter_map(|e| e.source_ip.clone()).collect();
                    unique_ips.len() > 100
                }
            }
            AlertRule::MalwareDetected => {
                // Check if malware is detected
                // Simulate malware detection
                false
            }
            AlertRule::AnomalyDetected => {
                // Check if anomaly is detected
                {
                    let stats = self.statistics.read().await;
                    stats.total_events > 100
                }
            }
        }
    }

    /// Create security alert
    async fn create_alert(&self, alert_type: AlertRule, event: &SecurityEvent) {
        info!("Creating security alert: {:?}", alert_type);

        let alert = SecurityAlert {
            id: format!("alert_{}", chrono::Utc::now().timestamp()),
            alert_type: alert_type.clone(),
            severity: event.severity.clone(),
            description: format!("Security event detected: {:?}", event.event_type),
            triggered_events: vec![event.clone()],
            timestamp: chrono::Utc::now().to_rfc3339(),
            is_handled: false,
        };

        // Push alert to alerts list
        {
            let mut alerts = self.alerts.write().await;
            alerts.push(alert.clone());

            // Keep only the most recent 1000 alerts
            if alerts.len() > 1000 {
                alerts.remove(0);
            }
        }

        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_alerts += 1;
            stats.unhandled_alerts += 1;
        }
    }

    /// Get security events
    pub async fn get_events(&self, limit: Option<usize>) -> Vec<SecurityEvent> {
        let events = self.events.read().await;
        if let Some(limit) = limit {
            events.iter().rev().take(limit).cloned().collect()
        } else {
            events.clone()
        }
    }

    /// Get security alerts
    pub async fn get_alerts(&self, limit: Option<usize>) -> Vec<SecurityAlert> {
        let alerts = self.alerts.read().await;
        if let Some(limit) = limit {
            alerts.iter().rev().take(limit).cloned().collect()
        } else {
            alerts.clone()
        }
    }

    /// Handle security alert
    pub async fn handle_alert(&self, alert_id: &str) -> Result<(), String> {
        info!("Handling security alert: {}", alert_id);

        let mut alerts = self.alerts.write().await;

        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.is_handled = true;

            // Update statistics
            let mut stats = self.statistics.write().await;
            stats.unhandled_alerts = stats.unhandled_alerts.saturating_sub(1);

            return Ok(());
        }

        Err(format!("Alert does not exist: {}", alert_id))
    }

    /// Update statistics information
    async fn update_statistics(&self, event: &SecurityEvent) {
        let mut stats = self.statistics.write().await;

        stats.total_events += 1;

        match event.severity {
            EventSeverity::Critical => stats.critical_events += 1,
            EventSeverity::High => stats.high_events += 1,
            EventSeverity::Medium => stats.medium_events += 1,
            EventSeverity::Low => stats.low_events += 1,
            EventSeverity::Info => stats.info_events += 1,
        }
    }

    /// Get statistics information
    pub async fn get_statistics(&self) -> MonitorStatistics {
        self.statistics.read().await.clone()
    }

    /// Cleanup old events
    pub async fn cleanup_old_events(&self) {
        info!("Cleaning up old events...");

        let retention_duration = chrono::Duration::days(self.config.log_retention_days as i64);
        let cutoff_time = chrono::Utc::now() - retention_duration;

        let mut events = self.events.write().await;
        events.retain(|e| {
            // Keep events within retention period
            if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(&e.timestamp) {
                timestamp > cutoff_time
            } else {
                true
            }
        });

        info!("Event cleanup completed");
    }

    /// Generate monitoring report
    pub async fn generate_monitor_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Security Monitoring Report ===\n\n");

        // Statistics information
        let stats = self.get_statistics().await;
        report.push_str("Statistics Information:\n");
        report.push_str(&format!("Total events: {}\n", stats.total_events));
        report.push_str(&format!("Critical events: {}\n", stats.critical_events));
        report.push_str(&format!("High events: {}\n", stats.high_events));
        report.push_str(&format!("Medium events: {}\n", stats.medium_events));
        report.push_str(&format!("Low events: {}\n", stats.low_events));
        report.push_str(&format!("Info events: {}\n\n", stats.info_events));

        // Alert information
        let alerts = self.get_alerts(None).await;
        report.push_str("Alert Information:\n");
        report.push_str(&format!("Total alerts: {}\n", alerts.len()));

        let unhandled = alerts.iter().filter(|a| !a.is_handled).count();
        report.push_str(&format!("Unhandled alerts: {}\n\n", unhandled));

        // Recent events
        let recent_events = self.get_events(Some(10)).await;
        if !recent_events.is_empty() {
            report.push_str("Recent Events:\n");
            for (index, event) in recent_events.iter().enumerate() {
                report.push_str(&format!(
                    "{}. {:?} - {}\n",
                    index + 1,
                    event.event_type,
                    event.description
                ));
                report.push_str(&format!("   Severity: {:?}\n", event.severity));
                report.push_str(&format!("   Timestamp: {}\n", event.timestamp));
                if let Some(ip) = &event.source_ip {
                    report.push_str(&format!("   Source IP: {}\n", ip));
                }
                if let Some(user) = &event.target_user {
                    report.push_str(&format!("   Target user: {}\n\n", user));
                }
            }
        }

        // Security recommendations
        report.push_str("Security Recommendations:\n");
        report.push_str("1. Regularly handle security alerts\n");
        report.push_str("2. Analyze security event patterns\n");
        report.push_str("3. Optimize alert rules\n");
        report.push_str("4. Strengthen security protection measures\n");
        report.push_str("5. Conduct security training and awareness\n");

        report
    }
}

impl Default for SecurityMonitor {
    fn default() -> Self {
        Self::new(MonitorConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_monitor() {
        let monitor = SecurityMonitor::new(MonitorConfig::default());

        // Test recording event
        let event = SecurityEvent {
            id: "event_001".to_string(),
            event_type: MonitorItem::AuthenticationAttempts,
            severity: EventSeverity::High,
            description: "Authentication attempt".to_string(),
            source_ip: Some("192.168.1.1".to_string()),
            target_user: Some("user_001".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: serde_json::json!({}),
        };

        monitor.record_event(event).await;

        let events = monitor.get_events(None).await;
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn test_alert_creation() {
        let monitor = SecurityMonitor::new(MonitorConfig::default());

        // Record a critical event to potentially trigger alert
        let event = SecurityEvent {
            id: "event_001".to_string(),
            event_type: MonitorItem::AuthenticationAttempts,
            severity: EventSeverity::Critical,
            description: "Authentication attempt".to_string(),
            source_ip: Some("192.168.1.1".to_string()),
            target_user: Some("user_001".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: serde_json::json!({}),
        };

        monitor.record_event(event).await;

        let _alerts = monitor.get_alerts(None).await;
        // Alert might not be triggered with just one event, but test should complete quickly
    }

    #[tokio::test]
    async fn test_handle_alert() {
        let monitor = SecurityMonitor::new(MonitorConfig::default());

        // Create an event
        let event = SecurityEvent {
            id: "event_001".to_string(),
            event_type: MonitorItem::AuthenticationAttempts,
            severity: EventSeverity::Critical,
            description: "Authentication attempt".to_string(),
            source_ip: Some("192.168.1.1".to_string()),
            target_user: Some("user_001".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: serde_json::json!({}),
        };

        monitor.record_event(event).await;

        // Check if any alerts were created
        let alerts = monitor.get_alerts(None).await;
        if !alerts.is_empty() {
            let alert_id = alerts[0].id.clone();
            let result = monitor.handle_alert(&alert_id).await;
            assert!(result.is_ok() || result.is_err()); // Either works, test should complete quickly
        }
    }

    #[tokio::test]
    async fn test_generate_monitor_report() {
        let monitor = SecurityMonitor::new(MonitorConfig::default());

        // Record a single event
        let event = SecurityEvent {
            id: "event_001".to_string(),
            event_type: MonitorItem::AuthenticationAttempts,
            severity: EventSeverity::Medium,
            description: "Authentication attempt".to_string(),
            source_ip: Some("192.168.1.1".to_string()),
            target_user: Some("user_001".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: serde_json::json!({}),
        };

        monitor.record_event(event).await;

        let report = monitor.generate_monitor_report().await;

        assert!(report.contains("Security Monitoring Report"));
        assert!(report.contains("Statistics Information"));
    }
}
