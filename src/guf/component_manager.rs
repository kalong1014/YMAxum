use crate::guf::adapter::{GufComponentInfo, GufComponentStatus};
use crate::ui::core::adapter::{GufVersion, get_adapter_registry};
use async_trait::async_trait;
use futures;
use log::{debug, error, info, warn};
use rand;
use serde_json;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, Mutex};

/// 组件消息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentMessage {
    /// 消息类型
    pub message_type: String,
    /// 消息数据
    pub data: serde_json::Value,
    /// 发送者
    pub sender: String,
    /// 接收者
    pub receiver: String,
    /// 消息 ID
    pub message_id: String,
}

/// 组件池配置
#[derive(Debug, Clone)]
pub struct ComponentPoolConfig {
    /// 最大组件数量
    pub max_size: usize,
    /// 最小组件数量
    pub min_size: usize,
    /// 空闲超时时间（秒）
    pub idle_timeout: u64,
    /// 组件类型
    pub component_type: String,
    /// 组件预热数量
    pub warmup_size: usize,
    /// 组件回收阈值
    pub cleanup_threshold: usize,
    /// 组件健康检查间隔（秒）
    pub health_check_interval: u64,
}

/// 组件池
pub struct ComponentPool {
    /// 组件池配置
    config: ComponentPoolConfig,
    /// 空闲组件
    idle_components: Arc<RwLock<Vec<Arc<dyn GufComponent>>>>,
    /// 活跃组件
    active_components: Arc<RwLock<std::collections::HashMap<String, Arc<dyn GufComponent>>>>,
    /// 组件工厂
    factory: Arc<dyn GufComponentFactory>,
    /// 组件配置
    component_config: serde_json::Value,
    /// 上次清理时间
    last_cleanup: std::sync::atomic::AtomicU64,
    /// 组件健康状态
    component_health: Arc<RwLock<std::collections::HashMap<String, bool>>>,
    /// 组件间通信通道
    component_communication: Arc<RwLock<std::collections::HashMap<String, mpsc::Sender<ComponentMessage>>>>,
    /// 健康检查任务
    health_check_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,

}

impl ComponentPool {
    /// 创建新的组件池
    pub fn new(
        config: ComponentPoolConfig,
        factory: Arc<dyn GufComponentFactory>,
        component_config: serde_json::Value,
    ) -> Self {
        Self {
            config,
            idle_components: Arc::new(RwLock::new(Vec::new())),
            active_components: Arc::new(RwLock::new(std::collections::HashMap::new())),
            factory,
            component_config,
            last_cleanup: std::sync::atomic::AtomicU64::new(0),
            component_health: Arc::new(RwLock::new(std::collections::HashMap::new())),
            component_communication: Arc::new(RwLock::new(std::collections::HashMap::new())),
            health_check_task: Arc::new(Mutex::new(None)),
        }
    }

    /// 初始化组件池
    pub async fn initialize(&self) -> Result<(), String> {
        // 预创建最小组件数量（并行处理）
        let warmup_size = std::cmp::max(self.config.min_size, self.config.warmup_size);
        let batch_size = 10; // 增加批处理大小以提高性能
        let mut new_components = Vec::with_capacity(warmup_size);
        let factory = self.factory.clone();
        let component_config = self.component_config.clone();
        let component_type = self.config.component_type.clone();

        // 分批创建组件，避免一次性创建过多任务
        for batch_start in (0..warmup_size).step_by(batch_size) {
            let batch_end = std::cmp::min(batch_start + batch_size, warmup_size);
            let mut batch_tasks = Vec::with_capacity(batch_end - batch_start);

            for i in batch_start..batch_end {
                let component_id = format!("{}_{}", component_type, i);
                let factory_clone = factory.clone();
                let config_clone = component_config.clone();

                batch_tasks.push(tokio::spawn(async move {
                    // 带超时的组件创建和初始化
                    match tokio::time::timeout(
                        tokio::time::Duration::from_secs(30),
                        async {
                            let component = factory_clone.create_component(&component_id, config_clone)?;
                            component.initialize().await?;
                            component.start().await?;
                            Ok(component)
                        }
                    ).await {
                        Ok(Ok(component)) => Ok(component),
                        Ok(Err(e)) => Err(e),
                        Err(_) => Err("Component initialization timed out".to_string()),
                    }
                }));
            }

            // 收集批次结果
            for task in batch_tasks {
                match task.await {
                    Ok(Ok(component)) => new_components.push(component),
                    Ok(Err(e)) => return Err(e),
                    Err(e) => return Err(format!("Component initialization task failed: {:?}", e)),
                }
            }

            // 批次之间添加短暂延迟，避免资源竞争
            if batch_end < warmup_size {
                tokio::time::sleep(tokio::time::Duration::from_millis(25)).await; // 减少延迟以提高性能
            }
        }

        let mut idle_components = self.idle_components.write().await;
        idle_components.extend(new_components);

        // 启动健康检查任务
        self.start_health_check().await;

        info!("Component pool initialized with {} components", idle_components.len());
        Ok(())
    }

