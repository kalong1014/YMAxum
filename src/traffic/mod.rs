//! 高级流量管理模块
//!
//! 提供流量整形、熔断、限流等功能

pub mod circuit_breaker;
pub mod load_balancing;
pub mod rate_limiting;
pub mod traffic_monitoring;
pub mod traffic_shaping;

pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
pub use load_balancing::{LoadBalancer, LoadBalancingConfig, LoadBalancingStrategy};
pub use rate_limiting::{RateLimitStrategy, RateLimiter, RateLimiterConfig};
pub use traffic_monitoring::{TrafficMonitor, TrafficMonitorConfig, TrafficStats};
pub use traffic_shaping::{TrafficPriority, TrafficShaping, TrafficShapingConfig};
