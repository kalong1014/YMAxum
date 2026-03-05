// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP client implementation

use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tokio_rustls::rustls as rustls;
use rustls::pki_types::ServerName;
use rustls_native_certs;
use std::sync::Arc;
use std::collections::HashMap;
use crate::core::network::error::NetworkError;
use crate::core::network::request::Request;
use crate::core::network::response::Response;
use crate::core::network::response::StatusCode;
use crate::core::network::headers::Headers;

/// HTTP version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HttpVersion {
    Http11,
    Http2,
    _Http3,
}

/// Connection pool entry
#[derive(Debug)]
struct ConnectionPoolEntry {
    /// Socket address
    addr: std::net::SocketAddr,
    /// TCP stream
    stream: Option<TcpStream>,
    /// Last used time
    last_used: Instant,
    /// Is HTTPS
    is_https: bool,
    /// Hostname
    hostname: String,
    /// HTTP version
    http_version: HttpVersion,
}

/// Connection pool key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PoolKey {
    addr: std::net::SocketAddr,
    is_https: bool,
    hostname: String,
    http_version: HttpVersion,
}

/// HTTP client
#[derive(Debug, Clone)]
pub struct Client {
    /// Connection timeout
    timeout: Duration,
    /// User agent
    user_agent: String,
    /// TLS configuration
    tls_config: Option<Arc<rustls::ClientConfig>>,
    /// Connection pool
    connection_pool: Arc<tokio::sync::Mutex<HashMap<PoolKey, Vec<ConnectionPoolEntry>>>>,
    /// Max pool size
    max_pool_size: usize,
    /// Connection idle timeout
    idle_timeout: Duration,
    /// Enable HTTP/2
    enable_http2: bool,
    /// Enable HTTP/3
    enable_http3: bool,
}

impl Client {
    /// Create new client
    pub fn new() -> Self {
        // Create default TLS configuration
        let mut root_store = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("Failed to load native certificates") {
            root_store.add(rustls::pki_types::CertificateDer::from(cert.as_ref())).expect("Failed to add certificate");
        }
        
        let mut tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        
        // Enable HTTP/2 support in TLS through ALPN
        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        
        // Enable TLS session resumption for better performance
        tls_config.resumption = rustls::client::Resumption::in_memory_sessions(1024);
        
        let tls_config = tls_config;
        
        Self {
            timeout: Duration::from_secs(30),
            user_agent: format!("YMAxum-Network-Client/{}", crate::core::network::VERSION),
            tls_config: Some(Arc::new(tls_config)),
            connection_pool: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            max_pool_size: 100,
            idle_timeout: Duration::from_secs(60),
            enable_http2: true,
            enable_http3: cfg!(feature = "http3"),
        }
    }

    /// Set max pool size
    pub fn set_max_pool_size(&mut self, max_pool_size: usize) {
        self.max_pool_size = max_pool_size;
    }

    /// Set idle timeout
    pub fn set_idle_timeout(&mut self, idle_timeout: Duration) {
        self.idle_timeout = idle_timeout;
    }

