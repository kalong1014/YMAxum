//! 插件通信器模块
//! 负责插件间的通信和事件处理

use chrono;
use log::{info, error, debug};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid;

/// 消息状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStatus {
    /// 待发送
    Pending,
    /// 发送中
    Sending,
    /// 已发送
    Sent,
    /// 已确认
    Confirmed,
    /// 失败
    Failed,
}

/// 插件消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMessage {
    /// 消息ID
    pub id: String,
    /// 发送插件
    pub from: String,
    /// 接收插件
    pub to: String,
    /// 消息类型
    pub r#type: String,
    /// 消息数据
    pub data: serde_json::Value,
    /// 消息时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 消息状态
    pub status: MessageStatus,
    /// 重试次数
    pub retry_count: u32,
    /// 超时时间（秒）
    pub timeout: u32,
}

/// 插件事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    /// 事件ID
    pub id: String,
    /// 事件名称
    pub name: String,
    /// 发送插件
    pub from: String,
    /// 事件数据
    pub data: serde_json::Value,
    /// 事件时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 事件回调类型
pub type EventCallback = Arc<dyn Fn(serde_json::Value) + Send + Sync>;

/// 插件通信器
#[derive(Clone)]
pub struct GufPluginCommunicator {
    /// 消息通道
    message_channels: Arc<
        tokio::sync::RwLock<
            std::collections::HashMap<String, tokio::sync::mpsc::Sender<PluginMessage>>,
        >,
    >,
    /// 事件订阅
    event_subscriptions: Arc<
        tokio::sync::RwLock<
            std::collections::HashMap<
                String,
                std::collections::HashMap<String, Vec<EventCallback>>,
            >,
        >,
    >,
    /// 消息历史
    message_history: Arc<tokio::sync::RwLock<std::collections::VecDeque<PluginMessage>>>,
    /// 消息确认
    message_confirmations: Arc<tokio::sync::RwLock<std::collections::HashMap<String, tokio::sync::oneshot::Sender<serde_json::Value>>>>,
    /// 消息重试队列
    message_retry_queue: Arc<Mutex<std::collections::VecDeque<(PluginMessage, tokio::time::Instant)>>>,
    /// 消息路由表
    message_routes: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<String>>>>,
}

impl GufPluginCommunicator {
    /// 创建新的插件通信器
    pub fn new() -> Self {
        let communicator = Self {
            message_channels: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            event_subscriptions: Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
            message_history: Arc::new(tokio::sync::RwLock::new(
                std::collections::VecDeque::with_capacity(1000),
            )),
            message_confirmations: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            message_retry_queue: Arc::new(Mutex::new(std::collections::VecDeque::new())),
            message_routes: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        };
        
        // 启动消息重试任务
        let communicator_clone = communicator.clone();
        tokio::spawn(async move {
            communicator_clone.process_retry_queue().await;
        });
        
        communicator
    }

