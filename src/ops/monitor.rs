use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Monitor configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Check interval (milliseconds)
    pub check_interval: u64,
    /// CPU usage threshold (%)
    pub cpu_threshold: f64,
    /// Memory usage threshold (%)
    pub memory_threshold: f64,
    /// Disk usage threshold (%)
    pub disk_threshold: f64,
    /// Network traffic threshold (bytes/second)
    pub network_threshold: u64,
    /// Request threshold (requests/minute)
    pub request_threshold: u32,
    /// Response time threshold (ms)
    pub response_time_threshold: u32,
    /// Error rate threshold (errors/minute)
    pub error_threshold: u32,
    /// CS response delay threshold (ms)
    pub cs_response_delay_threshold: u32,
    /// IM message delay threshold (ms)
    pub im_message_delay_threshold: u32,
    /// Database connections threshold
    pub database_connections_threshold: u32,
    /// Cache hit rate threshold (%)
    pub cache_hit_rate_threshold: f64,
    /// Enable popup notification
    pub enable_popup: bool,
    /// Enable error log
    pub enable_error_log: bool,
    /// Enable /monitor API
    pub enable_monitor_api: bool,
    /// Enable email notifications
    pub enable_email_notifications: bool,
    /// Email recipients
    pub email_recipients: Vec<String>,
}

/// Monitor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorData {
    /// Timestamp (seconds since epoch)
    pub timestamp: u64,
    /// CPU usage (%)
    pub cpu_usage: f64,
    /// CPU usage per core (%)
    pub cpu_usage_per_core: Vec<f64>,
    /// Memory usage (%)
    pub memory_usage: f64,
    /// Memory usage details (bytes)
    pub memory_details: MemoryDetails,
    /// Disk usage (%)
    pub disk_usage: f64,
    /// Disk usage per partition (%)
    pub disk_usage_per_partition: Vec<DiskPartitionUsage>,
    /// Network RX bytes per second
    pub network_rx_bytes: u64,
    /// Network TX bytes per second
    pub network_tx_bytes: u64,
    /// Network packets per second
    pub network_packets_per_second: (u64, u64), // (RX, TX)
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time (ms)
    pub avg_response_time: u32,
    /// Response time percentiles (ms)
    pub response_time_percentiles: ResponseTimePercentiles,
    /// 5xx errors per minute
    pub errors_per_minute: u32,
    /// Error rates by HTTP status code
    pub error_rates_by_status: Vec<ErrorRateByStatus>,
    /// CS online results
    pub cs_online_consults: u32,
    /// CS average response delay (ms)
    pub cs_avg_response_delay: u32,
    /// IM online users
    pub im_online_users: u32,
    /// IM average message delay (ms)
    pub im_avg_message_delay: u32,
    /// Active plugins
    pub active_plugins: u32,
    /// Plugin status details
    pub plugin_status_details: Vec<PluginStatusDetail>,
    /// Database connections
    pub database_connections: u32,
    /// Database connection details
    pub database_connection_details: DatabaseConnectionDetails,
    /// Cache hit rate (%)
    pub cache_hit_rate: f64,
    /// Cache details
    pub cache_details: CacheDetails,
    /// System load average
    pub system_load_average: (f64, f64, f64), // 1min, 5min, 15min
    /// Uptime (seconds)
    pub uptime: u64,
    /// Thread count
    pub thread_count: u32,
    /// Goroutine count (if applicable)
    pub goroutine_count: Option<u32>,
    /// Alert list
    pub alerts: Vec<String>,
}

/// Memory details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDetails {
    /// Total memory (bytes)
    pub total: u64,
    /// Used memory (bytes)
    pub used: u64,
    /// Free memory (bytes)
    pub free: u64,
    /// Buffers (bytes)
    pub buffers: u64,
    /// Cached (bytes)
    pub cached: u64,
}