    /// 启动健康检查任务
    async fn start_health_check(&self) {
        let component_health = self.component_health.clone();
        let active_components = self.active_components.clone();
        let idle_components = self.idle_components.clone();
        let interval = self.config.health_check_interval;

        let task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
                
                // 检查活跃组件
                let active = active_components.read().await;
                for (id, component) in active.iter() {
                    // 带超时的健康检查
                    let health = match tokio::time::timeout(
                        tokio::time::Duration::from_secs(5),
                        component.health_check()
                    ).await {
                        Ok(Ok(healthy)) => healthy,
                        Ok(Err(e)) => {
                            error!("Health check failed for component {}: {}", id, e);
                            false
                        }
                        Err(_) => {
                            error!("Health check timed out for component {}", id);
                            false
                        }
                    };
                    let mut health_map = component_health.write().await;
                    health_map.insert(id.clone(), health);
                }
                drop(active);

                // 检查空闲组件
                let idle = idle_components.read().await;
                for component in idle.iter() {
                    let id = component.id().to_string();
                    // 带超时的健康检查
                    let health = match tokio::time::timeout(
                        tokio::time::Duration::from_secs(5),
                        component.health_check()
                    ).await {
                        Ok(Ok(healthy)) => healthy,
                        Ok(Err(e)) => {
                            error!("Health check failed for component {}: {}", id, e);
                            false
                        }
                        Err(_) => {
                            error!("Health check timed out for component {}", id);
                            false
                        }
                    };
                    let mut health_map = component_health.write().await;
                    health_map.insert(id, health);
                }
            }
        });

        // 使用 Mutex 安全地修改 health_check_task
        let mut health_check_task = self.health_check_task.lock().await;
        *health_check_task = Some(task);
    }

    /// 发送消息给组件
    pub async fn send_message(&self, component_id: &str, message: ComponentMessage) -> Result<(), String> {
        let communication = self.component_communication.read().await;
        if let Some(sender) = communication.get(component_id) {
            sender.send(message).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Component {} not found or no communication channel", component_id))
        }
    }

    /// 注册组件通信通道
    pub async fn register_communication_channel(&self, component_id: &str, sender: mpsc::Sender<ComponentMessage>) {
        let mut communication = self.component_communication.write().await;
        communication.insert(component_id.to_string(), sender);
    }

    /// 获取组件
    pub async fn get_component(&self) -> Result<Arc<dyn GufComponent>, String> {
        // 尝试从空闲组件中获取健康的组件
        let mut idle_components = self.idle_components.write().await;
        let mut best_component = None;
        let mut best_index = None;

        // 查找健康的空闲组件，优先选择最近使用的
        for (i, component) in idle_components.iter().enumerate() {
            let component_id = component.id();
            let health = {
                let health_map = self.component_health.read().await;
                health_map.get(component_id).unwrap_or(&true).clone()
            };

            if health && component.status() == GufComponentStatus::Started {
                // 选择第一个健康的组件
                best_component = Some(component.clone());
                best_index = Some(i);
                break;
            }
        }

        // 如果找到健康的组件，使用它
        if let (Some(component), Some(index)) = (best_component, best_index) {
            // 从空闲列表中移除
            idle_components.remove(index);
            // 标记为活跃
            let mut active_components = self.active_components.write().await;
            active_components.insert(component.id().to_string(), component.clone());
            return Ok(component);
        }

        // 如果没有健康的空闲组件，尝试创建新组件
        drop(idle_components);

        // 检查是否达到最大组件数量
        let active_components = self.active_components.read().await;
        if active_components.len() >= self.config.max_size {
            // 尝试清理一些空闲组件
            self.cleanup().await?;
            
            // 再次检查
            let active_components = self.active_components.read().await;
            if active_components.len() >= self.config.max_size {
                return Err(format!(
                    "Component pool reached maximum size: {}",
                    self.config.max_size
                ));
            }
        }
        drop(active_components);

        // 创建新组件
        let component_id = format!(
            "{}_{}_{}",
            self.config.component_type,
            std::process::id(),
            rand::random::<u32>()
        );
        
        // 带超时的组件创建
        let component = match tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            async {
                let component = self.factory.create_component(&component_id, self.component_config.clone())?;
                component.initialize().await?;
                component.start().await?;
                Ok::<Arc<dyn GufComponent>, String>(component)
            }
        ).await {
            Ok(Ok(component)) => component,
            Ok(Err(e)) => return Err(format!("Failed to create component: {}", e)),
            Err(_) => return Err("Component creation timed out".to_string()),
        };

        // 为新组件创建通信通道
        let (sender, _receiver) = mpsc::channel(100);
        self.register_communication_channel(&component_id, sender).await;

        // 标记为活跃
        let mut active_components = self.active_components.write().await;
        active_components.insert(component.id().to_string(), component.clone());

        // 更新健康状态
        let mut health_map = self.component_health.write().await;
        health_map.insert(component.id().to_string(), true);

        Ok(component)
    }

    /// 返回组件到池
    pub async fn return_component(&self, component_id: &str) -> Result<(), String> {
        // 从活跃组件中移除
        let mut active_components = self.active_components.write().await;
        let component = match active_components.remove(component_id) {
            Some(c) => c,
            None => {
                return Err(format!(
                    "Component not found in active components: {}",
                    component_id
                ));
            }
        };
        drop(active_components);

        // 执行组件健康检查
        let health = match tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            component.health_check()
        ).await {
            Ok(Ok(healthy)) => healthy,
            Ok(Err(e)) => {
                error!("Health check failed when returning component {}: {}", component_id, e);
                false
            }
            Err(_) => {
                error!("Health check timed out when returning component {}", component_id);
                false
            }
        };

        // 更新健康状态
        let mut health_map = self.component_health.write().await;
        health_map.insert(component_id.to_string(), health);
        drop(health_map);

        // 检查组件状态和健康状态
        if component.status() != GufComponentStatus::Started || !health {
            // 组件状态异常或不健康，销毁并清理相关资源
            warn!("Returning unhealthy component {} to pool, destroying instead", component_id);
            
            // 带超时的组件停止和销毁
            match tokio::time::timeout(
                tokio::time::Duration::from_secs(10),
                async {
                    component.stop().await.ok();
                    component.destroy().await.ok();
                }
            ).await {
                Ok(_) => {},
                Err(_) => error!("Component cleanup timed out for {}", component_id),
            };
            
            // 清理健康状态和通信通道
            let mut health_map = self.component_health.write().await;
            health_map.remove(component_id);
            
            let mut communication = self.component_communication.write().await;
            communication.remove(component_id);
            
            return Ok(());
        }

        // 检查空闲组件数量
        let mut idle_components = self.idle_components.write().await;
        if idle_components.len() >= self.config.max_size {
            // 超出最大数量，销毁组件
            warn!("Component pool full, destroying component {}", component_id);
            
            // 带超时的组件停止和销毁
            match tokio::time::timeout(
                tokio::time::Duration::from_secs(10),
                async {
                    component.stop().await.ok();
                    component.destroy().await.ok();
                }
            ).await {
                Ok(_) => {},
                Err(_) => error!("Component cleanup timed out for {}", component_id),
            };
            
            // 清理健康状态和通信通道
            let mut health_map = self.component_health.write().await;
            health_map.remove(component_id);
            
            let mut communication = self.component_communication.write().await;
            communication.remove(component_id);
            
            return Ok(());
        }

        // 返回空闲组件池
        idle_components.push(component);
        debug!("Component {} returned to pool", component_id);

        Ok(())
    }

    /// 清理空闲组件
    pub async fn cleanup(&self) -> Result<(), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 检查是否需要清理
        let last_cleanup = self.last_cleanup.load(std::sync::atomic::Ordering::Relaxed);
        if now - last_cleanup < 60 {
            // 每分钟清理一次
            return Ok(());
        }

        // 更新最后清理时间
        self.last_cleanup
            .store(now, std::sync::atomic::Ordering::Relaxed);

        // 清理超时空闲组件和不健康的组件
        let mut idle_components = self.idle_components.write().await;
        let mut components_to_clean = Vec::new();

        // 标记需要移除的组件
        let mut i = 0;
        while i < idle_components.len() {
            let component = &idle_components[i];
            let component_id = component.id();
            
            // 检查组件健康状态
            let health = {
                let health_map = self.component_health.read().await;
                health_map.get(component_id).unwrap_or(&false).clone()
            };

            // 清理条件：
            // 1. 组件不健康
            // 2. 组件状态不是已启动
            // 3. 空闲组件数量超过最小组件数量且超过清理阈值
            let should_clean = !health || 
                             component.status() != GufComponentStatus::Started ||
                             (idle_components.len() > self.config.min_size && 
                              idle_components.len() > self.config.cleanup_threshold);

            if should_clean {
                let component = idle_components.remove(i);
                components_to_clean.push(component);
            } else {
                i += 1;
            }
        }
        drop(idle_components);

        info!("Cleaning up {} idle components", components_to_clean.len());

        // 并行清理组件
        let mut tasks = Vec::with_capacity(components_to_clean.len());
        for component in components_to_clean {
            let component_id = component.id().to_string();
            let component_health = self.component_health.clone();
            let component_communication = self.component_communication.clone();
            
            tasks.push(tokio::spawn(async move {
                // 带超时的组件停止和销毁
                match tokio::time::timeout(
                    tokio::time::Duration::from_secs(10),
                    async {
                        component.stop().await.ok();
                        component.destroy().await.ok();
                    }
                ).await {
                    Ok(_) => debug!("Successfully cleaned up component {}", component_id),
                    Err(_) => error!("Component cleanup timed out for {}", component_id),
                };
                
                // 清理健康状态和通信通道
                let mut health_map = component_health.write().await;
                health_map.remove(&component_id);
                
                let mut communication = component_communication.write().await;
                communication.remove(&component_id);
            }));
        }

        // 等待所有清理任务完成
        for task in tasks {
            let _ = task.await;
        }

        Ok(())
    }

    /// 获取池状态
    pub async fn get_status(&self) -> (usize, usize) {
        let idle = self.idle_components.read().await.len();
        let active = self.active_components.read().await.len();
        (idle, active)
    }

    /// 关闭组件池
    pub async fn shutdown(&self) -> Result<(), String> {
        // 停止健康检查任务
        let mut health_check_task = self.health_check_task.lock().await;
        if let Some(task) = health_check_task.take() {
            task.abort();
        }

        // 清理空闲组件
        let mut idle_components = self.idle_components.write().await;
        for component in idle_components.drain(..) {
            component.stop().await.ok();
            component.destroy().await.ok();
        }
        drop(idle_components);

        // 清理活跃组件
        let mut active_components = self.active_components.write().await;
        for component in active_components.values() {
            component.stop().await.ok();
            component.destroy().await.ok();
        }
        active_components.clear();

        // 清理健康状态和通信通道
        let mut health_map = self.component_health.write().await;
        health_map.clear();
        
        let mut communication = self.component_communication.write().await;
        communication.clear();

        Ok(())
    }
}

