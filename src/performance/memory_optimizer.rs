//! Memory optimization module
//! Provides memory usage monitoring, memory pool management, leak detection and other functions

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimizerConfig {
    /// Memory monitoring interval (seconds)
    pub monitor_interval: u64,
    /// Memory warning threshold (MB)
    pub warning_threshold: u64,
    /// Memory danger threshold (MB)
    pub danger_threshold: u64,
    /// Enable memory leak detection
    pub enable_leak_detection: bool,
    /// Enable object pool
    pub enable_object_pool: bool,
    /// Pool size
    pub pool_size: usize,
}

impl Default for MemoryOptimizerConfig {
    fn default() -> Self {
        Self {
            monitor_interval: 10,
            warning_threshold: 800,
            danger_threshold: 1000,
            enable_leak_detection: true,
            enable_object_pool: true,
            pool_size: 1000,
        }
    }
}

/// Memory statistics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    /// Current memory usage (MB)
    pub current_usage: u64,
    /// Peak memory usage since startup (MB)
    pub peak_usage: u64,
    /// Average memory usage (MB)
    pub avg_usage: f64,
    /// Memory usage rate (%)
    pub usage_rate: f64,
    /// Total memory (MB)
    pub total_memory: u64,
    /// Available memory (MB)
    pub available_memory: u64,
}

/// Memory leak report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLeakReport {
    /// Leak location
    pub location: String,
    /// Leak size (bytes)
    pub leak_size: usize,
    /// Leak type
    pub leak_type: LeakType,
    /// Severity level (1-10)
    pub severity: u8,
    /// Optimization suggestions
    pub suggestions: Vec<String>,
}

/// Memory leak type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeakType {
    /// Circular reference
    CircularReference,
    /// Unreleased resource
    UnreleasedResource,
    /// Cache leak
    CacheLeak,
    /// Event listener leak
    EventListenerLeak,
    /// Other
    Other,
}

/// Object pool entry
#[derive(Debug, Clone)]
pub struct PoolEntry<T> {
    /// Object
    pub value: Option<T>,
    /// Is in use
    pub in_use: bool,
    /// Last used time
    pub last_used: Instant,
}

