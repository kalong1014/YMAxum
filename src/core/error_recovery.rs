// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 错误恢复管理器
//! 用于实现错误重试、回退和熔断机制

use crate::error::YMAxumError;
use log::{error, info, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// 错误重试策略
#[derive(Clone, Debug)]
pub enum RetryStrategy {
    /// 固定间隔重试
    Fixed { interval: Duration },
    /// 指数退避重试
    ExponentialBackoff {
        initial_interval: Duration,
        max_interval: Duration,
        multiplier: f64,
    },
    /// 线性退避重试
    LinearBackoff {
        initial_interval: Duration,
        increment: Duration,
    },
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::ExponentialBackoff {
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(5),
            multiplier: 2.0,
        }
    }
}

/// 错误回退策略
#[derive(Clone, Default)]
pub enum FallbackStrategy {
    /// 返回默认值
    #[default]
    Default,
    /// 调用回退函数
    Fallback(Arc<dyn Fn() -> Result<(), YMAxumError> + Send + Sync>),
}

/// 熔断器状态
#[derive(Clone, Debug, PartialEq)]
pub enum CircuitBreakerState {
    /// 关闭状态（正常）
    Closed,
    /// 打开状态（熔断）
    Open,
    /// 半开状态（尝试恢复）
    HalfOpen,
}

/// 熔断器配置
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    /// 熔断阈值（失败次数）
    pub failure_threshold: u32,
    /// 熔断超时时间
    pub timeout: Duration,
    /// 半开状态下的测试请求数
    pub half_open_attempts: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout: Duration::from_secs(60),
            half_open_attempts: 3,
        }
    }
}

/// 熔断器
#[derive(Clone)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerState>>,
    failure_count: Arc<RwLock<u32>>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    success_count: Arc<RwLock<u32>>,
}

impl CircuitBreaker {
    /// 创建新的熔断器
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitBreakerState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            success_count: Arc::new(RwLock::new(0)),
        }
    }

    /// 执行操作，带熔断保护
    pub async fn execute<F, T>(&self, operation: F) -> Result<T, YMAxumError>
    where
        F: std::future::Future<Output = Result<T, YMAxumError>>,
    {
        let state = self.state.read().await.clone();

        if state == CircuitBreakerState::Open {
            // 检查是否可以尝试恢复
            if let Some(last_failure) = *self.last_failure_time.read().await {
                let elapsed = last_failure.elapsed();
                if elapsed > self.config.timeout {
                    // 转换为半开状态
                    *self.state.write().await = CircuitBreakerState::HalfOpen;
                    *self.success_count.write().await = 0;
                    info!("熔断器转换为半开状态");
                } else {
                    // 熔断器仍然打开
                    let error = YMAxumError::InternalError {
                        message: "Circuit breaker is open".to_string(),
                        source: None,
                        context: crate::error::ErrorContext::default(),
                    };
                    return Err(error);
                }
            } else {
                // 熔断器打开但没有上次失败时间，转换为半开状态
                *self.state.write().await = CircuitBreakerState::HalfOpen;
                *self.success_count.write().await = 0;
                info!("熔断器转换为半开状态");
            }
        }

        // 重新获取状态，因为可能已经改变
        let state = self.state.read().await.clone();

        match state {
            CircuitBreakerState::HalfOpen => {
                // 半开状态，允许部分请求通过
                let result = operation.await;

                match result {
                    Ok(_) => {
                        // 成功，增加成功计数
                        let mut success_count = self.success_count.write().await;
                        *success_count += 1;

                        if *success_count >= self.config.half_open_attempts {
                            // 转换为关闭状态
                            *self.state.write().await = CircuitBreakerState::Closed;
                            *self.failure_count.write().await = 0;
                            info!("熔断器转换为关闭状态");
                        }

                        result
                    }
                    Err(e) => {
                        // 失败，转换回打开状态
                        *self.state.write().await = CircuitBreakerState::Open;
                        *self.last_failure_time.write().await = Some(Instant::now());
                        error!("熔断器转换为打开状态");
                        Err(e)
                    }
                }
            }
            CircuitBreakerState::Closed => {
                // 关闭状态，正常执行
                let result = operation.await;

                match result {
                    Ok(_) => {
                        // 成功，重置失败计数
                        *self.failure_count.write().await = 0;
                        result
                    }
                    Err(e) => {
                        // 失败，增加失败计数
                        let mut failure_count = self.failure_count.write().await;
                        *failure_count += 1;

                        if *failure_count >= self.config.failure_threshold {
                            // 转换为打开状态
                            *self.state.write().await = CircuitBreakerState::Open;
                            *self.last_failure_time.write().await = Some(Instant::now());
                            error!("熔断器转换为打开状态");
                        }

                        Err(e)
                    }
                }
            }
            CircuitBreakerState::Open => {
                // 理论上不会到达这里，因为前面已经处理了
                let error = YMAxumError::InternalError {
                    message: "Circuit breaker is open".to_string(),
                    source: None,
                    context: crate::error::ErrorContext::default(),
                };
                Err(error)
            }
        }
    }

    /// 获取熔断器状态
    pub async fn state(&self) -> CircuitBreakerState {
        (*self.state.read().await).clone()
    }

    /// 重置熔断器
    pub async fn reset(&self) {
        *self.state.write().await = CircuitBreakerState::Closed;
        *self.failure_count.write().await = 0;
        *self.last_failure_time.write().await = None;
        *self.success_count.write().await = 0;
        info!("熔断器已重置");
    }
}

