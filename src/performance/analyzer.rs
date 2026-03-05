use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use crate::core::state::AppState;
use crate::performance::monitor::PerformanceMonitor;

/// Performance analyzer
pub struct PerformanceAnalyzer {
    app_state: Arc<AppState>,
    performance_monitor: Arc<RwLock<Option<PerformanceMonitor>>>,
    // Analysis results
    analysis_results: RwLock<AnalysisResults>,
}

/// Analysis results
#[derive(Debug, Clone)]
pub struct AnalysisResults {
    /// HTTP request analysis
    pub http_analysis: HttpAnalysis,
    /// Database analysis
    pub db_analysis: DatabaseAnalysis,
    /// Cache analysis
    pub cache_analysis: CacheAnalysis,
    /// System resource analysis
    pub system_analysis: SystemAnalysis,
    /// Analysis timestamp
    pub timestamp: Instant,
}

impl Default for AnalysisResults {
    fn default() -> Self {
        Self {
            http_analysis: HttpAnalysis::default(),
            db_analysis: DatabaseAnalysis::default(),
            cache_analysis: CacheAnalysis::default(),
            system_analysis: SystemAnalysis::default(),
            timestamp: Instant::now(),
        }
    }
}

/// HTTP request analysis
#[derive(Debug, Default, Clone)]
pub struct HttpAnalysis {
    /// Total requests
    pub total_requests: u64,
    /// Error requests
    pub error_requests: u64,
    /// Average response time (ms)
    pub avg_response_time: f64,
    /// P95 response time (ms)
    pub p95_response_time: f64,
    /// P99 response time (ms)
    pub p99_response_time: f64,
    /// Request rate (requests/second)
    pub request_rate: f64,
}

/// Database analysis
#[derive(Debug, Default, Clone)]
pub struct DatabaseAnalysis {
    /// Total queries
    pub total_queries: u64,
    /// Average query time (ms)
    pub avg_query_time: f64,
    /// P95 query time (ms)
    pub p95_query_time: f64,
    /// P99 query time (ms)
    pub p99_query_time: f64,
    /// Query rate (queries/second)
    pub query_rate: f64,
    /// Cache hit rate (%)
    pub cache_hit_rate: f64,
}

/// Cache analysis
#[derive(Debug, Default, Clone)]
pub struct CacheAnalysis {
    /// Total cache operations
    pub total_operations: u64,
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Hit rate (%)
    pub hit_rate: f64,
    /// Average cache response time (ms)
    pub avg_response_time: f64,
    /// Cache size
    pub cache_size: usize,
}

/// System resource analysis
#[derive(Debug, Default, Clone)]
pub struct SystemAnalysis {
    /// CPU usage (%)
    pub cpu_usage: f64,
    /// Memory usage (%)
    pub memory_usage: f64,
    /// Active connections
    pub active_connections: u64,
    /// Uptime (seconds)
    pub uptime: u64,
}