/// 组件池映射
pub type ComponentPoolMap = std::collections::HashMap<String, Arc<ComponentPool>>;

/// GUF 组件管理器
/// 负责管理 GUF 组件的生命周期和状态
pub struct GufComponentManager {
    /// 组件注册表
    component_registry: Arc<RwLock<GufComponentRegistry>>,
    /// 组件实例映射
    component_instances: Arc<RwLock<ComponentInstanceMap>>,
    /// 组件依赖图
    dependency_graph: Arc<RwLock<DependencyGraph>>,
    /// 组件池映射
    component_pools: Arc<RwLock<ComponentPoolMap>>,
    /// 初始化状态
    initialized: bool,
}

/// 组件实例映射
pub type ComponentInstanceMap = std::collections::HashMap<String, Arc<dyn GufComponent>>;

/// 依赖图
pub type DependencyGraph = std::collections::HashMap<String, Vec<String>>;

/// GUF 组件注册表
pub struct GufComponentRegistry {
    /// 已注册组件
    components: Vec<GufComponentInfo>,
}

impl GufComponentRegistry {
    /// 创建新的组件注册表
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// 注册组件
    pub fn register(&mut self, component_info: GufComponentInfo) {
        self.components.push(component_info);
    }

    /// 注销组件
    pub fn unregister(&mut self, component_id: &str) {
        self.components.retain(|c| c.id != component_id);
    }

