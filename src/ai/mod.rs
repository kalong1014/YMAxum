//! AI集成模块
//! 用于AI模型管理和部署、智能路由和负载均衡、AI驱动的性能优化、AI辅助开发工具

pub mod dev_tools;
pub mod intelligent_routing;
pub mod model_management;
pub mod performance_optimization;

/// AI集成管理器
#[derive(Debug, Clone)]
pub struct AiManager {
    model_management: model_management::ModelManager,
    intelligent_routing: intelligent_routing::IntelligentRouter,
    performance_optimization: performance_optimization::PerformanceOptimizer,
}

impl AiManager {
    /// 创建新的AI集成管理器
    pub fn new() -> Self {
        Self {
            model_management: model_management::ModelManager::new(),
            intelligent_routing: intelligent_routing::IntelligentRouter::new(),
            performance_optimization: performance_optimization::PerformanceOptimizer::new(),
        }
    }

    /// 初始化AI集成
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.model_management.initialize().await?;
        self.intelligent_routing.initialize().await?;
        self.performance_optimization.initialize().await?;
        Ok(())
    }

    /// 部署AI模型
    pub async fn deploy_model(
        &self,
        model: model_management::ModelInfo,
    ) -> Result<model_management::ModelDeploymentResult, Box<dyn std::error::Error>> {
        self.model_management.deploy_model(model).await
    }

    /// 智能路由
    pub async fn intelligent_route(
        &self,
        request: serde_json::Value,
    ) -> Result<intelligent_routing::RoutingResult, Box<dyn std::error::Error>> {
        self.intelligent_routing.route(request).await
    }

    /// 性能优化
    pub async fn optimize_performance(
        &self,
        metrics: serde_json::Value,
    ) -> Result<performance_optimization::OptimizationResult, Box<dyn std::error::Error>> {
        self.performance_optimization.optimize(metrics).await
    }
}
