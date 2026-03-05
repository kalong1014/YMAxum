use async_trait::async_trait;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore, mpsc, Mutex};
use tokio::time;

/// 事件批处理配置
#[derive(Debug, Clone)]
pub struct EventBatchingConfig {
    /// 批处理大小
    pub batch_size: usize,
    /// 批处理超时时间（毫秒）
    pub batch_timeout_ms: u64,
}

/// 事件节流配置
#[derive(Debug, Clone)]
pub struct EventThrottlingConfig {
    /// 时间窗口（毫秒）
    pub window_ms: u64,
    /// 最大事件数
    pub max_events: usize,
}

/// 事件过滤器
pub trait EventFilter: Send + Sync {
    /// 过滤事件
    fn filter(&self, event: &GufEvent) -> bool;
}

/// 事件分组器
pub trait EventGrouper: Send + Sync {
    /// 分组事件
    fn group(&self, event: &GufEvent) -> String;
}

/// 基于事件类型的过滤器
pub struct EventTypeFilter {
    allowed_types: std::collections::HashSet<String>,
}

impl EventTypeFilter {
    pub fn new(allowed_types: Vec<String>) -> Self {
        Self {
            allowed_types: allowed_types.into_iter().collect(),
        }
    }
}

impl EventFilter for EventTypeFilter {
    fn filter(&self, event: &GufEvent) -> bool {
        self.allowed_types.contains(event.event_type())
    }
}

/// 基于事件源的过滤器
pub struct EventSourceFilter {
    allowed_sources: std::collections::HashSet<String>,
}

impl EventSourceFilter {
    pub fn new(allowed_sources: Vec<String>) -> Self {
        Self {
            allowed_sources: allowed_sources.into_iter().collect(),
        }
    }
}

impl EventFilter for EventSourceFilter {
    fn filter(&self, event: &GufEvent) -> bool {
        self.allowed_sources.contains(&event.metadata().source)
    }
}

/// 基于事件优先级的过滤器
pub struct EventPriorityFilter {
    min_priority: u32,
}

impl EventPriorityFilter {
    pub fn new(min_priority: u32) -> Self {
        Self {
            min_priority,
        }
    }
}

impl EventFilter for EventPriorityFilter {
    fn filter(&self, event: &GufEvent) -> bool {
        event.metadata().priority >= self.min_priority
    }
}

/// 复合过滤器
pub struct CompositeFilter {
    filters: Vec<Arc<dyn EventFilter>>,
    operation: FilterOperation,
}

/// 过滤器操作类型
pub enum FilterOperation {
    And,
    Or,
}

impl CompositeFilter {
    pub fn new(filters: Vec<Arc<dyn EventFilter>>, operation: FilterOperation) -> Self {
        Self {
            filters,
            operation,
        }
    }
}

impl EventFilter for CompositeFilter {
    fn filter(&self, event: &GufEvent) -> bool {
        match self.operation {
            FilterOperation::And => {
                self.filters.iter().all(|filter| filter.filter(event))
            }
            FilterOperation::Or => {
                self.filters.iter().any(|filter| filter.filter(event))
            }
        }
    }
}

/// 基于事件类型的分组器
pub struct EventTypeGrouper;

impl EventGrouper for EventTypeGrouper {
    fn group(&self, event: &GufEvent) -> String {
        event.event_type().to_string()
    }
}

/// 基于事件源的分组器
pub struct EventSourceGrouper;

impl EventGrouper for EventSourceGrouper {
    fn group(&self, event: &GufEvent) -> String {
        event.metadata().source.clone()
    }
}

/// 基于事件优先级的分组器
pub struct EventPriorityGrouper {
    priority_levels: u32,
}

impl EventPriorityGrouper {
    pub fn new(priority_levels: u32) -> Self {
        Self {
            priority_levels,
        }
    }
}

impl EventGrouper for EventPriorityGrouper {
    fn group(&self, event: &GufEvent) -> String {
        format!("priority_{}", event.metadata().priority / self.priority_levels)
    }
}

/// 复合分组器
pub struct CompositeGrouper {
    groupers: Vec<Arc<dyn EventGrouper>>,
}

