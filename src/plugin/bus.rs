// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件通信总线模块
//! 提供插件间高效通信的机制

use serde::{Deserialize, Serialize};
use serde_json;
use chrono;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use log::info;

/// 插件通信消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMessage {
    /// 消息类型
    pub message_type: String,
    /// 消息数据
    pub data: serde_json::Value,
    /// 发送插件
    pub sender: String,
    /// 目标插件（可选，为空则广播给所有插件）
    pub target: Option<String>,
    /// 消息ID
    pub message_id: String,
    /// 时间戳
    pub timestamp: u64,
    /// 消息签名（用于验证消息真实性）
    pub signature: Option<String>,
    /// 消息优先级
    pub priority: u8, // 0-255，越高优先级越高
}

/// 插件通信总线
#[derive(Debug, Clone)]
pub struct PluginBus {
    /// 消息广播通道
    broadcasters: Arc<RwLock<HashMap<String, broadcast::Sender<PluginMessage>>>>,
    /// 消息监听器
    listeners: Arc<RwLock<HashMap<String, Vec<broadcast::Receiver<PluginMessage>>>>>,
}

impl PluginBus {
    /// 创建新的插件通信总线
    pub fn new() -> Self {
        Self {
            broadcasters: Arc::new(RwLock::new(HashMap::new())),
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 优化消息发送性能，使用批量发送减少锁竞争
    pub async fn send_messages_batch(&self, messages: &[PluginMessage]) -> Result<usize, String> {
        let broadcasters = self.broadcasters.read().await;
        let mut sent_count = 0;
        
        for message in messages {
            if let Some(target) = &message.target {
                if let Some(sender) = broadcasters.get(target) {
                    if sender.send(message.clone()).is_ok() {
                        sent_count += 1;
                    }
                }
            } else {
                // 广播消息
                for (_, sender) in broadcasters.iter() {
                    if sender.send(message.clone()).is_ok() {
                        sent_count += 1;
                    }
                }
            }
        }
        
        info!("批量发送 {} 条消息，成功 {} 条", messages.len(), sent_count);
        Ok(sent_count)
    }

    /// 注册插件到通信总线
    pub async fn register_plugin(&self, plugin_name: &str) {
        let mut broadcasters = self.broadcasters.write().await;
        let mut listeners = self.listeners.write().await;

        // 为插件创建广播通道
        let (sender, _) = broadcast::channel(1000);
        broadcasters.insert(plugin_name.to_string(), sender);

        // 初始化插件的监听器列表
        listeners.insert(plugin_name.to_string(), Vec::new());
    }

    /// 注销插件从通信总线
    pub async fn unregister_plugin(&self, plugin_name: &str) {
        let mut broadcasters = self.broadcasters.write().await;
        let mut listeners = self.listeners.write().await;

        broadcasters.remove(plugin_name);
        listeners.remove(plugin_name);
    }

    /// 发送消息到指定插件
    pub async fn send_message(&self, message: PluginMessage) -> Result<(), String> {
        // 验证消息格式
        if message.message_type.is_empty() {
            return Err("Message type cannot be empty".to_string());
        }

        if message.sender.is_empty() {
            return Err("Sender cannot be empty".to_string());
        }

        // 验证消息ID
        if message.message_id.is_empty() {
            return Err("Message ID cannot be empty".to_string());
        }

        // 验证消息签名（如果提供）
        if let Some(signature) = &message.signature {
            // 这里可以添加签名验证逻辑
            info!("Message signature provided: {}", signature);
        }

        let broadcasters = self.broadcasters.read().await;

        if let Some(target) = &message.target {
            // 发送给指定插件
            if let Some(sender) = broadcasters.get(target) {
                match sender.send(message.clone()) {
                    Ok(_) => {
                        info!("Message sent to plugin {}: {} (ID: {})", target, message.message_type, message.message_id);
                        Ok(())
                    },
                    Err(e) => Err(format!("Failed to send message: {:?}", e)),
                }
            } else {
                Err(format!("Target plugin {} not found", target))
            }
        } else {
            // 广播给所有插件
            let mut sent_count = 0;
            for (_plugin_name, sender) in broadcasters.iter() {
                if let Ok(_) = sender.send(message.clone()) {
                    sent_count += 1;
                }
            }
            info!("Message broadcasted to {} plugins: {} (ID: {})", sent_count, message.message_type, message.message_id);
            Ok(())
        }
    }

    /// 订阅指定类型的消息
    pub async fn subscribe(&self, plugin_name: &str, message_type: &str) -> Result<broadcast::Receiver<PluginMessage>, String> {
        let broadcasters = self.broadcasters.read().await;
        let mut listeners = self.listeners.write().await;

        if let Some(sender) = broadcasters.get(plugin_name) {
            let receiver = sender.subscribe();
            
            if let Some(plugin_listeners) = listeners.get_mut(plugin_name) {
                plugin_listeners.push(sender.subscribe());
                info!("Plugin {} subscribed to message type: {}", plugin_name, message_type);
            }
            
            Ok(receiver)
        } else {
            Err(format!("Plugin {} not registered", plugin_name))
        }
    }

    /// 取消订阅
    pub async fn unsubscribe(&self, plugin_name: &str) -> Result<(), String> {
        let mut listeners = self.listeners.write().await;
        
        if listeners.remove(plugin_name).is_some() {
            info!("Plugin {} unsubscribed from all messages", plugin_name);
            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_name))
        }
    }

    /// 获取当前注册的插件数量
    pub async fn get_registered_plugins_count(&self) -> usize {
        let broadcasters = self.broadcasters.read().await;
        broadcasters.len()
    }

    /// 广播消息给所有插件
    pub async fn broadcast(&self, message_type: &str, data: serde_json::Value, sender: &str) -> Result<(), String> {
        let message = PluginMessage {
            message_type: message_type.to_string(),
            data,
            sender: sender.to_string(),
            target: None,
            message_id: format!("{}-{}", sender, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: None,
            priority: 128, // 默认优先级
        };

        self.send_message(message).await
    }

    /// 发送消息给特定插件
    pub async fn send_to(&self, message_type: &str, data: serde_json::Value, sender: &str, target: &str) -> Result<(), String> {
        let message = PluginMessage {
            message_type: message_type.to_string(),
            data,
            sender: sender.to_string(),
            target: Some(target.to_string()),
            message_id: format!("{}-{}", sender, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: None,
            priority: 128, // 默认优先级
        };

        self.send_message(message).await
    }

    /// 发送高优先级消息
    pub async fn send_priority_message(&self, message_type: &str, data: serde_json::Value, sender: &str, target: Option<&str>) -> Result<(), String> {
        let message = PluginMessage {
            message_type: message_type.to_string(),
            data,
            sender: sender.to_string(),
            target: target.map(|t| t.to_string()),
            message_id: format!("{}-{}", sender, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: None,
            priority: 255, // 最高优先级
        };

        self.send_message(message).await
    }
}

impl Default for PluginBus {
    fn default() -> Self {
        Self::new()
    }
}