/// Object pool
pub struct ObjectPool<T> {
    /// Pool size
    pub size: usize,
    /// Object pool
    pub pool: Vec<PoolEntry<T>>,
    /// Factory function to create objects
    pub factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T: Clone + std::cmp::PartialEq> ObjectPool<T> {
    /// Create a new object pool
    pub fn new(size: usize, factory: Box<dyn Fn() -> T + Send + Sync>) -> Self {
        let pool = (0..size)
            .map(|_| PoolEntry {
                value: None,
                in_use: false,
                last_used: Instant::now(),
            })
            .collect();

        Self {
            size,
            pool,
            factory,
        }
    }

    /// Acquire object from pool
    pub fn acquire(&mut self) -> Option<T> {
        // Find available object
        for entry in &mut self.pool {
            if !entry.in_use {
                entry.in_use = true;
                entry.last_used = Instant::now();

                if let Some(ref value) = entry.value {
                    info!("Reusing object from pool");
                    return Some(value.clone());
                } else {
                    // Create new object and store it in the pool
                    let new_value = (self.factory)();
                    entry.value = Some(new_value.clone());
                    info!("Created new object and stored in pool");
                    return Some(new_value);
                }
            }
        }

        // No available object, create new object
        warn!("Pool is exhausted, creating new object");
        Some((self.factory)())
    }

    /// Release object back to pool
    pub fn release(&mut self, value: T) {
        // Find matching object and release it
        for entry in &mut self.pool {
            if entry.in_use
                && let Some(ref pool_value) = entry.value
                && pool_value == &value
            {
                entry.in_use = false;
                entry.last_used = Instant::now();
                info!("Object returned to pool");
                return;
            }
        }
    }

    /// Clean up idle objects
    pub fn cleanup(&mut self, idle_time: Duration) {
        let now = Instant::now();
        let mut cleaned = 0;

        for entry in &mut self.pool {
            if !entry.in_use && now.duration_since(entry.last_used) > idle_time {
                entry.value = None;
                cleaned += 1;
            }
        }

        if cleaned > 0 {
            info!("Cleaned up {} idle objects", cleaned);
        }
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> (usize, usize) {
        let in_use = self.pool.iter().filter(|e| e.in_use).count();
        let available = self
            .pool
            .iter()
            .filter(|e| !e.in_use && e.value.is_some())
            .count();
        (in_use, available)
    }
}

/// Memory optimizer
pub struct MemoryOptimizer {
    /// Configuration
    pub config: MemoryOptimizerConfig,
    /// Memory statistics history
    pub memory_stats: Vec<MemoryStatistics>,
    /// Memory leak reports
    pub leak_reports: Vec<MemoryLeakReport>,
    /// Memory allocation tracker
    pub allocation_tracker: HashMap<String, usize>,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new(config: MemoryOptimizerConfig) -> Self {
        Self {
            config,
            memory_stats: Vec::new(),
            leak_reports: Vec::new(),
            allocation_tracker: HashMap::new(),
        }
    }

    /// Monitor memory usage
    pub fn monitor_memory(&mut self) {
        let stats = self.get_current_memory_stats();

        info!(
            "Memory usage: {}MB / {}MB ({:.1}%)",
            stats.current_usage, stats.total_memory, stats.usage_rate
        );

        // Check for warning threshold
        if stats.current_usage > self.config.warning_threshold {
            warn!(
                "Memory usage exceeds warning threshold: {}MB > {}MB",
                stats.current_usage, self.config.warning_threshold
            );
        }

        // Check for danger threshold
        if stats.current_usage > self.config.danger_threshold {
            warn!(
                "Memory usage exceeds danger threshold: {}MB > {}MB",
                stats.current_usage, self.config.danger_threshold
            );
        }

        // Update peak usage
        if stats.current_usage > stats.peak_usage {
            info!("Memory usage reached new peak: {}MB", stats.current_usage);
        }

        // Record statistics
        self.memory_stats.push(stats);

        // Keep only the latest 100 records
        if self.memory_stats.len() > 100 {
            self.memory_stats.remove(0);
        }
    }

    /// Detect memory leaks
    pub fn detect_leaks(&mut self) -> Vec<MemoryLeakReport> {
        self.leak_reports.clear();

        // Check for large allocations
        for (location, size) in &self.allocation_tracker {
            if *size > 10 * 1024 * 1024 {
                // Allocations larger than 10MB are considered potential leaks
                self.leak_reports.push(MemoryLeakReport {
                    location: location.clone(),
                    leak_size: *size,
                    leak_type: LeakType::Other,
                    severity: 7,
                    suggestions: vec![
                        "Check configuration and reduce memory allocation".to_string(),
                        "Use memory profiling tools to analyze: (e.g., Valgrind, ASan)".to_string(),
                        "Use object pool to manage memory efficiently".to_string(),
                    ],
                });
            }
        }

        // Check for continuous growth
        if self.memory_stats.len() >= 10 {
            let recent_stats = &self.memory_stats[self.memory_stats.len() - 10..];
            let first_usage = recent_stats[0].current_usage as f64;
            let last_usage = recent_stats[recent_stats.len() - 1].current_usage as f64;
            let growth_rate = ((last_usage - first_usage) / first_usage) * 100.0;

            if growth_rate > 50.0 {
                // Memory growth exceeds 50%, considered a potential leak
                self.leak_reports.push(MemoryLeakReport {
                    location: "Overall memory".to_string(),
                    leak_size: (last_usage - first_usage) as usize,
                    leak_type: LeakType::Other,
                    severity: 8,
                    suggestions: vec![
                        "Check for circular references".to_string(),
                        "Check for unreleased resources in code".to_string(),
                        "Use memory profiling tools to locate leaks".to_string(),
                        "Regularly restart the application to clear memory".to_string(),
                    ],
                });
            }
        }

        self.leak_reports.clone()
    }

    /// Track memory allocation
    pub fn track_allocation(&mut self, location: String, size: usize) {
        *self.allocation_tracker.entry(location).or_insert(0) += size;
    }

    /// Track memory deallocation
    pub fn track_deallocation(&mut self, location: String, size: usize) {
        if let Some(allocated) = self.allocation_tracker.get_mut(&location) {
            *allocated = allocated.saturating_sub(size);
        }
    }

    /// Get memory statistics history
    pub fn get_memory_stats(&self) -> &[MemoryStatistics] {
        &self.memory_stats
    }

    /// Get memory leak reports
    pub fn get_leak_reports(&self) -> &[MemoryLeakReport] {
        &self.leak_reports
    }

    /// Generate optimization report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Memory Optimization Report ===\n\n");

        // Memory statistics summary
        if let Some(latest_stats) = self.memory_stats.last() {
            report.push_str("[Memory Statistics]\n");
            report.push_str(&format!(
                "Current usage: {}MB\n",
                latest_stats.current_usage
            ));
            report.push_str(&format!(
                "Peak usage since startup: {}MB\n",
                latest_stats.peak_usage
            ));
            report.push_str(&format!("Average usage: {:.2}MB\n", latest_stats.avg_usage));
            report.push_str(&format!("Usage rate: {:.1}%\n", latest_stats.usage_rate));
            report.push_str(&format!("Total memory: {}MB\n", latest_stats.total_memory));
            report.push_str(&format!(
                "Available memory: {}MB\n\n",
                latest_stats.available_memory
            ));
        }

        // Memory leak reports
        if !self.leak_reports.is_empty() {
            report.push_str("[Memory Leak Detection]\n");
            for (index, leak_report) in self.leak_reports.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", index + 1, leak_report.location));
                report.push_str(&format!("   Leak size: {} bytes\n", leak_report.leak_size));
                report.push_str(&format!("   Leak type: {:?}\n", leak_report.leak_type));
                report.push_str(&format!("   Severity level: {}/10\n", leak_report.severity));
                report.push_str("   Optimization suggestions:\n");
                for suggestion in &leak_report.suggestions {
                    report.push_str(&format!("   - {}\n", suggestion));
                }
                report.push('\n');
            }
        } else {
            report.push_str("[Memory Leak Detection] No memory leaks detected\n\n");
        }

        // Optimization suggestions
        report.push_str("[Optimization Suggestions]\n");
        report.push_str("1. Regularly monitor memory usage and set alerts\n");
        report.push_str("2. Use object pool to manage memory efficiently\n");
        report.push_str("3. Avoid circular references\n");
        report.push_str("4. Release resources promptly after use\n");
        report.push_str("5. Use memory profiling tools to analyze memory usage\n");
        report.push_str("6. Optimize data structures to reduce memory footprint\n");
        report.push_str("7. Regularly clean up memory allocation tracking\n");
        report.push_str("8. Set reasonable memory limits for containers and services\n");
        report.push_str("9. Use cache strategies to reduce memory pressure\n");
        report.push_str("10. Regularly restart long-running applications to clear memory\n");

        report
    }

