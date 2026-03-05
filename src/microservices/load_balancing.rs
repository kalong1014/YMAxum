//! 负载均衡模块
//! 用于服务的负载均衡和请求分发

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 负载均衡算法
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    Random,
    LeastConnections,
    LeastResponseTime,
    IpHash,
    WeightedRoundRobin,
}

/// 负载均衡结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingResult {
    pub service_id: String,
    pub service_address: String,
    pub service_port: u16,
    pub algorithm: LoadBalancingAlgorithm,
    pub duration: Duration,
    pub success: bool,
    pub response: Option<serde_json::Value>,
}

/// 负载均衡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: LoadBalancingAlgorithm,
    pub timeout: Duration,
    pub retries: u32,
    pub health_check_enabled: bool,
    pub health_check_interval: Duration,
    pub circuit_breaker_enabled: bool,
    pub circuit_breaker_threshold: f64,
    pub circuit_breaker_timeout: Duration,
}

/// 服务实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub weight: u32,
    pub connections: u32,
    pub response_time: Duration,
    pub healthy: bool,
    pub last_used: u64,
}

/// 负载均衡
#[derive(Debug, Clone)]
pub struct LoadBalancing {
    config: LoadBalancingConfig,
    services: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<ServiceInstance>>>>,
    service_discovery: Option<std::sync::Arc<super::service_discovery::ServiceDiscovery>>,
    round_robin_state: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, usize>>>,
}

impl LoadBalancing {
    /// 创建新的负载均衡
    pub fn new() -> Self {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            timeout: Duration::from_secs(30),
            retries: 3,
            health_check_enabled: true,
            health_check_interval: Duration::from_secs(10),
            circuit_breaker_enabled: true,
            circuit_breaker_threshold: 0.5,
            circuit_breaker_timeout: Duration::from_secs(30),
        };

        Self {
            config,
            services: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            service_discovery: None,
            round_robin_state: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 初始化负载均衡
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化负载均衡
        Ok(())
    }

    /// 负载均衡
    pub async fn balance_load(&self, service_name: &str, request: serde_json::Value) -> Result<LoadBalancingResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 从服务发现获取服务列表
        let service_instances = self.get_service_instances(service_name).await?;
        if service_instances.is_empty() {
            return Err(format!("No healthy instances found for service: {}", service_name).into());
        }

        // 选择服务实例
        let instance = self.select_service_instance(service_name, &service_instances, &request).await?;

        // 执行请求
        let (success, response) = self.execute_request(&instance, &request).await;

        // 更新服务实例状态
        self.update_service_instance_state(service_name, &instance, success, start_time.elapsed()).await;

        let duration = start_time.elapsed();