impl CompositeGrouper {
    pub fn new(groupers: Vec<Arc<dyn EventGrouper>>) -> Self {
        Self {
            groupers,
        }
    }
}

impl EventGrouper for CompositeGrouper {
    fn group(&self, event: &GufEvent) -> String {
        self.groupers
            .iter()
            .map(|grouper| grouper.group(event))
            .collect::<Vec<_>>()
            .join(":")
    }
}

/// 事件处理器配置
#[derive(Debug, Clone)]
pub struct EventHandlerConfig {
    /// 处理器优先级
    pub priority: u32,
    /// 处理器是否异步
    pub is_async: bool,
    /// 处理器超时时间（毫秒）
    pub timeout_ms: u64,
}

/// 事件统计
#[derive(Debug, Clone)]
pub struct EventStats {
    /// 事件类型
    pub event_type: String,
    /// 发布次数
    pub published: u64,
    /// 处理成功次数
    pub handled: u64,
    /// 处理失败次数
    pub failed: u64,
    /// 平均处理时间（毫秒）
    pub avg_processing_time: f64,
}

/// GUF 事件总线
/// 负责处理 GUF 生态系统的事件通知和响应
pub struct GufEventBus {
    /// 事件订阅者映射
    subscribers: Arc<RwLock<SubscriberMap>>,
    /// 事件发布通道
    event_sender: mpsc::Sender<GufEvent>,
    /// 事件处理任务
    event_handler_task: Option<tokio::task::JoinHandle<()>>,
    /// 事件总线状态
    status: Arc<RwLock<EventBusStatus>>,
    /// 批处理配置
    batching_config: EventBatchingConfig,
    /// 节流配置
    throttling_config: EventThrottlingConfig,
    /// 并发控制信号量
    concurrency_semaphore: Arc<Semaphore>,
    /// 事件计数器（用于节流）
    event_counters: Arc<RwLock<std::collections::HashMap<String, (Instant, usize)>>>,
    /// 事件过滤器
    event_filters: Arc<RwLock<Vec<Arc<dyn EventFilter>>>>,
    /// 事件分组器
    event_grouper: Arc<Mutex<Option<Arc<dyn EventGrouper>>>>,

    /// 事件统计
    event_stats: Arc<RwLock<std::collections::HashMap<String, EventStats>>>,
    /// 事件处理器配置
    handler_configs: Arc<RwLock<std::collections::HashMap<String, EventHandlerConfig>>>,
}

/// 事件订阅者映射
pub type SubscriberMap = std::collections::HashMap<String, Vec<EventSubscriber>>;

/// 事件订阅者
pub struct EventSubscriber {
    /// 订阅者 ID
    id: String,
    /// 事件处理器
    handler: std::sync::Arc<dyn EventHandler>,
    /// 订阅优先级
    priority: u32,
}

/// 事件处理器
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// 处理事件
    async fn handle(&self, event: &GufEvent) -> Result<(), String>;

    /// 获取处理器 ID
    fn id(&self) -> &str;
}

/// GUF 事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufEvent {
    /// 事件 ID
    id: String,
    /// 事件类型
    event_type: String,
    /// 事件数据
    data: serde_json::Value,
    /// 事件元数据
    metadata: EventMetadata,
}

/// 事件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// 事件发生时间
    timestamp: u64,
    /// 事件来源
    source: String,
    /// 事件优先级
    priority: u32,
}

/// 事件总线状态
#[derive(Debug, Clone)]
pub enum EventBusStatus {
    /// 运行中
    Running,
    /// 停止中
    Stopping,
    /// 已停止
    Stopped,
    /// 错误
    Error(String),
}

/// GUF 事件总线错误
#[derive(Debug, thiserror::Error)]
pub enum GufEventBusError {
    #[error("Event publish failed: {0}")]
    PublishFailed(String),

    #[error("Subscriber not found: {0}")]
    SubscriberNotFound(String),

    #[error("Event handler error: {0}")]
    HandlerError(String),

    #[error("Event bus not running")]
    NotRunning,
}

