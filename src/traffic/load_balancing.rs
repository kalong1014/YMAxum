//! 负载均衡模块
//!
//! 提供服务实例管理、流量分配等功能

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 负载均衡策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadBalancingStrategy {
    /// 轮询
    RoundRobin,
    /// 随机
    Random,
    /// 最少连接
    LeastConnections,
    /// 最少响应时间
    LeastResponseTime,
    /// IP哈希
    IpHash,
    /// 权重轮询
    WeightedRoundRobin,
    /// 权重随机
    WeightedRandom,
}

/// 负载均衡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// 启用负载均衡
    pub enabled: bool,
    /// 负载均衡策略
    pub strategy: LoadBalancingStrategy,
    /// 服务实例
    pub instances: Vec<ServiceInstance>,
    /// 健康检查配置
    pub health_check: HealthCheckConfig,
    /// 会话粘性配置
    pub session_stickiness: SessionStickinessConfig,
    /// 重试配置
    pub retry: RetryConfig,
}

/// 服务实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// 实例ID
    pub id: String,
    /// 实例名称
    pub name: String,
    /// 实例地址
    pub address: String,
    /// 实例端口
    pub port: u16,
    /// 实例权重
    pub weight: u32,
    /// 实例健康状态
    pub healthy: bool,
    /// 实例标签
    pub tags: Vec<String>,
    /// 实例元数据
    pub metadata: serde_json::Value,
}

/// 健康检查配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// 启用健康检查
    pub enabled: bool,
    /// 检查间隔（毫秒）
    pub interval_ms: u64,
    /// 超时时间（毫秒）
    pub timeout_ms: u64,
    /// 失败阈值
    pub failure_threshold: u32,
    /// 成功阈值
    pub success_threshold: u32,
    /// 检查路径
    pub path: String,
    /// 检查方法
    pub method: String,
}

/// 会话粘性配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStickinessConfig {
    /// 启用会话粘性
    pub enabled: bool,
    /// 粘性类型
    pub type_: String,
    /// 粘性超时（秒）
    pub timeout_seconds: u32,
    /// 粘性键
    pub sticky_key: String,
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// 启用重试
    pub enabled: bool,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔（毫秒）
    pub retry_interval_ms: u64,
    /// 重试状态码
    pub retry_status_codes: Vec<u16>,
}

/// 实例统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStats {
    /// 实例ID
    pub instance_id: String,
    /// 总请求数
    pub total_requests: u32,
    /// 成功请求数
    pub success_requests: u32,
    /// 失败请求数
    pub failure_requests: u32,
    /// 当前连接数
    pub current_connections: u32,
    /// 平均响应时间（毫秒）
    pub avg_response_time_ms: f64,
    /// 最后请求时间
    pub last_request_time: String,
}

/// 负载均衡
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    config: Arc<RwLock<LoadBalancingConfig>>,
    instance_stats: Arc<RwLock<HashMap<String, InstanceStats>>>,
    round_robin_index: Arc<RwLock<usize>>,
    session_mappings: Arc<RwLock<HashMap<String, String>>>,
}

