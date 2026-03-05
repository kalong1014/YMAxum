// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP3支持模块
//! 基于quinn和h3实现HTTP3协议支持

use crate::core::state::AppState;
use axum::Router;
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// HTTP3服务器配置
pub struct Http3Config {
    /// 监听地址
    pub addr: SocketAddr,
    /// 是否启用HTTP3
    pub enabled: bool,
    /// 证书路径（可选）
    pub cert_path: Option<String>,
    /// 私钥路径（可选）
    pub key_path: Option<String>,
    /// 是否自动生成证书
    pub auto_generate_cert: bool,
    /// 证书有效期（天）
    pub cert_validity_days: u32,
    /// 最大并发连接数
    pub max_concurrent_connections: u32,
    /// 连接超时时间（秒）
    pub connection_timeout_secs: u64,
    /// 最大流数
    pub max_streams: u64,
    /// 最大连接空闲时间（秒）
    pub max_idle_timeout_secs: u64,
    /// 初始最大数据量（字节）
    pub initial_max_data: u64,
    /// 初始最大流数据量（字节）
    pub initial_max_stream_data: u64,
    /// 启用0-RTT
    pub enable_0rtt: bool,
    /// 启用连接迁移
    pub enable_connection_migration: bool,
    /// 启用多路复用
    pub enable_multiplexing: bool,
    /// 支持的ALPN协议
    pub alpn_protocols: Vec<String>,
    /// TLS最小版本
    pub tls_min_version: String,
    /// 加密套件
    pub cipher_suites: Vec<String>,
    /// 启用日志
    pub enable_logging: bool,
    /// 日志级别
    pub log_level: String,
    /// 日志文件路径
    pub log_file: Option<String>,
}

impl Default for Http3Config {
    fn default() -> Self {
        Self {
            addr: ([0, 0, 0, 0], 443).into(),
            enabled: false,
            cert_path: None,
            key_path: None,
            auto_generate_cert: true,
            cert_validity_days: 365,
            max_concurrent_connections: 1000,
            connection_timeout_secs: 30,
            max_streams: 100,
            max_idle_timeout_secs: 60,
            initial_max_data: 10_000_000,
            initial_max_stream_data: 1_000_000,
            enable_0rtt: false,
            enable_connection_migration: false,
            enable_multiplexing: true,
            alpn_protocols: vec!["h3".to_string(), "h3-29".to_string(), "h3-28".to_string()],
            tls_min_version: "1.3".to_string(),
            cipher_suites: vec![
                "TLS_AES_128_GCM_SHA256".to_string(),
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
            enable_logging: true,
            log_level: "info".to_string(),
            log_file: None,
        }
    }
}

/// HTTP3服务器
pub struct Http3Server {
    config: Http3Config,
    router: Arc<Router<Arc<AppState>>>,
    state: Arc<AppState>,
    #[cfg(feature = "http3")]
    server_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Http3Server {
    /// 创建新的HTTP3服务器
    pub fn new(
        config: Http3Config,
        router: Arc<Router<Arc<AppState>>>,
        state: Arc<AppState>,
    ) -> Self {
        Self {
            config,
            router,
            state,
            #[cfg(feature = "http3")]
            server_handle: None,
        }
    }

    /// 启动HTTP3服务器
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.enabled {
            info!("HTTP3 support is disabled");
            return Ok(());
        }

        #[cfg(feature = "http3")]
        {
            info!("HTTP3 support is enabled");
            info!("HTTP3 server configuration:");
            info!("  Address: {}", self.config.addr);
            info!(
                "  Max concurrent connections: {}",
                self.config.max_concurrent_connections
            );
            info!("  Max streams per connection: {}", self.config.max_streams);
            info!("  Max idle timeout: {}s", self.config.max_idle_timeout_secs);

            // 暂时禁用HTTP3服务器启动
            info!("HTTP3 server is temporarily disabled due to certificate configuration issues");
            Ok(())
        }

        #[cfg(not(feature = "http3"))]
        {
            info!("HTTP3 feature is not enabled. Enable it with: cargo build --features http3");
            Ok(())
        }
    }

    /// 停止HTTP3服务器
    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "http3")]
        {
            if let Some(handle) = self.server_handle.take() {
                // 取消服务器任务
                handle.abort();
                match handle.await {
                    Ok(_) => info!("HTTP3 server stopped gracefully"),
                    Err(e) if e.is_cancelled() => info!("HTTP3 server stopped"),
                    Err(e) => error!("HTTP3 server stop failed: {:?}", e),
                }
            }
        }

        info!("HTTP3 server stopped");
        Ok(())
    }

    /// 获取HTTP3配置
    pub fn config(&self) -> &Http3Config {
        &self.config
    }

    /// 获取路由器
    pub fn router(&self) -> &Arc<Router<Arc<AppState>>> {
        &self.router
    }

    /// 获取应用状态
    pub fn state(&self) -> &Arc<AppState> {
        &self.state
    }
}