        Ok(LoadBalancingResult {
            service_id: instance.id.clone(),
            service_address: instance.address.clone(),
            service_port: instance.port,
            algorithm: self.config.algorithm.clone(),
            duration,
            success,
            response,
        })
    }

    /// 获取服务实例
    async fn get_service_instances(&self, service_name: &str) -> Result<Vec<ServiceInstance>, Box<dyn std::error::Error>> {
        // 如果有服务发现，从服务发现获取
        if let Some(service_discovery) = &self.service_discovery {
            let service_infos = service_discovery.discover_service(service_name).await?;
            let instances: Vec<ServiceInstance> = service_infos.into_iter().map(|info| {
                ServiceInstance {
                    id: info.id,
                    address: info.address,
                    port: info.port,
                    weight: 1,
                    connections: 0,
                    response_time: Duration::from_secs(0),
                    healthy: info.status == super::service_discovery::ServiceStatus::Healthy,
                    last_used: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }).collect();
            Ok(instances)
        } else {
            // 从本地缓存获取
            let services = self.services.read().await;
            match services.get(service_name) {
                Some(instances) => {
                    let healthy_instances: Vec<ServiceInstance> = instances
                        .iter()
                        .filter(|i| i.healthy)
                        .cloned()
                        .collect();
                    Ok(healthy_instances)
                }
                None => Ok(Vec::new()),
            }
        }
    }

    /// 选择服务实例
    async fn select_service_instance(&self, service_name: &str, instances: &[ServiceInstance], request: &serde_json::Value) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        match self.config.algorithm {
            LoadBalancingAlgorithm::RoundRobin => self.select_round_robin(service_name, instances).await,
            LoadBalancingAlgorithm::Random => self.select_random(instances).await,
            LoadBalancingAlgorithm::LeastConnections => self.select_least_connections(instances).await,
            LoadBalancingAlgorithm::LeastResponseTime => self.select_least_response_time(instances).await,
            LoadBalancingAlgorithm::IpHash => self.select_ip_hash(request, instances).await,
            LoadBalancingAlgorithm::WeightedRoundRobin => self.select_weighted_round_robin(service_name, instances).await,
        }
    }

    /// 轮询选择
    async fn select_round_robin(&self, service_name: &str, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        let mut state = self.round_robin_state.write().await;
        let index = state.entry(service_name.to_string()).or_insert(0);
        let instance = &instances[*index % instances.len()];
        *index += 1;
        Ok(instance.clone())
    }

    /// 随机选择
    async fn select_random(&self, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let index = rand::Rng::gen_range(&mut rng, 0..instances.len());
        Ok(instances[index].clone())
    }

    /// 最少连接选择
    async fn select_least_connections(&self, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        let mut min_connections = u32::MAX;
        let mut selected_instance = None;

        for instance in instances {
            if instance.connections < min_connections {
                min_connections = instance.connections;
                selected_instance = Some(instance.clone());
            }
        }

        selected_instance.ok_or_else(|| "No instances found".into())
    }

    /// 最少响应时间选择
    async fn select_least_response_time(&self, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        let mut min_response_time = Duration::from_secs(u64::MAX);
        let mut selected_instance = None;

        for instance in instances {
            if instance.response_time < min_response_time {
                min_response_time = instance.response_time;
                selected_instance = Some(instance.clone());
            }
        }

        selected_instance.ok_or_else(|| "No instances found".into())
    }

    /// IP哈希选择
    async fn select_ip_hash(&self, request: &serde_json::Value, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        // 从请求中提取IP地址
        let ip = request.get("ip").and_then(|v| v.as_str()).unwrap_or("127.0.0.1");
        
        // 计算IP哈希
        let hash = crc32fast::hash(ip.as_bytes());
        let index = hash as usize % instances.len();
        
        Ok(instances[index].clone())
    }

    /// 加权轮询选择
    async fn select_weighted_round_robin(&self, service_name: &str, instances: &[ServiceInstance]) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        // 计算总权重
        let total_weight: u32 = instances.iter().map(|i| i.weight).sum();
        
        if total_weight == 0 {
            return Err("Total weight is zero".into());
        }
        
        // 生成随机数
        let mut rng = rand::thread_rng();
        let random = rand::Rng::gen_range(&mut rng, 0..total_weight);
        
        // 选择服务实例
        let mut current_weight = 0;
        for instance in instances {
            current_weight += instance.weight;
            if random < current_weight {
                return Ok(instance.clone());
            }
        }
        
        Ok(instances[0].clone())
    }

    /// 执行请求
    async fn execute_request(&self, instance: &ServiceInstance, request: &serde_json::Value) -> (bool, Option<serde_json::Value>) {
        // 这里应该实现实际的请求执行逻辑
        // 为了演示，我们返回成功
        (true, Some(serde_json::json!({
            "message": "Request executed successfully",
            "service_id": instance.id,
            "request": request
        })))
    }

    /// 更新服务实例状态
    async fn update_service_instance_state(&self, service_name: &str, instance: &ServiceInstance, success: bool, response_time: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            for i in instances.iter_mut() {
                if i.id == instance.id {
                    i.connections = if success {
                        i.connections.saturating_sub(1)
                    } else {
                        i.connections
                    };
                    i.response_time = response_time;
                    i.last_used = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs();
                    break;
                }
            }
        }
        Ok(())
    }

    /// 注册服务实例
    pub async fn register_service_instance(&self, service_name: &str, instance: ServiceInstance) -> Result<(), Box<dyn std::error::Error>> {
        let mut services = self.services.write().await;
        let instances = services.entry(service_name.to_string()).or_insert(Vec::new());
        
        // 检查实例是否已存在
        let existing_index = instances.iter().position(|i| i.id == instance.id);
        match existing_index {
            Some(index) => {
                // 更新现有实例
                instances[index] = instance;
            }
            None => {
                // 添加新实例
                instances.push(instance);
            }
        }
        
        Ok(())
    }

    /// 注销服务实例
    pub async fn deregister_service_instance(&self, service_name: &str, instance_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            instances.retain(|i| i.id != instance_id);
        }
        Ok(())
    }

    /// 设置服务发现
    pub fn set_service_discovery(&mut self, service_discovery: std::sync::Arc<super::service_discovery::ServiceDiscovery>) {
        self.service_discovery = Some(service_discovery);
    }
}
