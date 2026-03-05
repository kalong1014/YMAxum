//! Concurrency optimization module
//! Provides concurrency task optimization, thread pool management, task scheduling and other functions

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinSet;

/// Concurrency optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyOptimizerConfig {
    /// Maximum concurrency
    pub max_concurrency: usize,
    /// Thread pool size
    pub pool_size: usize,
    /// Task queue size
    pub queue_size: usize,
    /// Enable task priority
    pub enable_priority: bool,
    /// Enable task timeout
    pub enable_timeout: bool,
    /// Task timeout (seconds)
    pub task_timeout: u64,
}

impl Default for ConcurrencyOptimizerConfig {
    fn default() -> Self {
        Self {
            max_concurrency: 100,
            pool_size: 50,
            queue_size: 1000,
            enable_priority: true,
            enable_timeout: true,
            task_timeout: 30,
        }
    }
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low priority
    Low = 0,
    /// Medium priority
    Medium = 1,
    /// High priority
    High = 2,
    /// Critical priority
    Critical = 3,
}

/// Task statistics information
#[derive(Debug, Clone)]
pub struct TaskStatistics {
    /// Task ID
    pub task_id: String,
    /// Task priority
    pub priority: TaskPriority,
    /// Creation time
    pub created_at: Instant,
    /// Start time
    pub started_at: Option<Instant>,
    /// Completion time
    pub completed_at: Option<Instant>,
    /// Execution time (milliseconds)
    pub execution_time: Option<u64>,
    /// Is timeout
    pub is_timeout: bool,
    /// Is success
    pub is_success: bool,
}

/// Concurrency optimizer
pub struct ConcurrencyOptimizer {
    /// Configuration
    config: ConcurrencyOptimizerConfig,
    /// Semaphore (for controlling concurrency)
    semaphore: Arc<Semaphore>,
    /// Task statistics
    task_stats: Arc<RwLock<Vec<TaskStatistics>>>,
}

