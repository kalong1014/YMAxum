/// 连接器模块
///
/// 负责与Godot引擎的连接和通信
use serde_json::Value;

/// 连接器接口
#[async_trait::async_trait]
pub trait Connector: Send + Sync {
    /// 初始化连接器
    async fn initialize(&self) -> Result<(), crate::error::Error>;

    /// 发送消息到Godot
    async fn send_message(&self, message: Value) -> Result<Value, crate::error::Error>;

    /// 接收来自Godot的消息
    async fn receive_message(&self) -> Result<Value, crate::error::Error>;

    /// 关闭连接器
    async fn close(&self) -> Result<(), crate::error::Error>;
}

/// 默认连接器实现
pub struct DefaultConnector {
    //
}

impl DefaultConnector {
    /// 创建新的默认连接器
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Connector for DefaultConnector {
    async fn initialize(&self) -> Result<(), crate::error::Error> {
        // 初始化默认连接器
        log::info!("Default connector initialized");
        Ok(())
    }

    async fn send_message(&self, message: Value) -> Result<Value, crate::error::Error> {
        // 模拟发送消息
        log::debug!("Sending message to Godot: {:?}", message);
        Ok(Value::Null)
    }

    async fn receive_message(&self) -> Result<Value, crate::error::Error> {
        // 模拟接收消息
        log::debug!("Receiving message from Godot");
        Ok(Value::Null)
    }

    async fn close(&self) -> Result<(), crate::error::Error> {
        // 关闭连接
        log::info!("Default connector closed");
        Ok(())
    }
}

/// 全局连接器实例
static CONNECTOR: tokio::sync::OnceCell<Box<dyn Connector>> = tokio::sync::OnceCell::const_new();

/// 获取连接器实例
pub async fn get_connector() -> &'static Box<dyn Connector> {
    CONNECTOR
        .get_or_init(|| async { Box::new(DefaultConnector::new()) as Box<dyn Connector> })
        .await
}

/// 初始化连接器系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化连接器
    let connector = get_connector().await;
    connector.initialize().await
}

/// 发送消息到Godot
pub async fn send_message(message: Value) -> Result<Value, crate::error::Error> {
    let connector = get_connector().await;
    connector.send_message(message).await
}

/// 接收来自Godot的消息
pub async fn receive_message() -> Result<Value, crate::error::Error> {
    let connector = get_connector().await;
    connector.receive_message().await
}

/// 关闭连接器
pub async fn close() -> Result<(), crate::error::Error> {
    let connector = get_connector().await;
    connector.close().await
}
