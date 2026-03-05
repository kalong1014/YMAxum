//! 事件管理模块
//! 负责事件的定义、触发和处理

use super::ServerlessConfig;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 事件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    /// 初始化中
    Initializing,
    /// 就绪
    Ready,
    /// 触发中
    Triggering,
    /// 失败
    Failed,
    /// 已停止
    Stopped,
}

/// 事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// 事件名称
    pub name: String,
    /// 事件类型
    pub r#type: String,
    /// 事件数据
    pub data: serde_json::Value,
    /// 事件时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 事件结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    /// 事件名称
    pub event_name: String,
    /// 处理状态
    pub status: String,
    /// 处理结果
    pub result: Option<serde_json::Value>,
    /// 处理时间（毫秒）
    pub duration: u64,
}

/// 事件处理器
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// 处理事件
    async fn handle_event(
        &self,
        event: &Event,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}

/// 事件管理器
#[derive(Clone)]
pub struct EventManager {
    /// 配置
    config: Arc<ServerlessConfig>,
    /// 事件存储
    events: Arc<RwLock<HashMap<String, super::EventConfig>>>,
    /// 事件处理器
    handlers: Arc<RwLock<HashMap<String, Box<dyn EventHandler>>>>,
}

impl EventManager {
    /// 创建新的事件管理器
    pub fn new(config: Arc<ServerlessConfig>) -> Self {
        Self {
            config,
            events: Arc::new(RwLock::new(HashMap::new())),
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 初始化事件管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("初始化事件管理器");

        // 注册配置中的事件
        for event_config in &self.config.events {
            self.register_event(event_config.clone()).await?;
        }

        // 注册默认事件处理器
        self.register_default_handlers().await;

        Ok(())
    }

    /// 注册事件
    pub async fn register_event(
        &self,
        event_config: super::EventConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("注册事件: {}", event_config.name);

        let mut events = self.events.write().await;
        events.insert(event_config.name.clone(), event_config);

        Ok(())
    }

    /// 注册默认事件处理器
    async fn register_default_handlers(&self) {
        // 注册HTTP事件处理器
        self.register_handler("http", Box::new(HttpEventHandler {}))
            .await;

        // 注册定时事件处理器
        self.register_handler("schedule", Box::new(ScheduleEventHandler {}))
            .await;

        // 注册消息事件处理器
        self.register_handler("message", Box::new(MessageEventHandler {}))
            .await;

        // 注册存储事件处理器
        self.register_handler("storage", Box::new(StorageEventHandler {}))
            .await;
    }

    /// 注册事件处理器
    pub async fn register_handler(&self, event_type: &str, handler: Box<dyn EventHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.insert(event_type.to_string(), handler);
    }

    /// 触发事件
    pub async fn trigger_event(
        &self,
        event: Event,
    ) -> Result<EventResult, Box<dyn std::error::Error>> {
        info!("触发事件: {}, 类型: {}", event.name, event.r#type);

        // 检查事件是否存在
        let events = self.events.read().await;
        let _event_config = events
            .get(&event.name)
            .ok_or_else(|| format!("事件不存在: {}", event.name))?;

        // 查找事件处理器
        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(&event.r#type)
            .ok_or_else(|| format!("事件处理器不存在: {}", event.r#type))?;

        // 记录开始时间
        let start_time = tokio::time::Instant::now();

        // 处理事件
        let result = handler.handle_event(&event).await;

        // 计算处理时间
        let duration = start_time.elapsed().as_millis() as u64;

        // 构建事件结果
        let event_result = match result {
            Ok(data) => EventResult {
                event_name: event.name.clone(),
                status: "success".to_string(),
                result: Some(data),
                duration,
            },
            Err(e) => EventResult {
                event_name: event.name.clone(),
                status: "failed".to_string(),
                result: Some(serde_json::json!({ "error": e.to_string() })),
                duration,
            },
        };

        info!(
            "事件处理完成: {}, 状态: {}, 耗时: {}ms",
            event.name, event_result.status, duration
        );
        Ok(event_result)
    }

    /// 获取事件状态
    pub async fn get_event_status(
        &self,
        event_name: &str,
    ) -> Result<EventStatus, Box<dyn std::error::Error>> {
        let events = self.events.read().await;
        if events.contains_key(event_name) {
            Ok(EventStatus::Ready)
        } else {
            Err(format!("事件不存在: {}", event_name).into())
        }
    }

    /// 列出所有事件
    pub async fn list_events(&self) -> Result<Vec<super::EventConfig>, Box<dyn std::error::Error>> {
        let events = self.events.read().await;
        Ok(events.values().cloned().collect())
    }

    /// 删除事件
    pub async fn delete_event(&self, event_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("删除事件: {}", event_name);

        let mut events = self.events.write().await;
        if events.remove(event_name).is_none() {
            return Err(format!("事件不存在: {}", event_name).into());
        }

        Ok(())
    }
}

/// HTTP事件处理器
#[derive(Debug, Clone)]
struct HttpEventHandler {}

#[async_trait::async_trait]
impl EventHandler for HttpEventHandler {
    async fn handle_event(
        &self,
        event: &Event,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("处理HTTP事件: {}", event.name);

        // 模拟HTTP事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(serde_json::json!({
            "status": "success",
            "message": "HTTP事件处理完成",
            "event_data": event.data
        }))
    }
}

/// 定时事件处理器
#[derive(Debug, Clone)]
struct ScheduleEventHandler {}

#[async_trait::async_trait]
impl EventHandler for ScheduleEventHandler {
    async fn handle_event(
        &self,
        event: &Event,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("处理定时事件: {}", event.name);

        // 模拟定时事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(serde_json::json!({
            "status": "success",
            "message": "定时事件处理完成",
            "event_data": event.data
        }))
    }
}

/// 消息事件处理器
#[derive(Debug, Clone)]
struct MessageEventHandler {}

#[async_trait::async_trait]
impl EventHandler for MessageEventHandler {
    async fn handle_event(
        &self,
        event: &Event,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("处理消息事件: {}", event.name);

        // 模拟消息事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(serde_json::json!({
            "status": "success",
            "message": "消息事件处理完成",
            "event_data": event.data
        }))
    }
}

/// 存储事件处理器
#[derive(Debug, Clone)]
struct StorageEventHandler {}

#[async_trait::async_trait]
impl EventHandler for StorageEventHandler {
    async fn handle_event(
        &self,
        event: &Event,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("处理存储事件: {}", event.name);

        // 模拟存储事件处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(serde_json::json!({
            "status": "success",
            "message": "存储事件处理完成",
            "event_data": event.data
        }))
    }
}