    /// 获取组件
    pub fn get(&self, component_id: &str) -> Option<&GufComponentInfo> {
        self.components.iter().find(|c| c.id == component_id)
    }

    /// 更新组件状态
    pub fn update_status(&mut self, component_id: &str, status: GufComponentStatus) {
        if let Some(component) = self.components.iter_mut().find(|c| c.id == component_id) {
            component.status = status;
        }
    }
}

/// GUF 组件 trait
#[async_trait]
pub trait GufComponent: Send + Sync {
    /// 获取组件 ID
    fn id(&self) -> &str;

    /// 获取组件名称
    fn name(&self) -> &str;

    /// 获取组件版本
    fn version(&self) -> &str;

    /// 获取组件状态
    fn status(&self) -> GufComponentStatus;

    /// 初始化组件
    async fn initialize(&self) -> Result<(), String>;

    /// 启动组件
    async fn start(&self) -> Result<(), String>;

    /// 停止组件
    async fn stop(&self) -> Result<(), String>;

    /// 销毁组件
    async fn destroy(&self) -> Result<(), String>;

    /// 获取组件依赖
    fn get_dependencies(&self) -> Vec<String>;

    /// 健康检查
    async fn health_check(&self) -> Result<bool, String>;
}

/// GUF 组件工厂
pub trait GufComponentFactory: Send + Sync {
    /// 创建组件实例
    fn create_component(
        &self,
        component_id: &str,
        config: serde_json::Value,
    ) -> Result<Arc<dyn GufComponent>, String>;
}

impl GufComponentManager {
    /// 创建新的组件管理器
    pub fn new() -> Self {
        // 创建默认的组件注册表
        let component_registry = Arc::new(RwLock::new(GufComponentRegistry::new()));
        Self {
            component_registry,
            component_instances: Arc::new(RwLock::new(ComponentInstanceMap::new())),
            dependency_graph: Arc::new(RwLock::new(DependencyGraph::new())),
            component_pools: Arc::new(RwLock::new(ComponentPoolMap::new())),
            initialized: false,
        }
    }

    /// 使用指定的组件注册表创建新的组件管理器
    pub fn new_with_registry(component_registry: Arc<RwLock<GufComponentRegistry>>) -> Self {
        Self {
            component_registry,
            component_instances: Arc::new(RwLock::new(ComponentInstanceMap::new())),
            dependency_graph: Arc::new(RwLock::new(DependencyGraph::new())),
            component_pools: Arc::new(RwLock::new(ComponentPoolMap::new())),
            initialized: false,
        }
    }

    /// 创建组件池
    pub async fn create_component_pool(
        &self,
        pool_id: &str,
        config: ComponentPoolConfig,
        factory: Arc<dyn GufComponentFactory>,
        component_config: serde_json::Value,
    ) -> Result<(), String> {
        let pool = Arc::new(ComponentPool::new(config, factory, component_config));
        pool.initialize().await?;

        let mut pools = self.component_pools.write().await;
        pools.insert(pool_id.to_string(), pool);

        Ok(())
    }

