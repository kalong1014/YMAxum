//! 熔断模块
//!
//! 提供服务健康检测、熔断保护等功能

use chrono;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 熔断器状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitState {
    /// 闭合状态（正常工作）
    Closed,
    /// 半开状态（尝试恢复）
    HalfOpen,
    /// 断开状态（服务不可用）
    Open,
}

/// 熔断配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// 启用熔断器
    pub enabled: bool,
    /// 失败阈值
    pub failure_threshold: u32,
    /// 成功阈值
    pub success_threshold: u32,
    /// 半开状态尝试次数
    pub half_open_attempts: u32,
    /// 重置时间（毫秒）
    pub reset_timeout_ms: u64,
    /// 最小请求数
    pub minimum_requests: u32,
    /// 窗口大小（毫秒）
    pub window_size_ms: u64,
    /// 错误率阈值（百分比）
    pub error_rate_threshold: f64,
}

/// 熔断器统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitStats {
    /// 总请求数
    pub total_requests: u32,
    /// 成功请求数
    pub success_requests: u32,
    /// 失败请求数
    pub failure_requests: u32,
    /// 错误率
    pub error_rate: f64,
    /// 状态变更时间
    pub state_change_time: String,
    /// 上次请求时间
    pub last_request_time: String,
}

/// 熔断器
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    config: Arc<RwLock<CircuitBreakerConfig>>,
    state: Arc<RwLock<CircuitState>>,
    stats: Arc<RwLock<CircuitStats>>,
    failure_count: Arc<RwLock<u32>>,
    success_count: Arc<RwLock<u32>>,
    last_state_change: Arc<RwLock<String>>,
}

impl CircuitBreaker {
    /// 创建新的熔断器
    pub fn new(config: CircuitBreakerConfig) -> Self {
        let stats = CircuitStats {
            total_requests: 0,
            success_requests: 0,
            failure_requests: 0,
            error_rate: 0.0,
            state_change_time: chrono::Utc::now().to_string(),
            last_request_time: chrono::Utc::now().to_string(),
        };

        Self {
            config: Arc::new(RwLock::new(config)),
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            stats: Arc::new(RwLock::new(stats)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_state_change: Arc::new(RwLock::new(chrono::Utc::now().to_string())),
        }
    }

    /// 初始化熔断器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化熔断器
        Ok(())
    }

    /// 执行请求
    pub async fn execute<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
        E: std::error::Error + Send + Sync + 'static + From<std::io::Error>,
    {
        // 检查熔断器状态
        let state = self.get_state().await;

        match state {
            CircuitState::Open => {
                // 检查是否可以从打开状态转换为半开状态
                if self.can_transition_to_half_open().await {
                    self.set_state(CircuitState::HalfOpen).await;
                    self.execute_half_open(f).await
                } else {
                    // 返回一个自定义错误，而不是使用不存在的ErrorKind::Unavailable
                    Err(std::convert::Into::into(std::io::Error::other(
                        "Circuit breaker is open",
                    )))
                }
            }
            CircuitState::HalfOpen => self.execute_half_open(f).await,
            CircuitState::Closed => self.execute_closed(f).await,
        }
    }

    /// 执行半开状态的请求
    async fn execute_half_open<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let result = f();

        match result {
            Ok(value) => {
                self.record_success().await;
                let success_count = *self.success_count.read().await;
                let config = self.config.read().await;

                if success_count >= config.success_threshold {
                    self.set_state(CircuitState::Closed).await;
                    self.reset_stats().await;
                }

                Ok(value)
            }
            Err(error) => {
                self.record_failure().await;
                self.set_state(CircuitState::Open).await;
                Err(error)
            }
        }
    }

    /// 执行闭合状态的请求
    async fn execute_closed<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let result = f();

        match result {
            Ok(value) => {
                self.record_success().await;
                Ok(value)
            }
            Err(error) => {
                self.record_failure().await;

                // 检查是否需要打开熔断器
                if self.should_open().await {
                    self.set_state(CircuitState::Open).await;
                }

                Err(error)
            }
        }
    }

    /// 记录成功
    async fn record_success(&self) {
        let mut success_count = self.success_count.write().await;
        *success_count += 1;

        let mut stats = self.stats.write().await;
        stats.success_requests += 1;
        stats.total_requests += 1;
        stats.error_rate = (stats.failure_requests as f64 / stats.total_requests as f64) * 100.0;
        stats.last_request_time = chrono::Utc::now().to_string();
    }

    /// 记录失败
    async fn record_failure(&self) {
        let mut failure_count = self.failure_count.write().await;
        *failure_count += 1;

        let mut stats = self.stats.write().await;
        stats.failure_requests += 1;
        stats.total_requests += 1;
        stats.error_rate = (stats.failure_requests as f64 / stats.total_requests as f64) * 100.0;
        stats.last_request_time = chrono::Utc::now().to_string();
    }

    /// 检查是否应该打开熔断器
    async fn should_open(&self) -> bool {
        let config = self.config.read().await;
        let stats = self.stats.read().await;
        let failure_count = *self.failure_count.read().await;

        // 检查最小请求数
        if stats.total_requests < config.minimum_requests {
            return false;
        }

        // 检查失败次数
        if failure_count >= config.failure_threshold {
            return true;
        }

        // 检查错误率
        if stats.error_rate >= config.error_rate_threshold {
            return true;
        }

        false
    }

    /// 检查是否可以转换为半开状态
    async fn can_transition_to_half_open(&self) -> bool {
        let last_state_change = self.last_state_change.read().await;
        let config = self.config.read().await;

        // 解析状态变更时间
        if let Ok(state_change_time) = chrono::DateTime::parse_from_rfc3339(&last_state_change) {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(state_change_time);

            // 检查是否超过重置时间
            duration.num_milliseconds() >= config.reset_timeout_ms as i64
        } else {
            false
        }
    }

    /// 获取状态
    pub async fn get_state(&self) -> CircuitState {
        self.state.read().await.clone()
    }

    /// 设置状态
    async fn set_state(&self, new_state: CircuitState) {
        let mut state = self.state.write().await;
        *state = new_state;

        let mut last_state_change = self.last_state_change.write().await;
        *last_state_change = chrono::Utc::now().to_string();

        let mut stats = self.stats.write().await;
        stats.state_change_time = chrono::Utc::now().to_string();
    }

    /// 重置统计信息
    async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.total_requests = 0;
        stats.success_requests = 0;
        stats.failure_requests = 0;
        stats.error_rate = 0.0;

        let mut failure_count = self.failure_count.write().await;
        *failure_count = 0;

        let mut success_count = self.success_count.write().await;
        *success_count = 0;
    }

    /// 获取配置
    pub async fn get_config(&self) -> CircuitBreakerConfig {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update_config(
        &self,
        config: CircuitBreakerConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> CircuitStats {
        self.stats.read().await.clone()
    }

    /// 手动重置熔断器
    pub async fn reset(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_state(CircuitState::Closed).await;
        self.reset_stats().await;
        Ok(())
    }
}
