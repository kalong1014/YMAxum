//! Performance benchmark testing module
//! Provides performance benchmark testing, performance comparison, and performance tuning suggestions

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

/// Benchmark type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BenchmarkType {
    /// CPU intensive benchmark
    CPU,
    /// Memory intensive benchmark
    Memory,
    /// I/O intensive benchmark
    IO,
    /// Network intensive benchmark
    Network,
    /// Mixed workload benchmark
    Mixed,
}

/// Resource usage during benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage (%)
    pub cpu_usage: f64,
    /// Memory usage (MB)
    pub memory_usage: f64,
    /// Disk I/O (MB/s)
    pub disk_io: f64,
    /// Network I/O (MB/s)
    pub network_io: f64,
}

/// Benchmark testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Benchmark name
    pub name: String,
    /// Benchmark description
    pub description: String,
    /// Benchmark type
    pub benchmark_type: BenchmarkType,
    /// Number of iterations
    pub iterations: usize,
    /// Number of warmup iterations
    pub warmup_iterations: usize,
    /// Enable GC
    pub enable_gc: bool,
    /// Number of concurrent threads
    pub concurrency: usize,
    /// Enable resource monitoring
    pub enable_resource_monitoring: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            name: "benchmark".to_string(),
            description: "Performance benchmark testing".to_string(),
            benchmark_type: BenchmarkType::Mixed,
            iterations: 100,
            warmup_iterations: 10,
            enable_gc: true,
            concurrency: 1,
            enable_resource_monitoring: false,
        }
    }
}

/// Benchmark testing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Benchmark type
    pub benchmark_type: BenchmarkType,
    /// Number of iterations
    pub iterations: usize,
    /// Number of concurrent threads
    pub concurrency: usize,
    /// Total execution time (milliseconds)
    pub total_time: f64,
    /// Average execution time (milliseconds)
    pub avg_time: f64,
    /// Minimum execution time (milliseconds)
    pub min_time: f64,
    /// Maximum execution time (milliseconds)
    pub max_time: f64,
    /// Median execution time (milliseconds)
    pub median_time: f64,
    /// Standard deviation (milliseconds)
    pub std_dev: f64,
    /// Throughput (operations/second)
    pub throughput: f64,
    /// Resource usage during benchmark
    pub resource_usage: Option<ResourceUsage>,
}

/// Performance comparison results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// Benchmark name
    pub name: String,
    /// Optimization before results
    pub before: BenchmarkResult,
    /// Optimization after results
    pub after: BenchmarkResult,
    /// Performance improvement (%)
    pub improvement: f64,
    /// Is significant improvement?
    pub is_significant: bool,
    /// Improvement details by metric
    pub improvement_details: HashMap<String, f64>,
}

/// Performance tuning suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningSuggestion {
    /// Optimization item
    pub item: String,
    /// Current value
    pub current_value: String,
    /// Suggested value
    pub suggested_value: String,
    /// Expected improvement (%)
    pub expected_improvement: f64,
    /// Priority (1-10)
    pub priority: u8,
    /// Implementation difficulty (1-10)
    pub difficulty: u8,
    /// Detailed explanation
    pub explanation: String,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Predefined benchmark scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScenario {
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Benchmark configuration
    pub config: BenchmarkConfig,
    /// Expected performance thresholds
    pub expected_thresholds: HashMap<String, f64>,
}

/// Benchmark testing runner
pub struct BenchmarkRunner {
    /// Benchmark results
    pub results: HashMap<String, BenchmarkResult>,
    /// Performance comparison results
    pub comparisons: Vec<PerformanceComparison>,
    /// Performance tuning suggestions
    pub suggestions: Vec<PerformanceTuningSuggestion>,
    /// Predefined benchmark scenarios
    pub scenarios: Vec<BenchmarkScenario>,
}

