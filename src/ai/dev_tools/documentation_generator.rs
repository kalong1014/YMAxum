//! 文档生成器模块
//! 基于AI生成代码文档、API文档和用户手册

use super::AiDevToolConfig;
use log::info;
use std::sync::Arc;

/// 文档生成器
#[derive(Debug, Clone)]
pub struct DocumentationGenerator {
    /// 配置
    config: Arc<AiDevToolConfig>,
}

impl DocumentationGenerator {
    /// 创建新的文档生成器
    pub fn new(config: Arc<AiDevToolConfig>) -> Self {
        Self { config }
    }

    /// 初始化文档生成器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.documentation.enabled {
            info!("初始化文档生成器");
        }
        Ok(())
    }

    /// 生成文档
    pub async fn generate_documentation(
        &self,
        code: &str,
        doc_type: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.documentation.enabled {
            return Err("文档生成功能未启用".into());
        }

        info!("生成{}类型的文档，格式: {:?}", doc_type, format);

        // 构建提示
        let prompt_message = self.build_prompt(code, doc_type, format);

        // 构建请求
        let request = DocumentationRequest {
            model: self.config.model_name.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "你是一个专业的技术文档撰写专家，擅长为各种代码和API生成清晰、详细、结构化的文档。请根据提供的代码和文档类型，生成高质量的文档。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt_message,
                },
            ],
            max_tokens: 3000,
            temperature: 0.7,
        };

        // 发送请求到AI API
        let response = self.send_request(&request).await?;

        Ok(response)
    }

    /// 构建提示
    fn build_prompt(&self, code: &str, doc_type: &str, format: Option<&str>) -> String {
        let format_str = format.unwrap_or(&self.config.documentation.format);

        format!(
            "请为以下代码生成{}类型的文档，格式为{}。文档应包含：\n1. 功能概述\n2. 详细说明\n3. 使用示例（如果适用）\n4. 注意事项（如果适用）\n\n代码：\n{}",
            doc_type, format_str, code
        )
    }

    /// 发送请求到AI API
    async fn send_request(
        &self,
        request: &DocumentationRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 这里应该实现实际的API调用
        // 为了演示，返回模拟响应
        Ok(format!(
            "# 文档生成结果\n\n## 功能概述\n这是为代码生成的{}文档。\n\n## 详细说明\n文档内容将基于提供的代码自动生成。\n\n## 使用示例\n```rust\n// 使用示例将根据代码自动生成\n```\n\n## 注意事项\n- 文档生成功能需要有效的API密钥\n- 生成的文档可能需要人工审核和调整",
            request
                .messages
                .last()
                .unwrap()
                .content
                .split(' ')
                .nth(2)
                .unwrap_or("")
        ))
    }
}

/// 文档生成请求
#[derive(Debug, Clone, serde::Serialize)]
struct DocumentationRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

/// 消息
#[derive(Debug, Clone, serde::Serialize)]
struct Message {
    role: String,
    content: String,
}
