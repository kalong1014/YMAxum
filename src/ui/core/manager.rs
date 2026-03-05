/// 管理器模块
///
/// 负责管理UI系统的核心功能和组件
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tokio::sync::{RwLock, broadcast};
use tokio::time::{Duration, Instant};

/// UI管理器
pub struct UIManager {
    /// 组件注册表
    components: HashMap<String, (Value, Instant)>,
    /// 系统状态
    system_state: Value,
    /// 是否初始化
    initialized: bool,
    /// 组件更新通知通道
    component_tx: broadcast::Sender<String>,
    /// 批处理更新队列
    pending_updates: HashMap<String, Value>,
    /// 活跃组件
    active_components: HashSet<String>,
    /// 组件缓存
    component_cache: HashMap<String, (Value, Duration)>,
}

impl UIManager {
    /// 创建新的UI管理器
    pub fn new() -> Self {
        let (component_tx, _) = broadcast::channel(100);

        Self {
            components: HashMap::new(),
            system_state: Value::Object(serde_json::Map::new()),
            initialized: false,
            component_tx,
            pending_updates: HashMap::new(),
            active_components: HashSet::new(),
            component_cache: HashMap::new(),
        }
    }

    /// 注册组件
    pub fn register_component(&mut self, component_id: &str, component: Value) {
        self.components
            .insert(component_id.to_string(), (component, Instant::now()));
        self.active_components.insert(component_id.to_string());
    }

    /// 获取组件
    pub fn get_component(&self, component_id: &str) -> Option<&Value> {
        // 首先检查活跃组件
        if let Some((component, _)) = self.components.get(component_id) {
            return Some(component);
        }

        // 然后检查缓存
        if let Some((component, _)) = self.component_cache.get(component_id) {
            return Some(component);
        }

        None
    }

    /// 更新组件
    pub fn update_component(&mut self, component_id: &str, component: Value) -> bool {
        let component_id_str = component_id.to_string();

        if self.components.contains_key(&component_id_str) {
            // 添加到批处理更新队列
            self.pending_updates
                .insert(component_id_str.clone(), component);
            true
        } else {
            false
        }
    }

    /// 批量更新组件
    pub fn batch_update_components(&mut self) {
        if self.pending_updates.is_empty() {
            return;
        }

        // 执行批处理更新
        for (component_id, component) in self.pending_updates.drain() {
            self.components
                .insert(component_id.clone(), (component, Instant::now()));
            // 发送更新通知
            let _ = self.component_tx.send(component_id);
        }
    }

    /// 移除组件
    pub fn remove_component(&mut self, component_id: &str) -> bool {
        let component_id_str = component_id.to_string();

        // 从活跃组件中移除
        self.active_components.remove(&component_id_str);

        // 从批处理队列中移除
        self.pending_updates.remove(&component_id_str);

        // 从组件注册表中移除
        if let Some((component, _)) = self.components.remove(&component_id_str) {
            // 添加到缓存
            self.component_cache
                .insert(component_id_str, (component, Duration::from_secs(300)));
            true
        } else {
            false
        }
    }

    /// 获取所有组件
    pub fn get_all_components(&self) -> &HashMap<String, (Value, Instant)> {
        &self.components
    }

    /// 设置系统状态
    pub fn set_system_state(&mut self, state: Value) {
        self.system_state = state;
    }

    /// 获取系统状态
    pub fn get_system_state(&self) -> &Value {
        &self.system_state
    }

    /// 初始化
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    /// 检查是否初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 获取组件更新通知接收器
    pub fn subscribe_to_component_updates(&self) -> broadcast::Receiver<String> {
        self.component_tx.subscribe()
    }

    /// 清理过期缓存
    pub fn cleanup_cache(&mut self) {
        let now = Instant::now();
        self.component_cache
            .retain(|_, (_, expiry)| now.elapsed() < *expiry);
    }

    /// 标记组件为活跃
    pub fn mark_component_active(&mut self, component_id: &str) {
        self.active_components.insert(component_id.to_string());
    }

    /// 标记组件为非活跃
    pub fn mark_component_inactive(&mut self, component_id: &str) {
        self.active_components.remove(component_id);
    }
}

/// 全局UI管理器
static UI_MANAGER: tokio::sync::OnceCell<RwLock<UIManager>> = tokio::sync::OnceCell::const_new();

/// 获取UI管理器
pub async fn get_ui_manager() -> &'static RwLock<UIManager> {
    UI_MANAGER
        .get_or_init(|| async { RwLock::new(UIManager::new()) })
        .await
}

/// 初始化UI管理器
pub async fn initialize() -> Result<(), crate::error::Error> {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.initialize();
    log::info!("UI manager initialized");

    // 启动批处理更新任务
    tokio::spawn(async {
        let manager = get_ui_manager().await;
        loop {
            tokio::time::sleep(Duration::from_millis(50)).await;

            let mut ui_manager = manager.write().await;
            ui_manager.batch_update_components();
            ui_manager.cleanup_cache();
        }
    });

    Ok(())
}

/// 注册组件
pub async fn register_component(component_id: &str, component: Value) {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.register_component(component_id, component);
}

/// 获取组件
pub async fn get_component(component_id: &str) -> Option<Value> {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    // 标记组件为活跃
    ui_manager.mark_component_active(component_id);

    ui_manager.get_component(component_id).cloned()
}

/// 更新组件
pub async fn update_component(component_id: &str, component: Value) -> bool {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.update_component(component_id, component)
}

/// 批量更新组件
pub async fn batch_update_components(updates: Vec<(String, Value)>) {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    for (component_id, component) in updates {
        ui_manager.update_component(&component_id, component);
    }
}

/// 移除组件
pub async fn remove_component(component_id: &str) -> bool {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.remove_component(component_id)
}

/// 获取所有组件
pub async fn get_all_components() -> HashMap<String, Value> {
    let manager = get_ui_manager().await;
    let ui_manager = manager.read().await;

    ui_manager
        .get_all_components()
        .iter()
        .map(|(id, (component, _))| (id.clone(), component.clone()))
        .collect()
}

/// 设置系统状态
pub async fn set_system_state(state: Value) {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.set_system_state(state);
}

/// 获取系统状态
pub async fn get_system_state() -> Value {
    let manager = get_ui_manager().await;
    let ui_manager = manager.read().await;

    ui_manager.get_system_state().clone()
}

/// 订阅组件更新
pub async fn subscribe_to_component_updates() -> broadcast::Receiver<String> {
    let manager = get_ui_manager().await;
    let ui_manager = manager.read().await;

    ui_manager.subscribe_to_component_updates()
}

/// 标记组件为非活跃
pub async fn mark_component_inactive(component_id: &str) {
    let manager = get_ui_manager().await;
    let mut ui_manager = manager.write().await;

    ui_manager.mark_component_inactive(component_id);
}
