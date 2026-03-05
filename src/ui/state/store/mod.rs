/// Store模块
///
/// 状态管理系统的核心，负责存储状态、处理动作分发和状态订阅
use super::actions::Action;
use super::reducers;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::{Notify, RwLock};

/// 订阅回调类型
pub type SubscriptionCallback = Box<dyn Fn() + Send + Sync>;

/// 订阅信息
pub struct Subscription {
    /// 订阅ID
    pub id: usize,
    /// 订阅的状态键
    pub key: String,
    /// 回调函数
    pub callback: SubscriptionCallback,
}

/// 状态存储
pub struct Store {
    /// 状态数据
    state: Value,
    /// 订阅列表
    subscriptions: HashMap<usize, Subscription>,
    /// 下一个订阅ID
    next_subscription_id: usize,
    /// 状态变更通知
    notify: Notify,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            state: Value::Object(serde_json::Map::new()),
            subscriptions: HashMap::new(),
            next_subscription_id: 1,
            notify: Notify::new(),
        }
    }
}

impl Store {
    /// 创建新的store
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取状态
    pub fn get_state(&self) -> &Value {
        &self.state
    }

    /// 设置状态
    pub fn set_state(&mut self, new_state: Value) {
        self.state = new_state;
        self.notify.notify_waiters();
    }

    /// 分发动作
    pub async fn dispatch(&mut self, action: Action) -> Result<(), crate::error::Error> {
        // 应用reducer
        let new_state = reducers::apply_reducer(&self.state, &action).await;

        // 更新状态
        self.set_state(new_state);

        // 触发订阅回调
        self.trigger_subscriptions().await;

        Ok(())
    }

    /// 订阅状态变化
    pub fn subscribe(&mut self, key: &str, callback: impl Fn() + Send + Sync + 'static) -> usize {
        let id = self.next_subscription_id;
        self.next_subscription_id += 1;

        self.subscriptions.insert(
            id,
            Subscription {
                id,
                key: key.to_string(),
                callback: Box::new(callback),
            },
        );

        id
    }

    /// 取消订阅
    pub fn unsubscribe(&mut self, subscription_id: usize) -> bool {
        self.subscriptions.remove(&subscription_id).is_some()
    }

    /// 触发订阅回调
    pub async fn trigger_subscriptions(&self) {
        for subscription in self.subscriptions.values() {
            (subscription.callback)();
        }
    }
}

/// 全局store
static STORE: tokio::sync::OnceCell<RwLock<Store>> = tokio::sync::OnceCell::const_new();

/// 获取store
pub async fn get_store() -> &'static RwLock<Store> {
    STORE
        .get_or_init(|| async { RwLock::new(Store::new()) })
        .await
}

/// 初始化store
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化store
    let store = get_store().await;
    let store_ref = store.read().await;

    log::info!("Store initialized");
    log::debug!("Initial state: {:?}", store_ref.get_state());

    Ok(())
}

/// 获取状态
pub async fn get_state<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    let store = get_store().await;
    let store_ref = store.read().await;
    let state = store_ref.get_state();

    if let Value::Object(state_obj) = state {
        if let Some(value) = state_obj.get(key) {
            serde_json::from_value(value.clone()).ok()
        } else {
            None
        }
    } else {
        None
    }
}

/// 分发动作
pub async fn dispatch(action: Action) -> Result<(), crate::error::Error> {
    let store = get_store().await;
    let mut store_mut = store.write().await;
    store_mut.dispatch(action).await
}

/// 订阅状态变化
pub async fn subscribe<F>(key: &str, callback: F) -> Result<usize, crate::error::Error>
where
    F: Fn() + Send + Sync + 'static,
{
    let store = get_store().await;
    let mut store_mut = store.write().await;
    let subscription_id = store_mut.subscribe(key, callback);
    Ok(subscription_id)
}

/// 取消订阅
pub async fn unsubscribe(subscription_id: usize) -> Result<(), crate::error::Error> {
    let store = get_store().await;
    let mut store_mut = store.write().await;

    if store_mut.unsubscribe(subscription_id) {
        Ok(())
    } else {
        Err(crate::error::Error::from(
            crate::error::YMAxumError::service_error(format!(
                "Subscription {} not found",
                subscription_id
            )),
        ))
    }
}
