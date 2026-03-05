//! WebSocket支持模块
//! 用于WebSocket连接的处理和管理

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket};
use chrono;

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

/// WebSocket连接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConnection {
    /// 连接ID
    pub connection_id: String,
    /// 客户端信息
    pub client_info: ClientInfo,
    /// 连接参数
    pub connection_params: serde_json::Value,
    /// 连接时间
    pub connection_time: String,
}

/// 客户端信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// 客户端ID
    pub client_id: String,
    /// 客户端类型
    pub client_type: String,
    /// 客户端版本
    pub client_version: String,
    /// 客户端IP
    pub client_ip: String,
    /// 浏览器信息
    pub browser_info: Option<String>,
}

/// WebSocket连接结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConnectionResult {
    /// 连接状态
    pub status: String,
    /// 连接ID
    pub connection_id: String,
    /// 会话ID
    pub session_id: String,
    /// 连接时间
    pub connection_time: String,
    /// 服务器信息
    pub server_info: ServerInfo,
}

/// 服务器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// 服务器ID
    pub server_id: String,
    /// 服务器版本
    pub server_version: String,
    /// 支持的协议
    pub supported_protocols: Vec<String>,
    /// 最大消息大小
    pub max_message_size: u32,
}

/// WebSocket消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    /// 消息ID
    pub message_id: String,
    /// 连接ID
    pub connection_id: String,
    /// 消息类型
    pub message_type: String,
    /// 消息内容
    pub content: serde_json::Value,
    /// 发送时间
    pub timestamp: String,
}

/// WebSocket连接句柄
#[derive(Debug, Clone)]
pub struct WebSocketConnectionHandle {
    /// 连接ID
    pub connection_id: String,
    /// 发送通道
    pub tx: mpsc::Sender<Message>,
    /// 客户端信息
    pub client_info: ClientInfo,
    /// 加入的房间
    pub rooms: Arc<RwLock<Vec<String>>>,
}

/// WebSocket服务器
#[derive(Debug, Clone)]
pub struct WebSocketServer {
    /// 活动连接映射
    active_connections: Arc<RwLock<std::collections::HashMap<String, WebSocketConnectionHandle>>>,
    /// 房间映射
    rooms: Arc<RwLock<std::collections::HashMap<String, Vec<String>>>>,
    /// 消息历史
    message_history: Arc<RwLock<Vec<WebSocketMessage>>>,
    /// 心跳间隔（毫秒）
    _heartbeat_interval: u64,
}

impl WebSocketServer {
    /// 创建新的WebSocket服务器
    pub fn new() -> Self {
        Self {
            active_connections: Arc::new(RwLock::new(std::collections::HashMap::new())),
            rooms: Arc::new(RwLock::new(std::collections::HashMap::new())),
            message_history: Arc::new(RwLock::new(Vec::new())),
            _heartbeat_interval: 30,
        }
    }