/// Disk partition usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskPartitionUsage {
    /// Partition path
    pub path: String,
    /// Usage (%)
    pub usage: f64,
    /// Total space (bytes)
    pub total: u64,
    /// Used space (bytes)
    pub used: u64,
    /// Free space (bytes)
    pub free: u64,
}

/// Response time percentiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimePercentiles {
    /// 50th percentile (ms)
    pub p50: u32,
    /// 90th percentile (ms)
    pub p90: u32,
    /// 95th percentile (ms)
    pub p95: u32,
    /// 99th percentile (ms)
    pub p99: u32,
}

/// Error rate by HTTP status code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateByStatus {
    /// HTTP status code
    pub status_code: u16,
    /// Count per minute
    pub count_per_minute: u32,
    /// Percentage of total requests
    pub percentage: f64,
}

/// Plugin status detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStatusDetail {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin status
    pub status: String,
    /// Plugin memory usage (bytes)
    pub memory_usage: Option<u64>,
    /// Plugin request count
    pub request_count: u32,
}

/// Database connection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionDetails {
    /// Connection pool size
    pub pool_size: u32,
    /// Active connections
    pub active: u32,
    /// Idle connections
    pub idle: u32,
    /// Connection wait time (ms)
    pub wait_time: u32,
    /// Connection errors per minute
    pub errors_per_minute: u32,
}

/// Cache details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDetails {
    /// Cache size
    pub size: u64,
    /// Cache capacity
    pub capacity: u64,
    /// Hit count
    pub hit_count: u64,
    /// Miss count
    pub miss_count: u64,
    /// Eviction count
    pub eviction_count: u64,
    /// Average get time (ms)
    pub avg_get_time: f64,
}

/// Alert type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// CPU usage high
    CpuHigh,
    /// Memory usage high
    MemoryHigh,
    /// Disk usage high
    DiskHigh,
    /// Network traffic high
    NetworkHigh,
    /// Request count high
    RequestsHigh,
    /// Response time high
    ResponseTimeHigh,
    /// Error rate high
    ErrorsHigh,
    /// CS response delay high
    CsResponseDelayHigh,
    /// IM message delay high
    ImMessageDelayHigh,
    /// Database connections high
    DatabaseConnectionsHigh,
    /// Cache hit rate low
    CacheHitRateLow,
    /// Other
    Other,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    /// Alert type
    pub alert_type: AlertType,
    /// Alert description
    pub description: String,
    /// Timestamp (seconds since epoch)
    pub timestamp: u64,
    /// Is handled
    pub is_handled: bool,
}

