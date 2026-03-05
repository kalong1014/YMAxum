//! 测试生成命令
//! 用于自动化生成测试用例

use crate::testing::TestingFrameworkManager;
use clap::Parser;
use log::info;

/// 测试生成命令参数
#[derive(Parser, Debug)]
pub struct TestGeneratorCommand {
    /// 要分析的代码路径
    #[arg(short, long, default_value = "./src")]
    pub code_path: String,

    /// 测试输出目录
    #[arg(short, long, default_value = "./tests/generated")]
    pub output_dir: String,

    /// 测试覆盖率目标
    #[arg(short, long, default_value = "0.8")]
    pub coverage_target: f64,

    /// 是否生成单元测试
    #[arg(short, long, default_value = "true")]
    pub unit_tests: bool,

    /// 是否生成集成测试
    #[arg(short, long, default_value = "true")]
    pub integration_tests: bool,

    /// 是否生成端到端测试
    #[arg(short, long, default_value = "false")]
    pub e2e_tests: bool,

    /// 是否使用AI辅助生成
    #[arg(short, long, default_value = "false")]
    pub use_ai: bool,

    /// AI模型名称
    #[arg(short, long, default_value = "gpt-3.5-turbo")]
    pub ai_model: String,
}

impl TestGeneratorCommand {
    /// 执行测试生成命令
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始执行测试生成命令");
        info!("代码路径: {}", self.code_path);
        info!("输出目录: {}", self.output_dir);

        // 创建测试框架管理器
        let manager = TestingFrameworkManager::new();

        // 初始化测试框架
        info!("初始化测试框架...");
        manager.initialize().await?;

        // 生成测试
        info!("生成测试用例...");
        let test_suite = manager.generate_tests(&self.code_path).await?;

        info!("测试生成完成！");
        info!("生成了 {} 个测试用例", test_suite.test_cases.len());
        info!("测试套件名称: {}", test_suite.name);
        info!("测试套件类型: {}", test_suite.r#type);

        Ok(())
    }
}
