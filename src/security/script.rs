//! Script security module
//! Provides security protection for script execution engine

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Script security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecurityConfig {
    /// Enable script content validation
    pub enable_content_validation: bool,
    /// Enable script execution sandbox
    pub enable_sandbox: bool,
    /// Maximum script execution time (seconds)
    pub max_execution_time: u64,
    /// Maximum memory usage (MB)
    pub max_memory_usage: u64,
    /// Maximum number of database queries per script
    pub max_db_queries: u64,
    /// Maximum number of API calls per script
    pub max_api_calls: u64,
    /// Allowed commands
    pub allowed_commands: HashSet<String>,
    /// Blocked patterns
    pub blocked_patterns: HashSet<String>,
    /// Enable script signature verification
    pub enable_signature_verification: bool,
}

impl Default for ScriptSecurityConfig {
    fn default() -> Self {
        Self {
            enable_content_validation: true,
            enable_sandbox: true,
            max_execution_time: 30,
            max_memory_usage: 100,
            max_db_queries: 100,
            max_api_calls: 100,
            allowed_commands: HashSet::from([
                "SCRIPT".to_string(),
                "FUNCTION".to_string(),
                "CONDITION".to_string(),
                "LOOP".to_string(),
                "VARIABLE".to_string(),
                "DATABASE".to_string(),
                "API".to_string(),
            ]),
            blocked_patterns: HashSet::from([
                "system".to_string(),
                "exec".to_string(),
                "shell".to_string(),
                "eval".to_string(),
                "require".to_string(),
                "import".to_string(),
            ]),
            enable_signature_verification: true,
        }
    }
}

/// Script security violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecurityViolation {
    /// Violation ID
    pub id: String,
    /// Violation type
    pub violation_type: ScriptViolationType,
    /// Severity level
    pub severity: ScriptViolationSeverity,
    /// Violation description
    pub description: String,
    /// Script name
    pub script_name: String,
    /// Violation location
    pub location: String,
    /// Remediation suggestions
    pub remediation: Vec<String>,
}

/// Script violation type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScriptViolationType {
    /// Unauthorized command
    UnauthorizedCommand,
    /// Blocked pattern found
    BlockedPattern,
    /// Execution time exceeded
    ExecutionTimeExceeded,
    /// Memory usage exceeded
    MemoryUsageExceeded,
    /// Database queries exceeded
    DbQueriesExceeded,
    /// API calls exceeded
    ApiCallsExceeded,
    /// Invalid signature
    InvalidSignature,
    /// Malformed script
    MalformedScript,
    /// Resource exhaustion
    ResourceExhaustion,
    /// Suspicious behavior
    SuspiciousBehavior,
}

/// Script violation severity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScriptViolationSeverity {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
}

/// Script security context
#[derive(Debug, Clone)]
pub struct ScriptSecurityContext {
    /// Script name
    pub script_name: String,
    /// Execution start time
    pub start_time: std::time::Instant,
    /// Database query count
    pub db_query_count: u64,
    /// API call count
    pub api_call_count: u64,
    /// Memory usage (bytes)
    pub memory_usage: u64,
    /// Executed commands
    pub executed_commands: Vec<String>,
}

impl Default for ScriptSecurityContext {
    fn default() -> Self {
        Self {
            script_name: "".to_string(),
            start_time: std::time::Instant::now(),
            db_query_count: 0,
            api_call_count: 0,
            memory_usage: 0,
            executed_commands: Vec::new(),
        }
    }
}

/// Script security manager
pub struct ScriptSecurityManager {
    /// Configuration
    config: ScriptSecurityConfig,
    /// Security violations
    violations: Vec<ScriptSecurityViolation>,
}

impl ScriptSecurityManager {
    /// Create new script security manager
    pub fn new(config: ScriptSecurityConfig) -> Self {
        Self {
            config,
            violations: Vec::new(),
        }
    }

    /// Create new script security manager with default configuration
    pub fn default() -> Self {
        Self::new(ScriptSecurityConfig::default())
    }

    /// Validate script content
    pub fn validate_script_content(
        &mut self,
        script_name: &str,
        script_content: &str,
    ) -> Result<(), ScriptSecurityViolation> {
        info!("Validating script content: {}", script_name);

        if !self.config.enable_content_validation {
            info!("Script content validation disabled");
            return Ok(());
        }

        // Check for blocked patterns
        for pattern in &self.config.blocked_patterns {
            if script_content.contains(pattern) {
                let violation = ScriptSecurityViolation {
                    id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                    violation_type: ScriptViolationType::BlockedPattern,
                    severity: ScriptViolationSeverity::Critical,
                    description: format!("Blocked pattern found: {}", pattern),
                    script_name: script_name.to_string(),
                    location: "script content".to_string(),
                    remediation: vec![
                        format!("Remove blocked pattern: {}", pattern),
                        "Use allowed alternatives".to_string(),
                    ],
                };
                self.violations.push(violation.clone());
                return Err(violation);
            }
        }

        info!("Script content validation passed: {}", script_name);
        Ok(())
    }