    /// 初始化WebSocket服务器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化WebSocket服务器模块
        info!("Initializing WebSocket server module...");
        Ok(())
    }

    /// 处理WebSocket连接
    pub async fn handle_connection(
        &self,
        connection: WebSocketConnection,
        mut ws: WebSocket,
    ) -> Result<WebSocketConnectionResult, Box<dyn std::error::Error>> {
        info!(
            "Handling WebSocket connection: {}",
            connection.connection_id
        );

        // 创建发送和接收通道
        let (tx, mut rx) = mpsc::channel(128);

        // 生成连接结果
        let result = WebSocketConnectionResult {
            status: "connected".to_string(),
            connection_id: connection.connection_id.clone(),
            session_id: format!(
                "session_{}_{}",
                connection.connection_id,
                chrono::Utc::now().timestamp()
            ),
            connection_time: chrono::Utc::now().to_string(),
            server_info: ServerInfo {
                server_id: "server_1".to_string(),
                server_version: "1.0.0".to_string(),
                supported_protocols: vec!["ws".to_string(), "wss".to_string()],
                max_message_size: 1024 * 1024, // 1MB
            },
        };

        // 创建连接句柄
        let handle = WebSocketConnectionHandle {
            connection_id: connection.connection_id.clone(),
            tx: tx.clone(),
            client_info: connection.client_info.clone(),
            rooms: Arc::new(RwLock::new(Vec::new())),
        };

        // 添加到活动连接映射
        let mut active_connections = self.active_connections.write().await;
        active_connections.insert(connection.connection_id.clone(), handle);

        // 启动WebSocket处理任务
        let server = self.clone();
        let connection_id = connection.connection_id.clone();
        tokio::spawn(async move {
            // 处理WebSocket连接
            loop {
                tokio::select! {
                    // 处理接收消息
                    message = ws.recv() => {
                        match message {
                            Some(Ok(Message::Text(text))) => {
                                server.handle_text_message(&connection_id, text).await;
                            }
                            Some(Ok(Message::Binary(_))) => {
                                warn!("Received binary message, not supported yet");
                            }
                            Some(Ok(Message::Ping(_))) => {
                                // 响应心跳
                                if let Err(e) = ws.send(Message::Pong(Bytes::new())).await {
                                    error!("Failed to send pong: {}", e);
                                    break;
                                }
                            }
                            Some(Ok(Message::Pong(_))) => {
                                // 收到心跳响应
                            }
                            Some(Ok(Message::Close(_))) => {
                                break;
                            }
                            Some(Err(e)) => {
                                error!("WebSocket error: {}", e);
                                break;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    // 处理发送消息
                    message = rx.recv() => {
                        match message {
                            Some(msg) => {
                                if ws.send(msg).await.is_err() {
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
            }

            // 连接关闭
            if let Err(e) = server.close_connection(connection_id.clone()).await {
                error!("Error closing connection: {}", e);
            }
        });

        Ok(result)
    }

    /// 处理文本消息
    async fn handle_text_message(&self, connection_id: &str, text: axum::extract::ws::Utf8Bytes) {
        info!("Received message from {}: {}", connection_id, text);

        // 尝试解析消息
        match serde_json::from_str::<WebSocketMessage>(&text) {
            Ok(message) => {
                // 添加到消息历史
                let mut message_history = self.message_history.write().await;
                message_history.push(message.clone());
                if message_history.len() > 1000 {
                    message_history.remove(0);
                }

                // 处理消息
                self.process_message(connection_id, message).await;
            }
            Err(e) => {
                error!("Failed to parse message: {}", e);
            }
        }
    }

    /// 处理消息
    async fn process_message(&self, connection_id: &str, message: WebSocketMessage) {
        // 根据消息类型处理
        match message.message_type.as_str() {
            "chat" => {
                // 聊天消息
                self.broadcast_message(&message).await;
            }
            "join_room" => {
                // 加入房间
                if let Some(room) = message.content.get("room").and_then(|v| v.as_str()) {
                    self.join_room(connection_id, room).await;
                }
            }
            "leave_room" => {
                // 离开房间
                if let Some(room) = message.content.get("room").and_then(|v| v.as_str()) {
                    self.leave_room(connection_id, room).await;
                }
            }
            "room_message" => {
                // 房间消息
                if let Some(room) = message.content.get("room").and_then(|v| v.as_str()) {
                    let room = room.to_string();
                    self.send_to_room(&room, message).await;
                }
            }
            _ => {
                // 其他消息类型
                info!("Unknown message type: {}", message.message_type);
            }
        }
    }

    /// 发送WebSocket消息
    pub async fn send_message(
        &self,
        message: WebSocketMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Sending WebSocket message: {} to connection: {}",
            message.message_id, message.connection_id
        );

        // 序列化消息
        let text = serde_json::to_string(&message)?;

        // 获取连接并发送消息
        let active_connections = self.active_connections.read().await;
        if let Some(handle) = active_connections.get(&message.connection_id) {
            if handle.tx.send(Message::Text(text.into())).await.is_err() {
                return Err(format!(
                    "Failed to send message to connection: {}",
                    message.connection_id
                )
                .into());
            }
        } else {
            return Err(format!("Connection not found: {}", message.connection_id).into());
        }

        // 添加到消息历史
        let mut message_history = self.message_history.write().await;
        message_history.push(message);
        if message_history.len() > 1000 {
            message_history.remove(0);
        }

        Ok(())
    }

    /// 广播消息
    async fn broadcast_message(&self, message: &WebSocketMessage) {
        let text = serde_json::to_string(message).unwrap();
        let active_connections = self.active_connections.read().await;

        for (id, handle) in &*active_connections {
            if id != &message.connection_id {
                let _ = handle.tx.send(Message::Text(text.clone().into())).await;
            }
        }
    }

    /// 发送到房间
    async fn send_to_room(&self, room: &str, message: WebSocketMessage) {
        let text = serde_json::to_string(&message).unwrap();
        let rooms = self.rooms.read().await;

        if let Some(connections) = rooms.get(room) {
            let active_connections = self.active_connections.read().await;

            for connection_id in connections {
                if let Some(handle) = active_connections.get(connection_id) {
                    let _ = handle.tx.send(Message::Text(text.clone().into())).await;
                }
            }
        }
    }

    /// 加入房间
    async fn join_room(&self, connection_id: &str, room: &str) {
        info!("Connection {} joining room: {}", connection_id, room);

        // 添加连接到房间
        let mut rooms = self.rooms.write().await;
        let room_connections = rooms.entry(room.to_string()).or_insert_with(Vec::new);
        if !room_connections.contains(&connection_id.to_string()) {
            room_connections.push(connection_id.to_string());
        }

        // 更新连接的房间列表
        let active_connections = self.active_connections.read().await;
        if let Some(handle) = active_connections.get(connection_id) {
            let mut connection_rooms = handle.rooms.write().await;
            if !connection_rooms.contains(&room.to_string()) {
                connection_rooms.push(room.to_string());
            }
        }
    }

    /// 离开房间
    async fn leave_room(&self, connection_id: &str, room: &str) {
        info!("Connection {} leaving room: {}", connection_id, room);

        // 从房间移除连接
        let mut rooms = self.rooms.write().await;
        if let Some(room_connections) = rooms.get_mut(room) {
            room_connections.retain(|id| id != connection_id);
            // 如果房间为空，删除房间
            if room_connections.is_empty() {
                rooms.remove(room);
            }
        }

        // 更新连接的房间列表
        let active_connections = self.active_connections.read().await;
        if let Some(handle) = active_connections.get(connection_id) {
            let mut connection_rooms = handle.rooms.write().await;
            connection_rooms.retain(|r| r != room);
        }
    }

    /// 关闭WebSocket连接
    pub async fn close_connection(
        &self,
        connection_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Closing WebSocket connection: {}", connection_id);

        // 获取连接并移除
        let mut active_connections = self.active_connections.write().await;
        let handle = active_connections.remove(&connection_id);

        if let Some(handle) = handle {
            // 从所有房间移除
            let connection_rooms = handle.rooms.read().await;
            let mut rooms = self.rooms.write().await;

            for room in connection_rooms.iter() {
                if let Some(room_connections) = rooms.get_mut(room) {
                    room_connections.retain(|id| id != &connection_id);
                    // 如果房间为空，删除房间
                    if room_connections.is_empty() {
                        rooms.remove(room);
                    }
                }
            }
        }

        Ok(())
    }

    /// 获取活动连接列表
    pub async fn get_active_connections(
        &self,
    ) -> Result<Vec<WebSocketConnectionResult>, Box<dyn std::error::Error>> {
        let active_connections = self.active_connections.read().await;
        let mut results = Vec::new();

        for handle in (*active_connections).values() {
            let result = WebSocketConnectionResult {
                status: "connected".to_string(),
                connection_id: handle.connection_id.clone(),
                session_id: format!("session_{}", handle.connection_id),
                connection_time: chrono::Utc::now().to_string(),
                server_info: ServerInfo {
                    server_id: "server_1".to_string(),
                    server_version: "1.0.0".to_string(),
                    supported_protocols: vec!["ws".to_string(), "wss".to_string()],
                    max_message_size: 1024 * 1024,
                },
            };
            results.push(result);
        }

        Ok(results)
    }

    /// 获取消息历史
    pub async fn get_message_history(
        &self,
    ) -> Result<Vec<WebSocketMessage>, Box<dyn std::error::Error>> {
        let message_history = self.message_history.read().await;
        Ok(message_history.clone())
    }

    /// 获取房间列表
    pub async fn get_rooms(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let rooms = self.rooms.read().await;
        Ok(rooms.clone())
    }
}