impl EventSubscriber {
    /// 创建新的事件订阅者
    pub fn new(id: String, handler: std::sync::Arc<dyn EventHandler>, priority: u32) -> Self {
        Self {
            id,
            handler,
            priority,
        }
    }
}

impl Clone for EventSubscriber {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            handler: self.handler.clone(),
            priority: self.priority,
        }
    }
}

impl GufEvent {
    /// 创建新的 GUF 事件
    pub fn new(event_type: String, data: serde_json::Value, source: String) -> Self {
        Self {
            id: format!("{}-{}", event_type, chrono::Utc::now().timestamp()),
            event_type,
            data,
            metadata: EventMetadata {
                timestamp: chrono::Utc::now().timestamp() as u64,
                source,
                priority: 0,
            },
        }
    }

    /// 使用默认来源创建新的 GUF 事件
    pub fn new_with_default_source(event_type: String, data: serde_json::Value) -> Self {
        Self::new(event_type, data, "default".to_string())
    }

    /// 获取事件 ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取事件类型
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// 获取事件数据
    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }

    /// 获取事件元数据
    pub fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

impl GufEventBus {
    /// 创建新的 GUF 事件总线
    pub fn new() -> Self {
        let (event_sender, _event_receiver) = mpsc::channel(1000); // 增加通道容量以提高性能
        let subscribers = Arc::new(RwLock::new(SubscriberMap::new()));
        let status = Arc::new(RwLock::new(EventBusStatus::Stopped));

        // 默认批处理配置
        let batching_config = EventBatchingConfig {
            batch_size: 20, // 增加批处理大小以提高性能
            batch_timeout_ms: 50, // 减少超时以提高响应速度
        };

        // 默认节流配置
        let throttling_config = EventThrottlingConfig {
            window_ms: 1000,
            max_events: 1000, // 增加最大事件数以提高性能
        };

        // 并发控制信号量（增加并发处理任务数）
        let concurrency_semaphore = Arc::new(Semaphore::new(20));

        // 事件计数器
        let event_counters = Arc::new(RwLock::new(std::collections::HashMap::new()));

        // 事件过滤器
        let event_filters = Arc::new(RwLock::new(Vec::new()));

        // 事件统计
        let event_stats = Arc::new(RwLock::new(std::collections::HashMap::new()));

        // 事件处理器配置
        let handler_configs = Arc::new(RwLock::new(std::collections::HashMap::new()));

        // 事件分组器
        let event_grouper = Arc::new(Mutex::new(None));

        Self {
            subscribers,
            event_sender,
            event_handler_task: None,
            status,
            batching_config,
            throttling_config,
            concurrency_semaphore,
            event_counters,
            event_filters,
            event_grouper,
            event_stats,
            handler_configs,
        }
    }

    /// 使用自定义配置创建事件总线
    pub fn with_config(
        batching_config: EventBatchingConfig,
        throttling_config: EventThrottlingConfig,
        max_concurrency: usize,
    ) -> Self {
        let (event_sender, _event_receiver) = mpsc::channel(100);
        let subscribers = Arc::new(RwLock::new(SubscriberMap::new()));
        let status = Arc::new(RwLock::new(EventBusStatus::Stopped));

        let concurrency_semaphore = Arc::new(Semaphore::new(max_concurrency));
        let event_counters = Arc::new(RwLock::new(std::collections::HashMap::new()));
        let event_filters = Arc::new(RwLock::new(Vec::new()));
        let event_stats = Arc::new(RwLock::new(std::collections::HashMap::new()));
        let handler_configs = Arc::new(RwLock::new(std::collections::HashMap::new()));

        // 事件分组器
        let event_grouper = Arc::new(Mutex::new(None));

        Self {
            subscribers,
            event_sender,
            event_handler_task: None,
            status,
            batching_config,
            throttling_config,
            concurrency_semaphore,
            event_counters,
            event_filters,
            event_grouper,
            event_stats,
            handler_configs,
        }
    }

    /// 初始化事件总线
    pub async fn init(&mut self) -> Result<(), String> {
        // 初始化事件总线
        // 这里可以添加初始化逻辑
        Ok(())
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        // 简单检查，实际应用中可能需要更复杂的逻辑
        true
    }

