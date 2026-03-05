//! Iterative compatibility protection module
//! Provides AI-assisted code generation, version compatibility automatic protection, and other features to ensure the compatibility and reliability of the iteration process
pub mod ai_tool;
pub mod compatibility;

use crate::core::iterate_api::{IterateError, IterateRequest};
use crate::core::state::AppState;
use crate::iterate::ai_tool::AIIterateTool;
use crate::iterate::compatibility::CompatibilityManager;
use std::sync::Arc;

/// Iterative compatibility protection service
#[derive(Debug, Clone)]
pub struct IterateService {
    /// AI iteration auxiliary tool
    pub ai_tool: Arc<AIIterateTool>,
    /// Compatibility manager
    pub compatibility_manager: Arc<CompatibilityManager>,
}

impl IterateService {
    /// Create a new iterative compatibility protection service
    pub fn new() -> Self {
        Self {
            ai_tool: Arc::new(AIIterateTool::new()),
            compatibility_manager: Arc::new(CompatibilityManager::new()),
        }
    }

    /// Generate plugin code
    pub async fn generate_plugin_code(
        &self,
        request: crate::iterate::ai_tool::AICodeGenRequest,
    ) -> Result<crate::iterate::ai_tool::AICodeGenResponse, IterateError> {
        self.ai_tool.generate_plugin_code(request).await
    }

    /// Regenerate plugin code
    pub async fn regenerate_plugin_code(
        &self,
        request: crate::iterate::ai_tool::AICodeGenRequest,
        previous_code: &str,
        error_message: &str,
    ) -> Result<crate::iterate::ai_tool::AICodeGenResponse, IterateError> {
        self.ai_tool
            .regenerate_plugin_code(request, previous_code, error_message)
            .await
    }

    /// Automatically adapt to iteration API
    pub async fn adapt_to_iterate_api(
        &self,
        code: &str,
        test_code: &str,
        config: &str,
    ) -> Result<IterateRequest, IterateError> {
        self.ai_tool
            .adapt_to_iterate_api(code, test_code, config)
            .await
    }

    /// Check compatibility
    pub async fn check_compatibility(
        &self,
        request: crate::iterate::compatibility::CompatibilityCheckRequest,
        state: Arc<AppState>,
    ) -> Result<crate::iterate::compatibility::CompatibilityCheckResponse, IterateError> {
        self.compatibility_manager
            .check_compatibility(request, state)
            .await
    }
}

impl Default for IterateService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::AppState;
    use log::info;

    #[tokio::test]
    async fn test_iterate_service() {
        let iterate_service = IterateService::new();
        let _app_state = Arc::new(AppState::new());

        // Test service creation
        assert!(iterate_service.ai_tool.templates.len() > 0);
        assert!(iterate_service.compatibility_manager.known_conflicts.len() > 0);

        info!("IterateService test passed");
    }
}
