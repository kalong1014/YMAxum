// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP server implementation

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::str::FromStr;
use tokio_rustls::rustls as rustls;
use rustls_pemfile::certs;
use tokio_rustls;
use std::fs::File;
use std::io::BufReader;
use crate::core::network::error::NetworkError;
use crate::core::network::request::Request;
use crate::core::network::request::Method;
use crate::core::network::response::Response;
use crate::core::network::headers::Headers;

/// Request handler
pub type RequestHandler = Arc<dyn Fn(&Request) -> Response + Send + Sync>;

/// HTTP server
pub struct Server {
    /// Address to listen on
    addr: String,
    /// Request handler
    handler: RequestHandler,
    /// TLS configuration
    tls_config: Option<Arc<rustls::ServerConfig>>,
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Server")
            .field("addr", &self.addr)
            .field("tls_config", &self.tls_config.is_some())
            .finish()
    }
}

impl Server {
    /// Create new server
    pub fn new(addr: &str, handler: RequestHandler) -> Self {
        Self {
            addr: addr.to_string(),
            handler,
            tls_config: None,
        }
    }

    /// Create new HTTPS server
    pub fn new_https(addr: &str, handler: RequestHandler, cert_path: &str, key_path: &str) -> Result<Self, NetworkError> {
        // Load certificate
        let cert_file = File::open(cert_path).map_err(|e| NetworkError::SslError(format!("Failed to open certificate file: {}", e)))?;
        let mut cert_reader = BufReader::new(cert_file);
        let certs: Vec<_> =
            certs(&mut cert_reader)
                .filter_map(|cert| cert.ok())
                .collect();

        // Load private key
        let key_file = File::open(key_path).map_err(|e| NetworkError::SslError(format!("Failed to open private key file: {}", e)))?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
            .filter_map(|key| key.ok())
            .collect::<Vec<_>>();
        
        if keys.is_empty() {
            return Err(NetworkError::SslError("No private keys found".to_string()));
        }
        
        // Create TLS configuration with HTTP/2 support and performance optimizations
        let mut tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, rustls::pki_types::PrivateKeyDer::Pkcs8(keys.remove(0)))
            .map_err(|e| NetworkError::SslError(format!("Failed to set certificate: {}", e)))?;

        // Enable HTTP/2 support through ALPN
        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        
        // Enable TLS session resumption for better performance
        tls_config.session_storage = rustls::server::ServerSessionMemoryCache::new(1024);