    /// 启动事件总线
    pub async fn start(&mut self) {
        let (event_sender, event_receiver) = mpsc::channel(100);
        self.event_sender = event_sender;

        let subscribers = self.subscribers.clone();
        let status = self.status.clone();
        let batching_config = self.batching_config.clone();
        let concurrency_semaphore = self.concurrency_semaphore.clone();
        let event_grouper = self.event_grouper.clone();

        let event_stats = self.event_stats.clone();
        let task = tokio::spawn(async move {
            let mut event_receiver = event_receiver;
            let mut event_batch = VecDeque::new();
            let mut batch_timer =
                time::interval(Duration::from_millis(batching_config.batch_timeout_ms));
            batch_timer.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

            loop {
                tokio::select! {
                    // 接收事件
                    event = event_receiver.recv() => {
                        match event {
                            Some(event) => {
                                event_batch.push_back(event);

                                // 检查是否达到批处理大小
                                if event_batch.len() >= batching_config.batch_size {
                                    let grouper = event_grouper.lock().await.clone();
                                    Self::process_event_batch(&subscribers, &concurrency_semaphore, event_batch.drain(..).collect(), &event_stats, grouper).await;
                                }
                            }
                            None => {
                                // 通道关闭，处理剩余事件
                                if !event_batch.is_empty() {
                                    let grouper = event_grouper.lock().await.clone();
                                    Self::process_event_batch(&subscribers, &concurrency_semaphore, event_batch.drain(..).collect(), &event_stats, grouper).await;
                                }
                                break;
                            }
                        }
                    }
                    // 批处理超时
                    _ = batch_timer.tick() => {
                        if !event_batch.is_empty() {
                            let grouper = event_grouper.lock().await.clone();
                            Self::process_event_batch(&subscribers, &concurrency_semaphore, event_batch.drain(..).collect(), &event_stats, grouper).await;
                        }
                    }
                }
            }

            // 更新状态
            let mut status = status.write().await;
            *status = EventBusStatus::Stopped;
        });

        self.event_handler_task = Some(task);

        // 更新状态
        let mut status = self.status.write().await;
        *status = EventBusStatus::Running;
    }

    /// 添加事件过滤器
    pub async fn add_filter(&self, filter: Arc<dyn EventFilter>) {
        let mut filters = self.event_filters.write().await;
        filters.push(filter);
    }

    /// 设置事件分组器
    pub async fn set_grouper(&self, grouper: Arc<dyn EventGrouper>) {
        // 使用 Mutex 安全地修改 event_grouper
        let mut event_grouper = self.event_grouper.lock().await;
        *event_grouper = Some(grouper);
    }

    /// 设置事件处理器配置
    pub async fn set_handler_config(&self, handler_id: String, config: EventHandlerConfig) {
        let mut configs = self.handler_configs.write().await;
        configs.insert(handler_id, config);
    }

    /// 发布事件
    pub async fn publish(&self, event: GufEvent) -> Result<(), GufEventBusError> {
        // 检查事件总线状态
        let status = self.status.read().await;
        if !matches!(*status, EventBusStatus::Running) {
            return Err(GufEventBusError::NotRunning);
        }
        drop(status);

        // 应用事件过滤器
        let filters = self.event_filters.read().await;
        for filter in filters.iter() {
            if !filter.filter(&event) {
                log::debug!("Event {} filtered out", event.event_type());
                return Ok(());
            }
        }
        drop(filters);

        // 检查事件节流
        if !self.check_throttling(event.event_type()).await {
            log::debug!("Event {} throttled", event.event_type());
            return Ok(()); // 节流时静默丢弃事件
        }

        // 更新事件统计
        self.update_event_stats(&event, true).await;

        // 保存事件类型用于日志
        let _event_type = event.event_type().to_string();
        
        // 发送事件
        match self.event_sender.send(event).await {
            Ok(_) => Ok(()),
            Err(e) => Err(GufEventBusError::PublishFailed(e.to_string()))
        }
    }

    /// 发布事件（带优先级）
    pub async fn publish_with_priority(&self, event: GufEvent, priority: u32) -> Result<(), GufEventBusError> {
        // 创建带优先级的事件
        let mut event_with_priority = event;
        event_with_priority.metadata.priority = priority;
        
        // 发布事件
        self.publish(event_with_priority).await
    }

