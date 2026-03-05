//! Health check and auto-recovery module
//! Provides health check and automatic recovery mechanisms

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Health check error
#[derive(Error, Debug)]
pub enum HealthCheckError {
    #[error("Health check failed: {reason}")]
    CheckFailed { reason: String },

    #[error("Health check timed out")]
    Timeout,

    #[error("Health check not configured")]
    NotConfigured,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded (partially functional)
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service is unknown (not checked yet)
    Unknown,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health check
    pub enabled: bool,
    /// Check interval (seconds)
    pub check_interval: u64,
    /// Timeout for health check (seconds)
    pub timeout: u64,
    /// Number of consecutive failures before marking as unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before marking as healthy
    pub success_threshold: u32,
    /// Enable auto-recovery
    pub auto_recovery: bool,
    /// Recovery action (restart, restart_all, notify)
    pub recovery_action: RecoveryAction,
}

/// Recovery action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryAction {
    /// Restart service
    Restart,
    /// Restart all services
    RestartAll,
    /// Send notification only
    Notify,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: 30,
            timeout: 10,
            failure_threshold: 3,
            success_threshold: 2,
            auto_recovery: true,
            recovery_action: RecoveryAction::Restart,
        }
    }
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Service name
    pub service_name: String,
    /// Health status
    pub status: HealthStatus,
    /// Timestamp
    pub timestamp: u64,
    /// Response time (milliseconds)
    pub response_time_ms: u64,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Health check function type
pub type HealthCheckFn = Arc<
    dyn Fn() -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<HealthCheckResult, HealthCheckError>>
                    + Send,
            >,
        > + Send
        + Sync,
>;

/// Health checker
#[derive(Clone)]
pub struct HealthChecker {
    /// Configuration
    pub config: HealthCheckConfig,
    /// Current health status
    pub current_status: Arc<RwLock<HealthStatus>>,
    /// Consecutive failure count
    pub failure_count: Arc<RwLock<u32>>,
    /// Consecutive success count
    pub success_count: Arc<RwLock<u32>>,
    /// Health check functions
    pub health_checks: Arc<RwLock<Vec<(String, HealthCheckFn)>>>,
    /// Is running
    pub is_running: Arc<RwLock<bool>>,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            current_status: Arc::new(RwLock::new(HealthStatus::Unknown)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            health_checks: Arc::new(RwLock::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Create health checker builder (流畅配置API)
    pub fn builder() -> HealthCheckerBuilder {
        HealthCheckerBuilder::new()
    }

    /// Execute all health checks synchronously
    pub async fn run_all_checks_sync(&self) -> Vec<(String, HealthCheckResult)> {
        let health_checks = self.health_checks.read().await;
        let mut results = Vec::new();

        for (service_name, check_fn) in health_checks.iter() {
            if let Ok(result) = check_fn().await {
                results.push((service_name.clone(), result));
            }
        }

        results
    }

    /// Check if health checker is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get all service names
    pub async fn get_service_names(&self) -> Vec<String> {
        let health_checks = self.health_checks.read().await;
        health_checks.iter().map(|(name, _)| name.clone()).collect()
    }

    /// Get service health status by name
    pub async fn get_service_health(&self, service_name: &str) -> Option<HealthStatus> {
        let results = self.run_all_checks_sync().await;
        results
            .into_iter()
            .find(|(name, _)| name == service_name)
            .map(|(_, result)| result.status)
    }

    /// Export health status as JSON
    pub async fn export_health_status_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let health_status = self.get_health_status().await;
        let all_checks = self.run_all_checks_sync().await;

        let export_data = serde_json::json!({
            "overall_status": health_status,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            "service_statuses": all_checks.into_iter()
                .map(|(name, result)| {
                    serde_json::json!({"name": name, "status": result.status, "response_time_ms": result.response_time_ms})
                })
                .collect::<Vec<_>>(),
            "failure_count": self.get_failure_count().await,
            "success_count": self.get_success_count().await,
        });

        Ok(serde_json::to_string_pretty(&export_data)?)
    }

    /// Add health check function
    pub async fn add_health_check(&self, service_name: String, check_fn: HealthCheckFn) {
        let mut health_checks = self.health_checks.write().await;
        health_checks.push((service_name, check_fn));
    }

    /// Remove health check function
    pub async fn remove_health_check(&self, service_name: &str) {
        let mut health_checks = self.health_checks.write().await;
        health_checks.retain(|(name, _)| name != service_name);
    }

    /// Start health checker
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            info!("Health checker is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!(
            "Starting health checker, check interval: {}s",
            self.config.check_interval
        );

        // Start health check task
        let checker_clone = self.clone();
        tokio::spawn(async move {
            if let Err(e) = checker_clone.run_health_check_task().await {
                error!("Health check task error: {:?}", e);
            }
        });

        Ok(())
    }