    /// Validate command execution
    pub fn validate_command(
        &mut self,
        script_name: &str,
        command: &str,
    ) -> Result<(), ScriptSecurityViolation> {
        info!("Validating command: {} in script: {}", command, script_name);

        // Check if command is allowed
        if !self.config.allowed_commands.contains(command) {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::UnauthorizedCommand,
                severity: ScriptViolationSeverity::High,
                description: format!("Unauthorized command: {}", command),
                script_name: script_name.to_string(),
                location: "command execution".to_string(),
                remediation: vec![
                    format!("Remove unauthorized command: {}", command),
                    "Use only allowed commands".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        Ok(())
    }

    /// Check execution time
    pub fn check_execution_time(
        &mut self,
        script_name: &str,
        context: &ScriptSecurityContext,
    ) -> Result<(), ScriptSecurityViolation> {
        let elapsed = context.start_time.elapsed().as_secs();
        if elapsed > self.config.max_execution_time {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::ExecutionTimeExceeded,
                severity: ScriptViolationSeverity::Medium,
                description: format!(
                    "Execution time exceeded: {}s > {}s",
                    elapsed, self.config.max_execution_time
                ),
                script_name: script_name.to_string(),
                location: "execution time".to_string(),
                remediation: vec![
                    "Optimize script performance".to_string(),
                    "Split script into smaller parts".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        Ok(())
    }

    /// Check memory usage
    pub fn check_memory_usage(
        &mut self,
        script_name: &str,
        memory_usage: u64,
    ) -> Result<(), ScriptSecurityViolation> {
        let memory_mb = memory_usage / (1024 * 1024);
        if memory_mb > self.config.max_memory_usage {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::MemoryUsageExceeded,
                severity: ScriptViolationSeverity::Medium,
                description: format!(
                    "Memory usage exceeded: {}MB > {}MB",
                    memory_mb, self.config.max_memory_usage
                ),
                script_name: script_name.to_string(),
                location: "memory usage".to_string(),
                remediation: vec![
                    "Optimize memory usage".to_string(),
                    "Reduce data processing in script".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        Ok(())
    }

    /// Check database query count
    pub fn check_db_queries(
        &mut self,
        script_name: &str,
        query_count: u64,
    ) -> Result<(), ScriptSecurityViolation> {
        if query_count > self.config.max_db_queries {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::DbQueriesExceeded,
                severity: ScriptViolationSeverity::Medium,
                description: format!(
                    "Database queries exceeded: {} > {}",
                    query_count, self.config.max_db_queries
                ),
                script_name: script_name.to_string(),
                location: "database queries".to_string(),
                remediation: vec![
                    "Optimize database queries".to_string(),
                    "Use batch operations".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        Ok(())
    }

    /// Check API call count
    pub fn check_api_calls(
        &mut self,
        script_name: &str,
        api_call_count: u64,
    ) -> Result<(), ScriptSecurityViolation> {
        if api_call_count > self.config.max_api_calls {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::ApiCallsExceeded,
                severity: ScriptViolationSeverity::Medium,
                description: format!(
                    "API calls exceeded: {} > {}",
                    api_call_count, self.config.max_api_calls
                ),
                script_name: script_name.to_string(),
                location: "API calls".to_string(),
                remediation: vec![
                    "Reduce API calls".to_string(),
                    "Use caching for repeated data".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        Ok(())
    }

    /// Verify script signature
    pub fn verify_signature(
        &mut self,
        script_name: &str,
        signature: Option<&str>,
    ) -> Result<(), ScriptSecurityViolation> {
        if !self.config.enable_signature_verification {
            info!("Script signature verification disabled");
            return Ok(());
        }

        if signature.is_none() {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::InvalidSignature,
                severity: ScriptViolationSeverity::Critical,
                description: "Missing script signature".to_string(),
                script_name: script_name.to_string(),
                location: "script signature".to_string(),
                remediation: vec![
                    "Add valid script signature".to_string(),
                    "Use authorized signing tool".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        // TODO: Implement actual signature verification
        // For now, just check if signature is not empty
        let sig = signature.unwrap();
        if sig.is_empty() {
            let violation = ScriptSecurityViolation {
                id: format!("VIOLATION-{}", uuid::Uuid::new_v4()),
                violation_type: ScriptViolationType::InvalidSignature,
                severity: ScriptViolationSeverity::Critical,
                description: "Empty script signature".to_string(),
                script_name: script_name.to_string(),
                location: "script signature".to_string(),
                remediation: vec![
                    "Add valid script signature".to_string(),
                    "Use authorized signing tool".to_string(),
                ],
            };
            self.violations.push(violation.clone());
            return Err(violation);
        }

        info!("Script signature verification passed: {}", script_name);
        Ok(())
    }

    /// Get security violations
    pub fn get_violations(&self) -> &Vec<ScriptSecurityViolation> {
        &self.violations
    }

    /// Clear security violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Create script security context
    pub fn create_context(&self, script_name: &str) -> ScriptSecurityContext {
        ScriptSecurityContext {
            script_name: script_name.to_string(),
            start_time: std::time::Instant::now(),
            db_query_count: 0,
            api_call_count: 0,
            memory_usage: 0,
            executed_commands: Vec::new(),
        }
    }

    /// Update script security context
    pub fn update_context(
        &mut self,
        context: &mut ScriptSecurityContext,
        command: &str,
        db_query: bool,
        api_call: bool,
    ) -> Result<(), ScriptSecurityViolation> {
        // Add executed command
        context.executed_commands.push(command.to_string());

        // Update counters
        if db_query {
            context.db_query_count += 1;
            self.check_db_queries(&context.script_name, context.db_query_count)?;
        }

        if api_call {
            context.api_call_count += 1;
            self.check_api_calls(&context.script_name, context.api_call_count)?;
        }

        // Check execution time
        self.check_execution_time(&context.script_name, context)?;

        // TODO: Implement memory usage tracking
        // For now, just return Ok

        Ok(())
    }
}

/// Script sandbox
pub struct ScriptSandbox {
    /// Security manager
    security_manager: ScriptSecurityManager,
    /// Security context
    context: ScriptSecurityContext,
}

impl ScriptSandbox {
    /// Create new script sandbox
    pub fn new(security_manager: ScriptSecurityManager, script_name: &str) -> Self {
        // Create context directly to avoid borrow issues
        let context = ScriptSecurityContext {
            script_name: script_name.to_string(),
            start_time: std::time::Instant::now(),
            db_query_count: 0,
            api_call_count: 0,
            memory_usage: 0,
            executed_commands: Vec::new(),
        };
        Self {
            security_manager,
            context,
        }
    }

    /// Execute command in sandbox
    pub fn execute_command(
        &mut self,
        command: &str,
        db_query: bool,
        api_call: bool,
    ) -> Result<(), ScriptSecurityViolation> {
        // Validate command
        self.security_manager
            .validate_command(&self.context.script_name, command)?;

        // Update context
        self.security_manager
            .update_context(&mut self.context, command, db_query, api_call)?;

        Ok(())
    }

    /// Get security violations
    pub fn get_violations(&self) -> &Vec<ScriptSecurityViolation> {
        self.security_manager.get_violations()
    }

    /// Get security context
    pub fn get_context(&self) -> &ScriptSecurityContext {
        &self.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_script_content() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";
        let valid_script = "SCRIPT EXECUTE NAME=test";
        let invalid_script = "SCRIPT EXECUTE NAME=test system('ls')";

        // Valid script should pass
        assert!(
            manager
                .validate_script_content(script_name, valid_script)
                .is_ok()
        );

        // Invalid script should fail
        assert!(
            manager
                .validate_script_content(script_name, invalid_script)
                .is_err()
        );
    }

    #[test]
    fn test_validate_command() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";
        let valid_command = "SCRIPT";
        let invalid_command = "SYSTEM";

        // Valid command should pass
        assert!(manager.validate_command(script_name, valid_command).is_ok());

        // Invalid command should fail
        assert!(
            manager
                .validate_command(script_name, invalid_command)
                .is_err()
        );
    }

    #[test]
    fn test_check_execution_time() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";
        let context = manager.create_context(script_name);

        // Should pass initially
        assert!(manager.check_execution_time(script_name, &context).is_ok());
    }

    #[test]
    fn test_check_db_queries() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";

        // Should pass with small count
        assert!(manager.check_db_queries(script_name, 10).is_ok());

        // Should fail with large count
        assert!(manager.check_db_queries(script_name, 1000).is_err());
    }

    #[test]
    fn test_check_api_calls() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";

        // Should pass with small count
        assert!(manager.check_api_calls(script_name, 10).is_ok());

        // Should fail with large count
        assert!(manager.check_api_calls(script_name, 1000).is_err());
    }

    #[test]
    fn test_verify_signature() {
        let mut manager = ScriptSecurityManager::default();
        let script_name = "test_script";

        // Should fail with no signature
        assert!(manager.verify_signature(script_name, None).is_err());

        // Should fail with empty signature
        assert!(manager.verify_signature(script_name, Some("")).is_err());

        // Should pass with non-empty signature
        assert!(
            manager
                .verify_signature(script_name, Some("test_signature"))
                .is_ok()
        );
    }
}