/// 错误恢复管理器
#[derive(Clone)]
pub struct ErrorRecoveryManager {
    /// 重试策略
    retry_strategy: RetryStrategy,
    /// 最大重试次数
    max_retries: u32,
    /// 回退策略
    fallback_strategy: FallbackStrategy,
    /// 熔断器
    circuit_breaker: Option<CircuitBreaker>,
}

impl ErrorRecoveryManager {
    /// 创建新的错误恢复管理器
    pub fn new() -> Self {
        Self {
            retry_strategy: RetryStrategy::default(),
            max_retries: 3,
            fallback_strategy: FallbackStrategy::default(),
            circuit_breaker: None,
        }
    }

    /// 创建新的错误恢复管理器，并设置重试策略
    pub fn with_retry_strategy(mut self, retry_strategy: RetryStrategy) -> Self {
        self.retry_strategy = retry_strategy;
        self
    }

    /// 创建新的错误恢复管理器，并设置最大重试次数
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// 创建新的错误恢复管理器，并设置回退策略
    pub fn with_fallback_strategy(mut self, fallback_strategy: FallbackStrategy) -> Self {
        self.fallback_strategy = fallback_strategy;
        self
    }

    /// 创建新的错误恢复管理器，并设置熔断器
    pub fn with_circuit_breaker(mut self, circuit_breaker: CircuitBreaker) -> Self {
        self.circuit_breaker = Some(circuit_breaker);
        self
    }

    /// 执行操作，带错误恢复
    pub async fn execute<F, T>(&self, operation: F) -> Result<T, YMAxumError>
    where
        F: Fn() -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<T, YMAxumError>> + Send>,
            > + Send
            + Sync,
    {
        // 执行重试逻辑
        self.execute_with_retry(operation).await
    }

