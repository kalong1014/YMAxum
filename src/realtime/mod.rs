//! 实时通信功能模块
//! 用于WebSocket支持、实时消息推送、多人协作编辑

pub mod collaborative_editing;
pub mod message_push;
pub mod websocket;

/// 实时通信管理器
#[derive(Debug, Clone)]
pub struct RealtimeCommunicationManager {
    websocket: websocket::WebSocketServer,
    message_push: message_push::MessagePushService,
    collaborative_editing: collaborative_editing::CollaborativeEditingService,
}

impl RealtimeCommunicationManager {
    /// 创建新的实时通信管理器
    pub fn new() -> Self {
        Self {
            websocket: websocket::WebSocketServer::new(),
            message_push: message_push::MessagePushService::new(),
            collaborative_editing: collaborative_editing::CollaborativeEditingService::new(),
        }
    }

    /// 初始化实时通信
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.websocket.initialize().await?;
        self.message_push.initialize().await?;
        self.collaborative_editing.initialize().await?;
        Ok(())
    }

    /// 处理WebSocket连接
    pub async fn handle_websocket(
        &self,
        connection: websocket::WebSocketConnection,
        ws: axum::extract::ws::WebSocket,
    ) -> Result<websocket::WebSocketConnectionResult, Box<dyn std::error::Error>> {
        self.websocket.handle_connection(connection, ws).await
    }

    /// 推送实时消息
    pub async fn push_message(
        &self,
        message: message_push::PushMessage,
    ) -> Result<message_push::PushResult, Box<dyn std::error::Error>> {
        self.message_push.push_message(message).await
    }

    /// 处理协作编辑操作
    pub async fn handle_collaborative_edit(
        &self,
        edit: collaborative_editing::EditOperation,
    ) -> Result<collaborative_editing::EditResult, Box<dyn std::error::Error>> {
        self.collaborative_editing.handle_edit(edit).await
    }
}
