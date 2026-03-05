use crate::core::iterate_api::IterateError;
use crate::core::state::AppState;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Compatibility check request
#[derive(Debug, Deserialize)]
pub struct CompatibilityCheckRequest {
    /// Plugin path
    pub plugin_path: String,
    /// Feature identifier
    pub feature_id: String,
    /// Dependency list
    pub dependencies: Vec<String>,
    /// Plugin version
    pub plugin_version: String,
    /// Core version requirement
    pub core_version: String,
    /// Plugin code
    pub plugin_code: String,
}

/// Compatibility check response
#[derive(Debug, Serialize)]
pub struct CompatibilityCheckResponse {
    /// Compatibility flag
    pub compatible: bool,
    /// Conflict list
    pub conflicts: Vec<CompatibilityConflict>,
    /// Fix suggestions
    pub fix_suggestions: Vec<String>,
    /// Compatibility score
    pub compatibility_score: u8,
}

/// Compatibility conflict
#[derive(Debug, Serialize)]
pub struct CompatibilityConflict {
    /// Conflict type
    pub conflict_type: String,
    /// Conflict description
    pub description: String,
    /// Impact scope
    pub impact: String,
    /// Fix priority
    pub priority: String,
}

/// Version compatibility automatic protection
#[derive(Debug, Clone)]
pub struct CompatibilityManager {
    /// Core version information
    pub core_version: String,
    /// Known conflict library list
    pub known_conflicts: Vec<KnownConflict>,
}

/// Known conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownConflict {
    /// Library name
    pub library: String,
    /// Conflict version range
    pub conflict_versions: String,
    /// Description
    pub description: String,
    /// Fix suggestion
    pub fix_suggestion: String,
}

impl CompatibilityManager {
    /// Create a new compatibility manager
    pub fn new() -> Self {
        Self {
            core_version: "1.0.0".to_string(),
            known_conflicts: Self::load_known_conflicts(),
        }
    }

    /// Load known conflict library list
    fn load_known_conflicts() -> Vec<KnownConflict> {
        vec![
            KnownConflict {
                library: "tokio".to_string(),
                conflict_versions: "<1.0.0".to_string(),
                description: "Tokio 0.x is not compatible with core".to_string(),
                fix_suggestion: "Upgrade to Tokio 1.x".to_string(),
            },
            KnownConflict {
                library: "axum".to_string(),
                conflict_versions: ">=0.8.0".to_string(),
                description: "Axum 0.8.0+ is not compatible with core".to_string(),
                fix_suggestion: "Use Axum 0.7.5".to_string(),
            },
            KnownConflict {
                library: "sqlx".to_string(),
                conflict_versions: "<0.8.0".to_string(),
                description: "SQLx <0.8.0 is not compatible with core".to_string(),
                fix_suggestion: "Upgrade to SQLx 0.8.6".to_string(),
            },
        ]
    }

    /// Check compatibility
    pub async fn check_compatibility(
        &self,
        request: CompatibilityCheckRequest,
        _state: Arc<AppState>,
    ) -> Result<CompatibilityCheckResponse, IterateError> {
        info!(
            "Checking compatibility for plugin: {}, version: {}",
            request.feature_id, request.plugin_version
        );

        let mut conflicts = Vec::new();
        let mut fix_suggestions = Vec::new();

        // Check version compatibility
        let version_conflict =
            self.check_version_compatibility(&request.core_version, &request.plugin_version);
        if let Some(conflict) = version_conflict {
            conflicts.push(conflict);
            fix_suggestions.push(format!(
                "Ensure core version {} is compatible with plugin version {}",
                request.core_version, request.plugin_version
            ));
        }

        // Check dependency compatibility
        let dependency_conflicts = self.check_dependency_compatibility(&request.dependencies);
        for conflict in &dependency_conflicts {
            let compatibility_conflict = CompatibilityConflict {
                conflict_type: "dependency_conflict".to_string(),
                description: conflict.description.clone(),
                impact: "runtime_crash".to_string(),
                priority: "high".to_string(),
            };
            conflicts.push(compatibility_conflict);
            fix_suggestions.push(conflict.fix_suggestion.clone());
        }

        // Check code compatibility
        let code_conflicts = self.check_code_compatibility(&request.plugin_code);
        conflicts.extend(code_conflicts);

        // Generate fix suggestions
        fix_suggestions.extend(self.generate_fix_suggestions(&conflicts));

        // Calculate compatibility score
        let compatibility_score = self.calculate_compatibility_score(&conflicts);

        info!(
            "Compatibility check completed for: {}, score: {}/100",
            request.feature_id, compatibility_score
        );

        Ok(CompatibilityCheckResponse {
            compatible: conflicts.is_empty(),
            conflicts,
            fix_suggestions,
            compatibility_score,
        })
    }