        Ok(Self {
            addr: addr.to_string(),
            handler,
            tls_config: Some(Arc::new(tls_config)),
        })
    }

    /// Start server
    pub async fn start(&self) -> Result<(), NetworkError> {
        let listener = TcpListener::bind(&self.addr).await.map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
        log::info!("Server listening on {}", self.addr);

        // 使用tokio的任务池来处理连接，充分利用异步IO
        // 配置任务池大小，根据CPU核心数自动调整
        let num_cpus = num_cpus::get();
        let worker_threads = std::cmp::max(4, num_cpus);
        log::info!("Server starting with {} worker threads", worker_threads);

        // 克隆必要的字段到闭包中
        let handler = self.handler.clone();
        let tls_config = self.tls_config.clone();

        // 使用spawn来处理连接接受，避免阻塞事件循环
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let handler_clone = handler.clone();
                        let tls_config_clone = tls_config.clone();
                        
                        // 使用spawn来处理每个连接，让tokio的调度器优化任务分配
                        tokio::spawn(async move {
                            if let Err(e) = if let Some(tls_config) = tls_config_clone {
                                Self::handle_https_connection(stream, &handler_clone, tls_config).await
                            } else {
                                Self::handle_http_connection(stream, &handler_clone).await
                            } {
                                log::error!("Error handling connection: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        // 服务器会一直运行，直到被中断
        Ok(())
    }

    /// Handle HTTP connection
    async fn handle_http_connection(mut stream: TcpStream, handler: &RequestHandler) -> Result<(), NetworkError> {
        // Read request with dynamic buffer optimization
        let mut buffer = Vec::with_capacity(4096);
        let mut buf = [0; 4096];
        loop {
            let n = stream.read(&mut buf).await.map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&buf[..n]);
            // 动态调整缓冲区大小，避免频繁分配
            if buffer.capacity() - buffer.len() < 2048 {
                buffer.reserve(4096);
            }
            // Check if we've read the entire request (ending with \r\n\r\n)
            if buffer.len() >= 4 && &buffer[buffer.len()-4..] == b"\r\n\r\n" {
                break;
            }
        }
        
        if buffer.is_empty() {
            return Ok(());
        }

        // Parse request
        let request = Self::parse_request(&buffer)?;

        // Handle request
        let response = handler(&request);

        // Send response with buffer optimization
        let response_message = response.to_http_message();
        let response_bytes = response_message.as_bytes();
        let mut total_written = 0;
        while total_written < response_bytes.len() {
            let n = stream.write(&response_bytes[total_written..]).await.map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                return Err(NetworkError::ConnectionError("Connection closed".to_string()));
            }
            total_written += n;
        }
        stream.flush().await.map_err(|e| NetworkError::Io(e))?;

        Ok(())
    }

    /// Handle HTTPS connection
    async fn handle_https_connection(stream: TcpStream, handler: &RequestHandler, tls_config: Arc<rustls::ServerConfig>) -> Result<(), NetworkError> {
        // Create TLS acceptor
        let acceptor = tokio_rustls::TlsAcceptor::from(tls_config);

        // Accept TLS connection
        let mut tls_stream = acceptor.accept(stream).await.map_err(|e| NetworkError::SslError(e.to_string()))?;

        // Read request with dynamic buffer optimization
        let mut buffer = Vec::with_capacity(4096);
        let mut buf = [0; 4096];
        loop {
            let n = tls_stream.read(&mut buf).await.map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&buf[..n]);
            // 动态调整缓冲区大小，避免频繁分配
            if buffer.capacity() - buffer.len() < 2048 {
                buffer.reserve(4096);
            }
            // Check if we've read the entire request (ending with \r\n\r\n)
            if buffer.len() >= 4 && &buffer[buffer.len()-4..] == b"\r\n\r\n" {
                break;
            }
        }
        
        if buffer.is_empty() {
            return Ok(());
        }

        // Parse request
        let request = Self::parse_request(&buffer)?;

        // Handle request
        let response = handler(&request);

        // Send response with buffer optimization
        let response_message = response.to_http_message();
        let response_bytes = response_message.as_bytes();
        let mut total_written = 0;
        while total_written < response_bytes.len() {
            let n = tls_stream.write(&response_bytes[total_written..]).await.map_err(|e| NetworkError::Io(e))?;
            if n == 0 {
                return Err(NetworkError::ConnectionError("Connection closed".to_string()));
            }
            total_written += n;
        }
        tls_stream.flush().await.map_err(|e| NetworkError::Io(e))?;

        Ok(())
    }



    /// Parse request
    fn parse_request(buffer: &[u8]) -> Result<Request, NetworkError> {
        let request_str = std::str::from_utf8(buffer).map_err(|_| NetworkError::InvalidRequest("Invalid request format".to_string()))?;
        
        // Split request into status line, headers, and body
        let parts: Vec<&str> = request_str.split("\r\n\r\n").collect();
        if parts.is_empty() {
            return Err(NetworkError::InvalidRequest("Invalid request format".to_string()));
        }
        
        let header_part = parts[0];
        let body_part = if parts.len() > 1 {
            parts[1..].join("\r\n\r\n")
        } else {
            "".to_string()
        };
        
        // Parse status line
        let lines: Vec<&str> = header_part.lines().collect();
        if lines.is_empty() {
            return Err(NetworkError::InvalidRequest("Invalid request format".to_string()));
        }
        
        let status_line = lines[0];
        let status_parts: Vec<&str> = status_line.split_whitespace().collect();
        if status_parts.len() < 3 {
            return Err(NetworkError::InvalidRequest("Invalid status line".to_string()));
        }
        
        let method = Method::from_str(status_parts[0]).map_err(|_| NetworkError::InvalidRequest("Invalid method".to_string()))?;
        let path = status_parts[1];
        
        // Parse path and query parameters
        let (path, query) = if let Some((p, q)) = path.split_once('?') {
            (p, q)
        } else {
            (path, "")
        };
        
        // Parse query parameters
        let mut query_params = std::collections::HashMap::new();
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                query_params.insert(key.to_string(), value.to_string());
            }
        }
        
        // Parse headers
        let mut headers = Headers::new();
        for line in &lines[1..] {
            if let Some((name, value)) = line.split_once(": ") {
                headers.add(name, value.trim());
            }
        }
        
        // Create request
        let mut request = Request::new(method, path);
        request.query = query_params;
        request.headers = headers;
        request.set_body(body_part.as_bytes());
        
        Ok(request)
    }
}
