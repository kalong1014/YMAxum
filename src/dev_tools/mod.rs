//! 开发者工具链模块
//! 
//! 提供SDK、CLI工具、IDE插件等开发者工具

pub mod sdk;
pub mod cli;
pub mod ide_plugins;
pub mod code_generator;
pub mod documentation;

/// 开发者工具管理器
#[derive(Debug, Clone)]
pub struct DevToolsManager {
    sdk_manager: sdk::SdkManager,
    cli_manager: cli::CliManager,
    ide_plugins_manager: ide_plugins::IdePluginsManager,
    code_generator: code_generator::CodeGenerator,
    documentation_generator: documentation::DocumentationGenerator,
}

impl DevToolsManager {
    /// 创建新的开发者工具管理器
    pub fn new() -> Self {
        Self {
            sdk_manager: sdk::SdkManager::new(),
            cli_manager: cli::CliManager::new(),
            ide_plugins_manager: ide_plugins::IdePluginsManager::new(),
            code_generator: code_generator::CodeGenerator::new(),
            documentation_generator: documentation::DocumentationGenerator::new(),
        }
    }

    /// 初始化开发者工具
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.sdk_manager.initialize().await?;
        self.cli_manager.initialize().await?;
        self.ide_plugins_manager.initialize().await?;
        self.code_generator.initialize().await?;
        self.documentation_generator.initialize().await?;
        Ok(())
    }

    /// 获取SDK管理器
    pub fn get_sdk_manager(&self) -> &sdk::SdkManager {
        &self.sdk_manager
    }

    /// 获取CLI管理器
    pub fn get_cli_manager(&self) -> &cli::CliManager {
        &self.cli_manager
    }

    /// 获取IDE插件管理器
    pub fn get_ide_plugins_manager(&self) -> &ide_plugins::IdePluginsManager {
        &self.ide_plugins_manager
    }

    /// 获取代码生成器
    pub fn get_code_generator(&self) -> &code_generator::CodeGenerator {
        &self.code_generator
    }

    /// 获取文档生成器
    pub fn get_documentation_generator(&self) -> &documentation::DocumentationGenerator {
        &self.documentation_generator
    }
}