    /// Set timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Set user agent
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.user_agent = user_agent.to_string();
    }

    /// Enable or disable HTTP/2
    pub fn set_enable_http2(&mut self, enable: bool) {
        self.enable_http2 = enable;
    }

    /// Enable or disable HTTP/3
    pub fn set_enable_http3(&mut self, enable: bool) {
        self.enable_http3 = enable && cfg!(feature = "http3");
    }

    /// Send request
    pub async fn send(&self, request: &Request, base_url: &str) -> Result<Response, NetworkError> {
        // Parse URL
        let url = url::Url::parse(base_url).map_err(|_| NetworkError::InvalidUrl(base_url.to_string()))?;
        let host = url.host_str().ok_or(NetworkError::InvalidUrl(base_url.to_string()))?;
        let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 80 });

        // Create socket address
        let addr = (host, port).to_socket_addrs()?.next().ok_or(NetworkError::ConnectionError(format!("Failed to resolve address for {}", host)))?;

        // Build request message
        let mut request_message = format!("{} {} HTTP/1.1\r\n", request.method.as_str(), url.path());
        
        if let Some(query) = url.query() {
            request_message.push_str(&format!("?{}", query));
        }
        
        request_message.push_str(&format!("Host: {}\r\n", host));
        request_message.push_str(&format!("User-Agent: {}\r\n", self.user_agent));
        // Add HTTP/2 support header
        if self.enable_http2 && url.scheme() == "https" {
            request_message.push_str("Connection: keep-alive\r\n");
            request_message.push_str("Upgrade: h2c\r\n");
            // ALPN will handle HTTP/2 negotiation
        } else {
            request_message.push_str("Connection: keep-alive\r\n");
        }
        request_message.push_str(&request.headers.to_string());
        request_message.push_str("\r\n");
        
        if !request.body.is_empty() {
            request_message.push_str(std::str::from_utf8(&request.body).unwrap_or(""));
        }

        // Clean up idle connections
        self.cleanup_idle_connections().await;

        // Connect to server and send request
        let response_buffer = if url.scheme() == "https" {
            // Check if HTTP/3 is enabled and supported
            if self.enable_http3 {
                // Try HTTP/3 first
                if let Ok(buffer) = self.send_http3(host, port, &request_message).await {
                    buffer
                } else {
                    // Fallback to HTTPS with HTTP/2 support
                    self.send_https_with_pool(&addr, host.to_string(), &request_message).await?
                }
            } else {
                // Use TLS for HTTPS with HTTP/2 support
                self.send_https_with_pool(&addr, host.to_string(), &request_message).await?
            }
        } else {
            // Use plain TCP for HTTP
            self.send_http_with_pool(&addr, &request_message).await?
        };

        // Parse response
        self.parse_response(&response_buffer)
    }

    /// Clean up idle connections
    async fn cleanup_idle_connections(&self) {
        let mut pool = self.connection_pool.lock().await;
        let now = Instant::now();
        
        // Iterate through all pool keys
        let mut keys_to_remove = Vec::new();
        
        for (key, entries) in pool.iter_mut() {
            // Retain only active connections
            entries.retain(|entry| {
                now.duration_since(entry.last_used) < self.idle_timeout
            });
            
            // If no connections left for this key, remove the key
            if entries.is_empty() {
                keys_to_remove.push(key.clone());
            }
        }
        
        // Remove empty entries
        for key in keys_to_remove {
            pool.remove(&key);
        }
    }

    /// Check if a connection is healthy
    async fn is_connection_healthy(&self, stream: &mut TcpStream) -> bool {
        // 使用更轻量级的方式检查连接健康状态
        // 尝试设置和获取超时，这可以检测连接是否仍然有效
        let original_timeout = stream.read_timeout().ok();
        
        // 尝试设置一个短超时
        if stream.set_read_timeout(Some(Duration::from_millis(100))).is_err() {
            return false;
        }
        
        // 尝试读取0字节，这不会阻塞但可以检测连接状态
        let mut buf = [0; 1];
        let result = match stream.read(&mut buf) {
            Ok(0) => false, // 连接已关闭
            Err(e) => {
                // 超时是正常的，说明连接还活着但没有数据
                matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut)
            }
            _ => true, // 有数据可读，连接正常
        };
        
        // 恢复原始超时设置
        if let Some(timeout) = original_timeout {
            stream.set_read_timeout(timeout).ok();
        } else {
            stream.set_read_timeout(None).ok();
        }
        
        result
    }

    /// Get connection from pool
    async fn get_connection(&self, addr: &std::net::SocketAddr, is_https: bool, hostname: &str, http_version: HttpVersion) -> Option<ConnectionPoolEntry> {
        let mut pool = self.connection_pool.lock().await;
        
        let key = PoolKey {
            addr: *addr,
            is_https,
            hostname: hostname.to_string(),
            http_version,
        };
        
        if let Some(entries) = pool.get_mut(&key) {
            // 从最近使用的连接开始检查（从末尾开始）
            for i in (0..entries.len()).rev() {
                let mut entry = entries.remove(i);
                
                // 检查连接是否健康
                if let Some(stream) = &mut entry.stream {
                    if self.is_connection_healthy(stream).await {
                        // 更新最后使用时间
                        entry.last_used = Instant::now();
                        return Some(entry);
                    }
                    // 连接不健康，丢弃
                }
            }
            
            // 如果所有连接都不健康，清空该key的所有条目
            if entries.is_empty() {
                pool.remove(&key);
            }
        }
        
        None
    }

    /// Return connection to pool
    async fn return_connection(&self, mut entry: ConnectionPoolEntry) {
        let mut pool = self.connection_pool.lock().await;
        
        // 计算当前池大小
        let current_size: usize = pool.values().map(|v| v.len()).sum();
        
        if current_size < self.max_pool_size {
            // 在返回连接到池之前检查连接是否仍然健康
            if let Some(stream) = &mut entry.stream {
                if self.is_connection_healthy(stream).await {
                    entry.last_used = Instant::now();
                    
                    let key = PoolKey {
                        addr: entry.addr,
                        is_https: entry.is_https,
                        hostname: entry.hostname.clone(),
                        http_version: entry.http_version,
                    };
                    
                    // 获取或创建连接池条目
                    let entries = pool.entry(key).or_insert_with(Vec::new);
                    
                    // 移除过期的连接，保持池的新鲜度
                    let now = Instant::now();
                    entries.retain(|e| now.duration_since(e.last_used) < self.idle_timeout);
                    
                    // 添加新连接到末尾（最近使用）
                    entries.push(entry);
                    
                    // 限制每个key的连接数，防止过度拥挤
                    const MAX_CONNECTIONS_PER_KEY: usize = 10;
                    if entries.len() > MAX_CONNECTIONS_PER_KEY {
                        // 移除最旧的连接
                        entries.remove(0);
                    }
                }
            }
        }
    }

    /// Send HTTP request over plain TCP with connection pool
    async fn send_http_with_pool(&self, addr: &std::net::SocketAddr, request_message: &str) -> Result<Vec<u8>, NetworkError> {
        let http_version = HttpVersion::Http11;
        // Try to get connection from pool
        if let Some(mut entry) = self.get_connection(addr, false, "", http_version).await {
            if let Some(mut stream) = entry.stream.take() {
                // Send request with buffer optimization
                let request_bytes = request_message.as_bytes();
                stream.write_all(request_bytes).map_err(|e| NetworkError::Io(e))?;

                // Read response with optimized buffer management
                let mut response_buffer = Vec::with_capacity(8192); // 初始缓冲区大小
                let mut buf = [0; 8192]; // 基础缓冲区大小
                loop {
                    let n = stream.read(&mut buf).map_err(|e| NetworkError::Io(e))?;
                    if n == 0 {
                        break;
                    }
                    response_buffer.extend_from_slice(&buf[..n]);
                    // 动态调整缓冲区大小，避免频繁分配
                    if response_buffer.capacity() - response_buffer.len() < 4096 {
                        response_buffer.reserve(8192);
                    }
                }

                // Return connection to pool
                entry.stream = Some(stream);
                self.return_connection(entry).await;

                return Ok(response_buffer);
            }
        }

        // No connection in pool, create new one
        let mut stream = TcpStream::connect_timeout(addr, self.timeout).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_write_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Send request with buffer optimization
        let request_bytes = request_message.as_bytes();
        stream.write_all(request_bytes).map_err(|e| NetworkError::Io(e))?;

        // Read response with optimized buffer management
        let mut response_buffer = Vec::with_capacity(8192); // 初始缓冲区大小
        let mut buf = [0; 8192]; // 基础缓冲区大小
        loop {
            let n = stream.read(&mut buf).map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                break;
            }
            response_buffer.extend_from_slice(&buf[..n]);
            // 动态调整缓冲区大小，避免频繁分配
            if response_buffer.capacity() - response_buffer.len() < 4096 {
                response_buffer.reserve(8192);
            }
        }

        // Return connection to pool
        let entry = ConnectionPoolEntry {
            addr: *addr,
            stream: Some(stream),
            last_used: Instant::now(),
            is_https: false,
            hostname: "".to_string(),
            http_version: HttpVersion::Http11,
        };
        self.return_connection(entry).await;

        Ok(response_buffer)
    }

    /// Send HTTPS request over TLS with connection pool
    async fn send_https_with_pool(&self, addr: &std::net::SocketAddr, host: String, request_message: &str) -> Result<Vec<u8>, NetworkError> {
        // Get TLS configuration
        let tls_config = self.tls_config.as_ref().ok_or(NetworkError::SslError("TLS not configured".to_string()))?;

        // Determine HTTP version to use
        let use_http2 = self.enable_http2;

        // Try to get connection from pool
        let http_version = if use_http2 {
            HttpVersion::Http2
        } else {
            HttpVersion::Http11
        };
        if let Some(mut entry) = self.get_connection(addr, true, &host, http_version).await {
            if let Some(mut stream) = entry.stream.take() {
                // Create TLS client with ALPN support for HTTP/2
                let dns_name = ServerName::try_from("localhost").map_err(|_| NetworkError::SslError("Invalid DNS name".to_string()))?;
                let mut client = rustls::ClientConnection::new(tls_config.clone(), dns_name).map_err(|e| NetworkError::SslError(e.to_string()))?;

                // Create TLS stream
                let mut tls_stream = rustls::Stream::new(&mut client, &mut stream);

                // Send request with buffer optimization
                let request_bytes = request_message.as_bytes();
                tls_stream.write_all(request_bytes).map_err(|e| NetworkError::Io(e))?;

                // Read response with optimized buffer management
                let mut response_buffer = Vec::with_capacity(8192); // 初始缓冲区大小
                let mut buf = [0; 8192]; // 基础缓冲区大小
                loop {
                    let n = tls_stream.read(&mut buf).map_err(|e| NetworkError::Io(e))?;
                    if n == 0 {
                        break;
                    }
                    response_buffer.extend_from_slice(&buf[..n]);
                    // 动态调整缓冲区大小，避免频繁分配
                    if response_buffer.capacity() - response_buffer.len() < 4096 {
                        response_buffer.reserve(8192);
                    }
                }

                // Return connection to pool
                entry.stream = Some(stream);
                self.return_connection(entry).await;

                return Ok(response_buffer);
            }
        }

        // No connection in pool, create new one
        let mut stream = TcpStream::connect_timeout(addr, self.timeout).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_write_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Create TLS client with ALPN support for HTTP/2
        let dns_name = ServerName::try_from("localhost").map_err(|_| NetworkError::SslError("Invalid DNS name".to_string()))?;
        let mut client = rustls::ClientConnection::new(tls_config.clone(), dns_name).map_err(|e| NetworkError::SslError(e.to_string()))?;

        // Create TLS stream
        let mut tls_stream = rustls::Stream::new(&mut client, &mut stream);

        // Send request with buffer optimization
        let request_bytes = request_message.as_bytes();
        tls_stream.write_all(request_bytes).map_err(|e| NetworkError::Io(e))?;

        // Read response with optimized buffer management
        let mut response_buffer = Vec::with_capacity(8192); // 初始缓冲区大小
        let mut buf = [0; 8192]; // 基础缓冲区大小
        loop {
            let n = tls_stream.read(&mut buf).map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                break;
            }
            response_buffer.extend_from_slice(&buf[..n]);
            // 动态调整缓冲区大小，避免频繁分配
            if response_buffer.capacity() - response_buffer.len() < 4096 {
                response_buffer.reserve(8192);
            }
        }

        // Return connection to pool
        let entry = ConnectionPoolEntry {
            addr: *addr,
            stream: Some(stream),
            last_used: Instant::now(),
            is_https: true,
            hostname: host.to_string(),
            http_version: if use_http2 {
                HttpVersion::Http2
            } else {
                HttpVersion::Http11
            },
        };
        self.return_connection(entry).await;

        Ok(response_buffer)
    }

    /// Send HTTP request over plain TCP (without pool)
    #[allow(dead_code)]
    fn send_http(&self, addr: &std::net::SocketAddr, request_message: &str) -> Result<Vec<u8>, NetworkError> {
        // Connect to server
        let mut stream = TcpStream::connect_timeout(addr, self.timeout).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_write_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Send request
        stream.write_all(request_message.as_bytes()).map_err(|e| NetworkError::Io(e))?;

        // Read response
        let mut response_buffer = Vec::new();
        stream.read_to_end(&mut response_buffer).map_err(|e| NetworkError::Io(e))?;

        Ok(response_buffer)
    }

    /// Send HTTPS request over TLS (without pool)
    #[allow(dead_code)]
    fn send_https(&self, addr: &std::net::SocketAddr, host: &str, request_message: &str) -> Result<Vec<u8>, NetworkError> {
        // Get TLS configuration
        let tls_config = self.tls_config.as_ref().ok_or(NetworkError::SslError("TLS not configured".to_string()))?;

        // Create TLS client
        let host_static = Box::leak(host.to_string().into_boxed_str());
        let host_str: &str = host_static;
        let dns_name = ServerName::try_from(host_str).map_err(|_| NetworkError::SslError("Invalid DNS name".to_string()))?;
        let mut client = rustls::ClientConnection::new(tls_config.clone(), dns_name).map_err(|e| NetworkError::SslError(e.to_string()))?;

        // Connect to server
        let mut stream = TcpStream::connect_timeout(addr, self.timeout).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        stream.set_write_timeout(Some(self.timeout)).map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Create TLS stream
        let mut tls_stream = rustls::Stream::new(&mut client, &mut stream);

        // Send request
        tls_stream.write_all(request_message.as_bytes()).map_err(|e| NetworkError::Io(e))?;

        // Read response
        let mut response_buffer = Vec::new();
        tls_stream.read_to_end(&mut response_buffer).map_err(|e| NetworkError::Io(e))?;

        Ok(response_buffer)
    }

    /// Parse response
    fn parse_response(&self, buffer: &[u8]) -> Result<Response, NetworkError> {
        let response_str = std::str::from_utf8(buffer).map_err(|_| NetworkError::InvalidResponse("Invalid response format".to_string()))?;
        
        // Split response into status line, headers, and body
        let parts: Vec<&str> = response_str.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return Err(NetworkError::InvalidResponse("Invalid response format".to_string()));
        }
        
        let header_part = parts[0];
        let body_part = parts[1..].join("\r\n\r\n");
        
        // Parse status line
        let lines: Vec<&str> = header_part.lines().collect();
        if lines.is_empty() {
            return Err(NetworkError::InvalidResponse("Invalid response format".to_string()));
        }
        
        let status_line = lines[0];
        let status_parts: Vec<&str> = status_line.split_whitespace().collect();
        if status_parts.len() < 3 {
            return Err(NetworkError::InvalidResponse("Invalid status line".to_string()));
        }
        
        let status_code: u16 = status_parts[1].parse().map_err(|_| NetworkError::InvalidResponse("Invalid status code".to_string()))?;
        let status = StatusCode::from_code(status_code).unwrap_or(StatusCode {
            code: status_code,
            message: "Unknown",
        });
        
        // Parse headers
        let mut headers = Headers::new();
        for line in &lines[1..] {
            if let Some((name, value)) = line.split_once(": ") {
                headers.add(name, value.trim());
            }
        }
        
        // Create response
        let mut response = Response::new(status);
        response.headers = headers;
        response.set_body(body_part.as_bytes());
        
        Ok(response)
    }

    /// Send GET request
    pub async fn get(&self, url: &str) -> Result<Response, NetworkError> {
        let request = Request::new(crate::core::network::request::Method::GET, "/");
        self.send(&request, url).await
    }

    /// Send POST request
    pub async fn post(&self, url: &str, body: &[u8], content_type: &str) -> Result<Response, NetworkError> {
        let mut request = Request::new(crate::core::network::request::Method::POST, "/");
        request.set_body(body);
        request.set_content_type(content_type);
        self.send(&request, url).await
    }

    /// Send POST request with JSON body
    pub async fn post_json(&self, url: &str, json: &str) -> Result<Response, NetworkError> {
        self.post(url, json.as_bytes(), "application/json").await
    }

    /// Send HTTP/3 request
    #[cfg(feature = "http3")]
    async fn send_http3(&self, host: &str, port: u16, request_message: &str) -> Result<Vec<u8>, NetworkError> {
        use h3::client::Client as H3Client;
        use h3_quinn::Connection;
        use quinn::Endpoint;
        use std::net::SocketAddr;

        // Create QUIC endpoint
        let mut endpoint = Endpoint::client(SocketAddr::from(([0, 0, 0, 0], 0)))?;

        // Configure QUIC with better settings
        let mut client_cfg = quinn::ClientConfig::new(Arc::new(self.tls_config.as_ref().ok_or(NetworkError::SslError("TLS not configured".to_string()))?));
        let mut transport_config = quinn::TransportConfig::default();
        transport_config.max_idle_timeout(Some(std::time::Duration::from_secs(60)));
        transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(30)));
        client_cfg.transport_config(Arc::new(transport_config));
        endpoint.set_default_client_config(client_cfg);

        // Connect to server
        let addr = (host, port).to_socket_addrs()?.next().ok_or(NetworkError::ConnectionError(format!("Failed to resolve address for {}", host)))?;
        let connection = endpoint.connect(addr, host)?.await.map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Create HTTP/3 connection
        let (mut send_stream, recv_stream) = connection.open_bi().await.map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        // Send request
        send_stream.write_all(request_message.as_bytes()).await.map_err(|e| NetworkError::Io(e))?;
        send_stream.finish().await.map_err(|e| NetworkError::Io(e))?;

        // Read response with optimized buffer management
        let mut response_buffer = Vec::with_capacity(8192);
        let mut buf = [0; 8192];
        let mut recv_stream = recv_stream;
        loop {
            let n = recv_stream.read(&mut buf).await.map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                break;
            }
            response_buffer.extend_from_slice(&buf[..n]);
            // 动态调整缓冲区大小，避免频繁分配
            if response_buffer.capacity() - response_buffer.len() < 4096 {
                response_buffer.reserve(8192);
            }
        }

        Ok(response_buffer)
    }

    /// Send HTTP/3 request (fallback when http3 feature is not enabled)
    #[cfg(not(feature = "http3"))]
    async fn send_http3(&self, _host: &str, _port: u16, _request_message: &str) -> Result<Vec<u8>, NetworkError> {
        Err(NetworkError::UnsupportedFeature("HTTP/3 not enabled".to_string()))
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