    /// Check version compatibility
    fn check_version_compatibility(
        &self,
        core_version: &str,
        plugin_version: &str,
    ) -> Option<CompatibilityConflict> {
        // Parse core version
        let core_parts: Vec<&str> = core_version.split('.').collect();
        if core_parts.len() < 2 {
            return Some(CompatibilityConflict {
                conflict_type: "version_format".to_string(),
                description: format!("Invalid core version format: {}", core_version),
                impact: "core_integration".to_string(),
                priority: "high".to_string(),
            });
        }

        // Parse plugin version
        let plugin_parts: Vec<&str> = plugin_version.split('.').collect();
        if plugin_parts.len() < 2 {
            return Some(CompatibilityConflict {
                conflict_type: "version_format".to_string(),
                description: format!("Invalid plugin version format: {}", plugin_version),
                impact: "core_integration".to_string(),
                priority: "high".to_string(),
            });
        }

        // Check if major version matches
        if core_parts[0] != plugin_parts[0] {
            return Some(CompatibilityConflict {
                conflict_type: "major_version_mismatch".to_string(),
                description: format!(
                    "Major version mismatch: core {} vs plugin {}",
                    core_parts[0], plugin_parts[0]
                ),
                impact: "core_integration".to_string(),
                priority: "high".to_string(),
            });
        }

        // Check if minor version is compatible
        let core_minor = core_parts[1].parse::<u32>().unwrap_or(0);
        let plugin_minor = plugin_parts[1].parse::<u32>().unwrap_or(0);
        if plugin_minor > core_minor {
            return Some(CompatibilityConflict {
                conflict_type: "minor_version_mismatch".to_string(),
                description: format!(
                    "Minor version mismatch: core {} vs plugin {}",
                    core_minor, plugin_minor
                ),
                impact: "feature_availability".to_string(),
                priority: "medium".to_string(),
            });
        }

        None
    }

    /// Check dependency compatibility
    fn check_dependency_compatibility(&self, dependencies: &[String]) -> Vec<KnownConflict> {
        let mut conflicts = Vec::new();

        for dep in dependencies {
            // Parse dependency format: name@version
            let parts: Vec<&str> = dep.split('@').collect();
            if parts.len() != 2 {
                warn!("Invalid dependency format: {}", dep);
                continue;
            }

            let dep_name = parts[0];
            let dep_version = parts[1];

            // Check for known conflicts
            for known_conflict in &self.known_conflicts {
                if known_conflict.library == dep_name {
                    // Simplified version range check
                    if self.is_version_in_conflict_range(
                        dep_version,
                        &known_conflict.conflict_versions,
                    ) {
                        conflicts.push(known_conflict.clone());
                    }
                }
            }
        }

        conflicts
    }

    /// Check code compatibility
    fn check_code_compatibility(&self, code: &str) -> Vec<CompatibilityConflict> {
        let mut conflicts = Vec::new();

        // Check used axum version
        if code.contains("axum::routing::Router") {
            // Check if incompatible axum 0.8+ features are used
            if code.contains(".with_state(") {
                // axum 0.7 uses .layer(Extension(...)) instead of .with_state()
                conflicts.push(CompatibilityConflict {
                    conflict_type: "api_usage".to_string(),
                    description: "Used incompatible axum API: .with_state() (axum 0.8+)"
                        .to_string(),
                    impact: "runtime_crash".to_string(),
                    priority: "high".to_string(),
                });
            }
        }

        // Check tokio version
        if code.contains("tokio::main") {
            // Check if tokio 1.x features are used
            if code.contains("tokio::spawn_blocking") {
                // tokio 1.x supports spawn_blocking
                // No need to handle
            }
        }

        // Check sqlx version
        if code.contains("sqlx::") && code.contains(".await?") {
            // sqlx 0.8+ uses async/await
            // No need to handle
        }

        conflicts
    }

    /// Generate fix suggestions
    fn generate_fix_suggestions(&self, conflicts: &[CompatibilityConflict]) -> Vec<String> {
        let mut suggestions = Vec::new();

        if conflicts
            .iter()
            .any(|c| c.conflict_type == "major_version_mismatch")
        {
            suggestions.push("Consider upgrading core version or downgrading plugin version to resolve major version conflicts".to_string());
        }

        if conflicts.iter().any(|c| c.conflict_type == "api_usage") {
            suggestions.push("Please check the core framework documentation and use compatible API calling methods".to_string());
        }

        if conflicts.iter().any(|c| c.priority == "high") {
            suggestions.push("Please prioritize fixing high-priority conflicts, otherwise it may lead to runtime crashes".to_string());
        }

        suggestions
    }

    /// Calculate compatibility score
    fn calculate_compatibility_score(&self, conflicts: &[CompatibilityConflict]) -> u8 {
        let mut score = 100u8;

        for conflict in conflicts {
            match conflict.priority.as_str() {
                "high" => score = score.saturating_sub(30),
                "medium" => score = score.saturating_sub(15),
                "low" => score = score.saturating_sub(5),
                _ => score = score.saturating_sub(10),
            }
        }

        score
    }

