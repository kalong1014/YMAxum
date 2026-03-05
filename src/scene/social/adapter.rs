// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::scene::SceneAdapter;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 社交场景适配器
#[derive(Clone)]
pub struct SocialScene {
    /// 用户关注关系
    follows: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// 消息通知
    notifications: Arc<RwLock<HashMap<String, Vec<Notification>>>>,
    /// 用户信息
    users: Arc<RwLock<HashMap<String, UserInfo>>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub id: String,
    /// 用户名
    pub username: String,
    /// 头像
    pub avatar: Option<String>,
    /// 简介
    pub bio: Option<String>,
    /// 关注数
    pub follow_count: u32,
    /// 粉丝数
    pub follower_count: u32,
    /// 发布内容数
    pub post_count: u32,
}

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    /// 关注通知
    Follow,
    /// 点赞通知
    Like,
    /// 评论通知
    Comment,
    /// 系统通知
    System,
    /// 消息通知
    Message,
}

/// 通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 通知ID
    pub id: String,
    /// 通知类型
    pub notification_type: NotificationType,
    /// 发送者ID
    pub sender_id: Option<String>,
    /// 接收者ID
    pub receiver_id: String,
    /// 内容
    pub content: String,
    /// 关联对象ID
    pub object_id: Option<String>,
    /// 是否已读
    pub read: bool,
    /// 创建时间
    pub created_at: i64,
}

/// 社交场景事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialEvent {
    /// 事件类型
    pub event_type: String,
    /// 事件数据
    pub data: serde_json::Value,
    /// 时间戳
    pub timestamp: i64,
}

