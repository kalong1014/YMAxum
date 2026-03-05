//! 流量整形模块
//!
//! 提供流量优先级管理、流量控制等功能

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// 流量优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrafficPriority {
    /// 低优先级
    Low,
    /// 中优先级
    Medium,
    /// 高优先级
    High,
    /// 最高优先级
    Critical,
}

/// 流量整形配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficShapingConfig {
    /// 启用流量整形
    pub enabled: bool,
    /// 带宽限制（字节/秒）
    pub bandwidth_limit: u64,
    /// 队列大小
    pub queue_size: usize,
    /// 优先级配置
    pub priority_config: PriorityConfig,
    /// 突发配置
    pub burst_config: BurstConfig,
    /// 流控规则
    pub rules: Vec<TrafficRule>,
}

/// 优先级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityConfig {
    /// 低优先级权重
    pub low_weight: u32,
    /// 中优先级权重
    pub medium_weight: u32,
    /// 高优先级权重
    pub high_weight: u32,
    /// 最高优先级权重
    pub critical_weight: u32,
    /// 优先级队列大小
    pub priority_queue_sizes: PriorityQueueSizes,
}

/// 优先级队列大小
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityQueueSizes {
    /// 低优先级队列大小
    pub low: usize,
    /// 中优先级队列大小
    pub medium: usize,
    /// 高优先级队列大小
    pub high: usize,
    /// 最高优先级队列大小
    pub critical: usize,
}

/// 突发配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstConfig {
    /// 启用突发
    pub enabled: bool,
    /// 突发大小（字节）
    pub burst_size: u64,
    /// 突发持续时间（毫秒）
    pub burst_duration: u32,
}

/// 流量规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficRule {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 匹配路径
    pub path_pattern: String,
    /// 匹配方法
    pub method_pattern: String,
    /// 匹配头部
    pub header_patterns: Vec<(String, String)>,
    /// 优先级
    pub priority: TrafficPriority,
    /// 带宽限制（字节/秒）
    pub bandwidth_limit: Option<u64>,
    /// 启用状态
    pub enabled: bool,
}

/// 流量整形
#[derive(Debug, Clone)]
pub struct TrafficShaping {
    config: Arc<RwLock<TrafficShapingConfig>>,
    priority_queues: Arc<RwLock<std::collections::HashMap<TrafficPriority, usize>>>,
}

impl TrafficShaping {
    /// 创建新的流量整形
    pub fn new(config: TrafficShapingConfig) -> Self {
        let mut priority_queues = std::collections::HashMap::new();
        priority_queues.insert(TrafficPriority::Low, 0);
        priority_queues.insert(TrafficPriority::Medium, 0);
        priority_queues.insert(TrafficPriority::High, 0);
        priority_queues.insert(TrafficPriority::Critical, 0);

        Self {
            config: Arc::new(RwLock::new(config)),
            priority_queues: Arc::new(RwLock::new(priority_queues)),
        }
    }

    /// 初始化流量整形
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化流量整形
        Ok(())
    }

    /// 获取流量优先级
    pub async fn get_priority(
        &self,
        path: &str,
        method: &str,
        headers: &[(String, String)],
    ) -> TrafficPriority {
        let config = self.config.read().await;

        for rule in &config.rules {
            if rule.enabled {
                // 检查路径匹配
                if !self.matches_pattern(path, &rule.path_pattern) {
                    continue;
                }

                // 检查方法匹配
                if !self.matches_pattern(method, &rule.method_pattern) {
                    continue;
                }

                // 检查头部匹配
                let headers_match = rule.header_patterns.iter().all(|(name, value)| {
                    headers.iter().any(|(h_name, h_value)| {
                        self.matches_pattern(h_name, name) && self.matches_pattern(h_value, value)
                    })
                });

                if headers_match {
                    return rule.priority.clone();
                }
            }
        }

        // 默认优先级
        TrafficPriority::Medium
    }

    /// 检查是否匹配模式
    fn matches_pattern(&self, value: &str, pattern: &str) -> bool {
        // 简单的通配符匹配
        if pattern == "*" {
            return true;
        }

        // 精确匹配
        value == pattern
    }

    /// 处理流量
    pub async fn shape_traffic(
        &self,
        priority: TrafficPriority,
        data_size: u64,
    ) -> Result<Duration, Box<dyn std::error::Error>> {
        let config = self.config.read().await;

        if !config.enabled {
            return Ok(Duration::from_millis(0));
        }

        // 更新队列大小
        let mut priority_queues = self.priority_queues.write().await;
        let current_queue_size = priority_queues.get(&priority).unwrap_or(&0) + data_size as usize;
        priority_queues.insert(priority.clone(), current_queue_size);

        // 计算延迟
        let delay = self.calculate_delay(&config, priority, data_size).await;

        Ok(delay)
    }

    /// 计算延迟
    async fn calculate_delay(
        &self,
        config: &TrafficShapingConfig,
        priority: TrafficPriority,
        data_size: u64,
    ) -> Duration {
        // 根据优先级和带宽限制计算延迟
        let weight = match priority {
            TrafficPriority::Low => config.priority_config.low_weight,
            TrafficPriority::Medium => config.priority_config.medium_weight,
            TrafficPriority::High => config.priority_config.high_weight,
            TrafficPriority::Critical => config.priority_config.critical_weight,
        };

        // 计算基于权重的带宽分配
        let total_weight = config.priority_config.low_weight
            + config.priority_config.medium_weight
            + config.priority_config.high_weight
            + config.priority_config.critical_weight;

        let allocated_bandwidth =
            (config.bandwidth_limit as f64 * weight as f64 / total_weight as f64) as u64;

        // 计算延迟
        if allocated_bandwidth > 0 {
            let delay_ms = (data_size * 1000) / allocated_bandwidth;
            Duration::from_millis(delay_ms)
        } else {
            Duration::from_millis(0)
        }
    }

    /// 获取配置
    pub async fn get_config(&self) -> TrafficShapingConfig {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update_config(
        &self,
        config: TrafficShapingConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 重置队列
    pub async fn reset_queues(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut priority_queues = self.priority_queues.write().await;
        priority_queues.clear();
        priority_queues.insert(TrafficPriority::Low, 0);
        priority_queues.insert(TrafficPriority::Medium, 0);
        priority_queues.insert(TrafficPriority::High, 0);
        priority_queues.insert(TrafficPriority::Critical, 0);
        Ok(())
    }

    /// 获取队列状态
    pub async fn get_queue_status(&self) -> std::collections::HashMap<TrafficPriority, usize> {
        self.priority_queues.read().await.clone()
    }
}