impl LoadBalancer {
    /// 创建新的负载均衡
    pub fn new(config: LoadBalancingConfig) -> Self {
        let mut instance_stats = HashMap::new();
        for instance in &config.instances {
            instance_stats.insert(
                instance.id.clone(),
                InstanceStats {
                    instance_id: instance.id.clone(),
                    total_requests: 0,
                    success_requests: 0,
                    failure_requests: 0,
                    current_connections: 0,
                    avg_response_time_ms: 0.0,
                    last_request_time: chrono::Utc::now().to_string(),
                },
            );
        }

        Self {
            config: Arc::new(RwLock::new(config)),
            instance_stats: Arc::new(RwLock::new(instance_stats)),
            round_robin_index: Arc::new(RwLock::new(0)),
            session_mappings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 初始化负载均衡
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化负载均衡
        Ok(())
    }

    /// 选择服务实例
    pub async fn select_instance(
        &self,
        client_ip: &str,
        session_id: Option<&str>,
    ) -> Result<ServiceInstance, Box<dyn std::error::Error>> {
        let config = self.config.read().await;

        if !config.enabled {
            return Err(Box::new(std::io::Error::other(
                "Load balancing is disabled",
            )));
        }

        // 获取健康实例
        let healthy_instances = config
            .instances
            .iter()
            .filter(|instance| instance.healthy)
            .cloned()
            .collect::<Vec<_>>();

        if healthy_instances.is_empty() {
            return Err(Box::new(std::io::Error::other(
                "No healthy instances available",
            )));
        }

        // 检查会话粘性
        if config.session_stickiness.enabled && session_id.is_some() {
            let session_id = session_id.unwrap();
            let instance = self.get_sticky_instance(session_id).await;
            if instance.is_some() {
                return Ok(instance.unwrap());
            }
        }

        // 根据策略选择实例
        let selected_instance = match config.strategy {
            LoadBalancingStrategy::RoundRobin => self.select_round_robin(&healthy_instances).await,
            LoadBalancingStrategy::Random => self.select_random(&healthy_instances).await,
            LoadBalancingStrategy::LeastConnections => {
                self.select_least_connections(&healthy_instances).await
            }
            LoadBalancingStrategy::LeastResponseTime => {
                self.select_least_response_time(&healthy_instances).await
            }
            LoadBalancingStrategy::IpHash => {
                self.select_ip_hash(&healthy_instances, client_ip).await
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                self.select_weighted_round_robin(&healthy_instances).await
            }
            LoadBalancingStrategy::WeightedRandom => {
                self.select_weighted_random(&healthy_instances).await
            }
        };

        // 记录会话粘性
        if config.session_stickiness.enabled && session_id.is_some() {
            let session_id = session_id.unwrap();
            self.set_sticky_instance(session_id, &selected_instance.id)
                .await;
        }

        Ok(selected_instance)
    }

    /// 轮询选择
    async fn select_round_robin(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        let mut index = self.round_robin_index.write().await;
        let instance = instances[*index % instances.len()].clone();
        *index += 1;
        instance
    }

    /// 随机选择
    async fn select_random(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        let mut rng = rand::thread_rng();
        let index = rand::Rng::gen_range(&mut rng, 0..instances.len());
        instances[index].clone()
    }

    /// 最少连接选择
    async fn select_least_connections(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        let stats = self.instance_stats.read().await;

        let mut selected_instance = &instances[0];
        let mut min_connections = stats
            .get(&selected_instance.id)
            .map(|s| s.current_connections)
            .unwrap_or(0);

        for instance in instances.iter().skip(1) {
            let connections = stats
                .get(&instance.id)
                .map(|s| s.current_connections)
                .unwrap_or(0);

            if connections < min_connections {
                min_connections = connections;
                selected_instance = instance;
            }
        }

        selected_instance.clone()
    }

    /// 最少响应时间选择
    async fn select_least_response_time(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        let stats = self.instance_stats.read().await;

        let mut selected_instance = &instances[0];
        let mut min_response_time = stats
            .get(&selected_instance.id)
            .map(|s| s.avg_response_time_ms)
            .unwrap_or(f64::MAX);

        for instance in instances.iter().skip(1) {
            let response_time = stats
                .get(&instance.id)
                .map(|s| s.avg_response_time_ms)
                .unwrap_or(f64::MAX);

            if response_time < min_response_time {
                min_response_time = response_time;
                selected_instance = instance;
            }
        }

        selected_instance.clone()
    }

    /// IP哈希选择
    async fn select_ip_hash(
        &self,
        instances: &[ServiceInstance],
        client_ip: &str,
    ) -> ServiceInstance {
        let hash = self.hash_ip(client_ip);
        let index = hash % instances.len() as u64;
        instances[index as usize].clone()
    }

    /// 权重轮询选择
    async fn select_weighted_round_robin(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        // 实现权重轮询算法
        let mut total_weight = 0;
        for instance in instances {
            total_weight += instance.weight;
        }

        let mut index = self.round_robin_index.write().await;
        let total_weight_usize = total_weight as usize;
        let weight_index = *index % total_weight_usize;
        *index += 1;

        let mut current_weight = 0;
        for instance in instances {
            current_weight += instance.weight;
            if weight_index < current_weight as usize {
                return instance.clone();
            }
        }

        instances[0].clone()
    }

    /// 权重随机选择
    async fn select_weighted_random(&self, instances: &[ServiceInstance]) -> ServiceInstance {
        // 实现权重随机算法
        let mut total_weight = 0;
        for instance in instances {
            total_weight += instance.weight;
        }

        let mut rng = rand::thread_rng();
        let weight = rand::Rng::gen_range(&mut rng, 0..total_weight);

        let mut current_weight = 0;
        for instance in instances {
            current_weight += instance.weight;
            if weight < current_weight {
                return instance.clone();
            }
        }

        instances[0].clone()
    }

    /// IP哈希计算
    fn hash_ip(&self, ip: &str) -> u64 {
        // 简单的IP哈希实现
        let mut hash = 0u64;
        for c in ip.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u64);
        }
        hash
    }

    /// 获取粘性实例
    async fn get_sticky_instance(&self, session_id: &str) -> Option<ServiceInstance> {
        let session_mappings = self.session_mappings.read().await;
        let instance_id = session_mappings.get(session_id);

        if let Some(instance_id) = instance_id {
            let config = self.config.read().await;
            config
                .instances
                .iter()
                .find(|instance| instance.id == *instance_id && instance.healthy)
                .cloned()
        } else {
            None
        }
    }

    /// 设置粘性实例
    async fn set_sticky_instance(&self, session_id: &str, instance_id: &str) {
        let mut session_mappings = self.session_mappings.write().await;
        session_mappings.insert(session_id.to_string(), instance_id.to_string());
    }

    /// 记录请求
    pub async fn record_request(&self, instance_id: &str, success: bool, response_time_ms: f64) {
        let mut stats = self.instance_stats.write().await;
        let instance_stats = stats
            .entry(instance_id.to_string())
            .or_insert(InstanceStats {
                instance_id: instance_id.to_string(),
                total_requests: 0,
                success_requests: 0,
                failure_requests: 0,
                current_connections: 0,
                avg_response_time_ms: 0.0,
                last_request_time: chrono::Utc::now().to_string(),
            });

        instance_stats.total_requests += 1;
        if success {
            instance_stats.success_requests += 1;
        } else {
            instance_stats.failure_requests += 1;
        }

        // 更新平均响应时间
        instance_stats.avg_response_time_ms = (instance_stats.avg_response_time_ms
            * (instance_stats.total_requests - 1) as f64
            + response_time_ms)
            / instance_stats.total_requests as f64;

        instance_stats.last_request_time = chrono::Utc::now().to_string();
    }

    /// 增加连接数
    pub async fn increment_connections(&self, instance_id: &str) {
        let mut stats = self.instance_stats.write().await;
        if let Some(instance_stats) = stats.get_mut(instance_id) {
            instance_stats.current_connections += 1;
        }
    }

    /// 减少连接数
    pub async fn decrement_connections(&self, instance_id: &str) {
        let mut stats = self.instance_stats.write().await;
        if let Some(instance_stats) = stats.get_mut(instance_id)
            && instance_stats.current_connections > 0
        {
            instance_stats.current_connections -= 1;
        }
    }

    /// 更新实例健康状态
    pub async fn update_instance_health(
        &self,
        instance_id: &str,
        healthy: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.config.write().await;

        if let Some(instance) = config.instances.iter_mut().find(|i| i.id == instance_id) {
            instance.healthy = healthy;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Instance {} not found", instance_id),
            )))
        }
    }

    /// 获取配置
    pub async fn get_config(&self) -> LoadBalancingConfig {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update_config(
        &self,
        config: LoadBalancingConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 获取实例统计信息
    pub async fn get_instance_stats(&self, instance_id: &str) -> Option<InstanceStats> {
        let stats = self.instance_stats.read().await;
        stats.get(instance_id).cloned()
    }

    /// 获取所有实例统计信息
    pub async fn get_all_instance_stats(&self) -> HashMap<String, InstanceStats> {
        self.instance_stats.read().await.clone()
    }

    /// 重置统计信息
    pub async fn reset_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        let mut stats = self.instance_stats.write().await;
        stats.clear();

        for instance in &config.instances {
            stats.insert(
                instance.id.clone(),
                InstanceStats {
                    instance_id: instance.id.clone(),
                    total_requests: 0,
                    success_requests: 0,
                    failure_requests: 0,
                    current_connections: 0,
                    avg_response_time_ms: 0.0,
                    last_request_time: chrono::Utc::now().to_string(),
                },
            );
        }

        Ok(())
    }
}