    /// 批量发布事件
    pub async fn publish_batch(&self, events: Vec<GufEvent>) -> Result<(), GufEventBusError> {
        // 检查事件总线状态
        let status = self.status.read().await;
        if !matches!(*status, EventBusStatus::Running) {
            return Err(GufEventBusError::NotRunning);
        }
        drop(status);

        // 批量发送事件
        for event in events {
            // 应用事件过滤器
            let filters = self.event_filters.read().await;
            let mut filtered = false;
            for filter in filters.iter() {
                if !filter.filter(&event) {
                    filtered = true;
                    break;
                }
            }
            drop(filters);

            if filtered {
                continue;
            }

            // 检查事件节流
            if !self.check_throttling(event.event_type()).await {
                continue;
            }

            // 更新事件统计
            self.update_event_stats(&event, true).await;

            // 发送事件
            let _ = self.event_sender.send(event).await;
        }

        Ok(())
    }

    /// 发布 Godot UI 事件
    pub async fn publish_godot_ui_event(
        &self,
        component_id: &str,
        event_name: &str,
        data: serde_json::Value,
    ) -> Result<(), GufEventBusError> {
        let godot_event = GufEvent::new(
            format!("godot.ui.{}", event_name),
            serde_json::json!({
                "component_id": component_id,
                "event_name": event_name,
                "data": data,
                "godot_version": "4.4.0"
            }),
            "godot_ui_framework".to_string(),
        );

        self.publish(godot_event).await
    }

    /// 检查事件节流
    async fn check_throttling(&self, event_type: &str) -> bool {
        let mut counters = self.event_counters.write().await;
        let now = Instant::now();

        // 获取或创建事件计数器
        let (last_reset, count) = counters.entry(event_type.to_string()).or_insert((now, 0));

        // 检查是否需要重置计数器
        if now.duration_since(*last_reset).as_millis() > self.throttling_config.window_ms as u128 {
            *last_reset = now;
            *count = 0;
        }

        // 检查是否超过最大事件数
        if *count >= self.throttling_config.max_events {
            return false;
        }

        // 增加计数
        *count += 1;
        true
    }

    /// 更新事件统计
    async fn update_event_stats(&self, event: &GufEvent, is_published: bool) {
        let mut stats = self.event_stats.write().await;
        let event_type = event.event_type().to_string();
        
        let stats_entry = stats.entry(event_type.clone()).or_insert(EventStats {
            event_type: event_type.clone(),
            published: 0,
            handled: 0,
            failed: 0,
            avg_processing_time: 0.0,
        });
        
        if is_published {
            stats_entry.published += 1;
        }
    }

    /// 记录事件处理结果
    #[allow(dead_code)]
    async fn record_event_handler_result(&self, event_type: &str, success: bool, processing_time_ms: f64) {
        let mut stats = self.event_stats.write().await;
        if let Some(stats_entry) = stats.get_mut(event_type) {
            if success {
                stats_entry.handled += 1;
            } else {
                stats_entry.failed += 1;
            }
            
            // 更新平均处理时间
            stats_entry.avg_processing_time = (
                stats_entry.avg_processing_time * (stats_entry.handled as f64 - 1.0) + processing_time_ms
            ) / stats_entry.handled as f64;
        }
    }