    /// Check if version is in conflict range
    fn is_version_in_conflict_range(&self, version: &str, range: &str) -> bool {
        // Simplified version range check
        if let Some(min_version) = range.strip_prefix(">=") {
            self.compare_versions(version, min_version) >= 0
        } else if let Some(min_version) = range.strip_prefix(">") {
            self.compare_versions(version, min_version) > 0
        } else if let Some(max_version) = range.strip_prefix("<=") {
            self.compare_versions(version, max_version) <= 0
        } else if let Some(max_version) = range.strip_prefix("<") {
            self.compare_versions(version, max_version) < 0
        } else if range.contains("<=") && range.contains(">=") {
            // Range format: >=1.0.0 <=2.0.0
            let parts: Vec<&str> = range.split(" ").collect();
            if parts.len() != 3 {
                return false;
            }
            let min_version = &parts[0][2..];
            let max_version = &parts[2][2..];
            self.compare_versions(version, min_version) >= 0
                && self.compare_versions(version, max_version) <= 0
        } else {
            // Exact version match
            version == range
        }
    }

    /// Compare version numbers
    fn compare_versions(&self, version1: &str, version2: &str) -> i32 {
        let parts1: Vec<&str> = version1.split('.').collect();
        let parts2: Vec<&str> = version2.split('.').collect();

        let max_len = std::cmp::max(parts1.len(), parts2.len());

        for i in 0..max_len {
            let v1 = parts1.get(i).unwrap_or(&"0").parse::<u32>().unwrap_or(0);
            let v2 = parts2.get(i).unwrap_or(&"0").parse::<u32>().unwrap_or(0);

            if v1 > v2 {
                return 1;
            } else if v1 < v2 {
                return -1;
            }
        }

        0
    }
}

impl Default for CompatibilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_version_compatibility() {
        let manager = CompatibilityManager::new();

        // Compatible versions
        let result = manager.check_version_compatibility("1.0.0", "1.0.0");
        assert!(result.is_none());

        // Compatible versions (plugin minor version lower than core)
        let result = manager.check_version_compatibility("1.5.0", "1.3.0");
        assert!(result.is_none());

        // Incompatible versions (major version mismatch)
        let result = manager.check_version_compatibility("1.0.0", "2.0.0");
        assert!(result.is_some());

        // Incompatible versions (plugin minor version higher than core)
        let result = manager.check_version_compatibility("1.3.0", "1.5.0");
        assert!(result.is_some());
    }

    #[test]
    fn test_check_dependency_compatibility() {
        let manager = CompatibilityManager::new();

        // Compatible dependencies
        let dependencies = vec!["tokio@1.28.0".to_string(), "axum@0.7.5".to_string()];
        let result = manager.check_dependency_compatibility(&dependencies);
        assert!(result.is_empty());

        // Incompatible dependencies
        let dependencies = vec!["tokio@0.2.0".to_string(), "axum@0.8.0".to_string()];
        let result = manager.check_dependency_compatibility(&dependencies);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_compare_versions() {
        let manager = CompatibilityManager::new();

        assert_eq!(manager.compare_versions("1.0.0", "1.0.0"), 0);
        assert_eq!(manager.compare_versions("1.1.0", "1.0.0"), 1);
        assert_eq!(manager.compare_versions("1.0.0", "1.1.0"), -1);
        assert_eq!(manager.compare_versions("2.0.0", "1.9.9"), 1);
    }

    #[test]
    fn test_calculate_compatibility_score() {
        let manager = CompatibilityManager::new();

        // No conflicts, full score
        let conflicts = Vec::new();
        let score = manager.calculate_compatibility_score(&conflicts);
        assert_eq!(score, 100);

        // 1 high priority conflict
        let conflicts = vec![CompatibilityConflict {
            conflict_type: "major_version_mismatch".to_string(),
            description: "Major version mismatch".to_string(),
            impact: "core_integration".to_string(),
            priority: "high".to_string(),
        }];
        let score = manager.calculate_compatibility_score(&conflicts);
        assert_eq!(score, 70);

        // Multiple conflicts
        let conflicts = vec![
            CompatibilityConflict {
                conflict_type: "major_version_mismatch".to_string(),
                description: "Major version mismatch".to_string(),
                impact: "core_integration".to_string(),
                priority: "high".to_string(),
            },
            CompatibilityConflict {
                conflict_type: "minor_version_mismatch".to_string(),
                description: "Minor version mismatch".to_string(),
                impact: "feature_availability".to_string(),
                priority: "medium".to_string(),
            },
        ];
        let score = manager.calculate_compatibility_score(&conflicts);
        assert_eq!(score, 55);
    }
}