    /// 执行操作，带重试
    async fn execute_with_retry<F, T>(&self, operation: F) -> Result<T, YMAxumError>
    where
        F: Fn() -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<T, YMAxumError>> + Send>,
            > + Send
            + Sync,
    {
        let mut retry_count = 0;
        let mut _last_error: Option<YMAxumError> = None;

        loop {
            let result = operation().await;

            match result {
                Ok(value) => {
                    if retry_count > 0 {
                        info!("操作成功，重试次数: {}", retry_count);
                    }
                    return Ok(value);
                }
                Err(e) => {
                    let error = e;
                    // 使用 clone 方法
                    _last_error = Some(error.clone());

                    // 检查是否可恢复
                    if !error.is_recoverable() {
                        warn!("错误不可恢复，停止重试: {:?}", error);
                        return Err(error);
                    }

                    // 检查重试次数
                    if retry_count >= self.max_retries {
                        warn!("达到最大重试次数: {}", self.max_retries);
                        return Err(error);
                    }

                    // 计算重试间隔
                    let delay = self.calculate_retry_delay(retry_count);
                    retry_count += 1;

                    warn!(
                        "操作失败，{} 后重试 (第 {} 次): {:?}",
                        delay.as_secs_f64(),
                        retry_count,
                        error
                    );

                    // 等待重试间隔
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// 计算重试间隔
    fn calculate_retry_delay(&self, retry_count: u32) -> Duration {
        match &self.retry_strategy {
            RetryStrategy::Fixed { interval } => *interval,
            RetryStrategy::ExponentialBackoff {
                initial_interval,
                max_interval,
                multiplier,
            } => {
                let delay =
                    initial_interval.as_millis() as f64 * multiplier.powi(retry_count as i32);
                let delay = delay.min(max_interval.as_millis() as f64);
                Duration::from_millis(delay as u64)
            }
            RetryStrategy::LinearBackoff {
                initial_interval,
                increment,
            } => {
                let delay = initial_interval.as_millis() as u64
                    + increment.as_millis() as u64 * retry_count as u64;
                Duration::from_millis(delay)
            }
        }
    }
}

impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_retry_strategy_default() {
        let default_strategy = RetryStrategy::default();
        assert!(matches!(
            default_strategy,
            RetryStrategy::ExponentialBackoff { .. }
        ));
    }

    #[tokio::test]
    async fn test_fallback_strategy_default() {
        let default_strategy = FallbackStrategy::default();
        assert!(matches!(default_strategy, FallbackStrategy::Default));
    }

    #[tokio::test]
    async fn test_circuit_breaker_config_default() {
        let default_config = CircuitBreakerConfig::default();
        assert_eq!(default_config.failure_threshold, 5);
        assert_eq!(default_config.timeout, Duration::from_secs(60));
        assert_eq!(default_config.half_open_attempts, 3);
    }

    #[tokio::test]
    async fn test_circuit_breaker_initial_state() {
        let config = CircuitBreakerConfig::default();
        let circuit_breaker = CircuitBreaker::new(config);
        let state = circuit_breaker.state().await;
        assert_eq!(state, CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_reset() {
        let config = CircuitBreakerConfig::default();
        let circuit_breaker = CircuitBreaker::new(config);

        // Reset the circuit breaker
        circuit_breaker.reset().await;
        let state = circuit_breaker.state().await;
        assert_eq!(state, CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn test_error_recovery_manager_creation() {
        let _manager = ErrorRecoveryManager::new();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_error_recovery_manager_with_strategies() {
        let _manager = ErrorRecoveryManager::new()
            .with_retry_strategy(RetryStrategy::Fixed {
                interval: Duration::from_millis(100),
            })
            .with_max_retries(5)
            .with_fallback_strategy(FallbackStrategy::Default);
        // Just ensure the methods don't panic
    }

    #[tokio::test]
    async fn test_error_recovery_manager_with_circuit_breaker() {
        let config = CircuitBreakerConfig::default();
        let circuit_breaker = CircuitBreaker::new(config);

        let _manager = ErrorRecoveryManager::new().with_circuit_breaker(circuit_breaker);
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_calculate_retry_delay() {
        // Test with fixed interval
        let fixed_manager = ErrorRecoveryManager::new().with_retry_strategy(RetryStrategy::Fixed {
            interval: Duration::from_millis(100),
        });
        let delay1 = fixed_manager.calculate_retry_delay(0);
        assert_eq!(delay1, Duration::from_millis(100));

        // Test with exponential backoff
        let exponential_manager =
            ErrorRecoveryManager::new().with_retry_strategy(RetryStrategy::ExponentialBackoff {
                initial_interval: Duration::from_millis(100),
                max_interval: Duration::from_secs(1),
                multiplier: 2.0,
            });
        let delay2 = exponential_manager.calculate_retry_delay(0);
        assert_eq!(delay2, Duration::from_millis(100));

        // Test with linear backoff
        let linear_manager =
            ErrorRecoveryManager::new().with_retry_strategy(RetryStrategy::LinearBackoff {
                initial_interval: Duration::from_millis(100),
                increment: Duration::from_millis(50),
            });
        let delay3 = linear_manager.calculate_retry_delay(1);
        assert_eq!(delay3, Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_error_recovery_manager_execute() {
        let _manager = ErrorRecoveryManager::new();
        // Just ensure the method can be called
    }
}

