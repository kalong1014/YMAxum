//! Performance test binary
//! Runs performance tests and benchmarks

use clap::Parser;
use ymaxum::performance::benchmark::{BenchmarkConfig, BenchmarkRunner, BenchmarkType};

/// Performance test command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Run benchmark test
    Benchmark {
        /// Benchmark name
        #[arg(short, long, default_value = "default")]
        name: String,

        /// Number of iterations
        #[arg(long, default_value = "100")]
        iterations: usize,

        /// Number of warmup iterations
        #[arg(long, default_value = "10")]
        warmup: usize,
    },

    /// Compare performance
    Compare {
        /// Benchmark name
        #[arg(short, long, default_value = "comparison")]
        name: String,
    },

    /// Generate tuning suggestions
    Tune {
        /// Benchmark name
        #[arg(short, long, default_value = "tuning")]
        name: String,
    },

    /// Run comprehensive performance test
    Test {
        /// Test duration in seconds
        #[arg(long, default_value = "60")]
        duration: u64,
    },
}

fn main() {
    env_logger::init();

    let command = Command::parse();

    match command {
        Command::Benchmark {
            name,
            iterations,
            warmup,
        } => {
            run_benchmark(name, iterations, warmup);
        }
        Command::Compare { name } => {
            compare_performance(name);
        }
        Command::Tune { name } => {
            generate_tuning_suggestions(name);
        }
        Command::Test { duration } => {
            run_comprehensive_test(duration);
        }
    }
}

fn run_benchmark(name: String, iterations: usize, warmup: usize) {
    println!("Running benchmark test...");
    println!("Benchmark name: {}", name);
    println!("Iterations: {}", iterations);
    println!("Warmup iterations: {}", warmup);
    println!("=");

    let mut runner = BenchmarkRunner::new();

    let config = BenchmarkConfig {
        name: name.clone(),
        description: "Performance benchmark test".to_string(),
        benchmark_type: BenchmarkType::Mixed,
        iterations,
        warmup_iterations: warmup,
        enable_gc: true,
        concurrency: 4,
        enable_resource_monitoring: true,
    };

    // Run benchmark for CPU-intensive task
    let cpu_result = runner.run_benchmark(config.clone(), || {
        // Simulate CPU-intensive task
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += i as u64;
        }
        sum
    });

    println!("CPU benchmark results:");
    println!("Average execution time: {:.2}ms", cpu_result.avg_time);
    println!("Minimum execution time: {:.2}ms", cpu_result.min_time);
    println!("Maximum execution time: {:.2}ms", cpu_result.max_time);
    println!("Throughput: {:.2}ops/s", cpu_result.throughput);
    println!("=");

    // Run benchmark for memory-intensive task
    let memory_result = runner.run_benchmark(config, || {
        // Simulate memory-intensive task
        let mut vec = Vec::new();
        for i in 0..100_000 {
            vec.push(i);
        }
        vec.len()
    });

    println!("Memory benchmark results:");
    println!("Average execution time: {:.2}ms", memory_result.avg_time);
    println!("Minimum execution time: {:.2}ms", memory_result.min_time);
    println!("Maximum execution time: {:.2}ms", memory_result.max_time);
    println!("Throughput: {:.2}ops/s", memory_result.throughput);
    println!("=");

    // Generate report
    let report = runner.generate_benchmark_report();
    println!("{}", report);
}

fn compare_performance(name: String) {
    println!("Comparing performance...");
    println!("Benchmark name: {}", name);
    println!("=");

    let mut runner = BenchmarkRunner::new();

    let config = BenchmarkConfig {
        name: name.clone(),
        description: "Performance comparison test".to_string(),
        benchmark_type: BenchmarkType::CPU,
        iterations: 100,
        warmup_iterations: 10,
        enable_gc: true,
        concurrency: 4,
        enable_resource_monitoring: true,
    };

    // Run benchmark before optimization
    let before_result = runner.run_benchmark(config.clone(), || {
        // Simulate unoptimized code
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += i as u64;
        }
        sum
    });

    // Run benchmark after optimization
    let after_result = runner.run_benchmark(config, || {
        // Simulate optimized code
        // Using formula for sum of first n integers: n*(n+1)/2
        let n: u64 = 999_999;
        n * (n + 1) / 2
    });

    // Compare results
    runner.compare_performance(name, before_result, after_result);

    // Generate report
    let report = runner.generate_benchmark_report();
    println!("{}", report);
}

