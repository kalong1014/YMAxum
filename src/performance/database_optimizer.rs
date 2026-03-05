//! Database optimization module
//! Provides database query optimization, index optimization, connection pool optimization and other functions
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Database optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseOptimizerConfig {
    /// Query timeout (seconds)
    pub query_timeout: u64,
    /// Connection pool size
    pub pool_size: u32,
    /// Enable query cache
    pub enable_query_cache: bool,
    /// Query cache size
    pub query_cache_size: usize,
    /// Enable slow query log
    pub enable_slow_query_log: bool,
    /// Slow query threshold (milliseconds)
    pub slow_query_threshold: u64,
}

impl Default for DatabaseOptimizerConfig {
    fn default() -> Self {
        Self {
            query_timeout: 30,
            pool_size: 10,
            enable_query_cache: true,
            query_cache_size: 1000,
            enable_slow_query_log: true,
            slow_query_threshold: 1000,
        }
    }
}

/// Query statistics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStatistics {
    /// Query SQL
    pub query: String,
    /// Execution count
    pub execution_count: u64,
    /// Total execution time (milliseconds)
    pub total_execution_time: u64,
    /// Average execution time (milliseconds)
    pub avg_execution_time: f64,
    /// Maximum execution time (milliseconds)
    pub max_execution_time: u64,
    /// Minimum execution time (milliseconds)
    pub min_execution_time: u64,
    /// Whether it is a slow query
    pub is_slow_query: bool,
}

/// Index suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSuggestion {
    /// Table name
    pub table_name: String,
    /// Column name
    pub column_name: String,
    /// Index type
    pub index_type: String,
    /// Estimated performance improvement (%)
    pub estimated_improvement: f64,
    /// Suggestion reason
    pub reason: String,
}

/// Database optimizer
pub struct DatabaseOptimizer {
    /// Configuration
    config: DatabaseOptimizerConfig,
    /// Query statistics
    query_stats: HashMap<String, QueryStatistics>,
    /// Index suggestions
    index_suggestions: Vec<IndexSuggestion>,
    /// Slow query log
    slow_queries: Vec<QueryStatistics>,
}

impl DatabaseOptimizer {
    /// Create a new database optimizer
    pub fn new(config: DatabaseOptimizerConfig) -> Self {
        Self {
            config,
            query_stats: HashMap::new(),
            index_suggestions: Vec::new(),
            slow_queries: Vec::new(),
        }
    }

    /// Analyze query performance
    pub fn analyze_query(&mut self, query: &str, execution_time: Duration) {
        let query_key = query.to_string();
        let execution_time_ms = execution_time.as_millis() as u64;

        // Update query statistics
        let stats = self
            .query_stats
            .entry(query_key.clone())
            .or_insert_with(|| QueryStatistics {
                query: query.to_string(),
                execution_count: 0,
                total_execution_time: 0,
                avg_execution_time: 0.0,
                max_execution_time: 0,
                min_execution_time: u64::MAX,
                is_slow_query: false,
            });

        stats.execution_count += 1;
        stats.total_execution_time += execution_time_ms;
        stats.avg_execution_time = stats.total_execution_time as f64 / stats.execution_count as f64;
        stats.max_execution_time = stats.max_execution_time.max(execution_time_ms);
        stats.min_execution_time = stats.min_execution_time.min(execution_time_ms);
        stats.is_slow_query = execution_time_ms > self.config.slow_query_threshold;

        // Record slow query
        if stats.is_slow_query && self.config.enable_slow_query_log {
            info!("Slow query detected: {}ms - {}", execution_time_ms, query);
            self.slow_queries.push(stats.clone());

            // Keep only the latest 100 slow queries
            if self.slow_queries.len() > 100 {
                self.slow_queries.remove(0);
            }
        }
    }

