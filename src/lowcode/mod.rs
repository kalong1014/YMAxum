//! 低代码/无代码平台模块
//! 用于可视化开发工具、拖拽式界面设计、自动化代码生成

pub mod visual_development;
pub mod drag_drop_ui;
pub mod code_generation;

/// 低代码/无代码平台管理器
#[derive(Debug, Clone)]
pub struct LowCodePlatformManager {
    visual_development: visual_development::VisualDevelopmentTool,
    drag_drop_ui: drag_drop_ui::DragDropUIDesigner,
    code_generation: code_generation::CodeGenerator,
}

impl LowCodePlatformManager {
    /// 创建新的低代码/无代码平台管理器
    pub fn new() -> Self {
        Self {
            visual_development: visual_development::VisualDevelopmentTool::new(),
            drag_drop_ui: drag_drop_ui::DragDropUIDesigner::new(),
            code_generation: code_generation::CodeGenerator::new(),
        }
    }

    /// 初始化低代码/无代码平台
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.visual_development.initialize().await?;
        self.drag_drop_ui.initialize().await?;
        self.code_generation.initialize().await?;
        Ok(())
    }

    /// 创建可视化开发项目
    pub async fn create_project(&self, project_info: visual_development::ProjectInfo) -> Result<visual_development::ProjectCreationResult, Box<dyn std::error::Error>> {
        self.visual_development.create_project(project_info).await
    }

    /// 设计拖拽式界面
    pub async fn design_ui(&self, ui_design: drag_drop_ui::UIDesign) -> Result<drag_drop_ui::UIDesignResult, Box<dyn std::error::Error>> {
        self.drag_drop_ui.design_ui(ui_design).await
    }

    /// 生成自动化代码
    pub async fn generate_code(&self, code_request: code_generation::CodeGenerationRequest) -> Result<code_generation::CodeGenerationResult, Box<dyn std::error::Error>> {
        self.code_generation.generate_code(code_request).await
    }
}
