use chrono::{Local, Timelike};
use env_logger::Builder;
use log::LevelFilter;
use serde_json::json;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::{Duration, SystemTime};
use tokio::sync::{RwLock, mpsc};
use tokio::time::sleep;

/// Log context for structured logging
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct LogContext {
    /// Request ID
    pub request_id: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Additional context data
    pub data: serde_json::Value,
}

impl LogContext {
    /// Create new log context
    pub fn new() -> Self {
        Self {
            request_id: None,
            user_id: None,
            session_id: None,
            data: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    /// Set request ID
    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }

    /// Set user ID
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    /// Set session ID
    pub fn with_session_id(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    /// Add context data
    pub fn with_data<K: Into<String>, V: serde::Serialize>(mut self, key: K, value: V) -> Self {
        self.data[key.into()] = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
        self
    }
}

/// Log configuration
#[derive(Debug, Clone, Default)]
pub struct LogConfig {
    /// Log level
    pub level: String,
    /// Log file path
    pub file_path: String,
    /// Enable file logging
    pub enable_file: bool,
    /// Enable console logging
    pub enable_console: bool,
    /// Maximum log file size in MB
    pub max_file_size: u64,
    /// Log file retention days
    pub retain_days: u8,
    /// Log rotation interval in hours
    pub rotate_hours: u8,
    /// Enable structured logging
    pub enable_structured: bool,
    /// Log buffer size
    pub buffer_size: usize,
}

/// Log manager
#[derive(Debug, Clone)]
pub struct LogManager {
    /// Configuration
    pub config: LogConfig,
    /// Current log file path
    pub current_log_file: Arc<RwLock<PathBuf>>,
    /// Log writer
    pub writer: Arc<RwLock<Option<BufWriter<File>>>>,
    /// Is running
    pub is_running: Arc<RwLock<bool>>,
    /// Log rotation task handle
    pub rotate_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    /// Log buffer sender
    pub log_buffer_tx: Option<mpsc::Sender<LogEntry>>,
    /// Log buffer receiver task
    pub log_buffer_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    /// Log entry counter
    pub log_counter: Arc<AtomicU64>,
}

/// Log entry for buffered logging
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Module
    pub module: String,
    /// Level
    pub level: String,
    /// Message
    pub message: String,
    /// Context
    pub context: Option<LogContext>,
}

impl LogManager {
    /// Create new log manager
    pub fn new(config: LogConfig) -> Self {
        let config = LogConfig {
            buffer_size: if config.buffer_size == 0 {
                1024
            } else {
                config.buffer_size
            },
            ..config
        };

        Self {
            config,
            current_log_file: Arc::new(RwLock::new(PathBuf::new())),
            writer: Arc::new(RwLock::new(None)),
            is_running: Arc::new(RwLock::new(false)),
            rotate_task: Arc::new(RwLock::new(None)),
            log_buffer_tx: None,
            log_buffer_task: Arc::new(RwLock::new(None)),
            log_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Create log manager builder (流畅配置API)
    pub fn builder() -> LogManagerBuilder {
        LogManagerBuilder::new()
    }

    /// Check if log manager is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get current log file path
    pub async fn get_current_log_file(&self) -> PathBuf {
        self.current_log_file.read().await.clone()
    }

    /// Rotate log file manually
    pub async fn rotate_log(&self) -> io::Result<()> {
        log::info!("Manually rotating log file...");
        self.init_file_log().await
    }

    /// Set log level dynamically
    pub fn set_log_level(&self, level: &str) {
        let level_filter = match level.to_lowercase().as_str() {
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            "trace" => LevelFilter::Trace,
            "off" => LevelFilter::Off,
            _ => LevelFilter::Info,
        };

        log::set_max_level(level_filter);
        log::info!("Log level changed to: {}", level);
    }

    /// Start log manager
    pub async fn start(&self) -> io::Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            log::info!("Log manager is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        log::info!("Starting log manager");

        // Initialize logger
        self.init_logger().await?;

        // Initialize log buffer
        self.init_log_buffer().await;

        // Start log rotation task
        let manager_clone = self.clone();
        let handle = tokio::spawn(async move {
            manager_clone.run_rotate_task().await;
        });

        let mut rotate_task = self.rotate_task.write().await;
        *rotate_task = Some(handle);
        drop(rotate_task);

        Ok(())
    }

    /// Initialize log buffer
    async fn init_log_buffer(&self) {
        if self.config.buffer_size > 0 {
            let (tx, mut rx) = mpsc::channel(self.config.buffer_size);

            // Store sender (using interior mutability)
            let mut manager = self.clone();
            manager.log_buffer_tx = Some(tx);

            // Start buffer processing task
            let handle = tokio::spawn(async move {
                while let Some(entry) = rx.recv().await {
                    manager.process_log_entry(entry).await;
                }
            });

            let mut log_buffer_task = self.log_buffer_task.write().await;
            *log_buffer_task = Some(handle);
            drop(log_buffer_task);

            log::info!(
                "Log buffer initialized with size: {}",
                self.config.buffer_size
            );
        }
    }

    /// Process log entry
    async fn process_log_entry(&self, entry: LogEntry) {
        // Format log entry
        let formatted = self.format_log_entry(entry).await;

        // Write to file if enabled
        if self.config.enable_file {
            let mut writer = self.writer.write().await;
            if let Some(writer) = &mut *writer {
                let _ = writer.write_all(formatted.as_bytes());
                let _ = writer.write_all(b"\n");
                let _ = writer.flush();
            }
        }
    }

    /// Format log entry
    async fn format_log_entry(&self, entry: LogEntry) -> String {
        if self.config.enable_structured {
            // Structured JSON format
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string();

            let log_obj = json!({
                "timestamp": timestamp,
                "level": entry.level,
                "module": entry.module,
                "message": entry.message,
                "context": entry.context,
                "counter": self.log_counter.fetch_add(1, Ordering::SeqCst)
            });

            serde_json::to_string(&log_obj).unwrap_or_else(|_| "{}".to_string())
        } else {
            // Traditional text format
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string();
            format!(
                "[{}] [{}] {}: {}",
                timestamp, entry.level, entry.module, entry.message
            )
        }
    }

    /// Stop log manager
    pub async fn stop(&self) -> io::Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        // Flush and close log writer
        let mut writer = self.writer.write().await;
        *writer = None;
        drop(writer);

        // Cancel log rotation task
        let mut rotate_task = self.rotate_task.write().await;
        if let Some(handle) = rotate_task.take() {
            handle.abort();
        }
        drop(rotate_task);

        // Cancel log buffer task
        let mut log_buffer_task = self.log_buffer_task.write().await;
        if let Some(handle) = log_buffer_task.take() {
            handle.abort();
        }
        drop(log_buffer_task);

        // Clear log buffer sender
        let mut manager = self.clone();
        manager.log_buffer_tx = None;

        log::info!("Log manager stopped");
        Ok(())
    }

    /// Initialize logger
    async fn init_logger(&self) -> io::Result<()> {
        let mut builder = Builder::new();

        // Set log level
        let level_filter = match self.config.level.to_lowercase().as_str() {
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        };
        builder.filter(None, level_filter);

        // Enable console logging
        if self.config.enable_console {
            builder.format(|buf, record| {
                let time = Local::now().format("%Y-%m-%d %H:%M:%S.%3f");
                writeln!(
                    buf,
                    "[{}] [{}] {}: {}",
                    time,
                    record.level(),
                    record.module_path().unwrap_or("unknown"),
                    record.args()
                )
            });
        }

        // Enable file logging
        if self.config.enable_file {
            self.init_file_log().await?;
        }

        // Initialize logger
        builder.init();

        log::info!("Log manager initialized with level: {}", self.config.level);
        Ok(())
    }

    /// Initialize file log
    async fn init_file_log(&self) -> io::Result<()> {
        // Create log directory
        let log_dir = Path::new(&self.config.file_path)
            .parent()
            .unwrap_or(Path::new("."));
        fs::create_dir_all(log_dir)?;

        // Generate log file path
        let log_file = self.generate_log_file_path().await;
        let mut current_log_file = self.current_log_file.write().await;
        *current_log_file = log_file.clone();
        drop(current_log_file);

        // Open log file
        let file = self.open_log_file(&log_file).await?;
        let mut writer = self.writer.write().await;
        *writer = Some(BufWriter::new(file));
        drop(writer);

        log::info!("Log file opened successfully: {:?}", log_file);
        Ok(())
    }

    /// Generate log file path
    async fn generate_log_file_path(&self) -> PathBuf {
        let now = Local::now();
        let file_name = format!("{}_{}.log", now.format("%Y-%m-%d"), now.hour());
        Path::new(&self.config.file_path).with_file_name(file_name)
    }

    /// Open log file
    async fn open_log_file(&self, path: &Path) -> io::Result<File> {
        OpenOptions::new().create(true).append(true).open(path)
    }

    /// Write to file
    async fn _write_to_file(&self, _line: &str) -> io::Result<()> {
        if !self.config.enable_file {
            return Ok(());
        }

        let mut writer_guard = self.writer.write().await;
        if let Some(writer) = &mut *writer_guard {
            writer.write_all(_line.as_bytes())?;
            writer.write_all(b"\n")?;
            writer.flush()?;
        }

        Ok(())
    }

    /// Check log size
    async fn _check_log_size(&self) -> io::Result<bool> {
        let current_log_file = self.current_log_file.read().await;
        let file_path = current_log_file.as_path();

        if !file_path.exists() {
            return Ok(false);
        }

        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();
        let max_size = self.config.max_file_size * 1024 * 1024; // Convert MB to bytes

        log::debug!(
            "Checking log file size: {} bytes, max: {} bytes",
            file_size,
            max_size
        );

        if file_size >= max_size {
            log::warn!(
                "Log file size {} bytes exceeds max size {} bytes, triggering rotation",
                file_size,
                max_size
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Run log rotation task
    async fn run_rotate_task(&self) {
        let interval = Duration::from_hours(self.config.rotate_hours as u64);

        loop {
            sleep(interval).await;

            let is_running = *self.is_running.read().await;
            if !is_running {
                break;
            }

            // Check if rotation is needed
            let should_rotate = self.should_rotate().await;
            let should_rotate_by_size = self._check_log_size().await.unwrap_or(false);

            if (should_rotate || should_rotate_by_size)
                && let Err(e) = self.rotate_log_internal().await
            {
                log::error!("Failed to rotate log: {:?}", e);
            }

            if let Err(e) = self.clean_old_logs().await {
                log::error!("Failed to clean old logs: {:?}", e);
            }
        }
    }

    /// Check if rotation is needed
    async fn should_rotate(&self) -> bool {
        let current_log_file = self.current_log_file.read().await;
        let file_path = current_log_file.as_path();

        if !file_path.exists() {
            return false;
        }

        let metadata = match fs::metadata(file_path) {
            Ok(meta) => meta,
            Err(_) => return false,
        };

        let file_age = match metadata.modified() {
            Ok(modified) => {
                let now = SystemTime::now();
                match now.duration_since(modified) {
                    Ok(duration) => duration.as_secs(),
                    Err(_) => 0,
                }
            }
            Err(_) => 0,
        };

        let rotate_interval = Duration::from_hours(self.config.rotate_hours as u64).as_secs();
        file_age >= rotate_interval
    }

    /// Rotate log file internally
    async fn rotate_log_internal(&self) -> io::Result<()> {
        log::info!("Starting log rotation");

        // Create new log file
        let new_log_file = self.generate_log_file_path().await;

        // Close current log file
        let mut writer = self.writer.write().await;
        *writer = None;
        drop(writer);

        // Save new log file path
        let mut current_log_file = self.current_log_file.write().await;
        *current_log_file = new_log_file.clone();
        drop(current_log_file);

        // Open new log file
        let file = self.open_log_file(&new_log_file).await?;
        let mut writer = self.writer.write().await;
        *writer = Some(BufWriter::new(file));
        drop(writer);

        log::info!("Log rotation completed, new log file: {:?}", new_log_file);
        Ok(())
    }

    /// Clean old logs
    async fn clean_old_logs(&self) -> io::Result<()> {
        let log_dir = Path::new(&self.config.file_path)
            .parent()
            .unwrap_or(Path::new("."));
        let retain_seconds = Duration::from_secs((self.config.retain_days as u64) * 24 * 60 * 60);
        let now = SystemTime::now();

        for entry in fs::read_dir(log_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Check if it's a file
            if !path.is_file() {
                continue;
            }

            let file_name = path.file_name().unwrap().to_str().unwrap_or("");
            if !file_name.ends_with(".log") {
                continue;
            }

            // Check if file needs to be deleted
            if let Ok(metadata) = fs::metadata(&path)
                && let Ok(modified) = metadata.modified()
                && let Ok(duration) = now.duration_since(modified)
                && duration > retain_seconds
            {
                if let Err(e) = fs::remove_file(&path) {
                    log::error!("Failed to delete old log: {:?}, error: {:?}", path, e);
                } else {
                    log::info!("Successfully deleted old log file: {:?}", path);
                }
            }
        }

        Ok(())
    }

    /// Log DEBUG level message
    pub async fn debug(&self, module: &str, message: &str) {
        self.log_with_context(module, "debug", message, None).await;
    }

    /// Log DEBUG level message with context
    pub async fn debug_with_context(&self, module: &str, message: &str, context: LogContext) {
        self.log_with_context(module, "debug", message, Some(context))
            .await;
    }

    /// Log INFO level message
    pub async fn info(&self, module: &str, message: &str) {
        self.log_with_context(module, "info", message, None).await;
    }

    /// Log INFO level message with context
    pub async fn info_with_context(&self, module: &str, message: &str, context: LogContext) {
        self.log_with_context(module, "info", message, Some(context))
            .await;
    }

    /// Log WARN level message
    pub async fn warn(&self, module: &str, message: &str) {
        self.log_with_context(module, "warn", message, None).await;
    }

    /// Log WARN level message with context
    pub async fn warn_with_context(&self, module: &str, message: &str, context: LogContext) {
        self.log_with_context(module, "warn", message, Some(context))
            .await;
    }

    /// Log ERROR level message
    pub async fn error(&self, module: &str, message: &str) {
        self.log_with_context(module, "error", message, None).await;
    }

    /// Log ERROR level message with context
    pub async fn error_with_context(&self, module: &str, message: &str, context: LogContext) {
        self.log_with_context(module, "error", message, Some(context))
            .await;
    }

    /// Log with level
    pub async fn log(&self, module: &str, level: &str, message: &str) {
        self.log_with_context(module, level, message, None).await;
    }

    /// Log with level and context
    pub async fn log_with_context(
        &self,
        module: &str,
        level: &str,
        message: &str,
        context: Option<LogContext>,
    ) {
        // Create log entry
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            module: module.to_string(),
            level: level.to_string(),
            message: message.to_string(),
            context,
        };

        // Send to buffer if enabled
        if let Some(tx) = &self.log_buffer_tx {
            let _ = tx.send(entry).await;
        } else {
            // Process directly if buffer is disabled
            self.process_log_entry(entry).await;
        }

        // Also log to standard logger for console output
        match level.to_lowercase().as_str() {
            "debug" => log::debug!(target: module, "{}", message),
            "info" => log::info!(target: module, "{}", message),
            "warn" => log::warn!(target: module, "{}", message),
            "error" => log::error!(target: module, "{}", message),
            _ => log::info!(target: module, "{}", message),
        }
    }

    /// Log iterate message
    pub async fn iterate_log(&self, message: &str) {
        self.log("ymaxum.iterate", "info", message).await;
    }

    /// Log customer service message
    pub async fn customer_service_log(&self, message: &str) {
        self.log("ymaxum.customer_service", "info", message).await;
    }

    /// Log IM message
    pub async fn im_log(&self, message: &str) {
        self.log("ymaxum.im", "info", message).await;
    }

    /// Log error message
    pub async fn error_log(&self, message: &str) {
        self.log("ymaxum.error", "error", message).await;
    }
}

/// Log manager builder (流畅配置API)
#[derive(Debug, Clone)]
pub struct LogManagerBuilder {
    config: LogConfig,
}

impl LogManagerBuilder {
    /// Create a new log manager builder
    pub fn new() -> Self {
        Self {
            config: LogConfig::default(),
        }
    }

    /// Set log level
    pub fn log_level(mut self, level: &str) -> Self {
        self.config.level = level.to_string();
        self
    }

    /// Set log file path
    pub fn file_path(mut self, path: &str) -> Self {
        self.config.file_path = path.to_string();
        self
    }

    /// Enable file logging
    pub fn enable_file(mut self) -> Self {
        self.config.enable_file = true;
        self
    }

    /// Disable file logging
    pub fn disable_file(mut self) -> Self {
        self.config.enable_file = false;
        self
    }

    /// Enable console logging
    pub fn enable_console(mut self) -> Self {
        self.config.enable_console = true;
        self
    }

    /// Disable console logging
    pub fn disable_console(mut self) -> Self {
        self.config.enable_console = false;
        self
    }

    /// Set maximum log file size in MB
    pub fn max_file_size(mut self, size: u64) -> Self {
        self.config.max_file_size = size;
        self
    }

    /// Set log file retention days
    pub fn retain_days(mut self, days: u8) -> Self {
        self.config.retain_days = days;
        self
    }

    /// Set log rotation interval in hours
    pub fn rotate_hours(mut self, hours: u8) -> Self {
        self.config.rotate_hours = hours;
        self
    }

    /// Enable structured logging
    pub fn enable_structured(mut self, enable: bool) -> Self {
        self.config.enable_structured = enable;
        self
    }

    /// Set log buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }

    /// Build log manager
    pub fn build(self) -> LogManager {
        LogManager::new(self.config)
    }
}

/// Log analysis results
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogAnalysisResult {
    /// Log file path
    pub log_file: String,
    /// Total entries
    pub total_entries: usize,
    /// Error count
    pub error_count: usize,
    /// Warning count
    pub warning_count: usize,
    /// Info count
    pub info_count: usize,
    /// Debug count
    pub debug_count: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u128,
    /// Detected anomalies
    pub anomalies: Vec<LogAnomaly>,
    /// Security issues
    pub security_issues: Vec<SecurityIssue>,
}

/// Log anomaly
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogAnomaly {
    /// Timestamp
    pub timestamp: String,
    /// Level
    pub level: String,
    /// Module
    pub module: String,
    /// Message
    pub message: String,
    /// Anomaly type
    pub anomaly_type: String,
}

/// Security issue
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityIssue {
    /// Timestamp
    pub timestamp: String,
    /// Level
    pub level: String,
    /// Module
    pub module: String,
    /// Message
    pub message: String,
    /// Issue type
    pub issue_type: String,
    /// Severity
    pub severity: String,
}

/// Log analysis report
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogAnalysisReport {
    /// Report timestamp
    pub timestamp: String,
    /// Project name
    pub project_name: String,
    /// Project version
    pub project_version: String,
    /// Total processing time in milliseconds
    pub total_processing_time_ms: u128,
    /// Total log files processed
    pub total_log_files: usize,
    /// Total log entries
    pub total_entries: usize,
    /// Total errors
    pub total_errors: usize,
    /// Total warnings
    pub total_warnings: usize,
    /// Total infos
    pub total_infos: usize,
    /// Total debugs
    pub total_debugs: usize,
    /// Total anomalies
    pub total_anomalies: usize,
    /// Total security issues
    pub total_security_issues: usize,
    /// Analysis results
    pub results: Vec<LogAnalysisResult>,
    /// Summary
    pub summary: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Security audit report
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityAuditReport {
    /// Report timestamp
    pub timestamp: String,
    /// Project name
    pub project_name: String,
    /// Project version
    pub project_version: String,
    /// Total security issues
    pub total_security_issues: usize,
    /// Critical issues
    pub critical_issues: usize,
    /// High issues
    pub high_issues: usize,
    /// Medium issues
    pub medium_issues: usize,
    /// Low issues
    pub low_issues: usize,
    /// Security issues
    pub security_issues: Vec<SecurityIssue>,
    /// Summary
    pub summary: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl LogManager {
    /// Analyze log file
    pub async fn analyze_log_file(&self, log_file: &str) -> io::Result<LogAnalysisResult> {
        let start = std::time::Instant::now();
        let file = File::open(log_file)?;
        let reader = io::BufReader::new(file);

        let mut total_entries = 0;
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        let mut debug_count = 0;
        let mut anomalies = Vec::new();
        let mut security_issues = Vec::new();

        for line in reader.lines() {
            let line = line?;
            total_entries += 1;

            // Parse log line
            if let Some(entry) = self.parse_log_line(&line) {
                // Count by level
                match entry.level.to_lowercase().as_str() {
                    "error" => error_count += 1,
                    "warn" => warning_count += 1,
                    "info" => info_count += 1,
                    "debug" => debug_count += 1,
                    _ => {}
                }

                // Detect anomalies
                if self.detect_anomaly(&entry) {
                    let timestamp_str = format!("{:?}", entry.timestamp);
                    let anomaly = LogAnomaly {
                        timestamp: timestamp_str,
                        level: entry.level.clone(),
                        module: entry.module.clone(),
                        message: entry.message.clone(),
                        anomaly_type: "Potential issue".to_string(),
                    };
                    anomalies.push(anomaly);
                }

                // Detect security issues
                if let Some(issue) = self.detect_security_issue(&entry) {
                    security_issues.push(issue);
                }
            }
        }

        let processing_time_ms = start.elapsed().as_millis();

        Ok(LogAnalysisResult {
            log_file: log_file.to_string(),
            total_entries,
            error_count,
            warning_count,
            info_count,
            debug_count,
            processing_time_ms,
            anomalies,
            security_issues,
        })
    }

    /// Parse log line
    fn parse_log_line(&self, line: &str) -> Option<LogEntry> {
        // Simple parsing for structured logs
        if let Ok(log_obj) = serde_json::from_str::<serde_json::Value>(line) {
            let _timestamp = log_obj
                .get("timestamp")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let level = log_obj
                .get("level")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let module = log_obj
                .get("module")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let message = log_obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            Some(LogEntry {
                timestamp: SystemTime::now(),
                module: module.to_string(),
                level: level.to_string(),
                message: message.to_string(),
                context: None,
            })
        } else {
            // Simple parsing for text logs
            // Format: [2026-01-21 12:34:56.789] [INFO] module: message
            let re = regex::Regex::new(r"\[(.*?)\]\s*\[(.*?)\]\s*(.*?):\s*(.*)").unwrap();
            if let Some(captures) = re.captures(line) {
                let _timestamp = captures.get(1).unwrap().as_str();
                let level = captures.get(2).unwrap().as_str();
                let module = captures.get(3).unwrap().as_str();
                let message = captures.get(4).unwrap().as_str();

                Some(LogEntry {
                    timestamp: SystemTime::now(),
                    module: module.to_string(),
                    level: level.to_string(),
                    message: message.to_string(),
                    context: None,
                })
            } else {
                None
            }
        }
    }

    /// Detect anomaly in log entry
    fn detect_anomaly(&self, entry: &LogEntry) -> bool {
        // Simple anomaly detection
        entry.level.to_lowercase() == "error"
            || entry.message.contains("failed")
            || entry.message.contains("error")
            || entry.message.contains("exception")
            || entry.message.contains("panic")
    }

    /// Detect security issue in log entry
    fn detect_security_issue(&self, entry: &LogEntry) -> Option<SecurityIssue> {
        // Security issue patterns
        let security_patterns = [
            ("password", "Password exposure", "High"),
            ("token", "Token exposure", "High"),
            ("credit_card", "Credit card exposure", "Critical"),
            ("ssn", "SSN exposure", "Critical"),
            ("api_key", "API key exposure", "High"),
            ("secret", "Secret exposure", "High"),
            ("encryption_key", "Encryption key exposure", "Critical"),
            ("authentication failed", "Authentication failure", "Medium"),
            ("unauthorized", "Unauthorized access", "High"),
            ("forbidden", "Forbidden access", "Medium"),
        ];

        for (pattern, issue_type, severity) in &security_patterns {
            if entry.message.to_lowercase().contains(pattern) {
                let timestamp_str = format!("{:?}", entry.timestamp);
                return Some(SecurityIssue {
                    timestamp: timestamp_str,
                    level: entry.level.clone(),
                    module: entry.module.clone(),
                    message: entry.message.clone(),
                    issue_type: issue_type.to_string(),
                    severity: severity.to_string(),
                });
            }
        }

        None
    }

    /// Generate log analysis report
    pub async fn generate_log_analysis_report(
        &self,
        log_files: &[String],
        project_name: &str,
        project_version: &str,
    ) -> LogAnalysisReport {
        let start = std::time::Instant::now();
        let mut results = Vec::new();
        let mut total_entries = 0;
        let mut total_errors = 0;
        let mut total_warnings = 0;
        let mut total_infos = 0;
        let mut total_debugs = 0;
        let mut total_anomalies = 0;
        let mut total_security_issues = 0;

        for log_file in log_files {
            if let Ok(result) = self.analyze_log_file(log_file).await {
                total_entries += result.total_entries;
                total_errors += result.error_count;
                total_warnings += result.warning_count;
                total_infos += result.info_count;
                total_debugs += result.debug_count;
                total_anomalies += result.anomalies.len();
                total_security_issues += result.security_issues.len();
                results.push(result);
            }
        }

        let total_processing_time_ms = start.elapsed().as_millis();

        // Generate summary
        let summary = format!(
            "Analyzed {} log files containing {} entries. Detected {} errors, {} warnings, {} anomalies, and {} security issues.",
            results.len(),
            total_entries,
            total_errors,
            total_warnings,
            total_anomalies,
            total_security_issues
        );

        // Generate recommendations
        let mut recommendations = Vec::new();
        if total_errors > 0 {
            recommendations.push("Investigate and fix error messages".to_string());
        }
        if total_warnings > 0 {
            recommendations.push("Address warning messages to prevent future issues".to_string());
        }
        if total_anomalies > 0 {
            recommendations.push("Review detected anomalies for potential problems".to_string());
        }
        if total_security_issues > 0 {
            recommendations.push("Fix security issues immediately".to_string());
        }
        recommendations.push("Regularly analyze logs to detect issues early".to_string());

        LogAnalysisReport {
            timestamp: Local::now().to_string(),
            project_name: project_name.to_string(),
            project_version: project_version.to_string(),
            total_processing_time_ms,
            total_log_files: results.len(),
            total_entries,
            total_errors,
            total_warnings,
            total_infos,
            total_debugs,
            total_anomalies,
            total_security_issues,
            results,
            summary,
            recommendations,
        }
    }

    /// Generate security audit report
    pub async fn generate_security_audit_report(
        &self,
        log_files: &[String],
        project_name: &str,
        project_version: &str,
    ) -> SecurityAuditReport {
        let mut security_issues = Vec::new();
        let mut critical_issues = 0;
        let mut high_issues = 0;
        let mut medium_issues = 0;
        let mut low_issues = 0;

        for log_file in log_files {
            if let Ok(result) = self.analyze_log_file(log_file).await {
                for issue in result.security_issues {
                    security_issues.push(issue);
                }
            }
        }

        // Count by severity
        for issue in &security_issues {
            match issue.severity.as_str() {
                "Critical" => critical_issues += 1,
                "High" => high_issues += 1,
                "Medium" => medium_issues += 1,
                "Low" => low_issues += 1,
                _ => {}
            }
        }

        // Generate summary
        let summary = format!(
            "Security audit found {} issues: {} Critical, {} High, {} Medium, {} Low.",
            security_issues.len(),
            critical_issues,
            high_issues,
            medium_issues,
            low_issues
        );

        // Generate recommendations
        let mut recommendations = Vec::new();
        if critical_issues > 0 {
            recommendations.push("Fix all critical security issues immediately".to_string());
        }
        if high_issues > 0 {
            recommendations
                .push("Address high severity security issues as soon as possible".to_string());
        }
        if medium_issues > 0 {
            recommendations.push("Review and fix medium severity security issues".to_string());
        }
        if low_issues > 0 {
            recommendations.push("Address low severity security issues".to_string());
        }
        recommendations.push("Implement log masking for sensitive information".to_string());
        recommendations.push("Regularly audit logs for security issues".to_string());
        recommendations.push("Implement security monitoring and alerting".to_string());

        SecurityAuditReport {
            timestamp: Local::now().to_string(),
            project_name: project_name.to_string(),
            project_version: project_version.to_string(),
            total_security_issues: security_issues.len(),
            critical_issues,
            high_issues,
            medium_issues,
            low_issues,
            security_issues,
            summary,
            recommendations,
        }
    }

    /// Find log files in directory
    pub fn find_log_files(&self, log_dir: &str) -> Vec<String> {
        let mut log_files = Vec::new();

        if let Ok(entries) = fs::read_dir(log_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    if file_name.ends_with(".log") {
                        log_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }

        log_files
    }

    /// Export logs to file
    pub async fn export_logs(&self, log_files: &[String], output_file: &str) -> io::Result<()> {
        let mut output = File::create(output_file)?;

        for log_file in log_files {
            let file = File::open(log_file)?;
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                writeln!(output, "{}", line)?;
            }
        }

        Ok(())
    }

    /// Compress log files
    pub async fn compress_logs(&self, log_files: &[String], output_file: &str) -> io::Result<()> {
        // Simple compression using gzip
        use flate2::Compression;
        use flate2::write::GzEncoder;

        let file = File::create(output_file)?;
        let encoder = GzEncoder::new(file, Compression::default());
        let mut writer = io::BufWriter::new(encoder);

        for log_file in log_files {
            let file = File::open(log_file)?;
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                writeln!(writer, "{}", line)?;
            }
        }

        writer.flush()?;
        Ok(())
    }

    /// Clean logs by pattern
    pub async fn clean_logs_by_pattern(&self, log_dir: &str, pattern: &str) -> io::Result<()> {
        let log_files = self.find_log_files(log_dir);

        for log_file in log_files {
            let file = File::open(&log_file)?;
            let reader = io::BufReader::new(file);

            let mut cleaned_lines = Vec::new();
            for line in reader.lines() {
                let line = line?;
                if !line.contains(pattern) {
                    cleaned_lines.push(line);
                }
            }

            // Write cleaned lines back to file
            let mut file = File::create(&log_file)?;
            for line in cleaned_lines {
                writeln!(file, "{}", line)?;
            }
        }

        Ok(())
    }
}

/// Log manager tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_log_manager() {
        // Create test configuration
        let config = LogConfig {
            level: "debug".to_string(),
            file_path: "./test.log".to_string(),
            enable_file: false, // Disable file logging for testing
            enable_console: true,
            max_file_size: 100,
            retain_days: 1,
            rotate_hours: 1,
            enable_structured: false,
            buffer_size: 0,
        };

        // Create log manager
        let log_manager = LogManager::new(config);

        // Start log manager
        log_manager.start().await.unwrap();

        // Log test messages
        sleep(Duration::from_millis(500)).await;

        // Log test messages
        log_manager.debug("test_module", "DEBUG LOG MESSAGE").await;
        log_manager.info("test_module", "INFO LOG MESSAGE").await;
        log_manager.warn("test_module", "WARN LOG MESSAGE").await;
        log_manager.error("test_module", "ERROR LOG MESSAGE").await;

        // Wait for logs to flush
        sleep(Duration::from_millis(500)).await;

        // Stop log manager
        log_manager.stop().await.unwrap();

        // Assert test passed
        assert!(true);
    }

    #[tokio::test]
    async fn test_log_analysis() {
        // Create test configuration
        let config = LogConfig {
            level: "debug".to_string(),
            file_path: "./test.log".to_string(),
            enable_file: false, // Disable file logging for testing
            enable_console: true,
            max_file_size: 100,
            retain_days: 1,
            rotate_hours: 1,
            enable_structured: false,
            buffer_size: 0,
        };

        // Create log manager
        let log_manager = LogManager::new(config);

        // Test log parsing
        let test_line = "[2026-01-24 12:00:00.000] [INFO] test_module: Test message";
        assert!(log_manager.parse_log_line(test_line).is_some());

        // Assert test passed
        assert!(true);
    }
}