    /// 从组件池获取组件
    pub async fn get_component_from_pool(
        &self,
        pool_id: &str,
    ) -> Result<Arc<dyn GufComponent>, String> {
        let pools = self.component_pools.read().await;
        let pool = pools
            .get(pool_id)
            .ok_or_else(|| format!("Component pool not found: {}", pool_id))?;

        let component = pool.get_component().await?;
        Ok(component)
    }

    /// 返回组件到组件池
    pub async fn return_component_to_pool(
        &self,
        pool_id: &str,
        component_id: &str,
    ) -> Result<(), String> {
        let pools = self.component_pools.read().await;
        let pool = pools
            .get(pool_id)
            .ok_or_else(|| format!("Component pool not found: {}", pool_id))?;

        pool.return_component(component_id).await
    }

    /// 清理组件池
    pub async fn cleanup_component_pools(&self) -> Result<(), String> {
        let pools = self.component_pools.read().await;
        for pool in pools.values() {
            pool.cleanup().await?;
        }

        Ok(())
    }

    /// 获取组件池状态
    pub async fn get_component_pool_status(&self, pool_id: &str) -> Result<(usize, usize), String> {
        let pools = self.component_pools.read().await;
        let pool = pools
            .get(pool_id)
            .ok_or_else(|| format!("Component pool not found: {}", pool_id))?;

        Ok(pool.get_status().await)
    }

    /// 关闭组件池
    pub async fn shutdown_component_pool(&self, pool_id: &str) -> Result<(), String> {
        let mut pools = self.component_pools.write().await;
        if let Some(pool) = pools.remove(pool_id) {
            pool.shutdown().await
        } else {
            Err(format!("Component pool not found: {}", pool_id))
        }
    }

    /// 关闭所有组件池
    pub async fn shutdown_all_component_pools(&self) -> Result<(), String> {
        let mut pools = self.component_pools.write().await;
        for pool in pools.values() {
            pool.shutdown().await?;
        }
        pools.clear();

        Ok(())
    }
}

impl Clone for GufComponentManager {
    fn clone(&self) -> Self {
        Self {
            component_registry: self.component_registry.clone(),
            component_instances: self.component_instances.clone(),
            dependency_graph: self.dependency_graph.clone(),
            component_pools: self.component_pools.clone(),
            initialized: self.initialized,
        }
    }
}

impl GufComponentManager {
    /// 初始化组件管理器
    pub async fn init(&mut self) -> Result<(), String> {
        // 初始化组件管理器
        // 这里可以添加初始化逻辑
        self.initialized = true;
        Ok(())
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        // 简单检查，实际应用中可能需要更复杂的逻辑
        self.initialized
    }

    /// 注册组件
    pub async fn register_component(&self, component_info: GufComponentInfo) -> Result<(), String> {
        let mut registry = self.component_registry.write().await;
        registry.register(component_info);
        Ok(())
    }

    /// 注销组件
    pub async fn unregister_component(&self, component_id: &str) -> Result<(), String> {
        // 停止并销毁组件
        self.destroy_component(component_id).await?;

        // 从注册表中移除
        let mut registry = self.component_registry.write().await;
        registry.unregister(component_id);

        // 从依赖图中移除
        let mut dependency_graph = self.dependency_graph.write().await;
        dependency_graph.remove(component_id);

        Ok(())
    }

    /// 创建组件
    pub async fn create_component(
        &self,
        factory: &dyn GufComponentFactory,
        component_id: &str,
        config: serde_json::Value,
    ) -> Result<Arc<dyn GufComponent>, String> {
        // 验证参数
        if component_id.is_empty() {
            return Err("Component ID cannot be empty".to_string());
        }

        // 检查组件是否已存在
        let instances = self.component_instances.read().await;
        if instances.contains_key(component_id) {
            return Err(format!("Component already exists: {}", component_id));
        }
        drop(instances);

        // 创建组件实例
        let component = factory
            .create_component(component_id, config)
            .map_err(|e| format!("Failed to create component: {}", e))?;

        // 注册到实例映射
        let mut instances = self.component_instances.write().await;
        instances.insert(component_id.to_string(), component.clone());

        // 更新依赖图
        let dependencies = component.get_dependencies();

        // 检查依赖循环
        if self
            .check_dependency_cycle(component_id, &dependencies)
            .await
        {
            instances.remove(component_id);
            return Err(format!(
                "Dependency cycle detected for component: {}",
                component_id
            ));
        }

        let mut dependency_graph = self.dependency_graph.write().await;
        dependency_graph.insert(component_id.to_string(), dependencies);

        // 更新组件状态
        let mut registry = self.component_registry.write().await;
        registry.update_status(component_id, GufComponentStatus::Registered);

        Ok(component)
    }

    /// 检查依赖循环
    async fn check_dependency_cycle(&self, component_id: &str, dependencies: &[String]) -> bool {
        // 检查直接循环依赖
        for dep_id in dependencies {
            if dep_id == component_id {
                return true;
            }
        }

        // 获取完整的依赖图
        let dependency_graph = self.dependency_graph.read().await;

        // 使用深度优先搜索检测循环依赖
        let mut visited = std::collections::HashSet::new();
        let mut recursion_stack = std::collections::HashSet::new();

        // 检查新添加的依赖是否会导致循环
        for dep_id in dependencies {
            if self.dfs_check_cycle(&dependency_graph, dep_id, component_id, &mut visited, &mut recursion_stack) {
                return true;
            }
        }

        false
    }