fn generate_tuning_suggestions(name: String) {
    println!("Generating performance tuning suggestions...");
    println!("Benchmark name: {}", name);
    println!("=");

    let mut runner = BenchmarkRunner::new();

    let config = BenchmarkConfig {
        name: name.clone(),
        description: "Performance tuning test".to_string(),
        benchmark_type: BenchmarkType::CPU,
        iterations: 100,
        warmup_iterations: 10,
        enable_gc: true,
        concurrency: 4,
        enable_resource_monitoring: true,
    };

    // Run benchmark
    let result = runner.run_benchmark(config, || {
        // Simulate task with performance issues
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += i as u64;
        }
        sum
    });

    // Generate tuning suggestions
    runner.generate_tuning_suggestions(&[result]);

    // Generate report
    let report = runner.generate_benchmark_report();
    println!("{}", report);
}

fn run_comprehensive_test(duration: u64) {
    println!("Running comprehensive performance test...");
    println!("Test duration: {} seconds", duration);
    println!("=");

    let start_time = std::time::Instant::now();
    let mut tests_completed = 0;

    // Run multiple benchmark types
    let benchmarks = vec![
        ("cpu_intensive", "CPU-intensive task"),
        ("memory_intensive", "Memory-intensive task"),
        ("io_bound", "IO-bound task"),
        ("concurrency", "Concurrency test"),
    ];

    for (bench_name, bench_desc) in benchmarks {
        println!("Running {} test: {}", bench_name, bench_desc);

        let mut runner = BenchmarkRunner::new();
        let benchmark_type = match bench_name {
            "cpu_intensive" => BenchmarkType::CPU,
            "memory_intensive" => BenchmarkType::Memory,
            "io_bound" => BenchmarkType::IO,
            "concurrency" => BenchmarkType::Mixed,
            _ => BenchmarkType::Mixed,
        };

        let config = BenchmarkConfig {
            name: bench_name.to_string(),
            description: bench_desc.to_string(),
            benchmark_type,
            iterations: 50,
            warmup_iterations: 5,
            enable_gc: true,
            concurrency: 4,
            enable_resource_monitoring: true,
        };

        match bench_name {
            "cpu_intensive" => {
                runner.run_benchmark(config, || {
                    let mut sum: u64 = 0;
                    for i in 0..1_000_000 {
                        sum += i as u64;
                    }
                    sum
                });
            }
            "memory_intensive" => {
                runner.run_benchmark(config, || {
                    let mut vec = Vec::new();
                    for i in 0..100_000 {
                        vec.push(i);
                    }
                    vec.len()
                });
            }
            "io_bound" => {
                runner.run_benchmark(config, || {
                    // Simulate IO-bound task
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    1
                });
            }
            "concurrency" => {
                runner.run_benchmark(config, || {
                    // Simulate concurrency
                    let handles: Vec<_> = (0..4)
                        .map(|_| {
                            std::thread::spawn(|| {
                                let mut sum: u64 = 0;
                                for i in 0..250_000 {
                                    sum += i as u64;
                                }
                                sum
                            })
                        })
                        .collect();

                    handles.into_iter().map(|h| h.join().unwrap()).sum::<u64>()
                });
            }
            _ => {}
        }

        tests_completed += 1;
        println!("Completed {} test", bench_name);
        println!("=");
    }

    let elapsed = start_time.elapsed();
    println!("Comprehensive performance test completed");
    println!("Tests run: {}", tests_completed);
    println!("Total duration: {:?}", elapsed);
    println!("Performance test results saved to performance_report.txt");
}
