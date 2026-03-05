//! AI辅助开发工具模块
//! 提供代码生成、智能调试和文档生成功能

pub mod code_generator;
pub mod documentation_generator;
pub mod intelligent_debugger;
pub mod utils;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// AI开发工具配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiDevToolConfig {
    /// 是否启用AI辅助开发工具
    pub enabled: bool,
    /// API密钥
    pub api_key: String,
    /// API端点
    pub api_endpoint: String,
    /// 模型名称
    pub model_name: String,
    /// 代码生成配置
    pub code_generation: CodeGenerationConfig,
    /// 智能调试配置
    pub intelligent_debugging: IntelligentDebuggingConfig,
    /// 文档生成配置
    pub documentation: DocumentationConfig,
    /// 测试生成配置
    pub test_generation: TestGenerationConfig,
}

/// 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationConfig {
    /// 是否启用
    pub enabled: bool,
    /// 最大代码长度
    pub max_code_length: usize,
    /// 是否包含注释
    pub include_comments: bool,
    /// 是否进行代码格式化
    pub format_code: bool,
}

/// 智能调试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentDebuggingConfig {
    /// 是否启用
    pub enabled: bool,
    /// 是否分析堆栈跟踪
    pub analyze_stack_trace: bool,
    /// 是否提供修复建议
    pub provide_fix_suggestions: bool,
    /// 是否自动应用修复
    pub auto_apply_fixes: bool,
}

/// 文档生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// 是否启用
    pub enabled: bool,
    /// 文档格式
    pub format: String,
    /// 是否包含示例
    pub include_examples: bool,
    /// 是否生成API文档
    pub generate_api_docs: bool,
    /// 是否生成用户手册
    pub generate_user_manual: bool,
}

/// 测试生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationConfig {
    /// 是否启用
    pub enabled: bool,
    /// 是否使用AI辅助
    pub use_ai_assistance: bool,
    /// 是否生成单元测试
    pub generate_unit_tests: bool,
    /// 是否生成集成测试
    pub generate_integration_tests: bool,
    /// 是否生成端到端测试
    pub generate_e2e_tests: bool,
}

/// AI开发工具管理器
#[derive(Debug, Clone)]
pub struct AiDevToolManager {
    /// 配置
    _config: Arc<AiDevToolConfig>,
    /// 代码生成器
    code_generator: code_generator::CodeGenerator,
    /// 智能调试器
    intelligent_debugger: intelligent_debugger::IntelligentDebugger,
    /// 文档生成器
    documentation_generator: documentation_generator::DocumentationGenerator,
    /// 性能优化器
    performance_optimizer: super::performance_optimization::PerformanceOptimizer,
    /// 测试生成器
    test_generator: crate::testing::test_generation::TestGenerator,
}

impl AiDevToolManager {
    /// 创建新的AI开发工具管理器
    pub fn new(config: AiDevToolConfig) -> Self {
        let config_arc = Arc::new(config.clone());

        Self {
            _config: config_arc.clone(),
            code_generator: code_generator::CodeGenerator::new(config_arc.clone()),
            intelligent_debugger: intelligent_debugger::IntelligentDebugger::new(
                config_arc.clone(),
            ),
            documentation_generator: documentation_generator::DocumentationGenerator::new(
                config_arc.clone(),
            ),
            performance_optimizer: super::performance_optimization::PerformanceOptimizer::new(),
            test_generator: crate::testing::test_generation::TestGenerator::new(
                crate::testing::test_generation::TestGenerationConfig {
                    enabled: config.test_generation.enabled,
                    coverage_target: 0.8,
                    use_ai_assistance: config.test_generation.use_ai_assistance,
                    generate_unit_tests: config.test_generation.generate_unit_tests,
                    generate_integration_tests: config.test_generation.generate_integration_tests,
                    generate_e2e_tests: config.test_generation.generate_e2e_tests,
                    ai_model: "gpt-4".to_string(),
                    output_directory: "./tests/generated".to_string(),
                },
            ),
        }
    }

    /// 初始化AI开发工具
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化代码生成器
        self.code_generator.initialize().await?;

        // 初始化智能调试器
        self.intelligent_debugger.initialize().await?;

        // 初始化文档生成器
        self.documentation_generator.initialize().await?;

        // 初始化性能优化器
        self.performance_optimizer.initialize().await?;

        // 初始化测试生成器
        self.test_generator.initialize().await?;

        Ok(())
    }

    /// 获取代码生成器
    pub fn get_code_generator(&self) -> &code_generator::CodeGenerator {
        &self.code_generator
    }

    /// 获取智能调试器
    pub fn get_intelligent_debugger(&self) -> &intelligent_debugger::IntelligentDebugger {
        &self.intelligent_debugger
    }

    /// 获取文档生成器
    pub fn get_documentation_generator(&self) -> &documentation_generator::DocumentationGenerator {
        &self.documentation_generator
    }

    /// 获取性能优化器
    pub fn get_performance_optimizer(
        &self,
    ) -> &super::performance_optimization::PerformanceOptimizer {
        &self.performance_optimizer
    }

    /// 获取测试生成器
    pub fn get_test_generator(&self) -> &crate::testing::test_generation::TestGenerator {
        &self.test_generator
    }

    /// 生成性能优化建议
    pub async fn generate_performance_optimization_suggestions(
        &self,
        metrics: serde_json::Value,
    ) -> Result<super::performance_optimization::OptimizationResult, Box<dyn std::error::Error>>
    {
        self.performance_optimizer.optimize(metrics).await
    }

    /// 预测性能趋势
    pub async fn predict_performance_trend(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.performance_optimizer.predict_performance_trend().await
    }

    /// 生成测试
    pub async fn generate_tests(
        &self,
        code_path: &str,
    ) -> Result<crate::testing::test_generation::TestSuite, Box<dyn std::error::Error>> {
        self.test_generator
            .generate_tests(code_path)
            .await
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>)
    }

    /// 生成代码
    pub async fn generate_code(
        &self,
        prompt: &str,
        language: &str,
        context: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.code_generator
            .generate_code(prompt, language, context)
            .await
    }

    /// 智能调试
    pub async fn debug_code(
        &self,
        code: &str,
        error_message: &str,
        stack_trace: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.intelligent_debugger
            .debug_code(code, error_message, stack_trace)
            .await
    }

    /// 生成文档
    pub async fn generate_documentation(
        &self,
        code: &str,
        doc_type: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.documentation_generator
            .generate_documentation(code, doc_type, format)
            .await
    }
}

/// 默认配置
impl Default for AiDevToolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: "".to_string(),
            api_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            model_name: "gpt-4".to_string(),
            code_generation: CodeGenerationConfig::default(),
            intelligent_debugging: IntelligentDebuggingConfig::default(),
            documentation: DocumentationConfig::default(),
            test_generation: TestGenerationConfig::default(),
        }
    }
}

/// 测试生成默认配置
impl Default for TestGenerationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            use_ai_assistance: true,
            generate_unit_tests: true,
            generate_integration_tests: true,
            generate_e2e_tests: false,
        }
    }
}

/// 代码生成默认配置
impl Default for CodeGenerationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_code_length: 5000,
            include_comments: true,
            format_code: true,
        }
    }
}

/// 智能调试默认配置
impl Default for IntelligentDebuggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analyze_stack_trace: true,
            provide_fix_suggestions: true,
            auto_apply_fixes: false,
        }
    }
}

/// 文档生成默认配置
impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: "markdown".to_string(),
            include_examples: true,
            generate_api_docs: true,
            generate_user_manual: false,
        }
    }
}