    /// Generate index suggestions
    pub fn generate_index_suggestions(&mut self) {
        self.index_suggestions.clear();

        // Analyze query statistics to generate index suggestions
        for (query, stats) in &self.query_stats {
            // Check if it is a WHERE condition query
            if query.contains("WHERE") && stats.execution_count > 10 {
                // Extract column name (simplified version)
                if let Some(column) = self.extract_column_from_where(query) {
                    let improvement = self.estimate_index_improvement(stats);

                    self.index_suggestions.push(IndexSuggestion {
                        table_name: self.extract_table_name(query),
                        column_name: column,
                        index_type: "B-Tree".to_string(),
                        estimated_improvement: improvement,
                        reason: format!("This column is used {} times in WHERE conditions, suggest adding index", stats.execution_count),
                    });
                }
            }

            // Check if it is a JOIN query
            if query.contains("JOIN")
                && stats.avg_execution_time > 100.0
                && let Some(column) = self.extract_join_column(query)
            {
                let improvement = self.estimate_index_improvement(stats);

                self.index_suggestions.push(IndexSuggestion {
                    table_name: self.extract_table_name(query),
                    column_name: column,
                    index_type: "B-Tree".to_string(),
                    estimated_improvement: improvement,
                    reason: "This column is used in JOIN conditions, suggest adding index"
                        .to_string(),
                });
            }
        }
    }

    /// Optimize query
    pub fn optimize_query(&self, query: &str) -> String {
        let optimized_query = query.to_string();

        // Optimization 1: Use LIMIT to limit result set
        if !optimized_query.contains("LIMIT") && optimized_query.contains("SELECT") {
            warn!(
                "Query does not use LIMIT, suggest adding LIMIT clause: {}",
                query
            );
        }

        // Optimization 2: Avoid SELECT *
        if optimized_query.contains("SELECT *") {
            warn!(
                "Query uses SELECT *, suggest specifying specific columns: {}",
                query
            );
        }

        // Optimization 3: Use index suggestions
        if optimized_query.contains("WHERE") && !optimized_query.contains("FORCE INDEX") {
            info!("Suggest using index hints: {}", query);
        }

        optimized_query
    }

    /// Get query statistics
    pub fn get_query_statistics(&self) -> &HashMap<String, QueryStatistics> {
        &self.query_stats
    }

    /// Get slow query log
    pub fn get_slow_queries(&self) -> &[QueryStatistics] {
        &self.slow_queries
    }

    /// Get index suggestions
    pub fn get_index_suggestions(&self) -> &[IndexSuggestion] {
        &self.index_suggestions
    }

    /// Generate optimization report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Database Optimization Report ===\n\n");

        // Query statistics summary
        report.push_str("[Query Statistics]\n");
        report.push_str(&format!("Total queries: {}\n", self.query_stats.len()));

        let slow_query_count = self
            .query_stats
            .values()
            .filter(|s| s.is_slow_query)
            .count();
        report.push_str(&format!("Slow queries: {}\n", slow_query_count));

        if !self.query_stats.is_empty() {
            let avg_time: f64 = self
                .query_stats
                .values()
                .map(|s| s.avg_execution_time)
                .sum::<f64>()
                / self.query_stats.len() as f64;
            report.push_str(&format!("Average query time: {:.2}ms\n\n", avg_time));
        }

        // Slow query list
        if !self.slow_queries.is_empty() {
            report.push_str("[Slow Query List]\n");
            for (index, query) in self.slow_queries.iter().enumerate().take(10) {
                report.push_str(&format!("{}. {}\n", index + 1, query.query));
                report.push_str(&format!("   Execution count: {}\n", query.execution_count));
                report.push_str(&format!(
                    "   Average time: {:.2}ms\n",
                    query.avg_execution_time
                ));
                report.push_str(&format!("   Max time: {}ms\n\n", query.max_execution_time));
            }
        }

        // Index suggestions
        if !self.index_suggestions.is_empty() {
            report.push_str("[Index Suggestions]\n");
            for (index, suggestion) in self.index_suggestions.iter().enumerate() {
                report.push_str(&format!(
                    "{}. Table: {}, Column: {}\n",
                    index + 1,
                    suggestion.table_name,
                    suggestion.column_name
                ));
                report.push_str(&format!("   Index type: {}\n", suggestion.index_type));
                report.push_str(&format!(
                    "   Estimated improvement: {:.1}%\n",
                    suggestion.estimated_improvement
                ));
                report.push_str(&format!("   Reason: {}\n\n", suggestion.reason));
            }
        } else {
            report.push_str("[Index Suggestions] No indexes found that need to be added\n\n");
        }

        // Optimization suggestions
        report.push_str("[Optimization Suggestions]\n");
        report.push_str("1. Regularly analyze slow query logs\n");
        report.push_str("2. Add indexes to frequently queried columns\n");
        report.push_str("3. Avoid using SELECT *\n");
        report.push_str("4. Use LIMIT to limit result sets\n");
        report.push_str("5. Optimize JOIN queries\n");
        report.push_str("6. Use query cache\n");
        report.push_str("7. Regularly maintain database (ANALYZE, VACUUM)\n");

