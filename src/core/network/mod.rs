// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Lightweight network library for HTTP/HTTPS handling
//! Provides optimized HTTP client and server functionality

pub mod client;
pub mod server;
pub mod error;
pub mod headers;
pub mod request;
pub mod response;
pub mod utils;

/// Network library version
pub const VERSION: &str = "1.0.0";

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Enable HTTP/2 support
    pub http2_enabled: bool,
    /// Enable HTTPS support
    pub https_enabled: bool,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Read timeout (seconds)
    pub read_timeout: u64,
    /// Write timeout (seconds)
    pub write_timeout: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            http2_enabled: true,
            https_enabled: true,
            max_connections: 100,
            connection_timeout: 30,
            read_timeout: 60,
            write_timeout: 60,
        }
    }
}

/// Initialize network library
pub fn init(config: NetworkConfig) {
    // Initialize network library with given configuration
    log::info!("Network library initialized with config: {:?}", config);
}
