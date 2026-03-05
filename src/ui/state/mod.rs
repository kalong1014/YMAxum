pub mod actions;
pub mod reducers;
/// 状态管理模块
///
/// 实现Redux-like的状态管理模式，包括状态存储、动作定义和reducer逻辑
pub mod store;

/// 状态管理配置
pub struct StateManagementConfig {
    /// 是否启用持久化
    pub enable_persistence: bool,
    /// 持久化存储键
    pub persistence_key: String,
    /// 是否启用中间件
    pub enable_middleware: bool,
}

impl Default for StateManagementConfig {
    fn default() -> Self {
        Self {
            enable_persistence: false,
            persistence_key: "ui_state".to_string(),
            enable_middleware: true,
        }
    }
}

/// 初始化状态管理系统
pub async fn initialize() -> Result<(), crate::error::Error> {
    // 初始化store
    store::initialize().await?;

    // 初始化reducers
    reducers::initialize().await?;

    // 初始化actions
    actions::initialize().await?;

    Ok(())
}

/// 获取状态
pub async fn get_state<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    store::get_state(key).await
}

/// 分发动作
pub async fn dispatch(action: actions::Action) -> Result<(), crate::error::Error> {
    store::dispatch(action).await
}

/// 订阅状态变化
pub async fn subscribe<F>(key: &str, callback: F) -> Result<usize, crate::error::Error>
where
    F: Fn() + Send + Sync + 'static,
{
    store::subscribe(key, callback).await
}

/// 取消订阅
pub async fn unsubscribe(subscription_id: usize) -> Result<(), crate::error::Error> {
    store::unsubscribe(subscription_id).await
}