/// Monitor service
#[derive(Debug, Clone)]
pub struct MonitorService {
    /// Configuration
    pub config: MonitorConfig,
    /// Current monitor data
    pub current_data: Arc<RwLock<MonitorData>>,
    /// Alert list
    pub alerts: Arc<RwLock<Vec<AlertInfo>>>,
    /// Is running
    pub is_running: Arc<RwLock<bool>>,
    /// Monitor task handle
    pub monitor_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl MonitorService {
    /// Create new monitor service
    pub fn new(config: MonitorConfig) -> Self {
        let current_data = MonitorData {
            timestamp: 0,
            cpu_usage: 0.0,
            cpu_usage_per_core: Vec::new(),
            memory_usage: 0.0,
            memory_details: MemoryDetails {
                total: 0,
                used: 0,
                free: 0,
                buffers: 0,
                cached: 0,
            },
            disk_usage: 0.0,
            disk_usage_per_partition: Vec::new(),
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            network_packets_per_second: (0, 0),
            requests_per_minute: 0,
            requests_per_second: 0.0,
            avg_response_time: 0,
            response_time_percentiles: ResponseTimePercentiles {
                p50: 0,
                p90: 0,
                p95: 0,
                p99: 0,
            },
            errors_per_minute: 0,
            error_rates_by_status: Vec::new(),
            cs_online_consults: 0,
            cs_avg_response_delay: 0,
            im_online_users: 0,
            im_avg_message_delay: 0,
            active_plugins: 0,
            plugin_status_details: Vec::new(),
            database_connections: 0,
            database_connection_details: DatabaseConnectionDetails {
                pool_size: 0,
                active: 0,
                idle: 0,
                wait_time: 0,
                errors_per_minute: 0,
            },
            cache_hit_rate: 0.0,
            cache_details: CacheDetails {
                size: 0,
                capacity: 0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
                avg_get_time: 0.0,
            },
            system_load_average: (0.0, 0.0, 0.0),
            uptime: 0,
            thread_count: 0,
            goroutine_count: None,
            alerts: Vec::new(),
        };

        Self {
            config,
            current_data: Arc::new(RwLock::new(current_data)),
            alerts: Arc::new(RwLock::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
            monitor_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Start monitor service
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            info!("Monitor service is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!(
            "Starting monitor service, check interval: {}ms",
            self.config.check_interval
        );

        // Start monitor task
        let service_clone = self.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = service_clone.run_monitor_task().await {
                error!("Monitor task error: {:?}", e);
            }
        });

        let mut monitor_task = self.monitor_task.write().await;
        *monitor_task = Some(handle);
        drop(monitor_task);

        Ok(())
    }

    /// Stop monitor service
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        // Cancel monitor task
        let mut monitor_task = self.monitor_task.write().await;
        if let Some(handle) = monitor_task.take() {
            handle.abort();
        }
        drop(monitor_task);

        info!("Monitor service stopped");
        Ok(())
    }

    /// Run monitor task
    async fn run_monitor_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        let interval = Duration::from_secs(self.config.check_interval);

        loop {
            sleep(interval).await;

            let is_running = *self.is_running.read().await;
            if !is_running {
                break;
            }

            // Collect monitor data
            let monitor_data = self.collect_monitor_data().await;

            // Save monitor data
            let mut current_data = self.current_data.write().await;
            *current_data = monitor_data.clone();
            drop(current_data);

            // Check alerts
            self.check_alerts(&monitor_data).await;
        }