    /// Stop health checker
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        info!("Health checker stopped");
        Ok(())
    }

    /// Run health check task
    async fn run_health_check_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        let interval = Duration::from_secs(self.config.check_interval);

        loop {
            sleep(interval).await;

            let is_running = *self.is_running.read().await;
            if !is_running {
                break;
            }

            // Run health checks
            let health_status = self.run_health_checks().await;

            // Update health status
            let mut current_status = self.current_status.write().await;
            *current_status = health_status.clone();
            drop(current_status);

            // Check if auto-recovery is needed
            if self.config.auto_recovery && health_status == HealthStatus::Unhealthy {
                self.trigger_recovery().await;
            }
        }

        Ok(())
    }

    /// Run health checks
    async fn run_health_checks(&self) -> HealthStatus {
        let health_checks = self.health_checks.read().await;

        if health_checks.is_empty() {
            return HealthStatus::Unknown;
        }

        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;

        for (service_name, check_fn) in health_checks.iter() {
            let start_time = Instant::now();
            let check_result = check_fn().await;
            let response_time = start_time.elapsed().as_millis() as u64;

            match check_result {
                Ok(result) => match result.status {
                    HealthStatus::Healthy => {
                        healthy_count += 1;
                        info!(
                            "Service '{}' is healthy (response time: {}ms)",
                            service_name, response_time
                        );
                    }
                    HealthStatus::Degraded => {
                        degraded_count += 1;
                        warn!(
                            "Service '{}' is degraded (response time: {}ms)",
                            service_name, response_time
                        );
                    }
                    HealthStatus::Unhealthy => {
                        unhealthy_count += 1;
                        error!(
                            "Service '{}' is unhealthy (response time: {}ms)",
                            service_name, response_time
                        );
                    }
                    HealthStatus::Unknown => {
                        warn!("Service '{}' status is unknown", service_name);
                    }
                },
                Err(e) => {
                    unhealthy_count += 1;
                    error!(
                        "Health check for service '{}' failed: {:?}",
                        service_name, e
                    );
                }
            }
        }

        // Determine overall health status
        if unhealthy_count > 0 {
            HealthStatus::Unhealthy
        } else if degraded_count > 0 {
            HealthStatus::Degraded
        } else if healthy_count > 0 {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unknown
        }
    }

    /// Trigger recovery action
    async fn trigger_recovery(&self) {
        let failure_count = *self.failure_count.read().await;

        if failure_count < self.config.failure_threshold {
            return;
        }

        warn!(
            "Triggering recovery action: {:?}",
            self.config.recovery_action
        );

        match self.config.recovery_action {
            RecoveryAction::Restart => {
                info!("Restarting service...");
                self.restart_service().await;
            }
            RecoveryAction::RestartAll => {
                info!("Restarting all services...");
                self.restart_all_services().await;
            }
            RecoveryAction::Notify => {
                info!("Sending recovery notification...");
                self.send_notification().await;
            }
        }
    }

    /// Restart service
    async fn restart_service(&self) {
        info!("Restarting service...");

        // Reset failure and success counts
        let mut failure_count = self.failure_count.write().await;
        *failure_count = 0;
        drop(failure_count);

        let mut success_count = self.success_count.write().await;
        *success_count = 0;
        drop(success_count);

        // TODO: Implement actual service restart logic
        // This depends on the specific service being monitored
    }

    /// Restart all services
    async fn restart_all_services(&self) {
        info!("Restarting all services...");

        // Reset failure and success counts
        let mut failure_count = self.failure_count.write().await;
        *failure_count = 0;
        drop(failure_count);

        let mut success_count = self.success_count.write().await;
        *success_count = 0;
        drop(success_count);

        // TODO: Implement actual service restart logic
        // This depends on the specific services being monitored
    }

    /// Send notification
    async fn send_notification(&self) {
        info!("Sending recovery notification...");

        // TODO: Implement actual notification logic
        // This could send email, SMS, or push notification
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> HealthStatus {
        let status = self.current_status.read().await;
        status.clone()
    }

    /// Get failure count
    pub async fn get_failure_count(&self) -> u32 {
        *self.failure_count.read().await
    }

    /// Get success count
    pub async fn get_success_count(&self) -> u32 {
        *self.success_count.read().await
    }
}