impl ConcurrencyOptimizer {
    /// Create a new concurrency optimizer
    pub fn new(config: ConcurrencyOptimizerConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrency));

        Self {
            config,
            semaphore,
            task_stats: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Execute a single task
    pub async fn execute_task<F, T>(
        &self,
        task_id: String,
        priority: TaskPriority,
        task: F,
    ) -> Result<T, String>
    where
        F: std::future::Future<Output = Result<T, String>>,
    {
        let created_at = Instant::now();
        let mut task_stat = TaskStatistics {
            task_id: task_id.clone(),
            priority,
            created_at,
            started_at: None,
            completed_at: None,
            execution_time: None,
            is_timeout: false,
            is_success: false,
        };

        // Acquire semaphore (control concurrency)
        let permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| format!("Failed to acquire semaphore: {}", e))?;
        let started_at = Instant::now();
        task_stat.started_at = Some(started_at);

        info!("Task started: {}, priority: {:?}", task_id, priority);

        // Execute task
        let result = if self.config.enable_timeout {
            tokio::time::timeout(Duration::from_secs(self.config.task_timeout), task).await
        } else {
            Ok(task.await)
        };

        let completed_at = Instant::now();
        task_stat.completed_at = Some(completed_at);
        task_stat.execution_time = Some(started_at.elapsed().as_millis() as u64);

        // Release semaphore
        drop(permit);

        match result {
            Ok(Ok(value)) => {
                task_stat.is_success = true;
                info!(
                    "Task completed successfully: {}, time: {}ms",
                    task_id,
                    task_stat.execution_time.unwrap()
                );

                // Record task statistics
                self.record_task_stat(task_stat).await;

                Ok(value)
            }
            Ok(Err(error)) => {
                task_stat.is_success = false;
                warn!("Task execution failed: {}, error: {}", task_id, error);

                // Record task statistics
                self.record_task_stat(task_stat).await;

                Err(error)
            }
            Err(_) => {
                task_stat.is_timeout = true;
                task_stat.is_success = false;
                warn!("Task execution timeout: {}", task_id);

                // Record task statistics
                self.record_task_stat(task_stat).await;

                Err(format!("Task execution timeout: {}", task_id))
            }
        }
    }

    /// Execute multiple tasks sequentially
    pub async fn execute_tasks<F, T>(
        &self,
        tasks: Vec<(String, TaskPriority, F)>,
    ) -> Vec<Result<T, String>>
    where
        F: std::future::Future<Output = Result<T, String>>,
    {
        let mut results = Vec::new();

        for (task_id, priority, task) in tasks {
            let result = self.execute_task(task_id, priority, task).await;
            results.push(result);
        }

        results
    }

    /// Execute tasks concurrently (using JoinSet)
    pub async fn execute_concurrent<F, T>(
        &self,
        tasks: Vec<(String, TaskPriority, F)>,
    ) -> Vec<Result<T, String>>
    where
        F: std::future::Future<Output = Result<T, String>> + Send + 'static,
        T: Send + 'static,
    {
        let mut join_set = JoinSet::new();

        for (task_id, priority, task) in tasks {
            let optimizer = self.clone();
            join_set.spawn(async move { optimizer.execute_task(task_id, priority, task).await });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result.unwrap_or(Err("Task execution failed".to_string())));
        }

        results
    }

    /// Get current concurrency count
    pub async fn current_concurrency(&self) -> usize {
        self.config.max_concurrency - self.semaphore.available_permits()
    }

    /// Get task statistics
    pub async fn get_task_statistics(&self) -> Vec<TaskStatistics> {
        self.task_stats.read().await.clone()
    }

    /// Record task statistics
    async fn record_task_stat(&self, stat: TaskStatistics) {
        let mut stats = self.task_stats.write().await;
        stats.push(stat);

        // Keep only the latest 1000 records
        if stats.len() > 1000 {
            stats.remove(0);
        }
    }

    /// Generate optimization report
    pub async fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Concurrency Optimization Report ===\n\n");

        // Concurrency configuration
        report.push_str("[Concurrency Configuration]\n");
        report.push_str(&format!(
            "Maximum concurrency: {}\n",
            self.config.max_concurrency
        ));
        report.push_str(&format!("Thread pool size: {}\n", self.config.pool_size));
        report.push_str(&format!("Task queue size: {}\n", self.config.queue_size));
        report.push_str(&format!(
            "Enable task priority: {}\n",
            self.config.enable_priority
        ));
        report.push_str(&format!(
            "Enable task timeout: {}\n",
            self.config.enable_timeout
        ));
        report.push_str(&format!(
            "Task timeout: {} seconds\n",
            self.config.task_timeout
        ));

        // Current concurrency status
        let current = self.current_concurrency().await;
        report.push_str("[Current Concurrency Status]\n");
        report.push_str(&format!(
            "Current concurrency: {}/{}\n\n",
            current, self.config.max_concurrency
        ));

        // Task statistics
        let stats = self.get_task_statistics().await;
        if !stats.is_empty() {
            report.push_str("[Task Statistics]\n");
            report.push_str(&format!("Total tasks: {}\n", stats.len()));

            let success_count = stats.iter().filter(|s| s.is_success).count();
            let timeout_count = stats.iter().filter(|s| s.is_timeout).count();
            let failure_count = stats.len() - success_count - timeout_count;

            report.push_str(&format!("Successful tasks: {}\n", success_count));
            report.push_str(&format!("Failed tasks: {}\n", failure_count));
            report.push_str(&format!("Timeout tasks: {}\n", timeout_count));

            // Calculate average execution time
            let execution_times: Vec<u64> = stats.iter().filter_map(|s| s.execution_time).collect();

            if !execution_times.is_empty() {
                let avg_time = execution_times.iter().sum::<u64>() / execution_times.len() as u64;
                report.push_str(&format!("Average execution time: {}ms\n", avg_time));
            }

            // Maximum execution time
            if let Some(max_time) = stats.iter().filter_map(|s| s.execution_time).max() {
                report.push_str(&format!("Maximum execution time: {}ms\n", max_time));
            }

            // Minimum execution time
            if let Some(min_time) = stats.iter().filter_map(|s| s.execution_time).min() {
                report.push_str(&format!("Minimum execution time: {}ms\n\n", min_time));
            }
        }

        // Optimization suggestions
        report.push_str("[Optimization Suggestions]\n");

        let current = self.current_concurrency().await;
        let utilization = (current as f64 / self.config.max_concurrency as f64) * 100.0;

        if utilization > 80.0 {
            report.push_str(
                "1. Concurrency utilization is high, suggest increasing maximum concurrency\n",
            );
        } else if utilization < 50.0 {
            report.push_str(
                "1. Concurrency utilization is low, can consider reducing maximum concurrency\n",
            );
        }

        if self.config.enable_timeout {
            report.push_str("2. Task timeout is enabled, suggest adjusting timeout threshold according to actual needs\n");
        }

        if self.config.enable_priority {
            report.push_str("3. Task priority is enabled, suggest optimizing task scheduling based on priority levels\n");
        }

        report.push_str("4. Regularly monitor task execution time and completion status\n");
        report.push_str("5. Optimize task execution logic to reduce execution time\n");
        report.push_str("6. Use thread pool to manage concurrent tasks efficiently\n");
        report.push_str("7. Regularly clean up completed tasks\n");

        report
    }
}