impl PerformanceAnalyzer {
    /// Create new performance analyzer
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            performance_monitor: Arc::new(RwLock::new(None)),
            analysis_results: RwLock::new(AnalysisResults::default()),
        }
    }

    /// Initialize performance analyzer
    pub async fn initialize(&self) {
        if let Some(monitor) = self.app_state.get_performance_monitor().await {
            *self.performance_monitor.write().await = Some(monitor.clone());
        }
    }

    /// Run performance analysis
    pub async fn run_analysis(&mut self) -> Result<AnalysisResults, anyhow::Error> {
        let mut results = AnalysisResults::default();
        results.timestamp = Instant::now();

        // Analyze HTTP requests
        self.analyze_http(&mut results.http_analysis).await;

        // Analyze database performance
        self.analyze_database(&mut results.db_analysis).await;

        // Analyze cache performance
        self.analyze_cache(&mut results.cache_analysis).await;

        // Analyze system resources
        self.analyze_system(&mut results.system_analysis).await;

        // Update analysis results
        *self.analysis_results.write().await = results.clone();

        Ok(results)
    }

    /// Analyze HTTP requests
    async fn analyze_http(&self, analysis: &mut HttpAnalysis) {
        if let Some(monitor) = self.performance_monitor.read().await.as_ref() {
            // Get HTTP metrics
            let (total_requests, error_requests) = monitor.get_http_metrics();
            let response_times = monitor.get_response_times();

            analysis.total_requests = total_requests;
            analysis.error_requests = error_requests;

            if !response_times.is_empty() {
                // Calculate average response time
                let sum: f64 = response_times.iter().sum();
                analysis.avg_response_time = sum / response_times.len() as f64;

                // Calculate P95 and P99 response times
                let mut sorted_times = response_times.clone();
                sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p95_index = (sorted_times.len() as f64 * 0.95).floor() as usize;
                let p99_index = (sorted_times.len() as f64 * 0.99).floor() as usize;

                if p95_index < sorted_times.len() {
                    analysis.p95_response_time = sorted_times[p95_index];
                }

                if p99_index < sorted_times.len() {
                    analysis.p99_response_time = sorted_times[p99_index];
                }
            }

            // Calculate request rate
            let uptime = self.app_state.uptime();
            if uptime > 0 {
                analysis.request_rate = total_requests as f64 / uptime as f64;
            }
        }
    }

    /// Analyze database performance
    async fn analyze_database(&self, analysis: &mut DatabaseAnalysis) {
        if let Some(monitor) = self.performance_monitor.read().await.as_ref() {
            // Get database metrics
            let (total_queries, query_times) = monitor.get_database_metrics_old();
            let cache_hit_rate = monitor.get_cache_hit_rate();

            analysis.total_queries = total_queries;
            analysis.cache_hit_rate = cache_hit_rate;

            if !query_times.is_empty() {
                // Calculate average query time
                let sum: f64 = query_times.iter().sum();
                analysis.avg_query_time = sum / query_times.len() as f64;

                // Calculate P95 and P99 query times
                let mut sorted_times = query_times.clone();
                sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p95_index = (sorted_times.len() as f64 * 0.95).floor() as usize;
                let p99_index = (sorted_times.len() as f64 * 0.99).floor() as usize;

                if p95_index < sorted_times.len() {
                    analysis.p95_query_time = sorted_times[p95_index];
                }

                if p99_index < sorted_times.len() {
                    analysis.p99_query_time = sorted_times[p99_index];
                }
            }

            // Calculate query rate
            let uptime = self.app_state.uptime();
            if uptime > 0 {
                analysis.query_rate = total_queries as f64 / uptime as f64;
            }
        }
    }

    /// Analyze cache performance
    async fn analyze_cache(&self, analysis: &mut CacheAnalysis) {
        if let Some(monitor) = self.performance_monitor.read().await.as_ref() {
            // Get cache metrics
            let (hits, misses) = monitor.get_cache_metrics();
            let response_times = monitor.get_cache_response_times();

            analysis.hits = hits;
            analysis.misses = misses;
            analysis.total_operations = hits + misses;

            // Calculate hit rate
            if analysis.total_operations > 0 {
                analysis.hit_rate = (hits as f64 / analysis.total_operations as f64) * 100.0;
            }

            // Calculate average cache response time
            if !response_times.is_empty() {
                let sum: f64 = response_times.iter().sum();
                analysis.avg_response_time = sum / response_times.len() as f64;
            }

            // Get cache size
            #[cfg(feature = "cache")]
            if let Some(cache) = self.app_state.memory_cache.get("cache_size")
                && let Ok(size) = cache.parse::<usize>()
            {
                analysis.cache_size = size;
            }
        }
    }

    /// Analyze system resources
    async fn analyze_system(&self, analysis: &mut SystemAnalysis) {
        // Get system uptime
        analysis.uptime = self.app_state.uptime();

        // Get active connections
        if let Some(monitor) = self.performance_monitor.read().await.as_ref() {
            analysis.active_connections = monitor.get_active_connections();
        }

        // TODO: Add CPU and memory usage analysis
        // This would require system-specific libraries
        analysis.cpu_usage = 0.0;
        analysis.memory_usage = 0.0;
    }

    /// Get latest analysis results
    pub async fn get_latest_results(&self) -> AnalysisResults {
        self.analysis_results.read().await.clone()
    }

    /// Generate performance report
    pub async fn generate_report(&self) -> String {
        let results = self.get_latest_results().await;

        format!(
            "# Performance Analysis Report\n\n{}",
            format!(
                "## Analysis Time: {}\n\n## HTTP Request Analysis\n- Total Requests: {}\n- Error Requests: {}\n- Average Response Time: {:.2} ms\n- P95 Response Time: {:.2} ms\n- P99 Response Time: {:.2} ms\n- Request Rate: {:.2} requests/second\n\n## Database Analysis\n- Total Queries: {}\n- Average Query Time: {:.2} ms\n- P95 Query Time: {:.2} ms\n- P99 Query Time: {:.2} ms\n- Query Rate: {:.2} queries/second\n- Cache Hit Rate: {:.2}%\n\n## Cache Analysis\n- Total Operations: {}\n- Cache Hits: {}\n- Cache Misses: {}\n- Hit Rate: {:.2}%\n- Average Cache Response Time: {:.2} ms\n- Cache Size: {} entries\n\n## System Resource Analysis\n- Uptime: {} seconds\n- Active Connections: {}\n- CPU Usage: {:.2}%\n- Memory Usage: {:.2}%\n",
                results.timestamp.elapsed().as_secs(),
                results.http_analysis.total_requests,
                results.http_analysis.error_requests,
                results.http_analysis.avg_response_time,
                results.http_analysis.p95_response_time,
                results.http_analysis.p99_response_time,
                results.http_analysis.request_rate,
                results.db_analysis.total_queries,
                results.db_analysis.avg_query_time,
                results.db_analysis.p95_query_time,
                results.db_analysis.p99_query_time,
                results.db_analysis.query_rate,
                results.db_analysis.cache_hit_rate,
                results.cache_analysis.total_operations,
                results.cache_analysis.hits,
                results.cache_analysis.misses,
                results.cache_analysis.hit_rate,
                results.cache_analysis.avg_response_time,
                results.cache_analysis.cache_size,
                results.system_analysis.uptime,
                results.system_analysis.active_connections,
                results.system_analysis.cpu_usage,
                results.system_analysis.memory_usage
            )
        )
    }
}
