// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::{error, warn};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::{RwLock, mpsc};

/// 消息处理器类型
type MessageHandler = Arc<dyn Fn(String, Vec<u8>) + Send + Sync>;

#[derive(Clone)]
pub enum ConnectionType {
    TCP,
    UDP,
}

#[derive(Clone)]
pub struct Connection {
    pub id: String,
    pub conn_type: ConnectionType,
    pub addr: SocketAddr,
    pub status: ConnectionStatus,
    pub last_activity: u64,
    pub player_id: Option<String>,
    pub tx: Option<mpsc::Sender<Vec<u8>>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConnectionStatus {
    Connected,
    Authenticated,
    Closed,
}

impl Connection {
    pub fn new(id: &str, conn_type: ConnectionType, addr: SocketAddr) -> Self {
        Self {
            id: id.to_string(),
            conn_type,
            addr,
            status: ConnectionStatus::Connected,
            last_activity: chrono::Utc::now().timestamp() as u64,
            player_id: None,
            tx: None,
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = chrono::Utc::now().timestamp() as u64;
    }

    pub fn set_player_id(&mut self, player_id: &str) {
        self.player_id = Some(player_id.to_string());
        self.status = ConnectionStatus::Authenticated;
    }

    pub fn clear_player_id(&mut self) {
        self.player_id = None;
        self.status = ConnectionStatus::Connected;
    }

    pub fn is_active(&self, timeout: u64) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now - self.last_activity < timeout
    }
}

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    max_connections: u32,
    _tcp_listener: Option<TcpListener>,
    udp_socket: Option<Arc<UdpSocket>>,
    message_handler: Option<MessageHandler>,
}

impl ConnectionManager {
    pub async fn new(max_connections: u32) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            max_connections,
            _tcp_listener: None,
            udp_socket: None,
            message_handler: None,
        })
    }

    pub async fn start_tcp_server(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 在tokio::spawn中启动TCP服务器以避免阻塞
        let connections = self.connections.clone();
        let max_connections = self.max_connections;
        let message_handler = self.message_handler.clone();
        let addr = addr.to_string();

        tokio::spawn(async move {
            let connections_map = connections;
            let max_conns = max_connections;
            let handler = message_handler;

            // spawn新线程以绑定监听器
            let tcp_listener = match TcpListener::bind(&addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    error!("TCP bind error: {}", e);
                    return;
                }
            };

            loop {
                let (stream, addr) = match tcp_listener.accept().await {
                    Ok((stream, addr)) => (stream, addr),
                    Err(e) => {
                        error!("TCP accept error: {}", e);
                        continue;
                    }
                };

                let conn_count = connections_map.read().await.len();
                if conn_count >= max_conns as usize {
                    warn!("Max TCP connections reached, rejecting {}", addr);
                    continue;
                }

                let conn_id = format!("tcp_{}", uuid::Uuid::new_v4());
                let conn = Connection::new(&conn_id, ConnectionType::TCP, addr);

                let (tx, mut rx) = mpsc::channel(100);
                let mut conn_clone = conn.clone();
                conn_clone.tx = Some(tx);

                connections_map
                    .write()
                    .await
                    .insert(conn_id.clone(), conn_clone);

                let listener_clone = connections_map.clone();
                let handler_clone = handler.clone();
                let conn_id_clone = conn_id.clone();

                tokio::spawn(async move {
                    let mut stream = stream;
                    let conn_id = conn_id_clone;
                    let listener = listener_clone;
                    let handler = handler_clone;

                    let mut buffer = [0; 4096];
                    loop {
                        tokio::select! {
                            result = stream.read(&mut buffer) => {
                                match result {
                                    Ok(0) => {
                                        // Connection closed
                                        break;
                                    }
                                    Ok(n) => {
                                        let data = buffer[..n].to_vec();
                                        if let Some(handler) = &handler {
                                            handler(conn_id.clone(), data);
                                        }

                                        // Update last activity
                                        let mut write_guard = listener.write().await;
                                        if let Some(conn) = write_guard.get_mut(&conn_id) {
                                            conn.update_activity();
                                        }
                                    }
                                    Err(e) => {
                                        error!("TCP read error: {}", e);
                                        break;
                                    }
                                }
                            }
                            Some(data) = rx.recv() => {
                                if let Err(e) = stream.write_all(&data).await {
                                    eprintln!("TCP write error: {}", e);
                                    break;
                                }
                            }
                        }
                    }

                    // Remove connection
                    listener.write().await.remove(&conn_id);
                });
            }
        });

        Ok(())
    }

    pub async fn start_udp_server(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr).await?;
        let socket = Arc::new(socket);
        self.udp_socket = Some(socket.clone());

        let connections = self.connections.clone();
        let message_handler = self.message_handler.clone();

        tokio::spawn(async move {
            let socket = socket;
            let listener = connections;
            let handler = message_handler;

            let mut buffer = [0; 4096];
            loop {
                let (n, addr) = match socket.recv_from(&mut buffer).await {
                    Ok((n, addr)) => (n, addr),
                    Err(e) => {
                        eprintln!("UDP recv error: {}", e);
                        continue;
                    }
                };

                let data = buffer[..n].to_vec();

                // Find or create connection
                let conn_id = format!("udp_{}", addr);

                // 简化实现：直接创建新连接，不重复检查
                let conn = Connection::new(&conn_id, ConnectionType::UDP, addr);
                let mut write_guard = listener.write().await;
                let conn_entry = write_guard.entry(conn_id.clone()).or_insert_with(|| conn);
                conn_entry.update_activity();

                if let Some(handler) = &handler {
                    handler(conn_id, data);
                }
            }
        });

        Ok(())
    }

    pub fn set_message_handler(&mut self, handler: Box<dyn Fn(String, Vec<u8>) + Send + Sync>) {
        self.message_handler = Some(Arc::new(handler));
    }

    pub async fn send_message(
        &self,
        conn_id: &str,
        message: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connections = self.connections.read().await;
        if let Some(conn) = connections.get(conn_id) {
            match conn.tx {
                Some(ref tx) => {
                    tx.send(message.to_vec()).await?;
                }
                None => {
                    // UDP connection, send directly
                    if let Some(ref socket) = self.udp_socket {
                        socket.send_to(message, conn.addr).await?;
                    }
                }
            }
            Ok(())
        } else {
            Err("Connection not found".into())
        }
    }

    pub async fn broadcast_message(&self, message: Vec<u8>) {
        let connections = self.connections.read().await;
        for conn in connections.values() {
            let _ = self.send_message(&conn.id, &message).await;
        }
    }

    pub async fn get_connection(&self, conn_id: &str) -> Option<Connection> {
        let connections = self.connections.read().await;
        connections.get(conn_id).cloned()
    }

    pub async fn remove_connection(&self, conn_id: &str) -> bool {
        let mut connections = self.connections.write().await;
        connections.remove(conn_id).is_some()
    }

    pub async fn get_connection_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }

    pub async fn cleanup_inactive_connections(&self, timeout: u64) -> Vec<String> {
        let _now = chrono::Utc::now().timestamp() as u64;
        let mut connections = self.connections.write().await;

        let inactive_ids: Vec<String> = connections
            .values()
            .filter(|conn| !conn.is_active(timeout))
            .map(|conn| conn.id.clone())
            .collect();

        for id in &inactive_ids {
            connections.remove(id);
        }

        inactive_ids
    }
}