    /// 处理事件批次
    async fn process_event_batch(
        subscribers: &Arc<RwLock<SubscriberMap>>,
        semaphore: &Arc<Semaphore>,
        events: Vec<GufEvent>,
        event_stats: &Arc<RwLock<std::collections::HashMap<String, EventStats>>>,
        grouper: Option<Arc<dyn EventGrouper>>,
    ) {
        // 按事件类型或自定义分组器分组
        let mut events_by_group = std::collections::HashMap::new();
        for event in events {
            let group_key = match &grouper {
                Some(g) => g.group(&event),
                None => event.event_type().to_string(),
            };
            events_by_group
                .entry(group_key)
                .or_insert_with(Vec::new)
                .push(event);
        }

        // 处理每个分组的事件
        for (group_key, group_events) in events_by_group {
            debug!("Processing event group {} with {} events", group_key, group_events.len());
            
            // 批量处理事件，减少锁竞争
            let mut event_tasks = Vec::new();

            for event in group_events {
                let event_type = event.event_type().to_string();
                // 获取该事件类型的订阅者
                let subscriber_list = {
                    let subscribers = subscribers.read().await;
                    subscribers.get(&event_type).map(|subs| subs.to_vec())
                };

                if let Some(subscribers) = subscriber_list {
                    // 按优先级排序订阅者
                    let mut sorted_subscribers = subscribers;
                    sorted_subscribers.sort_by(|a, b| b.priority.cmp(&a.priority));

                    // 为每个订阅者创建一个处理任务
                    for subscriber in sorted_subscribers {
                        let event = event.clone();
                        let semaphore = semaphore.clone();
                        let event_type_clone = event_type.clone();
                        let event_stats_clone = event_stats.clone();

                        // 使用信号量控制并发
                        event_tasks.push(tokio::spawn(async move {
                            let permit = match semaphore.acquire().await {
                                Ok(permit) => permit,
                                Err(_) => return, // 信号量被关闭
                            };

                            // 带超时的事件处理
                            let start_time = Instant::now();
                            let result = tokio::time::timeout(
                                tokio::time::Duration::from_secs(30),
                                subscriber.handler.handle(&event)
                            ).await;
                            let processing_time = start_time.elapsed().as_millis() as f64;
                            
                            // 记录处理结果
                            let success = result.is_ok();
                            let mut stats = event_stats_clone.write().await;
                            if let Some(stats_entry) = stats.get_mut(&event_type_clone) {
                                if success {
                                    stats_entry.handled += 1;
                                } else {
                                    stats_entry.failed += 1;
                                }
                                
                                // 更新平均处理时间
                                stats_entry.avg_processing_time = (
                                    stats_entry.avg_processing_time * (stats_entry.handled as f64 - 1.0) + processing_time
                                ) / stats_entry.handled as f64;
                            }
                            
                            if let Err(e) = result {
                                log::error!(
                                    "Error handling event {} by subscriber {}: {}",
                                    event.id(),
                                    subscriber.id,
                                    e
                                );
                            }

                            // 释放信号量
                            drop(permit);
                        }));
                    }
                }
            }

            // 等待所有事件处理任务完成，避免任务堆积
            // 注意：这里我们不等待任务完成，因为事件处理应该是异步的
            // 但我们限制了并发数量，所以不会有任务堆积的问题
        }
    }

    /// 处理 Godot UI Framework v4.4 事件
    pub async fn handle_godot_ui_event(
        &self,
        component_id: &str,
        event_name: &str,
        data: serde_json::Value,
    ) -> Result<(), GufEventBusError> {
        // 创建 Godot UI 事件
        let godot_event = GufEvent::new(
            format!("godot.ui.{}", event_name),
            serde_json::json!({
                "component_id": component_id,
                "event_name": event_name,
                "data": data,
                "godot_version": "4.4.0"
            }),
            "godot_ui_framework".to_string(),
        );

        // 发布事件
        self.publish(godot_event).await
    }

    /// 停止事件总线
    pub async fn stop(&mut self) {
        // 更新状态
        let mut status = self.status.write().await;
        *status = EventBusStatus::Stopping;
        drop(status);

        // 关闭事件发送通道
        drop(std::mem::replace(
            &mut self.event_sender,
            mpsc::channel(1).0,
        ));

        // 等待事件处理任务完成
        if let Some(task) = self.event_handler_task.take()
            && let Err(e) = task.await
        {
            error!("Error stopping event bus: {}", e);
        }

        // 更新状态
        let mut status = self.status.write().await;
        *status = EventBusStatus::Stopped;
    }

    /// 订阅事件
    pub async fn subscribe(
        &self,
        event_type: String,
        subscriber: EventSubscriber,
    ) -> Result<(), GufEventBusError> {
        // 检查事件总线状态
        let status = self.status.read().await;
        if !matches!(*status, EventBusStatus::Running) {
            return Err(GufEventBusError::NotRunning);
        }
        drop(status);

        // 添加订阅者
        let mut subscribers = self.subscribers.write().await;
        subscribers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(subscriber);

        Ok(())
    }

