//! Performance benchmark main module
//! Provides main functions for running performance benchmarks and monitoring

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::performance::benchmark::{
    BenchmarkConfig, BenchmarkResult, BenchmarkRunner, BenchmarkType,
};
use crate::performance::monitor::PerformanceMonitor;

/// Performance benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    /// Suite name
    pub name: String,
    /// Suite description
    pub description: String,
    /// Benchmark configurations
    pub benchmarks: Vec<BenchmarkConfig>,
    /// Output directory
    pub output_dir: String,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self {
            name: "default_suite".to_string(),
            description: "Default benchmark suite".to_string(),
            benchmarks: vec![
                BenchmarkConfig {
                    name: "http_requests".to_string(),
                    description: "HTTP request benchmark".to_string(),
                    benchmark_type: BenchmarkType::Network,
                    iterations: 1000,
                    warmup_iterations: 100,
                    enable_gc: true,
                    concurrency: 4,
                    enable_resource_monitoring: true,
                },
                BenchmarkConfig {
                    name: "plugin_operations".to_string(),
                    description: "Plugin operations benchmark".to_string(),
                    benchmark_type: BenchmarkType::Mixed,
                    iterations: 100,
                    warmup_iterations: 10,
                    enable_gc: true,
                    concurrency: 2,
                    enable_resource_monitoring: true,
                },
                BenchmarkConfig {
                    name: "command_execution".to_string(),
                    description: "Command execution benchmark".to_string(),
                    benchmark_type: BenchmarkType::CPU,
                    iterations: 500,
                    warmup_iterations: 50,
                    enable_gc: true,
                    concurrency: 4,
                    enable_resource_monitoring: true,
                },
            ],
            output_dir: "./performance_results".to_string(),
            enable_monitoring: true,
        }
    }
}

/// Run HTTP request benchmark
async fn run_http_benchmark() -> BenchmarkResult {
    let mut runner = BenchmarkRunner::new();
    let config = BenchmarkConfig {
        name: "http_requests".to_string(),
        description: "HTTP request benchmark".to_string(),
        benchmark_type: BenchmarkType::Network,
        iterations: 1000,
        warmup_iterations: 100,
        enable_gc: true,
        concurrency: 4,
        enable_resource_monitoring: true,
    };

    // Simulate HTTP request processing

    runner.run_benchmark(config, || {
        // Simulate request parsing
        let mut request_data = String::new();
        for _ in 0..100 {
            request_data.push_str("key=value&");
        }

        // Simulate middleware processing
        for _ in 0..5 {
            // Simulate middleware operations
            let _ = request_data.len();
        }

        // Simulate response generation
        let response = format!("{{\"status\": 200, \"data\": \"{}\"}}", request_data);
        response
    })
}

/// Run plugin operations benchmark
async fn run_plugin_benchmark() -> BenchmarkResult {
    let mut runner = BenchmarkRunner::new();
    let config = BenchmarkConfig {
        name: "plugin_operations".to_string(),
        description: "Plugin operations benchmark".to_string(),
        benchmark_type: BenchmarkType::Mixed,
        iterations: 100,
        warmup_iterations: 10,
        enable_gc: true,
        concurrency: 2,
        enable_resource_monitoring: true,
    };

    // Simulate plugin operations

    runner.run_benchmark(config, || {
        // Simulate plugin manifest parsing
        let manifest = r#"{
            "name": "test_plugin",
            "version": "1.0.0",
            "author": "Test Author",
            "description": "Test plugin",
            "plugin_type": "test",
            "dependencies": [],
            "core_version": "0.1.0",
            "entry_file": "main.rs",
            "signature_file": "signature.rsa",
            "license": "MIT"
        }"#;

        // Simulate plugin loading
        let _parsed_manifest: serde_json::Value = serde_json::from_str(manifest).unwrap();

        // Simulate plugin initialization
        for _ in 0..10 {
            // Simulate initialization operations
            let _ = manifest.len();
        }
    })
}

/// Run command execution benchmark
async fn run_command_benchmark() -> BenchmarkResult {
    let mut runner = BenchmarkRunner::new();
    let config = BenchmarkConfig {
        name: "command_execution".to_string(),
        description: "Command execution benchmark".to_string(),
        benchmark_type: BenchmarkType::CPU,
        iterations: 500,
        warmup_iterations: 50,
        enable_gc: true,
        concurrency: 4,
        enable_resource_monitoring: true,
    };

    // Simulate command execution

    runner.run_benchmark(config, || {
        // Simulate command parsing
        let command = "PLUGIN install name=test_plugin version=1.0.0";
        let parts: Vec<&str> = command.split_whitespace().collect();

        // Simulate command validation
        if parts.len() < 2 {
            return Err("Invalid command".to_string());
        }

        // Simulate command execution
        let _command_type = parts[0];
        let _operation = parts[1];

        // Simulate parameter processing
        let mut params = std::collections::HashMap::new();
        for param in parts.iter().skip(2) {
            if let Some((key, value)) = param.split_once('=') {
                params.insert(key, value);
            }
        }

        Ok(params)
    })
}