    /// 深度优先搜索检查循环依赖
    fn dfs_check_cycle(
        &self,
        dependency_graph: &DependencyGraph,
        current_id: &str,
        target_id: &str,
        visited: &mut std::collections::HashSet<String>,
        recursion_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        // 如果当前节点就是目标节点，发现循环
        if current_id == target_id {
            return true;
        }

        // 标记当前节点为已访问
        if !visited.insert(current_id.to_string()) {
            // 如果已经访问过但不在递归栈中，说明没有循环
            return recursion_stack.contains(current_id);
        }

        // 将当前节点加入递归栈
        recursion_stack.insert(current_id.to_string());

        // 检查所有依赖
        if let Some(deps) = dependency_graph.get(current_id) {
            for dep_id in deps {
                if self.dfs_check_cycle(dependency_graph, dep_id, target_id, visited, recursion_stack) {
                    return true;
                }
            }
        }

        // 将当前节点从递归栈中移除
        recursion_stack.remove(current_id);

        false
    }

    /// 创建 Godot 组件
    pub async fn create_godot_component(
        &self,
        component_type: &str,
        props: serde_json::Value,
    ) -> Result<Arc<dyn GufComponent>, String> {
        // 验证参数
        if component_type.is_empty() {
            return Err("Component type cannot be empty".to_string());
        }

        // 使用 Godot UI Framework v4.4 版本
        let godot_version = GufVersion {
            major: 4,
            minor: 4,
            patch: 0,
        };

        // 获取适配器注册表
        let registry = get_adapter_registry().await;

        // 获取适用于 v4.4 的适配器
        let adapter = registry
            .get_adapter(&godot_version)
            .ok_or_else(|| "No adapter found for Godot UI Framework v4.4".to_string())?;

        // 创建 Godot 组件
        let component_result = adapter.create_component(component_type, props).await;
        let component_data =
            component_result.map_err(|e| format!("Failed to create Godot component: {:?}", e))?;

        // 提取组件 ID
        let component_id = component_data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Failed to get component ID".to_string())?
            .to_string();

        // 检查组件是否已存在
        let instances = self.component_instances.read().await;
        if instances.contains_key(&component_id) {
            return Err(format!("Godot component already exists: {}", component_id));
        }
        drop(instances);

        // 创建 GufComponent 包装器
        let godot_component = GodotComponent::new(
            component_id.clone(),
            component_type.to_string(),
            "4.4.0".to_string(),
            component_data,
            adapter.clone(),
        );

        // 注册到实例映射
        let mut instances = self.component_instances.write().await;
        instances.insert(component_id.clone(), Arc::new(godot_component));

        // 更新组件状态
        let mut registry = self.component_registry.write().await;
        registry.update_status(&component_id, GufComponentStatus::Registered);

        // 返回创建的组件
        instances
            .get(&component_id)
            .ok_or_else(|| "Failed to retrieve created component".to_string())
            .cloned()
    }

    /// 初始化组件
    pub async fn initialize_component(&self, component_id: &str) -> Result<(), String> {
        // 验证参数
        if component_id.is_empty() {
            return Err("Component ID cannot be empty".to_string());
        }

        // 检查组件是否存在
        let component = {
            let instances = self.component_instances.read().await;
            instances
                .get(component_id)
                .ok_or_else(|| format!("Component {} not found", component_id))?
                .clone()
        };

        // 检查组件当前状态
        let current_status = {
            let registry = self.component_registry.read().await;
            registry.get(component_id).map(|c| c.status.clone())
        };

        if let Some(status) = current_status {
            match status {
                GufComponentStatus::Started => {
                    return Err(format!("Component {} is already started", component_id));
                }
                GufComponentStatus::Initializing => {
                    return Err(format!(
                        "Component {} is already initializing",
                        component_id
                    ));
                }
                _ => {}
            }
        }

        // 更新组件状态
        {
            let mut registry = self.component_registry.write().await;
            registry.update_status(component_id, GufComponentStatus::Initializing);
        }

        // 初始化组件
        match component.initialize().await {
            Ok(_) => {
                // 更新组件状态
                let mut registry = self.component_registry.write().await;
                registry.update_status(component_id, GufComponentStatus::Started);
                Ok(())
            }
            Err(e) => {
                // 更新组件状态为错误
                let mut registry = self.component_registry.write().await;
                registry.update_status(component_id, GufComponentStatus::Error(e.clone()));
                Err(format!(
                    "Failed to initialize component {}: {}",
                    component_id, e
                ))
            }
        }
    }

    /// 启动组件
    pub fn start_component(
        &self,
        component_id: &str,
    ) -> futures::future::BoxFuture<'_, Result<(), String>> {
        let self_clone = self.clone();
        let component_id = component_id.to_string();