    /// 订阅 Godot UI Framework v4.4 事件
    pub async fn subscribe_godot_ui_event(
        &self,
        event_name: &str,
        subscriber: EventSubscriber,
    ) -> Result<(), GufEventBusError> {
        // 构建 Godot UI 事件类型
        let event_type = format!("godot.ui.{}", event_name);

        // 调用通用订阅方法
        self.subscribe(event_type, subscriber).await
    }

    /// 取消订阅
    pub async fn unsubscribe(
        &self,
        event_type: &str,
        subscriber_id: &str,
    ) -> Result<(), GufEventBusError> {
        // 检查事件总线状态
        let status = self.status.read().await;
        if !matches!(*status, EventBusStatus::Running) {
            return Err(GufEventBusError::NotRunning);
        }
        drop(status);

        // 移除订阅者
        let mut subscribers = self.subscribers.write().await;
        if let Some(event_subscribers) = subscribers.get_mut(event_type) {
            let initial_len = event_subscribers.len();
            event_subscribers.retain(|s| s.id != subscriber_id);

            if event_subscribers.len() == initial_len {
                return Err(GufEventBusError::SubscriberNotFound(
                    subscriber_id.to_string(),
                ));
            }
        } else {
            return Err(GufEventBusError::SubscriberNotFound(
                subscriber_id.to_string(),
            ));
        }

        Ok(())
    }

    /// 获取事件总线状态
    pub async fn status(&self) -> EventBusStatus {
        let status = self.status.read().await;
        status.clone()
    }

    /// 获取订阅者数量
    pub async fn get_subscriber_count(&self, event_type: &str) -> usize {
        let subscribers = self.subscribers.read().await;
        if let Some(event_subscribers) = subscribers.get(event_type) {
            event_subscribers.len()
        } else {
            0
        }
    }

    /// 获取事件统计信息
    pub async fn get_event_stats(&self, event_type: &str) -> Option<EventStats> {
        let stats = self.event_stats.read().await;
        stats.get(event_type).cloned()
    }

    /// 获取所有事件统计信息
    pub async fn get_all_event_stats(&self) -> Vec<EventStats> {
        let stats = self.event_stats.read().await;
        stats.values().cloned().collect()
    }

    /// 重置事件统计信息
    pub async fn reset_event_stats(&self) {
        let mut stats = self.event_stats.write().await;
        stats.clear();
    }
}

/// 示例事件处理器
pub struct ExampleEventHandler {
    id: String,
}

impl ExampleEventHandler {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[async_trait]
impl EventHandler for ExampleEventHandler {
    async fn handle(&self, event: &GufEvent) -> Result<(), String> {
        debug!(
            "Example handler {} received event {}: {:?}",
            self.id,
            event.id(),
            event.data()
        );
        Ok(())
    }

    fn id(&self) -> &str {
        &self.id
    }
}

/// 示例事件总线使用
pub async fn example_usage() {
    // 创建事件总线
    let mut event_bus = GufEventBus::new();

    // 启动事件总线
    event_bus.start().await;

    // 创建事件处理器
    let handler1 = ExampleEventHandler::new("handler1".to_string());
    let handler2 = ExampleEventHandler::new("handler2".to_string());

    // 订阅事件
    event_bus
        .subscribe(
            "user.created".to_string(),
            EventSubscriber::new("subscriber1".to_string(), std::sync::Arc::new(handler1), 10),
        )
        .await
        .unwrap();

    event_bus
        .subscribe(
            "user.created".to_string(),
            EventSubscriber::new("subscriber2".to_string(), std::sync::Arc::new(handler2), 5),
        )
        .await
        .unwrap();

    // 发布事件
    let event = GufEvent::new(
        "user.created".to_string(),
        serde_json::json!({
            "user_id": "123",
            "username": "testuser"
        }),
        "api".to_string(),
    );

    event_bus.publish(event).await.unwrap();

    // 等待事件处理
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 停止事件总线
    event_bus.stop().await;
}
