//! 错误日志模块
//! 用于记录和管理错误日志

use log::{debug, error, info, warn};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::sync::{LazyLock, Mutex};

/// 错误日志配置
pub struct ErrorLoggerConfig {
    /// 日志目录
    pub log_dir: String,
    /// 是否启用控制台日志
    pub enable_console: bool,
    /// 是否启用文件日志
    pub enable_file: bool,
    /// 日志文件大小限制（MB）
    pub file_size_limit_mb: usize,
    /// 日志保留天数
    pub retention_days: usize,
}

impl Default for ErrorLoggerConfig {
    fn default() -> Self {
        Self {
            log_dir: "./logs".to_string(),
            enable_console: true,
            enable_file: true,
            file_size_limit_mb: 100,
            retention_days: 7,
        }
    }
}

/// 错误日志记录器
pub struct ErrorLogger {
    file: Mutex<Option<File>>,
    config: ErrorLoggerConfig,
    current_file_size: Mutex<usize>,
}

impl ErrorLogger {
    /// 创建新的错误日志记录器
    pub fn new(config: ErrorLoggerConfig) -> Self {
        Self {
            file: Mutex::new(None),
            config,
            current_file_size: Mutex::new(0),
        }
    }

    /// 初始化错误日志文件
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let log_dir = Path::new(&self.config.log_dir);
        create_dir_all(log_dir)?;

        // 清理过期日志
        self.cleanup_old_logs()?;

        // 创建新的日志文件
        self.rotate_log()?;

        Ok(())
    }

    /// 记录调试日志
    pub fn debug(&self, message: &str) {
        if self.config.enable_console {
            debug!("{}", message);
        }
        if self.config.enable_file {
            self.write_to_file("DEBUG", message);
        }
    }

    /// 记录信息日志
    pub fn info(&self, message: &str) {
        if self.config.enable_console {
            info!("{}", message);
        }
        if self.config.enable_file {
            self.write_to_file("INFO", message);
        }
    }

    /// 记录警告日志
    pub fn warn(&self, message: &str) {
        if self.config.enable_console {
            warn!("{}", message);
        }
        if self.config.enable_file {
            self.write_to_file("WARN", message);
        }
    }

    /// 记录错误日志
    pub fn error(&self, message: &str) {
        if self.config.enable_console {
            error!("{}", message);
        }
        if self.config.enable_file {
            self.write_to_file("ERROR", message);
        }
    }

    /// 记录致命错误日志
    pub fn fatal(&self, message: &str) {
        if self.config.enable_console {
            error!("FATAL: {}", message);
        }
        if self.config.enable_file {
            self.write_to_file("FATAL", message);
        }
    }

    /// 记录带有上下文的错误日志
    pub fn error_with_context(&self, message: &str, context: &str) {
        let full_message = format!("{} - Context: {}", message, context);
        if self.config.enable_console {
            error!("{}", full_message);
        }
        if self.config.enable_file {
            self.write_to_file("ERROR", &full_message);
        }
    }

    /// 写入日志到文件
    fn write_to_file(&self, level: &str, message: &str) {
        let mut file_guard = self.file.lock().unwrap();
        let mut size_guard = self.current_file_size.lock().unwrap();

        if let Some(ref mut file) = *file_guard {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let log_entry = format!("[{}] [{}] {}\n", timestamp, level, message);

            // 检查文件大小
            let entry_size = log_entry.len();
            if *size_guard + entry_size > self.config.file_size_limit_mb * 1024 * 1024 {
                drop(file_guard);
                drop(size_guard);
                if let Err(e) = self.rotate_log() {
                    eprintln!("Failed to rotate log: {}", e);
                }
                return;
            }

            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write to error log: {}", e);
            } else {
                *size_guard += entry_size;
            }

            if let Err(e) = file.flush() {
                eprintln!("Failed to flush error log: {}", e);
            }
        }
    }

    /// 轮换日志文件
    fn rotate_log(&self) -> Result<(), Box<dyn std::error::Error>> {
        let log_dir = Path::new(&self.config.log_dir);
        let log_path = log_dir.join(format!(
            "error_{}.log",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        *self.file.lock().unwrap() = Some(file);
        *self.current_file_size.lock().unwrap() = 0;

        Ok(())
    }

    /// 清理过期日志
    fn cleanup_old_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let log_dir = Path::new(&self.config.log_dir);
        if !log_dir.exists() {
            return Ok(());
        }

        let cutoff_time =
            chrono::Local::now() - chrono::Duration::days(self.config.retention_days as i64);

        for entry in std::fs::read_dir(log_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && let Ok(metadata) = path.metadata()
                && let Ok(modified) = metadata.modified()
            {
                let modified_time = chrono::DateTime::<chrono::Local>::from(modified);
                if modified_time < cutoff_time {
                    let _ = std::fs::remove_file(path);
                }
            }
        }

        Ok(())
    }
}

/// 全局错误日志记录器
pub static GLOBAL_ERROR_LOGGER: LazyLock<ErrorLogger> =
    LazyLock::new(|| ErrorLogger::new(ErrorLoggerConfig::default()));

/// 初始化全局错误日志记录器
pub fn init_error_logger(
    config: Option<ErrorLoggerConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_cfg) = config {
        // 重新初始化全局日志记录器
        // 注意：这里只是演示，实际中可能需要更复杂的初始化逻辑
        *GLOBAL_ERROR_LOGGER.file.lock().unwrap() = None;
        *GLOBAL_ERROR_LOGGER.current_file_size.lock().unwrap() = 0;
        // 这里应该更新配置，但由于static变量的限制，我们暂时使用默认配置
    }
    GLOBAL_ERROR_LOGGER.init()
}

/// 记录调试日志
pub fn log_debug(message: &str) {
    GLOBAL_ERROR_LOGGER.debug(message);
}

/// 记录信息日志
pub fn log_info(message: &str) {
    GLOBAL_ERROR_LOGGER.info(message);
}

/// 记录警告日志
pub fn log_warn(message: &str) {
    GLOBAL_ERROR_LOGGER.warn(message);
}

/// 记录错误日志
pub fn log_error(message: &str) {
    GLOBAL_ERROR_LOGGER.error(message);
}

/// 记录带有上下文的错误日志
pub fn log_error_with_context(message: &str, context: &str) {
    GLOBAL_ERROR_LOGGER.error_with_context(message, context);
}

/// 记录致命错误日志
pub fn log_fatal(message: &str) {
    GLOBAL_ERROR_LOGGER.fatal(message);
}
