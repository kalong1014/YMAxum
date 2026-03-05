//! 性能优化模块
//! 提供性能分析和优化功能
pub mod alert;
pub mod analyzer;
pub mod benchmark;
pub mod benchmark_main;
pub mod cache_optimizer;
pub mod concurrency_optimizer;
pub mod database_optimizer;
pub mod memory_optimizer;
pub mod monitor;
pub mod test_and_tuner;

pub use alert::{Alert, AlertConfig, AlertLevel, AlertManager, AlertType};
pub use analyzer::PerformanceAnalyzer;
pub use benchmark::{
    BenchmarkConfig, BenchmarkResult, BenchmarkRunner, PerformanceComparison,
    PerformanceTuningSuggestion,
};
pub use benchmark_main::{BenchmarkSuite, run_custom_benchmark, run_default_benchmark};
pub use cache_optimizer::CacheOptimizer;
pub use concurrency_optimizer::ConcurrencyOptimizer;
pub use database_optimizer::DatabaseOptimizer;
pub use memory_optimizer::MemoryOptimizer;
pub use monitor::PerformanceMonitor;
pub use test_and_tuner::{
    PerformanceTestAndTuner, PerformanceTestConfig, PerformanceTestResult, TuningSuggestion,
};