        Ok(())
    }

    /// Collect monitor data
    async fn collect_monitor_data(&self) -> MonitorData {
        // Get timestamp (seconds since epoch)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        // Simulate getting monitor data from monitoring API
        let cpu_usage = self.get_cpu_usage().await;
        let cpu_usage_per_core = self.get_cpu_usage_per_core().await;
        let memory_usage = self.get_memory_usage().await;
        let memory_details = self.get_memory_details().await;
        let disk_usage = self.get_disk_usage().await;
        let disk_usage_per_partition = self.get_disk_usage_per_partition().await;
        let (network_rx_bytes, network_tx_bytes) = self.get_network_traffic().await;
        let network_packets_per_second = self.get_network_packets_per_second().await;
        let requests_per_minute = self.get_requests_per_minute().await;
        let requests_per_second = self.get_requests_per_second().await;
        let avg_response_time = self.get_avg_response_time().await;
        let response_time_percentiles = self.get_response_time_percentiles().await;
        let errors_per_minute = self.get_errors_per_minute().await;
        let error_rates_by_status = self.get_error_rates_by_status().await;
        let cs_online_consults = self.get_cs_online_consults().await;
        let cs_avg_response_delay = self.get_cs_avg_response_delay().await;
        let im_online_users = self.get_im_online_users().await;
        let im_avg_message_delay = self.get_im_avg_message_delay().await;
        let active_plugins = self.get_active_plugins().await;
        let plugin_status_details = self.get_plugin_status_details().await;
        let database_connections = self.get_database_connections().await;
        let database_connection_details = self.get_database_connection_details().await;
        let cache_hit_rate = self.get_cache_hit_rate().await;
        let cache_details = self.get_cache_details().await;
        let system_load_average = self.get_system_load_average().await;
        let uptime = self.get_uptime().await;
        let thread_count = self.get_thread_count().await;
        let goroutine_count = self.get_goroutine_count().await;

        MonitorData {
            timestamp,
            cpu_usage,
            cpu_usage_per_core,
            memory_usage,
            memory_details,
            disk_usage,
            disk_usage_per_partition,
            network_rx_bytes,
            network_tx_bytes,
            network_packets_per_second,
            requests_per_minute,
            requests_per_second,
            avg_response_time,
            response_time_percentiles,
            errors_per_minute,
            error_rates_by_status,
            cs_online_consults,
            cs_avg_response_delay,
            im_online_users,
            im_avg_message_delay,
            active_plugins,
            plugin_status_details,
            database_connections,
            database_connection_details,
            cache_hit_rate,
            cache_details,
            system_load_average,
            uptime,
            thread_count,
            goroutine_count,
            alerts: Vec::new(),
        }
    }

    /// Get CPU usage
    async fn get_cpu_usage(&self) -> f64 {
        // Get CPU usage from system monitoring API
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        let cpus = sys.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        let total_cpu_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();

        total_cpu_usage as f64 / cpus.len() as f64
    }

    /// Get memory usage
    async fn get_memory_usage(&self) -> f64 {
        // Get memory usage from system monitoring API
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();

        (used_memory as f64 / total_memory as f64) * 100.0
    }

    /// Get requests per minute
    async fn get_requests_per_minute(&self) -> u32 {
        // Simulate getting request count from monitoring API
        // In actual project, call system monitoring API to get request count
        500
    }

    /// Get average response time
    async fn get_avg_response_time(&self) -> u32 {
        // Simulate getting average response time from monitoring API
        // In actual project, call system monitoring API to get response time
        50
    }

    /// Get CS online results
    async fn get_cs_online_consults(&self) -> u32 {
        // Simulate getting CS online results from monitoring API
        // In actual project, call system monitoring API to get CS online results
        10
    }

    /// Get CS average response delay
    async fn get_cs_avg_response_delay(&self) -> u32 {
        // Simulate getting CS average response delay from monitoring API
        // In actual project, call system monitoring API to get CS response delay
        800
    }

    /// Get IM online users
    async fn get_im_online_users(&self) -> u32 {
        // Simulate getting IM online users from monitoring API
        // In actual project, call system monitoring API to get IM online users
        100
    }

    /// Get IM average message delay
    async fn get_im_avg_message_delay(&self) -> u32 {
        // Simulate getting IM average message delay from monitoring API
        // In actual project, call system monitoring API to get IM message delay
        500
    }

    /// Get disk usage
    async fn get_disk_usage(&self) -> f64 {
        // Get disk usage from system monitoring API
        // Simulate disk usage for now
        45.0 // 45% disk usage
    }

    /// Get network traffic
    async fn get_network_traffic(&self) -> (u64, u64) {
        // Simulate getting network traffic from monitoring API
        // In actual project, call system monitoring API to get network traffic
        (1024 * 1024, 512 * 1024) // 1MB/s RX, 512KB/s TX
    }

    /// Get errors per minute
    async fn get_errors_per_minute(&self) -> u32 {
        // Simulate getting errors per minute from monitoring API
        // In actual project, call system monitoring API to get error count
        5
    }

    /// Get active plugins
    async fn get_active_plugins(&self) -> u32 {
        // Simulate getting active plugins from monitoring API
        // In actual project, call plugin system API to get active plugins
        3
    }

    /// Get database connections
    async fn get_database_connections(&self) -> u32 {
        // Simulate getting database connections from monitoring API
        // In actual project, call database pool API to get connection count
        10
    }

    /// Get cache hit rate
    async fn get_cache_hit_rate(&self) -> f64 {
        // Simulate getting cache hit rate from monitoring API
        // In actual project, call cache system API to get hit rate
        95.0
    }

    /// Get CPU usage per core
    async fn get_cpu_usage_per_core(&self) -> Vec<f64> {
        // Get CPU usage per core from system monitoring API
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        let cpus = sys.cpus();
        cpus.iter().map(|cpu| cpu.cpu_usage() as f64).collect()
    }

    /// Get memory details
    async fn get_memory_details(&self) -> MemoryDetails {
        // Get memory details from system monitoring API
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total = sys.total_memory();
        let used = sys.used_memory();
        let free = total - used;

        MemoryDetails {
            total,
            used,
            free,
            buffers: 0, // Simulated value
            cached: 0,  // Simulated value
        }
    }

    /// Get disk usage per partition
    async fn get_disk_usage_per_partition(&self) -> Vec<DiskPartitionUsage> {
        // Simulate getting disk usage per partition from monitoring API
        vec![
            DiskPartitionUsage {
                path: "/".to_string(),
                usage: 45.0,
                total: 100 * 1024 * 1024 * 1024, // 100GB
                used: 45 * 1024 * 1024 * 1024,   // 45GB
                free: 55 * 1024 * 1024 * 1024,   // 55GB
            },
            DiskPartitionUsage {
                path: "/data".to_string(),
                usage: 60.0,
                total: 500 * 1024 * 1024 * 1024, // 500GB
                used: 300 * 1024 * 1024 * 1024,  // 300GB
                free: 200 * 1024 * 1024 * 1024,  // 200GB
            },
        ]
    }

    /// Get network packets per second
    async fn get_network_packets_per_second(&self) -> (u64, u64) {
        // Simulate getting network packets per second from monitoring API
        (1000, 500) // 1000 RX packets/s, 500 TX packets/s
    }

    /// Get requests per second
    async fn get_requests_per_second(&self) -> f64 {
        // Simulate getting requests per second from monitoring API
        8.3 // Approximately 500 requests per minute
    }

    /// Get response time percentiles
    async fn get_response_time_percentiles(&self) -> ResponseTimePercentiles {
        // Simulate getting response time percentiles from monitoring API
        ResponseTimePercentiles {
            p50: 30,
            p90: 80,
            p95: 120,
            p99: 200,
        }
    }

    /// Get error rates by HTTP status code
    async fn get_error_rates_by_status(&self) -> Vec<ErrorRateByStatus> {
        // Simulate getting error rates by status code from monitoring API
        vec![
            ErrorRateByStatus {
                status_code: 404,
                count_per_minute: 10,
                percentage: 2.0,
            },
            ErrorRateByStatus {
                status_code: 500,
                count_per_minute: 5,
                percentage: 1.0,
            },
        ]
    }

    /// Get plugin status details
    async fn get_plugin_status_details(&self) -> Vec<PluginStatusDetail> {
        // Simulate getting plugin status details from monitoring API
        vec![
            PluginStatusDetail {
                name: "analytics".to_string(),
                version: "1.0.0".to_string(),
                status: "enabled".to_string(),
                memory_usage: Some(10 * 1024 * 1024), // 10MB
                request_count: 100,
            },
            PluginStatusDetail {
                name: "auth".to_string(),
                version: "1.0.0".to_string(),
                status: "enabled".to_string(),
                memory_usage: Some(5 * 1024 * 1024), // 5MB
                request_count: 200,
            },
            PluginStatusDetail {
                name: "cache".to_string(),
                version: "1.0.0".to_string(),
                status: "enabled".to_string(),
                memory_usage: Some(15 * 1024 * 1024), // 15MB
                request_count: 150,
            },
        ]
    }

    /// Get database connection details
    async fn get_database_connection_details(&self) -> DatabaseConnectionDetails {
        // Simulate getting database connection details from monitoring API
        DatabaseConnectionDetails {
            pool_size: 50,
            active: 10,
            idle: 20,
            wait_time: 5,
            errors_per_minute: 0,
        }
    }

    /// Get cache details
    async fn get_cache_details(&self) -> CacheDetails {
        // Simulate getting cache details from monitoring API
        CacheDetails {
            size: 10000,
            capacity: 20000,
            hit_count: 95000,
            miss_count: 5000,
            eviction_count: 1000,
            avg_get_time: 1.5,
        }
    }

    /// Get system load average
    async fn get_system_load_average(&self) -> (f64, f64, f64) {
        // Simulate getting system load average from monitoring API
        (0.5, 0.6, 0.7) // 1min, 5min, 15min
    }

    /// Get uptime
    async fn get_uptime(&self) -> u64 {
        // Simulate getting uptime from monitoring API
        86400 // 1 day
    }

    /// Get thread count
    async fn get_thread_count(&self) -> u32 {
        // Simulate getting thread count from monitoring API
        50
    }

    /// Get goroutine count (if applicable)
    async fn get_goroutine_count(&self) -> Option<u32> {
        // Simulate getting goroutine count from monitoring API (if applicable)
        None // Not applicable in Rust
    }

    /// Check alerts
    async fn check_alerts(&self, data: &MonitorData) {
        let mut alerts = Vec::new();

        // Check CPU usage high
        if data.cpu_usage > self.config.cpu_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::CpuHigh,
                description: format!(
                    "CPU usage high: {:.2}% (threshold: {:.2}%)",
                    data.cpu_usage, self.config.cpu_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check memory usage high
        if data.memory_usage > self.config.memory_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::MemoryHigh,
                description: format!(
                    "Memory usage high: {:.2}% (threshold: {:.2}%)",
                    data.memory_usage, self.config.memory_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check disk usage high
        if data.disk_usage > self.config.disk_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::DiskHigh,
                description: format!(
                    "Disk usage high: {:.2}% (threshold: {:.2}%)",
                    data.disk_usage, self.config.disk_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check network traffic high
        if data.network_rx_bytes > self.config.network_threshold
            || data.network_tx_bytes > self.config.network_threshold
        {
            let alert = AlertInfo {
                alert_type: AlertType::NetworkHigh,
                description: format!(
                    "Network traffic high: RX {} bytes/s, TX {} bytes/s (threshold: {} bytes/s)",
                    data.network_rx_bytes, data.network_tx_bytes, self.config.network_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check request count high
        if data.requests_per_minute > self.config.request_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::RequestsHigh,
                description: format!(
                    "Request count high: {} (threshold: {})",
                    data.requests_per_minute, self.config.request_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check response time high
        if data.avg_response_time > self.config.response_time_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::ResponseTimeHigh,
                description: format!(
                    "Response time high: {}ms (threshold: {}ms)",
                    data.avg_response_time, self.config.response_time_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check error rate high
        if data.errors_per_minute > self.config.error_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::ErrorsHigh,
                description: format!(
                    "Error rate high: {} errors/minute (threshold: {} errors/minute)",
                    data.errors_per_minute, self.config.error_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check CS response delay high
        if data.cs_avg_response_delay > self.config.cs_response_delay_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::CsResponseDelayHigh,
                description: format!(
                    "CS response delay high: {}ms (threshold: {}ms)",
                    data.cs_avg_response_delay, self.config.cs_response_delay_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check IM message delay high
        if data.im_avg_message_delay > self.config.im_message_delay_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::ImMessageDelayHigh,
                description: format!(
                    "IM message delay high: {}ms (threshold: {}ms)",
                    data.im_avg_message_delay, self.config.im_message_delay_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check database connections high
        if data.database_connections > self.config.database_connections_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::DatabaseConnectionsHigh,
                description: format!(
                    "Database connections high: {} (threshold: {})",
                    data.database_connections, self.config.database_connections_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Check cache hit rate low
        if data.cache_hit_rate < self.config.cache_hit_rate_threshold {
            let alert = AlertInfo {
                alert_type: AlertType::CacheHitRateLow,
                description: format!(
                    "Cache hit rate low: {:.2}% (threshold: {:.2}%)",
                    data.cache_hit_rate, self.config.cache_hit_rate_threshold
                ),
                timestamp: data.timestamp,
                is_handled: false,
            };
            alerts.push(alert);
        }

        // Handle alerts
        for alert in alerts {
            self.handle_alert(&alert).await;
        }
    }

    /// Handle alert
    async fn handle_alert(&self, alert: &AlertInfo) {
        // Save alert to alert list
        let mut alerts = self.alerts.write().await;
        alerts.push(alert.clone());

        // Keep only last 100 alerts
        if alerts.len() > 100 {
            let drain_count = alerts.len() - 100;
            alerts.drain(0..drain_count);
        }
        drop(alerts);

        // Show popup notification
        if self.config.enable_popup {
            self.show_popup(alert).await;
        }

        // Error log
        if self.config.enable_error_log {
            self.log_error(alert).await;
        }

        // Send email notification
        if self.config.enable_email_notifications && !self.config.email_recipients.is_empty() {
            self.send_email_notification(alert).await;
        }

        info!("Alert triggered: {}", alert.description);
    }

    /// Send email notification
    async fn send_email_notification(&self, alert: &AlertInfo) {
        // Simulate sending email notification
        // In actual project, call email service API to send alert email
        info!(
            "Sending email notification to {:?}: {}",
            self.config.email_recipients, alert.description
        );
        // For demonstration purposes, we'll just log the email sending
        // In a real implementation, you would use an email library like lettre
    }

    /// Show popup notification
    async fn show_popup(&self, alert: &AlertInfo) {
        // Simulate showing popup notification
        // In actual project, call system popup API to show alert
        warn!("Alert popup: {}", alert.description);
    }

    /// Log error
    async fn log_error(&self, alert: &AlertInfo) {
        error!("Alert error: {}", alert.description);
    }

    /// Get current monitor data
    pub async fn get_current_data(&self) -> MonitorData {
        let current_data = self.current_data.read().await;
        current_data.clone()
    }

    /// Get alert list
    pub async fn get_alerts(&self) -> Vec<AlertInfo> {
        let alerts = self.alerts.read().await;
        alerts.clone()
    }

    /// Mark alert as handled
    pub async fn mark_alert_as_handled(&self, index: usize) -> bool {
        let mut alerts = self.alerts.write().await;
        if index < alerts.len() {
            alerts[index].is_handled = true;
            true
        } else {
            false
        }
    }

    /// Clear handled alerts
    pub async fn clear_handled_alerts(&self) {
        let mut alerts = self.alerts.write().await;
        alerts.retain(|alert| !alert.is_handled);
    }
}

/// Monitor service tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_monitor_service() {
        // Create test configuration
        let config = MonitorConfig {
            enabled: true,
            check_interval: 1,
            cpu_threshold: 80.0,
            memory_threshold: 80.0,
            disk_threshold: 90.0,
            network_threshold: 1000000, // 1MB/s
            request_threshold: 1000,
            response_time_threshold: 500,
            error_threshold: 100,
            cs_response_delay_threshold: 1000,
            im_message_delay_threshold: 1000,
            database_connections_threshold: 100,
            cache_hit_rate_threshold: 70.0,
            enable_popup: false,
            enable_error_log: true,
            enable_monitor_api: true,
            enable_email_notifications: false,
            email_recipients: vec![],
        };

        // Create monitor service
        let monitor_service = MonitorService::new(config);

        // Start monitor service
        monitor_service.start().await.unwrap();

        // Wait for monitor to collect data
        sleep(Duration::from_millis(1500)).await;

        // Get monitor data
        let monitor_data = monitor_service.get_current_data().await;
        assert!(monitor_data.timestamp > 0);
        assert!(monitor_data.cpu_usage >= 0.0);
        assert!(monitor_data.memory_usage >= 0.0);

        // Get alerts
        let _alerts = monitor_service.get_alerts().await;
        // Change alert status to ensure compilation

        // Stop monitor service
        monitor_service.stop().await.unwrap();
    }
}