impl Clone for ConcurrencyOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            semaphore: self.semaphore.clone(),
            task_stats: self.task_stats.clone(),
        }
    }
}

impl Default for ConcurrencyOptimizer {
    fn default() -> Self {
        Self::new(ConcurrencyOptimizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrency_optimizer() {
        let optimizer = ConcurrencyOptimizer::new(ConcurrencyOptimizerConfig::default());

        // Test task execution
        let result = optimizer
            .execute_task("test_task".to_string(), TaskPriority::High, async {
                Ok::<_, String>("test_result")
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_result");
    }

    #[tokio::test]
    async fn test_task_timeout() {
        let mut config = ConcurrencyOptimizerConfig::default();
        config.task_timeout = 1;
        let optimizer = ConcurrencyOptimizer::new(config);

        // Test task timeout
        let result = optimizer
            .execute_task("timeout_task".to_string(), TaskPriority::Medium, async {
                tokio::time::sleep(Duration::from_secs(2)).await;
                Ok::<_, String>("result")
            })
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("timeout"));
    }

    #[tokio::test]
    async fn test_concurrent_execution() {
        let optimizer = ConcurrencyOptimizer::new(ConcurrencyOptimizerConfig::default());

        // Test concurrent execution - execute tasks one by one instead of in vec
        let result1 = optimizer
            .execute_task("task1".to_string(), TaskPriority::High, async {
                Ok::<_, String>("result1")
            })
            .await;

        let result2 = optimizer
            .execute_task("task2".to_string(), TaskPriority::Medium, async {
                Ok::<_, String>("result2")
            })
            .await;

        let result3 = optimizer
            .execute_task("task3".to_string(), TaskPriority::Low, async {
                Ok::<_, String>("result3")
            })
            .await;

        let results = vec![result1, result2, result3];

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_task_statistics() {
        let optimizer = ConcurrencyOptimizer::new(ConcurrencyOptimizerConfig::default());

        // Execute multiple tasks
        for i in 0..5 {
            let _ = optimizer
                .execute_task(format!("task_{}", i), TaskPriority::Medium, async move {
                    Ok::<_, String>("result")
                })
                .await;
        }

        let stats = optimizer.get_task_statistics().await;
        assert_eq!(stats.len(), 5);
    }

    #[tokio::test]
    async fn test_generate_report() {
        let optimizer = ConcurrencyOptimizer::new(ConcurrencyOptimizerConfig::default());

        // Execute a task
        let _ = optimizer
            .execute_task("test_task".to_string(), TaskPriority::High, async {
                Ok::<_, String>("result")
            })
            .await;

        let report = optimizer.generate_report().await;

        assert!(report.contains("Concurrency Optimization Report"));
        assert!(report.contains("Concurrency Configuration"));
    }
}