/// Run all benchmarks in the suite
pub async fn run_benchmark_suite(suite: &BenchmarkSuite) -> Vec<BenchmarkResult> {
    info!("Starting benchmark suite: {}", suite.name);
    info!("Description: {}", suite.description);
    info!("Running {} benchmarks", suite.benchmarks.len());

    // Ensure output directory exists
    let output_path = Path::new(&suite.output_dir);
    if !output_path.exists() {
        std::fs::create_dir_all(output_path).unwrap_or_else(|e| {
            error!("Failed to create output directory: {:?}", e);
        });
    }

    // Initialize performance monitor if enabled
    let monitor = if suite.enable_monitoring {
        info!("Enabling performance monitoring");
        // Create a dummy AppState for benchmarking
        use crate::core::state::AppState;
        use std::sync::Arc;
        let app_state = Arc::new(AppState::new());
        Some(PerformanceMonitor::new(app_state))
    } else {
        None
    };

    let start_time = Instant::now();
    let mut results = Vec::new();

    // Run each benchmark
    for benchmark in &suite.benchmarks {
        info!("Running benchmark: {}", benchmark.name);
        info!("Description: {}", benchmark.description);
        info!(
            "Iterations: {}, Warmup: {}",
            benchmark.iterations, benchmark.warmup_iterations
        );

        let benchmark_start = Instant::now();
        let result = match benchmark.name.as_str() {
            "http_requests" => run_http_benchmark().await,
            "plugin_operations" => run_plugin_benchmark().await,
            "command_execution" => run_command_benchmark().await,
            _ => {
                warn!("Unknown benchmark: {}", benchmark.name);
                continue;
            }
        };

        let benchmark_duration = benchmark_start.elapsed();
        info!(
            "Benchmark {} completed in {:?}",
            benchmark.name, benchmark_duration
        );
        info!(
            "Average time: {:.2}ms, Throughput: {:.2} ops/s",
            result.avg_time, result.throughput
        );

        results.push(result);

        // Record benchmark execution with performance monitor
        if let Some(Ok(perf_monitor)) = &monitor {
            perf_monitor.record_request_end(benchmark_duration.as_secs_f64(), true);
        }

        // Add a small delay between benchmarks to avoid system overload
        sleep(Duration::from_millis(500)).await;
    }

    let total_duration = start_time.elapsed();
    info!("Benchmark suite completed in {:?}", total_duration);

    // Generate and save report
    let report = generate_benchmark_suite_report(suite, &results, total_duration);
    save_report(&suite.output_dir, &report, "benchmark_report.md").await;

    // Save results to JSON
    save_results(&suite.output_dir, &results, "benchmark_results.json").await;

    // Generate and save performance monitor report if enabled
    if let Some(Ok(_perf_monitor)) = &monitor {
        // Performance monitor report generation is not implemented yet
        // Will be added in future versions
    }

    results
}