        Box::pin(async move {
            // 验证参数
            if component_id.is_empty() {
                return Err("Component ID cannot be empty".to_string());
            }

            // 检查组件是否存在
            let component = {
                let instances = self_clone.component_instances.read().await;
                instances
                    .get(&component_id)
                    .ok_or_else(|| format!("Component {} not found", component_id))?
                    .clone()
            };

            // 检查组件当前状态
            let current_status = {
                let registry = self_clone.component_registry.read().await;
                registry.get(&component_id).map(|c| c.status.clone())
            };

            if let Some(status) = current_status {
                match status {
                    GufComponentStatus::Started => {
                        return Err(format!("Component {} is already started", component_id));
                    }
                    GufComponentStatus::Error(_) => {
                        return Err(format!("Component {} is in error state", component_id));
                    }
                    _ => {}
                }
            }

            // 启动依赖组件
            {
                let dependency_graph = self_clone.dependency_graph.read().await;
                if let Some(dependencies) = dependency_graph.get(&component_id) {
                    for dep_id in dependencies {
                        if let Err(e) = self_clone.start_component(dep_id).await {
                            return Err(format!("Failed to start dependency {}: {}", dep_id, e));
                        }
                    }
                }
            }

            // 启动组件
            match component.start().await {
                Ok(_) => {
                    // 更新组件状态
                    let mut registry = self_clone.component_registry.write().await;
                    registry.update_status(&component_id, GufComponentStatus::Started);
                    Ok(())
                }
                Err(e) => {
                    // 更新组件状态为错误
                    let mut registry = self_clone.component_registry.write().await;
                    registry.update_status(&component_id, GufComponentStatus::Error(e.clone()));
                    Err(format!("Failed to start component {}: {}", component_id, e))
                }
            }
        })
    }

    /// 停止组件
    pub async fn stop_component(&self, component_id: &str) -> Result<(), String> {
        // 验证参数
        if component_id.is_empty() {
            return Err("Component ID cannot be empty".to_string());
        }

        // 检查组件是否存在
        let component = {
            let instances = self.component_instances.read().await;
            instances
                .get(component_id)
                .ok_or_else(|| format!("Component {} not found", component_id))?
                .clone()
        };

        // 检查组件当前状态
        let current_status = {
            let registry = self.component_registry.read().await;
            registry.get(component_id).map(|c| c.status.clone())
        };

        if let Some(status) = current_status {
            if status == GufComponentStatus::Stopped {
                return Err(format!("Component {} is already stopped", component_id));
            }
        }

        // 停止组件
        match component.stop().await {
            Ok(_) => {
                // 更新组件状态
                let mut registry = self.component_registry.write().await;
                registry.update_status(component_id, GufComponentStatus::Stopped);
                Ok(())
            }
            Err(e) => {
                // 更新组件状态为错误
                let mut registry = self.component_registry.write().await;
                registry.update_status(component_id, GufComponentStatus::Error(e.clone()));
                Err(format!("Failed to stop component {}: {}", component_id, e))
            }
        }
    }

    /// 销毁组件
    pub async fn destroy_component(&self, component_id: &str) -> Result<(), String> {
        // 验证参数
        if component_id.is_empty() {
            return Err("Component ID cannot be empty".to_string());
        }

        // 检查组件是否存在
        let instances = self.component_instances.read().await;
        if !instances.contains_key(component_id) {
            return Err(format!("Component {} not found", component_id));
        }
        drop(instances);

        // 停止组件
        if let Err(e) = self.stop_component(component_id).await {
            log::warn!("Failed to stop component before destruction: {}", e);
        }

        // 销毁组件
        let component = {
            let instances = self.component_instances.read().await;
            instances.get(component_id).cloned()
        };

        if let Some(component) = component
            && let Err(e) = component.destroy().await
        {
            log::warn!("Failed to destroy component: {}", e);
        }

        // 从实例映射中移除
        let mut instances = self.component_instances.write().await;
        instances.remove(component_id);

        // 从依赖图中移除
        let mut dependency_graph = self.dependency_graph.write().await;
        dependency_graph.remove(component_id);

        // 从所有其他组件的依赖中移除
        for (_, dependencies) in dependency_graph.iter_mut() {
            dependencies.retain(|dep| dep != component_id);
        }

        // 更新组件状态
        let mut registry = self.component_registry.write().await;
        registry.update_status(component_id, GufComponentStatus::Stopped);

        Ok(())
    }

    /// 获取组件
    pub async fn get_component(&self, component_id: &str) -> Option<Arc<dyn GufComponent>> {
        let instances = self.component_instances.read().await;
        instances.get(component_id).cloned()
    }

    /// 获取所有组件
    pub async fn get_all_components(&self) -> Vec<Arc<dyn GufComponent>> {
        let instances = self.component_instances.read().await;
        instances.values().cloned().collect()
    }

    /// 批量获取组件（减少锁竞争）
    pub async fn get_components_batch(
        &self,
        component_ids: &[&str],
    ) -> std::collections::HashMap<String, Arc<dyn GufComponent>> {
        let instances = self.component_instances.read().await;
        let mut result = std::collections::HashMap::with_capacity(component_ids.len());

        for id in component_ids {
            if let Some(component) = instances.get(*id) {
                result.insert(id.to_string(), component.clone());
            }
        }

        result
    }

    /// 批量启动组件（并行处理）
    pub async fn start_components_batch(
        &self,
        component_ids: &[&str],
    ) -> Result<Vec<String>, String> {
        let mut errors = Vec::new();

        // 并行处理
        let tasks: Vec<_> = component_ids
            .iter()
            .map(|id| {
                let self_clone = self.clone();
                let id = id.to_string();

                tokio::spawn(async move {
                    let result = self_clone.start_component(&id).await;
                    (id, result)
                })
            })
            .collect();

        // 收集结果
        for task in tasks {
            if let Ok((id, result)) = task.await
                && let Err(e) = result
            {
                errors.push(format!("Component {}: {}", id, e));
            }
        }

        if !errors.is_empty() {
            Err(errors.join("; "))
        } else {
            Ok(component_ids.iter().map(|s| s.to_string()).collect())
        }
    }

    /// 获取组件状态
    pub async fn get_component_status(&self, component_id: &str) -> Option<GufComponentStatus> {
        let registry = self.component_registry.read().await;
        registry.get(component_id).map(|c| c.status.clone())
    }

    /// 检查组件是否健康
    pub async fn check_component_health(&self, component_id: &str) -> Result<bool, String> {
        let instances = self.component_instances.read().await;
        let component = instances
            .get(component_id)
            .ok_or_else(|| format!("Component {} not found", component_id))?;

        // 检查组件状态
        match component.status() {
            GufComponentStatus::Started => Ok(true),
            GufComponentStatus::Error(_) => Ok(false),
            _ => Ok(false),
        }
    }

    /// 启动所有组件
    pub async fn start_all_components(&self) -> Result<(), String> {
        let instances = self.component_instances.read().await;
        let component_ids: Vec<String> = instances.keys().cloned().collect();
        drop(instances);

        for component_id in component_ids {
            if let Err(e) = self.start_component(&component_id).await {
                error!("Failed to start component {}: {}", component_id, e);
            }
        }

        Ok(())
    }

    /// 停止所有组件
    pub async fn stop_all_components(&self) -> Result<(), String> {
        let instances = self.component_instances.read().await;
        let component_ids: Vec<String> = instances.keys().cloned().collect();
        drop(instances);

        for component_id in component_ids {
            if let Err(e) = self.stop_component(&component_id).await {
                error!("Failed to stop component {}: {}", component_id, e);
            }
        }

        Ok(())
    }
}