/// Health checker builder (流畅配置API)
#[derive(Debug, Clone)]
pub struct HealthCheckerBuilder {
    config: HealthCheckConfig,
}

impl HealthCheckerBuilder {
    /// Create a new health checker builder
    pub fn new() -> Self {
        Self {
            config: HealthCheckConfig::default(),
        }
    }

    /// Set check interval
    pub fn check_interval(mut self, interval: u64) -> Self {
        self.config.check_interval = interval;
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set failure threshold
    pub fn failure_threshold(mut self, threshold: u32) -> Self {
        self.config.failure_threshold = threshold;
        self
    }

    /// Set success threshold
    pub fn success_threshold(mut self, threshold: u32) -> Self {
        self.config.success_threshold = threshold;
        self
    }

    /// Enable auto-recovery
    pub fn enable_auto_recovery(mut self) -> Self {
        self.config.auto_recovery = true;
        self
    }

    /// Disable auto-recovery
    pub fn disable_auto_recovery(mut self) -> Self {
        self.config.auto_recovery = false;
        self
    }

    /// Set recovery action to restart
    pub fn restart_on_failure(mut self) -> Self {
        self.config.recovery_action = RecoveryAction::Restart;
        self
    }

    /// Set recovery action to restart all services
    pub fn restart_all_on_failure(mut self) -> Self {
        self.config.recovery_action = RecoveryAction::RestartAll;
        self
    }

    /// Set recovery action to notify only
    pub fn notify_on_failure(mut self) -> Self {
        self.config.recovery_action = RecoveryAction::Notify;
        self
    }

    /// Build health checker
    pub fn build(self) -> HealthChecker {
        HealthChecker::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker() {
        let mut config = HealthCheckConfig::default();
        config.check_interval = 0; // 立即执行健康检查
        let checker = HealthChecker::new(config);

        // Add a simple health check
        let check_fn: HealthCheckFn = Arc::new(|| {
            Box::pin(async {
                Ok(HealthCheckResult {
                    service_name: "test-service".to_string(),
                    status: HealthStatus::Healthy,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    response_time_ms: 10,
                    error: None,
                })
            })
        });

        checker
            .add_health_check("test-service".to_string(), check_fn)
            .await;

        // Start health checker
        checker.start().await.unwrap();

        // Wait a bit for health check to run
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Check health status
        let health_status = checker.get_health_status().await;
        assert_eq!(health_status, HealthStatus::Healthy);

        // Stop health checker
        checker.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_health_checker_unhealthy() {
        let mut config = HealthCheckConfig::default();
        config.failure_threshold = 2;
        config.check_interval = 1;
        let checker = HealthChecker::new(config);

        // Add a failing health check
        let check_fn: HealthCheckFn = Arc::new(|| {
            Box::pin(async {
                Ok(HealthCheckResult {
                    service_name: "test-service".to_string(),
                    status: HealthStatus::Unhealthy,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    response_time_ms: 1000,
                    error: Some("Service unavailable".to_string()),
                })
            })
        });

        checker
            .add_health_check("test-service".to_string(), check_fn)
            .await;

        // Start health checker
        checker.start().await.unwrap();

        // Wait for health checks to run
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Check health status
        let health_status = checker.get_health_status().await;
        assert_eq!(health_status, HealthStatus::Unhealthy);

        // Stop health checker
        checker.stop().await.unwrap();
    }
}