impl BenchmarkRunner {
    /// Create a new benchmark testing runner
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            comparisons: Vec::new(),
            suggestions: Vec::new(),
            scenarios: Self::load_predefined_scenarios(),
        }
    }

    /// Load predefined benchmark scenarios
    fn load_predefined_scenarios() -> Vec<BenchmarkScenario> {
        let mut scenarios = Vec::new();

        // CPU intensive scenario
        scenarios.push(BenchmarkScenario {
            name: "cpu_intensive".to_string(),
            description: "CPU intensive benchmark scenario".to_string(),
            config: BenchmarkConfig {
                name: "cpu_intensive".to_string(),
                description: "CPU intensive benchmark scenario".to_string(),
                benchmark_type: BenchmarkType::CPU,
                iterations: 1000,
                warmup_iterations: 100,
                enable_gc: true,
                concurrency: 4,
                enable_resource_monitoring: true,
            },
            expected_thresholds: HashMap::from([
                ("avg_time".to_string(), 50.0),
                ("throughput".to_string(), 1000.0),
                ("cpu_usage".to_string(), 80.0),
            ]),
        });

        // Memory intensive scenario
        scenarios.push(BenchmarkScenario {
            name: "memory_intensive".to_string(),
            description: "Memory intensive benchmark scenario".to_string(),
            config: BenchmarkConfig {
                name: "memory_intensive".to_string(),
                description: "Memory intensive benchmark scenario".to_string(),
                benchmark_type: BenchmarkType::Memory,
                iterations: 100,
                warmup_iterations: 10,
                enable_gc: true,
                concurrency: 2,
                enable_resource_monitoring: true,
            },
            expected_thresholds: HashMap::from([
                ("avg_time".to_string(), 200.0),
                ("throughput".to_string(), 100.0),
                ("memory_usage".to_string(), 500.0),
            ]),
        });

        // I/O intensive scenario
        scenarios.push(BenchmarkScenario {
            name: "io_intensive".to_string(),
            description: "I/O intensive benchmark scenario".to_string(),
            config: BenchmarkConfig {
                name: "io_intensive".to_string(),
                description: "I/O intensive benchmark scenario".to_string(),
                benchmark_type: BenchmarkType::IO,
                iterations: 50,
                warmup_iterations: 5,
                enable_gc: true,
                concurrency: 4,
                enable_resource_monitoring: true,
            },
            expected_thresholds: HashMap::from([
                ("avg_time".to_string(), 500.0),
                ("throughput".to_string(), 50.0),
                ("disk_io".to_string(), 10.0),
            ]),
        });

        // Mixed workload scenario
        scenarios.push(BenchmarkScenario {
            name: "mixed_workload".to_string(),
            description: "Mixed workload benchmark scenario".to_string(),
            config: BenchmarkConfig {
                name: "mixed_workload".to_string(),
                description: "Mixed workload benchmark scenario".to_string(),
                benchmark_type: BenchmarkType::Mixed,
                iterations: 200,
                warmup_iterations: 20,
                enable_gc: true,
                concurrency: 4,
                enable_resource_monitoring: true,
            },
            expected_thresholds: HashMap::from([
                ("avg_time".to_string(), 100.0),
                ("throughput".to_string(), 500.0),
                ("cpu_usage".to_string(), 60.0),
                ("memory_usage".to_string(), 300.0),
            ]),
        });

        scenarios
    }

    /// Run benchmark test
    pub fn run_benchmark<F, R>(&mut self, config: BenchmarkConfig, test_fn: F) -> BenchmarkResult
    where
        F: Fn() -> R + Send + Sync + Copy + 'static,
        R: Send + 'static,
    {
        info!("Starting benchmark test: {}", config.name);
        info!(
            "Benchmark type: {:?}, Concurrency: {}",
            config.benchmark_type, config.concurrency
        );

        // Warmup
        info!("Warmup: {} iterations", config.warmup_iterations);
        for _ in 0..config.warmup_iterations {
            if config.concurrency > 1 {
                self.run_concurrent(test_fn, config.concurrency);
            } else {
                test_fn();
            }
        }

        // Run benchmark
        info!("Running benchmark: {} iterations", config.iterations);
        let mut durations = Vec::with_capacity(config.iterations);

        // Start resource monitoring if enabled
        let resource_monitor = if config.enable_resource_monitoring {
            Some(self.start_resource_monitoring())
        } else {
            None
        };

        for _ in 0..config.iterations {
            let start = Instant::now();
            if config.concurrency > 1 {
                self.run_concurrent(test_fn, config.concurrency);
            } else {
                test_fn();
            }
            durations.push(start.elapsed());
        }

        // Stop resource monitoring
        let resource_usage = if config.enable_resource_monitoring {
            resource_monitor.map(|monitor| self.stop_resource_monitoring(monitor))
        } else {
            None
        };

        // Calculate statistics using nanoseconds for better precision
        let total_nanos: u128 = durations.iter().map(|d| d.as_nanos()).sum();
        let total_time: f64 = total_nanos as f64 / 1_000_000.0; // Convert to milliseconds
        let avg_time = total_time / config.iterations as f64;
        let min_nanos = durations.iter().map(|d| d.as_nanos()).min().unwrap_or(0);
        let min_time = min_nanos as f64 / 1_000_000.0; // Convert to milliseconds
        let max_nanos = durations.iter().map(|d| d.as_nanos()).max().unwrap_or(0);
        let max_time = max_nanos as f64 / 1_000_000.0; // Convert to milliseconds

        // Calculate median
        let mut sorted_durations = durations.clone();
        sorted_durations.sort_by_key(|d| d.as_nanos());
        let median_time = if sorted_durations.is_empty() {
            0.0
        } else if sorted_durations.len() % 2 == 0 {
            let mid = sorted_durations.len() / 2;
            let mid1 = sorted_durations[mid - 1].as_nanos() as f64 / 1_000_000.0;
            let mid2 = sorted_durations[mid].as_nanos() as f64 / 1_000_000.0;
            (mid1 + mid2) / 2.0
        } else {
            sorted_durations[sorted_durations.len() / 2].as_nanos() as f64 / 1_000_000.0
        };

        // Calculate standard deviation
        let variance = durations
            .iter()
            .map(|d| {
                let time_ms = d.as_nanos() as f64 / 1_000_000.0;
                let diff = time_ms - avg_time;
                diff * diff
            })
            .sum::<f64>()
            / config.iterations as f64;
        let std_dev = variance.sqrt();

        // Calculate throughput
        let throughput = if total_time > 0.0 {
            (config.iterations as f64 * config.concurrency as f64) / total_time * 1000.0
        } else {
            0.0
        };

        let result = BenchmarkResult {
            name: config.name.clone(),
            benchmark_type: config.benchmark_type,
            iterations: config.iterations,
            concurrency: config.concurrency,
            total_time,
            avg_time,
            min_time,
            max_time,
            median_time,
            std_dev,
            throughput,
            resource_usage,
        };

        // Store result
        self.results.insert(config.name.clone(), result.clone());

        info!("Benchmark test completed: {}", config.name);
        info!(
            "Average execution time: {:.2}ms, throughput: {:.2}ops/s",
            avg_time, throughput
        );

        result
    }

    /// Run concurrent execution of test function
    fn run_concurrent<F, R>(&self, test_fn: F, concurrency: usize)
    where
        F: Fn() -> R + Send + Sync + Copy + 'static,
        R: Send + 'static,
    {
        let barrier = Arc::new(Barrier::new(concurrency));
        let mut handles = Vec::with_capacity(concurrency);

        for _ in 0..concurrency {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                // Wait for all threads to start
                barrier_clone.wait();
                test_fn();
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// Start resource monitoring
    fn start_resource_monitoring(&self) -> u64 {
        // In a real implementation, this would start a separate thread to monitor resources
        // For now, we'll just return a dummy ID
        1
    }

    /// Stop resource monitoring and return resource usage
    fn stop_resource_monitoring(&self, _monitor_id: u64) -> ResourceUsage {
        // In a real implementation, this would stop the resource monitoring thread and return actual usage
        // For now, we'll return dummy values
        ResourceUsage {
            cpu_usage: 50.0,
            memory_usage: 200.0,
            disk_io: 5.0,
            network_io: 1.0,
        }
    }

    /// Run predefined benchmark scenario
    pub fn run_scenario<F, R>(&mut self, scenario_name: &str, test_fn: F) -> Option<BenchmarkResult>
    where
        F: Fn() -> R + Send + Sync + Copy + 'static,
        R: Send + 'static,
    {
        if let Some(scenario) = self.scenarios.iter().find(|s| s.name == scenario_name) {
            info!("Running predefined scenario: {}", scenario.name);
            let result = self.run_benchmark(scenario.config.clone(), test_fn);
            Some(result)
        } else {
            info!("Scenario not found: {}", scenario_name);
            None
        }
    }

    /// Compare performance
    pub fn compare_performance(
        &mut self,
        name: String,
        before: BenchmarkResult,
        after: BenchmarkResult,
    ) {
        let improvement = if before.avg_time > 0.0 {
            ((before.avg_time - after.avg_time) / before.avg_time) * 100.0
        } else {
            0.0
        };

        // Calculate improvement details for different metrics
        let mut improvement_details = HashMap::new();
        improvement_details.insert("execution_time".to_string(), improvement);

        if before.throughput > 0.0 {
            let throughput_improvement =
                ((after.throughput - before.throughput) / before.throughput) * 100.0;
            improvement_details.insert("throughput".to_string(), throughput_improvement);
        }

        if before.std_dev > 0.0 {
            let stability_improvement = ((before.std_dev - after.std_dev) / before.std_dev) * 100.0;
            improvement_details.insert("stability".to_string(), stability_improvement);
        }

        // Determine if improvement is significant (greater than 10%)
        let is_significant = improvement > 10.0;
        let comparison = PerformanceComparison {
            name: name.clone(),
            before,
            after,
            improvement,
            is_significant,
            improvement_details,
        };
        info!(
            "Performance comparison: {}, improvement: {:.1}%",
            name, improvement
        );

        self.comparisons.push(comparison);
    }

    /// Generate performance tuning suggestions
    pub fn generate_tuning_suggestions(&mut self, results: &[BenchmarkResult]) {
        self.suggestions.clear();

        for result in results {
            // Check execution time
            if result.avg_time > 1000.0 {
                self.suggestions.push(PerformanceTuningSuggestion {
                    item: "Execution time".to_string(),
                    current_value: format!("{:.2}ms", result.avg_time),
                    suggested_value: "< 100ms".to_string(),
                    expected_improvement: 90.0,
                    priority: 9,
                    difficulty: 7,
                    explanation: "High execution time indicates potential performance bottlenecks in the code.".to_string(),
                    implementation_steps: vec![
                        "Identify bottlenecks using profiler tools".to_string(),
                        "Optimize critical code paths".to_string(),
                        "Consider algorithmic improvements".to_string(),
                        "Implement caching for repeated computations".to_string(),
                    ],
                });
            } else if result.avg_time > 500.0 {
                self.suggestions.push(PerformanceTuningSuggestion {
                    item: "Execution time".to_string(),
                    current_value: format!("{:.2}ms", result.avg_time),
                    suggested_value: "< 500ms".to_string(),
                    expected_improvement: 50.0,
                    priority: 7,
                    difficulty: 5,
                    explanation:
                        "Execution time is higher than optimal, indicating potential optimizations."
                            .to_string(),
                    implementation_steps: vec![
                        "Review and optimize critical code sections".to_string(),
                        "Check for unnecessary computations".to_string(),
                        "Optimize data structures and algorithms".to_string(),
                    ],
                });
            }

            // Check standard deviation
            if result.std_dev > result.avg_time * 0.5 {
                self.suggestions.push(PerformanceTuningSuggestion {
                    item: "Performance stability".to_string(),
                    current_value: format!("Standard deviation: {:.2}ms", result.std_dev),
                    suggested_value: "Reduce standard deviation".to_string(),
                    expected_improvement: 30.0,
                    priority: 6,
                    difficulty: 6,
                    explanation: "High standard deviation indicates inconsistent performance, which can lead to unpredictable behavior.".to_string(),
                    implementation_steps: vec![
                        "Identify sources of variability".to_string(),
                        "Optimize resource allocation".to_string(),
                        "Implement better error handling for external dependencies".to_string(),
                        "Consider adding retries with backoff for unstable operations".to_string(),
                    ],
                });
            }

            // Check throughput
            if result.throughput < 100.0 {
                self.suggestions.push(PerformanceTuningSuggestion {
                    item: "Throughput".to_string(),
                    current_value: format!("{:.2}ops/s", result.throughput),
                    suggested_value: "> 1000ops/s".to_string(),
                    expected_improvement: 900.0,
                    priority: 8,
                    difficulty: 8,
                    explanation: "Low throughput indicates the system is not processing requests efficiently.".to_string(),
                    implementation_steps: vec![
                        "Implement parallel processing where appropriate".to_string(),
                        "Optimize I/O operations".to_string(),
                        "Consider batching requests".to_string(),
                        "Review and optimize database queries".to_string(),
                    ],
                });
            }

            // Check concurrency
            if result.concurrency < 4 {
                self.suggestions.push(PerformanceTuningSuggestion {
                    item: "Concurrency".to_string(),
                    current_value: format!("{}", result.concurrency),
                    suggested_value: "4 or more".to_string(),
                    expected_improvement: 300.0,
                    priority: 7,
                    difficulty: 4,
                    explanation:
                        "Low concurrency may not fully utilize available system resources."
                            .to_string(),
                    implementation_steps: vec![
                        "Increase concurrency level in benchmark config".to_string(),
                        "Ensure thread safety in the code".to_string(),
                        "Optimize shared resource usage".to_string(),
                        "Consider using async/await for I/O bound operations".to_string(),
                    ],
                });
            }

            // Check resource usage
            if let Some(resource_usage) = &result.resource_usage {
                if resource_usage.cpu_usage > 90.0 {
                    self.suggestions.push(PerformanceTuningSuggestion {
                        item: "CPU usage".to_string(),
                        current_value: format!("{:.2}%", resource_usage.cpu_usage),
                        suggested_value: "< 80%".to_string(),
                        expected_improvement: 20.0,
                        priority: 8,
                        difficulty: 6,
                        explanation: "High CPU usage indicates the system is CPU-bound and may benefit from optimization.".to_string(),
                        implementation_steps: vec![
                            "Identify CPU-intensive operations".to_string(),
                            "Optimize algorithms and data structures".to_string(),
                            "Consider using SIMD instructions where appropriate".to_string(),
                            "Implement caching for computationally expensive operations".to_string(),
                        ],
                    });
                }

                if resource_usage.memory_usage > 500.0 {
                    self.suggestions.push(PerformanceTuningSuggestion {
                        item: "Memory usage".to_string(),
                        current_value: format!("{:.2}MB", resource_usage.memory_usage),
                        suggested_value: "< 300MB".to_string(),
                        expected_improvement: 40.0,
                        priority: 7,
                        difficulty: 5,
                        explanation: "High memory usage can lead to increased garbage collection and potential out-of-memory errors.".to_string(),
                        implementation_steps: vec![
                            "Identify memory-intensive operations".to_string(),
                            "Optimize memory allocation and deallocation".to_string(),
                            "Consider using more memory-efficient data structures".to_string(),
                            "Implement object pooling for frequently created objects".to_string(),
                        ],
                    });
                }
            }
        }

        // Sort by priority
        self.suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Export benchmark results to JSON file
    pub fn export_results_json(&self, file_path: &str) -> io::Result<()> {
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, &self.results)?;
        Ok(())
    }

    /// Export benchmark results to CSV file
    pub fn export_results_csv(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;

        // Write header
        writeln!(
            file,
            "Name,Type,Iterations,Concurrency,Total Time (ms),Avg Time (ms),Min Time (ms),Max Time (ms),Median Time (ms),Std Dev (ms),Throughput (ops/s),CPU Usage (%),Memory Usage (MB),Disk I/O (MB/s),Network I/O (MB/s)"
        )?;

        // Write data
        for result in self.results.values() {
            let cpu_usage = if let Some(ru) = &result.resource_usage {
                ru.cpu_usage
            } else {
                0.0
            };
            let memory_usage = if let Some(ru) = &result.resource_usage {
                ru.memory_usage
            } else {
                0.0
            };
            let disk_io = if let Some(ru) = &result.resource_usage {
                ru.disk_io
            } else {
                0.0
            };
            let network_io = if let Some(ru) = &result.resource_usage {
                ru.network_io
            } else {
                0.0
            };

            writeln!(
                file,
                "{},{:?},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
                result.name,
                result.benchmark_type,
                result.iterations,
                result.concurrency,
                result.total_time,
                result.avg_time,
                result.min_time,
                result.max_time,
                result.median_time,
                result.std_dev,
                result.throughput,
                cpu_usage,
                memory_usage,
                disk_io,
                network_io
            )?;
        }

        Ok(())
    }

    /// Get benchmark results
    pub fn get_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.results
    }

    /// Get comparison results
    pub fn get_comparisons(&self) -> &[PerformanceComparison] {
        &self.comparisons
    }

    /// Get tuning suggestions
    pub fn get_suggestions(&self) -> &[PerformanceTuningSuggestion] {
        &self.suggestions
    }

    /// Get predefined scenarios
    pub fn get_scenarios(&self) -> &[BenchmarkScenario] {
        &self.scenarios
    }

    /// Generate benchmark testing report
    pub fn generate_benchmark_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Performance Benchmark Testing Report ===\n\n");

        // Benchmark results
        if !self.results.is_empty() {
            report.push_str("[Benchmark Results]\n");
            for (index, (name, result)) in self.results.iter().enumerate() {
                report.push_str(&format!(
                    "{}. {} ({:?})\n",
                    index + 1,
                    name,
                    result.benchmark_type
                ));
                report.push_str(&format!("   Number of iterations: {}\n", result.iterations));
                report.push_str(&format!("   Concurrency: {}\n", result.concurrency));
                report.push_str(&format!(
                    "   Average execution time: {:.2}ms\n",
                    result.avg_time
                ));
                report.push_str(&format!(
                    "   Minimum execution time: {:.2}ms\n",
                    result.min_time
                ));
                report.push_str(&format!(
                    "   Maximum execution time: {:.2}ms\n",
                    result.max_time
                ));
                report.push_str(&format!(
                    "   Median execution time: {:.2}ms\n",
                    result.median_time
                ));
                report.push_str(&format!("   Standard deviation: {:.2}ms\n", result.std_dev));
                report.push_str(&format!("   Throughput: {:.2}ops/s\n", result.throughput));

                // Resource usage
                if let Some(resource_usage) = &result.resource_usage {
                    report.push_str(&format!("   CPU usage: {:.2}%\n", resource_usage.cpu_usage));
                    report.push_str(&format!(
                        "   Memory usage: {:.2}MB\n",
                        resource_usage.memory_usage
                    ));
                    report.push_str(&format!("   Disk I/O: {:.2}MB/s\n", resource_usage.disk_io));
                    report.push_str(&format!(
                        "   Network I/O: {:.2}MB/s\n",
                        resource_usage.network_io
                    ));
                }

                report.push('\n');
            }
        }

        // Performance comparison results
        if !self.comparisons.is_empty() {
            report.push_str("[Performance Comparison Results]\n");
            for (index, comparison) in self.comparisons.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", index + 1, comparison.name));
                report.push_str(&format!(
                    "   Optimization before: {:.2}ms\n",
                    comparison.before.avg_time
                ));
                report.push_str(&format!(
                    "   Optimization after: {:.2}ms\n",
                    comparison.after.avg_time
                ));
                report.push_str(&format!(
                    "   Performance improvement: {:.1}%\n",
                    comparison.improvement
                ));
                report.push_str(&format!(
                    "   Is significant: {}\n",
                    if comparison.is_significant {
                        "Yes"
                    } else {
                        "No"
                    }
                ));

                // Improvement details
                report.push_str("   Improvement details:\n");
                for (metric, improvement) in &comparison.improvement_details {
                    report.push_str(&format!("     - {}: {:.1}%\n", metric, improvement));
                }

                report.push('\n');
            }
        }

        // Performance tuning suggestions
        if !self.suggestions.is_empty() {
            report.push_str("[Performance Tuning Suggestions]\n");
            for (index, suggestion) in self.suggestions.iter().enumerate() {
                report.push_str(&format!(
                    "{}. {} (Priority: {}/10)\n",
                    index + 1,
                    suggestion.item,
                    suggestion.priority
                ));
                report.push_str(&format!("   Current value: {}\n", suggestion.current_value));
                report.push_str(&format!(
                    "   Suggested value: {}\n",
                    suggestion.suggested_value
                ));
                report.push_str(&format!(
                    "   Expected improvement: {:.1}%\n",
                    suggestion.expected_improvement
                ));
                report.push_str(&format!(
                    "   Implementation difficulty: {}/10\n",
                    suggestion.difficulty
                ));
                report.push_str(&format!("   Explanation: {}\n", suggestion.explanation));
                report.push_str("   Implementation steps:\n");
                for (step_idx, step) in suggestion.implementation_steps.iter().enumerate() {
                    report.push_str(&format!("     {}. {}\n", step_idx + 1, step));
                }
                report.push('\n');
            }
        }

        // General suggestions
        report.push_str("[General Suggestions]\n");
        report.push_str("1. Regularly run performance benchmark tests\n");
        report.push_str("2. Compare performance before and after optimization\n");
        report.push_str("3. Apply performance tuning suggestions based on priority\n");
        report.push_str("4. Test with different concurrency levels\n");
        report.push_str("5. Monitor resource usage during performance testing\n");
        report.push_str("6. Run predefined scenarios to identify specific bottlenecks\n");
        report.push_str("7. Export and analyze benchmark results over time\n");
        report.push_str("8. Plan performance optimization work based on results\n");
        report.push_str("9. Continuously monitor and optimize performance\n");
        report.push_str("10. Consider different benchmark types to identify specific issues\n");

        report
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_runner() {
        let mut runner = BenchmarkRunner::new();

        let config = BenchmarkConfig {
            name: "test_benchmark".to_string(),
            description: "Benchmark test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 10,
            warmup_iterations: 2,
            enable_gc: true,
            concurrency: 1,
            enable_resource_monitoring: false,
        };

        let result = runner.run_benchmark(config, || {
            // Simulate benchmark task
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });

        assert_eq!(result.name, "test_benchmark");
        assert_eq!(result.iterations, 10);
        assert!(result.avg_time >= 0.0);
    }

    #[test]
    fn test_concurrent_benchmark() {
        let mut runner = BenchmarkRunner::new();

        let config = BenchmarkConfig {
            name: "concurrent_test".to_string(),
            description: "Concurrent benchmark test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 5,
            warmup_iterations: 1,
            enable_gc: true,
            concurrency: 4,
            enable_resource_monitoring: false,
        };

        let result = runner.run_benchmark(config, || {
            // Simulate benchmark task
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });

        assert_eq!(result.name, "concurrent_test");
        assert_eq!(result.concurrency, 4);
        assert!(result.avg_time >= 0.0);
    }

    #[test]
    fn test_compare_performance() {
        let mut runner = BenchmarkRunner::new();

        let before = BenchmarkResult {
            name: "test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 100,
            concurrency: 1,
            total_time: 10000.0,
            avg_time: 100.0,
            min_time: 50.0,
            max_time: 200.0,
            median_time: 95.0,
            std_dev: 20.0,
            throughput: 10.0,
            resource_usage: None,
        };

        let after = BenchmarkResult {
            name: "test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 100,
            concurrency: 1,
            total_time: 5000.0,
            avg_time: 50.0,
            min_time: 25.0,
            max_time: 100.0,
            median_time: 48.0,
            std_dev: 10.0,
            throughput: 20.0,
            resource_usage: None,
        };

        runner.compare_performance("test".to_string(), before, after);

        let comparisons = runner.get_comparisons();
        assert_eq!(comparisons.len(), 1);
        assert_eq!(comparisons[0].improvement, 50.0);
        assert!(
            comparisons[0]
                .improvement_details
                .contains_key("execution_time")
        );
        assert!(
            comparisons[0]
                .improvement_details
                .contains_key("throughput")
        );
    }

    #[test]
    fn test_generate_tuning_suggestions() {
        let mut runner = BenchmarkRunner::new();

        let results = vec![BenchmarkResult {
            name: "slow_test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 100,
            concurrency: 1,
            total_time: 150000.0,
            avg_time: 1500.0,
            min_time: 1000.0,
            max_time: 2000.0,
            median_time: 1450.0,
            std_dev: 300.0,
            throughput: 0.67,
            resource_usage: Some(ResourceUsage {
                cpu_usage: 95.0,
                memory_usage: 600.0,
                disk_io: 10.0,
                network_io: 2.0,
            }),
        }];

        runner.generate_tuning_suggestions(&results);

        let suggestions = runner.get_suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions.len() > 1); // Should have multiple suggestions
    }

    #[test]
    fn test_generate_benchmark_report() {
        let mut runner = BenchmarkRunner::new();

        let result = BenchmarkResult {
            name: "test".to_string(),
            benchmark_type: BenchmarkType::CPU,
            iterations: 100,
            concurrency: 1,
            total_time: 10000.0,
            avg_time: 100.0,
            min_time: 50.0,
            max_time: 200.0,
            median_time: 95.0,
            std_dev: 20.0,
            throughput: 10.0,
            resource_usage: None,
        };

        runner.results.insert("test".to_string(), result);

        let report = runner.generate_benchmark_report();

        assert!(report.contains("Performance Benchmark Testing Report"));
        assert!(report.contains("Benchmark Results"));
        assert!(report.contains("test"));
    }

    #[test]
    fn test_predefined_scenarios() {
        let runner = BenchmarkRunner::new();
        let scenarios = runner.get_scenarios();
        assert!(!scenarios.is_empty());
        assert!(scenarios.len() >= 4); // Should have at least 4 predefined scenarios
    }
}
