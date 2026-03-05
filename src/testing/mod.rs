//! 自动化测试框架模块
//! 用于自动化测试用例生成、测试覆盖率分析和测试结果可视化

pub mod coverage_analysis;
pub mod result_visualization;
pub mod test_generation;

/// 自动化测试框架管理器
#[derive(Debug, Clone)]
pub struct TestingFrameworkManager {
    test_generation: test_generation::TestGenerator,
    coverage_analysis: coverage_analysis::CoverageAnalyzer,
    result_visualization: result_visualization::ResultVisualizer,
}

impl TestingFrameworkManager {
    /// 创建新的自动化测试框架管理器
    pub fn new() -> Self {
        Self {
            test_generation: test_generation::TestGenerator::new(
                test_generation::TestGenerationConfig::default(),
            ),
            coverage_analysis: coverage_analysis::CoverageAnalyzer::new(),
            result_visualization: result_visualization::ResultVisualizer::new(),
        }
    }

    /// 初始化自动化测试框架
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.test_generation.initialize().await?;
        self.coverage_analysis.initialize().await?;
        self.result_visualization.initialize().await?;
        Ok(())
    }

    /// 生成自动化测试用例
    pub async fn generate_tests(
        &self,
        code_path: &str,
    ) -> Result<test_generation::TestSuite, Box<dyn std::error::Error>> {
        self.test_generation
            .generate_tests(code_path)
            .await
            .map_err(|e| e.into())
    }

    /// 分析测试覆盖率
    pub async fn analyze_coverage(
        &self,
        config: coverage_analysis::CoverageAnalysisConfig,
    ) -> Result<coverage_analysis::CoverageAnalysisResult, Box<dyn std::error::Error>> {
        self.coverage_analysis.analyze_coverage(config).await
    }

    /// 可视化测试结果
    pub async fn visualize_results(
        &self,
        config: result_visualization::VisualizationConfig,
    ) -> Result<result_visualization::VisualizationResult, Box<dyn std::error::Error>> {
        self.result_visualization.visualize_results(config).await
    }
}