        report
    }

    /// Extract column name from WHERE condition
    fn extract_column_from_where(&self, query: &str) -> Option<String> {
        // Simplified version: extract the first column name in WHERE condition
        if let Some(where_pos) = query.find("WHERE") {
            let after_where = &query[where_pos + 5..];
            if let Some(eq_pos) = after_where.find('=') {
                let column = after_where[..eq_pos].trim();
                if !column.is_empty() {
                    return Some(column.to_string());
                }
            }
        }
        None
    }

    /// Extract column name from JOIN condition
    fn extract_join_column(&self, query: &str) -> Option<String> {
        // Simplified version: extract the first column name in JOIN condition
        if let Some(join_pos) = query.find("JOIN") {
            let after_join = &query[join_pos + 4..];
            if let Some(on_pos) = after_join.find("ON") {
                let after_on = &after_join[on_pos + 2..];
                if let Some(eq_pos) = after_on.find('=') {
                    let column = after_on[..eq_pos].trim();
                    if !column.is_empty() {
                        return Some(column.to_string());
                    }
                }
            }
        }
        None
    }

    /// Extract table name
    fn extract_table_name(&self, query: &str) -> String {
        // Simplified version: extract table name from FROM clause
        if let Some(from_pos) = query.find("FROM") {
            let after_from = &query[from_pos + 4..];
            let table_name = after_from
                .split_whitespace()
                .next()
                .unwrap_or("unknown")
                .to_string();
            return table_name;
        }
        "unknown".to_string()
    }

    /// Estimate index performance improvement
    fn estimate_index_improvement(&self, stats: &QueryStatistics) -> f64 {
        // Simplified version: estimate based on query count and average execution time
        if stats.avg_execution_time > 500.0 {
            80.0
        } else if stats.avg_execution_time > 200.0 {
            60.0
        } else if stats.avg_execution_time > 100.0 {
            40.0
        } else {
            20.0
        }
    }
}

impl Default for DatabaseOptimizer {
    fn default() -> Self {
        Self::new(DatabaseOptimizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_optimizer() {
        // Test query analysis
        let query = "SELECT * FROM users WHERE id = 1";
        let mut optimizer = DatabaseOptimizer::new(DatabaseOptimizerConfig::default());
        optimizer.analyze_query(query, Duration::from_millis(50));

        let stats = optimizer.get_query_statistics();
        assert_eq!(stats.len(), 1);
        assert!(stats.contains_key(query));
    }

    #[test]
    fn test_slow_query_detection() {
        let mut optimizer = DatabaseOptimizer::new(DatabaseOptimizerConfig::default());

        // Test slow query detection
        let query = "SELECT * FROM users WHERE id = 1";
        optimizer.analyze_query(query, Duration::from_millis(2000));

        let slow_queries = optimizer.get_slow_queries();
        assert_eq!(slow_queries.len(), 1);
        assert!(slow_queries[0].is_slow_query);
    }

    #[test]
    fn test_optimize_query() {
        let optimizer = DatabaseOptimizer::new(DatabaseOptimizerConfig::default());

        // Test query optimization
        let query = "SELECT * FROM users";
        let optimized = optimizer.optimize_query(query);

        assert_eq!(optimized, query);
    }

    #[test]
    fn test_generate_index_suggestions() {
        let mut optimizer = DatabaseOptimizer::new(DatabaseOptimizerConfig::default());

        // Add some query statistics
        for _ in 0..20 {
            optimizer.analyze_query(
                "SELECT * FROM users WHERE id = 1",
                Duration::from_millis(100),
            );
        }

        optimizer.generate_index_suggestions();
        let suggestions = optimizer.get_index_suggestions();

        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_generate_report() {
        let mut optimizer = DatabaseOptimizer::new(DatabaseOptimizerConfig::default());

        // Add some query statistics
        optimizer.analyze_query(
            "SELECT * FROM users WHERE id = 1",
            Duration::from_millis(50),
        );

        let report = optimizer.generate_report();

        assert!(report.contains("Database Optimization Report"));
        assert!(report.contains("Query Statistics"));
    }
}