    /// 初始化插件通信器
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件通信器");
        Ok(())
    }

    /// 注册消息通道
    pub async fn register_message_channel(
        &self,
        plugin_name: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<PluginMessage>, String> {
        info!("注册消息通道: {}", plugin_name);

        let (sender, receiver) = tokio::sync::mpsc::channel(100);

        let mut message_channels = self.message_channels.write().await;
        message_channels.insert(plugin_name.to_string(), sender);

        Ok(receiver)
    }

    /// 发送消息
    pub async fn send_message(
        &self,
        from_plugin: &str,
        to_plugin: &str,
        message: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        info!("发送消息从 {} 到 {}", from_plugin, to_plugin);

        // 检查目标插件是否存在
        let message_channels = self.message_channels.read().await;
        let sender = message_channels
            .get(to_plugin)
            .ok_or_else(|| format!("目标插件不存在: {}", to_plugin))?;

        // 创建消息
        let mut plugin_message = PluginMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: from_plugin.to_string(),
            to: to_plugin.to_string(),
            r#type: "request".to_string(),
            data: message,
            timestamp: chrono::Utc::now(),
            status: MessageStatus::Pending,
            retry_count: 0,
            timeout: 30,
        };

        // 创建消息确认通道
        let (confirm_sender, confirm_receiver) = tokio::sync::oneshot::channel();
        
        // 保存确认通道
        let mut message_confirmations = self.message_confirmations.write().await;
        message_confirmations.insert(plugin_message.id.clone(), confirm_sender);
        drop(message_confirmations);

        // 发送消息
        plugin_message.status = MessageStatus::Sending;
        let send_result = sender
            .send(plugin_message.clone())
            .await;

        match send_result {
            Ok(_) => {
                plugin_message.status = MessageStatus::Sent;
                info!("消息发送成功: {}", plugin_message.id);
                
                // 等待确认
                let confirm_result = tokio::time::timeout(
                    Duration::from_secs(plugin_message.timeout as u64),
                    confirm_receiver
                ).await;
                
                match confirm_result {
                    Ok(Ok(confirm_data)) => {
                        plugin_message.status = MessageStatus::Confirmed;
                        info!("消息确认成功: {}", plugin_message.id);
                        
                        // 记录消息历史
                        let mut message_history = self.message_history.write().await;
                        message_history.push_back(plugin_message);
                        if message_history.len() > 1000 {
                            message_history.pop_front();
                        }
                        
                        Ok(confirm_data)
                    }
                    Ok(Err(_)) => {
                        plugin_message.status = MessageStatus::Failed;
                        error!("消息确认通道关闭: {}", plugin_message.id);
                        self.add_to_retry_queue(plugin_message).await;
                        Err("消息确认失败".to_string())
                    }
                    Err(_) => {
                        plugin_message.status = MessageStatus::Failed;
                        error!("消息确认超时: {}", plugin_message.id);
                        self.add_to_retry_queue(plugin_message).await;
                        Err("消息确认超时".to_string())
                    }
                }
            }
            Err(e) => {
                plugin_message.status = MessageStatus::Failed;
                error!("消息发送失败: {} - {}", plugin_message.id, e);
                self.add_to_retry_queue(plugin_message).await;
                Err(format!("消息发送失败: {}", e))
            }
        }
    }

    /// 处理消息重试队列
    async fn process_retry_queue(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            // 先获取需要重试的消息，然后释放锁
            let messages_to_retry = {
                let mut retry_queue = self.message_retry_queue.lock().unwrap();
                let now = tokio::time::Instant::now();
                
                // 处理到期的重试消息
                let mut messages = Vec::new();
                
                while let Some((_message, retry_time)) = retry_queue.front() {
                    if now >= *retry_time {
                        let message = retry_queue.pop_front().unwrap().0;
                        messages.push(message);
                    } else {
                        break;
                    }
                }
                
                messages
            };
            
            // 重试消息
            for mut message in messages_to_retry {
                if message.retry_count >= 3 {
                    error!("消息重试次数达到上限，放弃重试: {}", message.id);
                    continue;
                }
                
                message.retry_count += 1;
                message.status = MessageStatus::Sending;
                
                debug!("重试消息: {} ({}次)", message.id, message.retry_count);
                
                let message_channels = self.message_channels.read().await;
                if let Some(sender) = message_channels.get(&message.to) {
                    let send_result = sender.send(message.clone()).await;
                    if send_result.is_err() {
                        error!("消息重试失败: {} - {:?}", message.id, send_result.err());
                        self.add_to_retry_queue(message).await;
                    }
                }
            }
        }
    }

    /// 添加消息到重试队列
    async fn add_to_retry_queue(&self, message: PluginMessage) {
        let message_id = message.id.clone();
        let retry_delay = Duration::from_secs(((message.retry_count + 1) * 5).into());
        let retry_time = tokio::time::Instant::now() + retry_delay;
        
        let mut retry_queue = self.message_retry_queue.lock().unwrap();
        retry_queue.push_back((message, retry_time));
        debug!("消息添加到重试队列: {}", message_id);
    }

    /// 确认消息
    pub async fn confirm_message(&self, message_id: &str, data: serde_json::Value) -> Result<(), String> {
        let mut message_confirmations = self.message_confirmations.write().await;
        if let Some(sender) = message_confirmations.remove(message_id) {
            if sender.send(data).is_err() {
                error!("消息确认发送失败: {}", message_id);
                return Err("消息确认发送失败".to_string());
            }
            info!("消息确认发送成功: {}", message_id);
            Ok(())
        } else {
            Err(format!("消息确认通道不存在: {}", message_id))
        }
    }

    /// 发布事件
    pub async fn publish_event(
        &self,
        plugin_name: &str,
        event: &str,
        data: serde_json::Value,
    ) -> Result<(), String> {
        info!("发布事件: {} 来自 {}", event, plugin_name);

        // 创建事件
        let plugin_event = PluginEvent {
            id: uuid::Uuid::new_v4().to_string(),
            name: event.to_string(),
            from: plugin_name.to_string(),
            data: data.clone(),
            timestamp: chrono::Utc::now(),
        };

        // 通知所有订阅者
        let event_subscriptions = self.event_subscriptions.read().await;
        if let Some(subscribers) = event_subscriptions.get(event) {
            for (subscriber_name, callbacks) in subscribers {
                info!("通知订阅者: {} 事件: {}", subscriber_name, event);
                for callback in callbacks {
                    // 异步执行回调
                    let data_clone = data.clone();
                    let callback_clone = callback.clone();
                    tokio::spawn(async move {
                        (callback_clone)(data_clone);
                    });
                }
            }
        }

        info!("事件发布完成: {}", plugin_event.id);
        Ok(())
    }

    /// 订阅事件
    pub async fn subscribe_event(
        &self,
        plugin_name: &str,
        event: &str,
        callback: EventCallback,
    ) -> Result<(), String> {
        info!("订阅事件: {} 插件: {}", event, plugin_name);

        let mut event_subscriptions = self.event_subscriptions.write().await;
        let subscribers = event_subscriptions
            .entry(event.to_string())
            .or_insert_with(std::collections::HashMap::new);
        let callbacks = subscribers
            .entry(plugin_name.to_string())
            .or_insert_with(Vec::new);
        callbacks.push(callback);

        info!("事件订阅成功: {} 插件: {}", event, plugin_name);
        Ok(())
    }

    /// 取消订阅事件
    pub async fn unsubscribe_event(&self, plugin_name: &str, event: &str) -> Result<(), String> {
        info!("取消订阅事件: {} 插件: {}", event, plugin_name);

        let mut event_subscriptions = self.event_subscriptions.write().await;
        if let Some(subscribers) = event_subscriptions.get_mut(event) {
            subscribers.remove(plugin_name);
            // 如果事件没有订阅者，删除事件
            if subscribers.is_empty() {
                event_subscriptions.remove(event);
            }
        }

        info!("事件取消订阅成功: {} 插件: {}", event, plugin_name);
        Ok(())
    }

    /// 获取消息历史
    pub async fn get_message_history(&self, limit: usize) -> Result<Vec<PluginMessage>, String> {
        let message_history = self.message_history.read().await;
        let history: Vec<PluginMessage> =
            message_history.iter().rev().take(limit).cloned().collect();
        Ok(history)
    }

    /// 获取事件订阅列表
    pub async fn get_event_subscriptions(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
        let event_subscriptions = self.event_subscriptions.read().await;
        let mut subscriptions = std::collections::HashMap::new();

        for (event, subscribers) in event_subscriptions.iter() {
            let subscriber_names: Vec<String> = subscribers.keys().cloned().collect();
            subscriptions.insert(event.clone(), subscriber_names);
        }

        Ok(subscriptions)
    }

    /// 检查消息通道是否存在
    pub async fn message_channel_exists(&self, plugin_name: &str) -> bool {
        let message_channels = self.message_channels.read().await;
        message_channels.contains_key(plugin_name)
    }

    /// 移除消息通道
    pub async fn remove_message_channel(&self, plugin_name: &str) -> Result<(), String> {
        info!("移除消息通道: {}", plugin_name);

        let mut message_channels = self.message_channels.write().await;
        if message_channels.remove(plugin_name).is_none() {
            return Err(format!("消息通道不存在: {}", plugin_name));
        }

        // 同时移除该插件的所有事件订阅
        let mut event_subscriptions = self.event_subscriptions.write().await;
        // 收集需要删除的事件
        let mut events_to_remove = Vec::new();

        for (event, subscribers) in event_subscriptions.iter_mut() {
            subscribers.remove(plugin_name);
            // 如果事件没有订阅者，标记为删除
            if subscribers.is_empty() {
                events_to_remove.push(event.clone());
            }
        }

        // 删除标记的事件
        for event in events_to_remove {
            event_subscriptions.remove(&event);
        }

        // 移除消息路由
        let mut message_routes = self.message_routes.write().await;
        message_routes.remove(plugin_name);
        // 从其他路由中移除该插件
        for (_, routes) in message_routes.iter_mut() {
            routes.retain(|route| route != plugin_name);
        }

        info!("消息通道移除成功: {}", plugin_name);
        Ok(())
    }

    /// 添加消息路由
    pub async fn add_message_route(&self, plugin_name: &str, route: &str) -> Result<(), String> {
        info!("添加消息路由: {} -> {}", plugin_name, route);
        
        let mut message_routes = self.message_routes.write().await;
        let routes = message_routes.entry(plugin_name.to_string()).or_insert_with(Vec::new);
        
        if !routes.contains(&route.to_string()) {
            routes.push(route.to_string());
        }
        
        Ok(())
    }

    /// 移除消息路由
    pub async fn remove_message_route(&self, plugin_name: &str, route: &str) -> Result<(), String> {
        info!("移除消息路由: {} -> {}", plugin_name, route);
        
        let mut message_routes = self.message_routes.write().await;
        if let Some(routes) = message_routes.get_mut(plugin_name) {
            routes.retain(|r| r != route);
            if routes.is_empty() {
                message_routes.remove(plugin_name);
            }
        }
        
        Ok(())
    }

    /// 获取消息路由
    pub async fn get_message_routes(&self, plugin_name: &str) -> Result<Vec<String>, String> {
        let message_routes = self.message_routes.read().await;
        match message_routes.get(plugin_name) {
            Some(routes) => Ok(routes.clone()),
            None => Ok(Vec::new()),
        }
    }

    /// 广播消息
    pub async fn broadcast_message(&self, from_plugin: &str, message_type: &str, data: serde_json::Value) -> Result<Vec<Result<serde_json::Value, String>>, String> {
        info!("广播消息: {} 类型: {}", from_plugin, message_type);
        
        let message_channels = self.message_channels.read().await;
        let mut results = Vec::new();
        
        for (plugin_name, sender) in message_channels.iter() {
            if plugin_name == from_plugin {
                continue;
            }
            
            let message = PluginMessage {
                id: uuid::Uuid::new_v4().to_string(),
                from: from_plugin.to_string(),
                to: plugin_name.clone(),
                r#type: message_type.to_string(),
                data: data.clone(),
                timestamp: chrono::Utc::now(),
                status: MessageStatus::Pending,
                retry_count: 0,
                timeout: 30,
            };
            
            let send_result = sender.send(message.clone()).await;
            match send_result {
                Ok(_) => {
                    results.push(Ok(serde_json::json!({
                        "message_id": message.id,
                        "status": "success",
                        "to": plugin_name
                    })));
                }
                Err(e) => {
                    results.push(Err(format!("发送失败到 {}: {}", plugin_name, e)));
                }
            }
        }
        
        Ok(results)
    }

    /// 批量发送消息
    pub async fn send_batch_messages(&self, from_plugin: &str, messages: Vec<(String, serde_json::Value)>) -> Result<Vec<Result<serde_json::Value, String>>, String> {
        info!("批量发送消息: {} 数量: {}", from_plugin, messages.len());
        
        let mut results = Vec::new();
        
        for (to_plugin, data) in messages {
            let result = self.send_message(from_plugin, &to_plugin, data).await;
            results.push(result);
        }
        
        Ok(results)
    }

    /// 获取消息统计
    pub async fn get_message_stats(&self) -> Result<serde_json::Value, String> {
        let message_history = self.message_history.read().await;
        let message_confirmations = self.message_confirmations.read().await;
        let retry_queue = self.message_retry_queue.lock().unwrap();
        
        let mut stats = serde_json::json!({
            "total_messages": message_history.len(),
            "pending_confirmations": message_confirmations.len(),
            "retry_queue_size": retry_queue.len(),
            "status_counts": serde_json::json!({})
        });
        
        // 统计各状态消息数量
        let mut status_counts = std::collections::HashMap::new();
        for message in message_history.iter() {
            let status_str = format!("{:?}", message.status);
            *status_counts.entry(status_str).or_insert(0) += 1;
        }
        
        stats["status_counts"] = serde_json::to_value(status_counts).unwrap();
        
        Ok(stats)
    }
}
