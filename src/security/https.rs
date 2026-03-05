//! HTTPS配置模块
//! 提供HTTPS配置鍜屽己鍒禜TTPS功能

use std::sync::Arc;
use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::fs;

/// HTTPS配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsConfig {
    /// 是否启用HTTPS
    pub enabled: bool,
    /// 璇佷功璺緞
    pub cert_path: Option<String>,
    /// 绉侀挜璺緞
    pub key_path: Option<String>,
    /// 是否浣跨敤内置璇佷功锛堢敤浜庡紑鍙戠幆澧冿級
    pub use_builtin_cert: bool,
    /// 目标戝惉绔彛
    pub port: u16,
    /// 是否强制启用HTTPS
    pub force_https: bool,
    /// HSTS启用状态
    pub hsts_enabled: bool,
    /// HSTS最大过期时间（秒）
    pub hsts_max_age: u64,
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cert_path: None,
            key_path: None,
            use_builtin_cert: true,
            port: 443,
            force_https: true,
            hsts_enabled: true,
            hsts_max_age: 31536000,
        }
    }
}

/// HTTPS閿欒类型
#[derive(Error, Debug)]
pub enum HttpsError {
    #[error("Certificate file not found: {0}")]
    CertFileNotFound(String),
    #[error("Private key file not found: {0}")]
    KeyFileNotFound(String),
    #[error("Failed to read certificate file: {0}")]
    ReadCertFailed(String),
    #[error("Failed to read private key file: {0}")]
    ReadKeyFailed(String),
    #[error("Invalid certificate format: {0}")]
    InvalidCertFormat(String),
    #[error("Invalid private key format: {0}")]
    InvalidKeyFormat(String),
    #[error("HTTPS configuration error: {0}")]
    ConfigError(String),
}

/// HTTPS服务
#[derive(Clone)]
pub struct HttpsService {
    config: Arc<HttpsConfig>,
}

impl HttpsService {
    /// 创建鏂扮殑HTTPS服务实例
    pub fn new(config: HttpsConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 楠岃瘉HTTPS配置
    pub async fn validate_config(&self) -> Result<(), HttpsError> {
        if !self.config.enabled {
            return Ok(());
        }

        // 濡傛灉浣跨敤内置璇佷功锛屼笉闇€瑕侀獙璇佹枃浠?        if self.config.use_builtin_cert {
            return Ok(());
        }

        // 楠岃瘉璇佷功鍜岀閽ユ枃浠舵槸鍚﹀瓨鍦?        let cert_path = self.config.cert_path.as_ref()
            .ok_or_else(|| HttpsError::ConfigError("Certificate path is required when not using builtin cert".to_string()))?;

        let key_path = self.config.key_path.as_ref()
            .ok_or_else(|| HttpsError::ConfigError("Private key path is required when not using builtin cert".to_string()))?;

        // 妫€鏌ユ枃浠舵槸鍚﹀瓨鍦?        if !Path::new(cert_path).exists() {
            return Err(HttpsError::CertFileNotFound(cert_path.clone()));
        }

        if !Path::new(key_path).exists() {
            return Err(HttpsError::KeyFileNotFound(key_path.clone()));
        }

        // 灏濊瘯璇诲彇文件鍐呭
        fs::read_to_string(cert_path).await
            .map_err(|e| HttpsError::ReadCertFailed(e.to_string()))?;

        fs::read_to_string(key_path).await
            .map_err(|e| HttpsError::ReadKeyFailed(e.to_string()))?;

        Ok(())
    }

    /// 鑾峰彇HTTPS目标戝惉鍦板潃
    pub fn get_listen_addr(&self) -> String {
        if self.config.enabled {
            format!("0.0.0.0:{}", self.config.port)
        } else {
            format!("0.0.0.0:{}", 80)
        }
    }

    /// 鑾峰彇HTTP閲嶅畾鍚戠洃鍚湴鍧€锛堢敤浜庡皢HTTP璇锋眰閲嶅畾鍚戝埌HTTPS锛?    pub fn get_redirect_addr(&self) -> Option<String> {
        if self.config.enabled && self.config.force_https {
            Some("0.0.0.0:80".to_string())
        } else {
            None
        }
    }

    /// 检查是否需要强制HTTPS
    pub fn is_force_https(&self) -> bool {
        self.config.enabled && self.config.force_https
    }

    /// 获取HSTS头信息
    pub fn get_hsts_header(&self) -> Option<String> {
        if self.config.enabled && self.config.hsts_enabled {
            Some(format!("max-age={}; includeSubDomains", self.config.hsts_max_age))
        } else {
            None
        }
    }

    /// 鍔犺浇璇佷功鍜岀閽?    pub async fn load_certificates(&self) -> Result<(Vec<u8>, Vec<u8>), HttpsError> {
        if !self.config.enabled {
            return Err(HttpsError::ConfigError("HTTPS is not enabled".to_string()));
        }

        if self.config.use_builtin_cert {
            // 杩欓噷鍙互实现鐢熸垚鑷鍚嶈瘉涔︾殑核心緫
            // 涓轰簡绠€鍖栵紝鏆傛椂杩斿洖绌哄悜閲?            Ok((Vec::new(), Vec::new()))
        } else {
            let cert_path = self.config.cert_path.as_ref()
                .ok_or_else(|| HttpsError::ConfigError("Certificate path is required".to_string()))?;
            
            let key_path = self.config.key_path.as_ref()
                .ok_or_else(|| HttpsError::ConfigError("Private key path is required".to_string()))?;

            let cert = fs::read(cert_path).await
                .map_err(|e| HttpsError::ReadCertFailed(e.to_string()))?;
            
            let key = fs::read(key_path).await
                .map_err(|e| HttpsError::ReadKeyFailed(e.to_string()))?;

            Ok((cert, key))
        }
    }
}