impl SocialScene {
    /// 创建新的社交场景
    pub fn new() -> Self {
        Self {
            follows: Arc::new(RwLock::new(HashMap::new())),
            notifications: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加用户
    pub async fn add_user(&self, user: UserInfo) {
        let mut users = self.users.write().await;
        users.insert(user.id.clone(), user.clone());
        info!("User added: {}", user.username);
    }

    /// 获取用户信息
    pub async fn get_user(&self, user_id: &str) -> Option<UserInfo> {
        let users = self.users.read().await;
        users.get(user_id).cloned()
    }

    /// 关注用户
    pub async fn follow_user(&self, follower_id: &str, following_id: &str) -> Result<(), String> {
        if follower_id == following_id {
            return Err("Cannot follow yourself".to_string());
        }

        let mut follows = self.follows.write().await;
        let mut users = self.users.write().await;

        // 检查用户是否存在
        if !users.contains_key(follower_id) {
            return Err(format!("User not found: {}", follower_id));
        }
        if !users.contains_key(following_id) {
            return Err(format!("User not found: {}", following_id));
        }

        // 添加关注关系
        let follower_follows = follows
            .entry(follower_id.to_string())
            .or_insert_with(HashSet::new);
        if follower_follows.insert(following_id.to_string()) {
            // 更新关注数和粉丝数
            if let Some(follower) = users.get_mut(follower_id) {
                follower.follow_count += 1;
            }
            if let Some(following) = users.get_mut(following_id) {
                following.follower_count += 1;
            }

            // 发送通知
            let notification = Notification {
                id: uuid::Uuid::new_v4().to_string(),
                notification_type: NotificationType::Follow,
                sender_id: Some(follower_id.to_string()),
                receiver_id: following_id.to_string(),
                content: format!("{} followed you", users.get(follower_id).unwrap().username),
                object_id: None,
                read: false,
                created_at: chrono::Utc::now().timestamp(),
            };

            let mut notifications = self.notifications.write().await;
            let user_notifications = notifications
                .entry(following_id.to_string())
                .or_insert_with(Vec::new);
            user_notifications.push(notification);

            info!("{} followed {}", follower_id, following_id);
            Ok(())
        } else {
            Err("Already following this user".to_string())
        }
    }

    /// 取消关注用户
    pub async fn unfollow_user(&self, follower_id: &str, following_id: &str) -> Result<(), String> {
        let mut follows = self.follows.write().await;
        let mut users = self.users.write().await;

        // 检查用户是否存在
        if !users.contains_key(follower_id) {
            return Err(format!("User not found: {}", follower_id));
        }
        if !users.contains_key(following_id) {
            return Err(format!("User not found: {}", following_id));
        }

        // 移除关注关系
        if let Some(follower_follows) = follows.get_mut(follower_id) {
            if follower_follows.remove(following_id) {
                // 更新关注数和粉丝数
                if let Some(follower) = users.get_mut(follower_id) {
                    follower.follow_count = follower.follow_count.saturating_sub(1);
                }
                if let Some(following) = users.get_mut(following_id) {
                    following.follower_count = following.follower_count.saturating_sub(1);
                }

                info!("{} unfollowed {}", follower_id, following_id);
                Ok(())
            } else {
                Err("Not following this user".to_string())
            }
        } else {
            Err("Not following this user".to_string())
        }
    }

    /// 获取用户关注列表
    pub async fn get_following(&self, user_id: &str) -> Vec<String> {
        let follows = self.follows.read().await;
        follows
            .get(user_id)
            .unwrap_or(&HashSet::new())
            .iter()
            .cloned()
            .collect()
    }

    /// 获取用户粉丝列表
    pub async fn get_followers(&self, user_id: &str) -> Vec<String> {
        let follows = self.follows.read().await;
        let mut followers = Vec::new();

        for (follower, following_set) in follows.iter() {
            if following_set.contains(user_id) {
                followers.push(follower.clone());
            }
        }

        followers
    }

    /// 发送通知
    pub async fn send_notification(&self, notification: Notification) {
        let mut notifications = self.notifications.write().await;
        let user_notifications = notifications
            .entry(notification.receiver_id.clone())
            .or_insert_with(Vec::new);
        user_notifications.push(notification.clone());

        info!(
            "Notification sent to {}: {:?}",
            notification.receiver_id, notification.notification_type
        );
    }

    /// 获取用户通知
    pub async fn get_notifications(
        &self,
        user_id: &str,
        limit: u32,
        offset: u32,
    ) -> Vec<Notification> {
        let notifications = self.notifications.read().await;
        notifications
            .get(user_id)
            .unwrap_or(&Vec::new())
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect()
    }

    /// 标记通知为已读
    pub async fn mark_notification_read(
        &self,
        user_id: &str,
        notification_id: &str,
    ) -> Result<(), String> {
        let mut notifications = self.notifications.write().await;
        if let Some(user_notifications) = notifications.get_mut(user_id) {
            if let Some(notification) = user_notifications
                .iter_mut()
                .find(|n| n.id == notification_id)
            {
                notification.read = true;
                Ok(())
            } else {
                Err("Notification not found".to_string())
            }
        } else {
            Err("No notifications found for user".to_string())
        }
    }

    /// 标记所有通知为已读
    pub async fn mark_all_notifications_read(&self, user_id: &str) -> Result<(), String> {
        let mut notifications = self.notifications.write().await;
        if let Some(user_notifications) = notifications.get_mut(user_id) {
            for notification in user_notifications.iter_mut() {
                notification.read = true;
            }
            Ok(())
        } else {
            Err("No notifications found for user".to_string())
        }
    }

    /// 获取未读通知数
    pub async fn get_unread_notification_count(&self, user_id: &str) -> u32 {
        let notifications = self.notifications.read().await;
        notifications
            .get(user_id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|n| !n.read)
            .count() as u32
    }

    /// 处理社交事件
    pub async fn handle_event(&self, event: SocialEvent) {
        debug!("Handling social event: {:?}", event.event_type);

        // 根据事件类型处理
        match event.event_type.as_str() {
            "follow" => {
                if let Ok(data) = serde_json::from_value::<FollowEventData>(event.data.clone()) {
                    let _ = self
                        .follow_user(&data.follower_id, &data.following_id)
                        .await;
                }
            }
            "unfollow" => {
                if let Ok(data) = serde_json::from_value::<FollowEventData>(event.data.clone()) {
                    let _ = self
                        .unfollow_user(&data.follower_id, &data.following_id)
                        .await;
                }
            }
            "send_notification" => {
                if let Ok(data) = serde_json::from_value::<Notification>(event.data.clone()) {
                    self.send_notification(data).await;
                }
            }
            _ => {
                debug!("Unknown social event type: {}", event.event_type);
            }
        }
    }
}

/// 关注事件数据
#[derive(Debug, Clone, Deserialize)]
pub struct FollowEventData {
    /// 关注者ID
    pub follower_id: String,
    /// 被关注者ID
    pub following_id: String,
}

impl SceneAdapter for SocialScene {
    fn name(&self) -> &'static str {
        "social"
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing social scene");

        // 初始化默认数据
        let default_users = vec![
            UserInfo {
                id: "1".to_string(),
                username: "admin".to_string(),
                avatar: Some("https://example.com/avatars/admin.png".to_string()),
                bio: Some("System administrator".to_string()),
                follow_count: 0,
                follower_count: 0,
                post_count: 0,
            },
            UserInfo {
                id: "2".to_string(),
                username: "user1".to_string(),
                avatar: Some("https://example.com/avatars/user1.png".to_string()),
                bio: Some("Regular user".to_string()),
                follow_count: 0,
                follower_count: 0,
                post_count: 0,
            },
        ];

        // 直接执行异步操作，使用现有的 Tokio 运行时
        for user in default_users {
            let adapter = self.clone();
            tokio::spawn(async move {
                adapter.add_user(user).await;
            });
        }

        info!("Social scene initialized");
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting social scene");
        // 启动社交场景服务
        info!("Social scene started");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Stopping social scene");
        // 停止社交场景服务
        info!("Social scene stopped");
        Ok(())
    }
}