    /// Get current memory statistics
    fn get_current_memory_stats(&self) -> MemoryStatistics {
        // Simplified version: use simulated values
        // In actual implementation, should use system API to get actual memory usage
        let current_usage = 500u64;
        let total_memory = 1024u64;
        let available_memory = total_memory - current_usage;
        let usage_rate = (current_usage as f64 / total_memory as f64) * 100.0;

        let peak_usage = self
            .memory_stats
            .iter()
            .map(|s| s.peak_usage)
            .max()
            .unwrap_or(current_usage);

        let avg_usage = if self.memory_stats.is_empty() {
            current_usage as f64
        } else {
            self.memory_stats
                .iter()
                .map(|s| s.current_usage as f64)
                .sum::<f64>()
                / self.memory_stats.len() as f64
        };

        MemoryStatistics {
            current_usage,
            peak_usage,
            avg_usage,
            usage_rate,
            total_memory,
            available_memory,
        }
    }
}

impl Default for MemoryOptimizer {
    fn default() -> Self {
        Self::new(MemoryOptimizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer() {
        let mut optimizer = MemoryOptimizer::new(MemoryOptimizerConfig::default());

        // Test memory monitoring
        optimizer.monitor_memory();

        let stats = optimizer.get_memory_stats();
        assert_eq!(stats.len(), 1);
    }

    #[test]
    fn test_object_pool() {
        let mut pool = ObjectPool::new(10, Box::new(|| "test_object"));

        // Test acquiring object
        let obj1 = pool.acquire();
        assert!(obj1.is_some());

        // Test releasing object
        if let Some(obj) = obj1 {
            pool.release(obj);
        }

        let (in_use, available) = pool.get_stats();
        assert_eq!(in_use, 0);
        assert_eq!(available, 1);
    }

    #[test]
    fn test_leak_detection() {
        let mut optimizer = MemoryOptimizer::new(MemoryOptimizerConfig::default());

        // Track memory allocation
        optimizer.track_allocation("test_location".to_string(), 20 * 1024 * 1024);

        // Detect leaks
        let leaks = optimizer.detect_leaks();

        assert!(!leaks.is_empty());
    }

    #[test]
    fn test_generate_report() {
        let mut optimizer = MemoryOptimizer::new(MemoryOptimizerConfig::default());

        // Monitor memory
        optimizer.monitor_memory();

        let report = optimizer.generate_report();

        assert!(report.contains("Memory Optimization Report"));
        assert!(report.contains("Memory Statistics"));
    }
}