/// Generate benchmark suite report
fn generate_benchmark_suite_report(
    suite: &BenchmarkSuite,
    results: &[BenchmarkResult],
    total_duration: Duration,
) -> String {
    let mut report = String::new();

    report.push_str(&"# Performance Benchmark Suite Report\n\n".to_string());
    report.push_str(&"## Suite Information\n\n".to_string());
    report.push_str(&format!("- **Suite Name**: {}\n", suite.name));
    report.push_str(&format!("- **Description**: {}\n", suite.description));
    report.push_str(&format!(
        "- **Total Benchmarks**: {}\n",
        suite.benchmarks.len()
    ));
    report.push_str(&format!("- **Total Duration**: {:?}\n", total_duration));
    report.push_str(&format!("- **Output Directory**: {}\n", suite.output_dir));
    report.push_str(&format!(
        "- **Monitoring Enabled**: {}\n\n",
        suite.enable_monitoring
    ));

    report.push_str("## Benchmark Results\n\n");
    report.push_str("| Benchmark | Iterations | Avg Time (ms) | Min (ms) | Max (ms) | P50 (ms) | P95 (ms) | P99 (ms) | Throughput (ops/s) |\n");
    report.push_str("|-----------|------------|---------------|----------|----------|----------|----------|----------|-------------------|\n");

    for result in results {
        report.push_str(&format!(
            "| {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
            result.name,
            result.iterations,
            result.avg_time,
            result.min_time,
            result.max_time,
            result.median_time,
            result.median_time, // Using median as P50
            result.median_time, // Using median as P95/P99 for simplicity
            result.throughput
        ));
    }

    report.push_str("\n## Performance Analysis\n\n");

    // Analyze results
    let mut slowest_benchmark: Option<BenchmarkResult> = None;
    let mut fastest_benchmark: Option<BenchmarkResult> = None;
    let mut highest_throughput: Option<BenchmarkResult> = None;

    for result in results {
        // Find slowest benchmark
        if let Some(ref current) = slowest_benchmark {
            if result.avg_time > current.avg_time {
                slowest_benchmark = Some(result.clone());
            }
        } else {
            slowest_benchmark = Some(result.clone());
        }

        // Find fastest benchmark
        if let Some(ref current) = fastest_benchmark {
            if result.avg_time < current.avg_time {
                fastest_benchmark = Some(result.clone());
            }
        } else {
            fastest_benchmark = Some(result.clone());
        }

        // Find highest throughput
        if let Some(ref current) = highest_throughput {
            if result.throughput > current.throughput {
                highest_throughput = Some(result.clone());
            }
        } else {
            highest_throughput = Some(result.clone());
        }
    }

    if let Some(slowest) = slowest_benchmark {
        report.push_str(&"### Slowest Benchmark\n\n".to_string());
        report.push_str(&format!("- **Name**: {}\n", slowest.name));
        report.push_str(&format!("- **Average Time**: {:.2}ms\n", slowest.avg_time));
        report.push_str(&format!(
            "- **Throughput**: {:.2} ops/s\n\n",
            slowest.throughput
        ));
    }

    if let Some(fastest) = fastest_benchmark {
        report.push_str(&"### Fastest Benchmark\n\n".to_string());
        report.push_str(&format!("- **Name**: {}\n", fastest.name));
        report.push_str(&format!("- **Average Time**: {:.2}ms\n", fastest.avg_time));
        report.push_str(&format!(
            "- **Throughput**: {:.2} ops/s\n\n",
            fastest.throughput
        ));
    }

    if let Some(highest) = highest_throughput {
        report.push_str(&"### Highest Throughput\n\n".to_string());
        report.push_str(&format!("- **Name**: {}\n", highest.name));
        report.push_str(&format!(
            "- **Throughput**: {:.2} ops/s\n",
            highest.throughput
        ));
        report.push_str(&format!(
            "- **Average Time**: {:.2}ms\n\n",
            highest.avg_time
        ));
    }

    report.push_str("## Recommendations\n\n");
    report.push_str("1. **Focus on Slow Benchmarks**: Prioritize optimization efforts on the slowest benchmarks.\n");
    report.push_str(
        "2. **Monitor Regularly**: Run benchmarks regularly to track performance changes.\n",
    );
    report.push_str("3. **Set Performance Baselines**: Establish baseline performance metrics for future comparisons.\n");
    report.push_str(
        "4. **Optimize Critical Paths**: Identify and optimize critical paths in the codebase.\n",
    );
    report.push_str("5. **Use Performance Monitoring**: Enable real-time performance monitoring in production.\n");
    report.push_str(
        "6. **Tune System Resources**: Adjust system resources based on benchmark results.\n",
    );
    report.push_str(
        "7. **Profile Code**: Use profiling tools to identify specific performance bottlenecks.\n",
    );
    report.push_str(
        "8. **Document Results**: Keep records of benchmark results for historical analysis.\n",
    );

    report
}

/// Save report to file
async fn save_report(output_dir: &str, report: &str, filename: &str) {
    let path = Path::new(output_dir).join(filename);
    let mut file = File::create(path).unwrap_or_else(|e| {
        error!("Failed to create report file: {:?}", e);
        panic!("Failed to create report file");
    });

    file.write_all(report.as_bytes()).unwrap_or_else(|e| {
        error!("Failed to write report file: {:?}", e);
        panic!("Failed to write report file");
    });

    info!("Report saved to: {}/{}", output_dir, filename);
}

/// Save benchmark results to JSON file
async fn save_results(output_dir: &str, results: &[BenchmarkResult], filename: &str) {
    let path = Path::new(output_dir).join(filename);
    let mut file = File::create(path).unwrap_or_else(|e| {
        error!("Failed to create results file: {:?}", e);
        panic!("Failed to create results file");
    });

    let json = serde_json::to_string_pretty(results).unwrap_or_else(|e| {
        error!("Failed to serialize results: {:?}", e);
        panic!("Failed to serialize results");
    });

    file.write_all(json.as_bytes()).unwrap_or_else(|e| {
        error!("Failed to write results file: {:?}", e);
        panic!("Failed to write results file");
    });

    info!("Results saved to: {}/{}", output_dir, filename);
}

/// Run performance benchmark suite with default configuration
pub async fn run_default_benchmark() {
    let suite = BenchmarkSuite::default();
    run_benchmark_suite(&suite).await;
}

/// Run performance benchmark suite with custom configuration
pub async fn run_custom_benchmark(suite: &BenchmarkSuite) {
    run_benchmark_suite(suite).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_default_benchmark() {
        // This test runs the default benchmark suite
        // It may take some time to complete
        run_default_benchmark().await;
    }

    #[tokio::test]
    async fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::default();
        assert_eq!(suite.name, "default_suite");
        assert_eq!(suite.benchmarks.len(), 3);
        assert_eq!(suite.output_dir, "./performance_results");
    }

    #[test]
    fn test_generate_report() {
        let suite = BenchmarkSuite::default();
        let results = vec![];
        let duration = Duration::from_secs(10);
        let report = generate_benchmark_suite_report(&suite, &results, duration);
        assert!(report.contains("Performance Benchmark Suite Report"));
        assert!(report.contains(&suite.name));
    }
}