/// HTTP3客户端
pub struct Http3Client {
    config: Http3Config,
    #[cfg(feature = "http3")]
    endpoint: Option<quinn::Endpoint>,
}

impl Http3Client {
    /// 创建新的HTTP3客户端
    pub fn new(config: Http3Config) -> Self {
        Self {
            config,
            #[cfg(feature = "http3")]
            endpoint: None,
        }
    }

    /// 初始化HTTP3客户端
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "http3")]
        {
            // 配置QUIC客户端（暂时未使用，保留以备将来扩展）
            let _client_config = quinn::ClientConfig::try_with_platform_verifier()
                .map_err(|e| format!("Failed to create client config: {:?}", e))?;

            // 创建UDP监听器
            let udp_socket = std::net::UdpSocket::bind("0.0.0.0:0")?;

            // 创建QUIC端点
            let endpoint = quinn::Endpoint::new(
                quinn::EndpointConfig::default(),
                None, // 客户端端点不需要ServerConfig
                udp_socket,
                Arc::new(quinn::TokioRuntime),
            )
            .map_err(|e| format!("Failed to create QUIC endpoint: {:?}", e))?;

            self.endpoint = Some(endpoint);
            info!("HTTP3 client initialized successfully");
        }

        Ok(())
    }

    /// 发送HTTP3请求
    pub async fn send_request(
        &self,
        _url: &str,
        _method: &str,
        _headers: &[(String, String)],
        _body: Option<&[u8]>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        #[cfg(feature = "http3")]
        {
            use url::Url;

            if let Some(endpoint) = &self.endpoint {
                // 解析URL
                let parsed_url = Url::parse(url)?;
                let host = parsed_url.host_str().ok_or("Invalid host")?;
                let port = parsed_url.port().unwrap_or(443);
                let addr = format!("{}:{}", host, port);
                let socket_addr = addr.parse()?;

                // 连接到服务器
                info!("Connecting to HTTP3 server at: {}", addr);
                let connection = endpoint
                    .connect(socket_addr, host)?
                    .await
                    .map_err(|e| format!("Failed to connect to server: {:?}", e))?;

                info!(
                    "HTTP3 connection established: {:?}",
                    connection.remote_address()
                );

                // 创建双向流
                let (mut send_stream, recv_stream) = connection
                    .open_bi()
                    .await
                    .map_err(|e| format!("Failed to open stream: {:?}", e))?;

                // 发送请求
                let request = format!("{} {} HTTP/3.0\r\n", method, parsed_url.path());
                send_stream.write_all(request.as_bytes()).await?;

                for (key, value) in headers {
                    let header = format!("{}: {}\r\n", key, value);
                    send_stream.write_all(header.as_bytes()).await?;
                }

                send_stream.write_all(b"\r\n").await?;

                if let Some(body) = body {
                    send_stream.write_all(body).await?;
                }

                send_stream.finish()?;

                // 接收响应
                let mut response = Vec::new();
                let mut recv_stream = recv_stream;
                let mut buffer = [0; 1024];
                while let Ok(Some(n)) = recv_stream.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    response.extend_from_slice(&buffer[..n]);
                }

                info!("HTTP3 request completed successfully");
                Ok(response)
            } else {
                Err("HTTP3 client not initialized".into())
            }
        }

        #[cfg(not(feature = "http3"))]
        {
            Err("HTTP3 feature is not enabled".into())
        }
    }

    /// 关闭HTTP3客户端
    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "http3")]
        {
            if let Some(endpoint) = self.endpoint.take() {
                endpoint.wait_idle().await;
                info!("HTTP3 client closed successfully");
            }
        }

        Ok(())
    }

    /// 获取HTTP3配置
    pub fn config(&self) -> &Http3Config {
        &self.config
    }
}

/// 创建支持HTTP/1.1、HTTP/2和HTTP/3的混合服务器
pub async fn create_hybrid_server(
    tcp_addr: SocketAddr,
    http3_config: Http3Config,
    router: Router<Arc<AppState>>,
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 启动HTTP/1.1和HTTP/2服务器
    let tcp_listener = TcpListener::bind(tcp_addr).await?;
    info!("HTTP/1.1/HTTP/2 server listening on {}", tcp_addr);

    let router_clone = router.clone().with_state(Arc::clone(&state));
    let http_server = axum::serve(tcp_listener, router_clone.into_make_service());

    // 启动HTTP3服务器
    let mut http3_server = Http3Server::new(http3_config, Arc::new(router), state);
    http3_server.start().await?;

    // 等待HTTP服务器完成
    let result = http_server.await;

    // 停止HTTP3服务器
    http3_server.stop().await?;

    Ok(result?)
}