/// Godot 组件实现
/// 包装 Godot UI Framework v4.4 的组件实例
pub struct GodotComponent {
    id: String,
    name: String,
    version: String,
    status: GufComponentStatus,
    component_data: serde_json::Value,
    adapter: Arc<dyn crate::ui::core::adapter::GufAdapter>,
    dependencies: Vec<String>,
}

impl GodotComponent {
    pub fn new(
        id: String,
        name: String,
        version: String,
        component_data: serde_json::Value,
        adapter: Arc<dyn crate::ui::core::adapter::GufAdapter>,
    ) -> Self {
        Self {
            id,
            name,
            version,
            status: GufComponentStatus::Registered,
            component_data,
            adapter,
            dependencies: Vec::new(),
        }
    }

    /// 获取组件数据
    pub fn component_data(&self) -> &serde_json::Value {
        &self.component_data
    }
}

#[async_trait]
impl GufComponent for GodotComponent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn status(&self) -> GufComponentStatus {
        self.status.clone()
    }

    async fn initialize(&self) -> Result<(), String> {
        // Godot 组件初始化
        Ok(())
    }

    async fn start(&self) -> Result<(), String> {
        // Godot 组件启动
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        // Godot 组件停止
        Ok(())
    }

    async fn destroy(&self) -> Result<(), String> {
        // 销毁 Godot 组件
        self.adapter
            .destroy_component(&self.id)
            .await
            .map_err(|e| format!("Failed to destroy Godot component: {:?}", e))
    }

    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    async fn health_check(&self) -> Result<bool, String> {
        // Godot 组件健康检查
        Ok(self.status == GufComponentStatus::Started)
    }
}

/// 示例组件实现
pub struct ExampleComponent {
    id: String,
    name: String,
    version: String,
    status: GufComponentStatus,
    dependencies: Vec<String>,
}

impl ExampleComponent {
    pub fn new(id: String, name: String, version: String, dependencies: Vec<String>) -> Self {
        Self {
            id,
            name,
            version,
            status: GufComponentStatus::Registered,
            dependencies,
        }
    }
}

#[async_trait]
impl GufComponent for ExampleComponent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn status(&self) -> GufComponentStatus {
        self.status.clone()
    }

    async fn initialize(&self) -> Result<(), String> {
        Ok(())
    }

    async fn start(&self) -> Result<(), String> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        Ok(())
    }

    async fn destroy(&self) -> Result<(), String> {
        Ok(())
    }

    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    async fn health_check(&self) -> Result<bool, String> {
        // 示例组件健康检查
        Ok(self.status == GufComponentStatus::Started)
    }
}

/// GUF 组件管理器错误
#[derive(Debug, thiserror::Error)]
pub enum GufComponentManagerError {
    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Component initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Component start failed: {0}")]
    StartFailed(String),

    #[error("Component stop failed: {0}")]
    StopFailed(String),

    #[error("Component destroy failed: {0}")]
    DestroyFailed(String),

    #[error("Dependency error: {0}")]
    DependencyError(String),
}
