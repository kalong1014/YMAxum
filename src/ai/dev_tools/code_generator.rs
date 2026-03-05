//! 代码生成器模块
//! 基于AI生成高质量、符合规范的代码

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::AiDevToolConfig;

/// 代码生成请求
#[derive(Debug, Serialize)]
struct CodeGenerationRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

/// 消息结构
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// 代码生成响应
#[derive(Debug, Deserialize)]
struct CodeGenerationResponse {
    _id: String,
    _object: String,
    _created: u64,
    _model: String,
    choices: Vec<Choice>,
    _usage: Usage,
}

/// 响应选择
#[derive(Debug, Deserialize)]
struct Choice {
    _index: u32,
    message: Message,
    _finish_reason: String,
}

/// 用法统计
#[derive(Debug, Deserialize)]
struct Usage {
    _prompt_tokens: u32,
    _completion_tokens: u32,
    _total_tokens: u32,
}

/// 代码生成器
#[derive(Debug, Clone)]
pub struct CodeGenerator {
    /// 配置
    config: Arc<AiDevToolConfig>,
    /// HTTP客户端
    client: Client,
}

impl CodeGenerator {
    /// 创建新的代码生成器
    pub fn new(config: Arc<AiDevToolConfig>) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// 初始化代码生成器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("代码生成器初始化完成");
        Ok(())
    }

    /// 生成代码
    pub async fn generate_code(
        &self,
        prompt: &str,
        language: &str,
        context: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.code_generation.enabled {
            return Err("代码生成功能未启用".into());
        }

        info!("生成{}代码，提示: {}", language, prompt);

        // 构建提示
        let prompt_message = self.build_prompt(prompt, language, context);

        // 构建请求
        let request = CodeGenerationRequest {
            model: self.config.model_name.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: format!(
                        "你是一个专业的{}开发者，生成高质量、符合规范的代码。代码应该包含适当的注释，并且格式良好。",
                        language
                    ),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt_message,
                },
            ],
            max_tokens: self.config.code_generation.max_code_length as u32,
            temperature: 0.7,
        };

        // 发送请求
        let response = self
            .client
            .post(&self.config.api_endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API请求失败: {}", e))?;

        // 解析响应
        let response_json = response
            .json::<CodeGenerationResponse>()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 提取生成的代码
        let generated_code = response_json.choices[0].message.content.trim().to_string();

        // 处理代码（移除Markdown标记等）
        let processed_code = self.process_generated_code(&generated_code, language);

        // 格式化代码
        let formatted_code = if self.config.code_generation.format_code {
            self.format_code(&processed_code, language).await?
        } else {
            processed_code
        };

        info!("代码生成完成，长度: {} 字符", formatted_code.len());
        Ok(formatted_code)
    }

    /// 构建提示
    fn build_prompt(&self, prompt: &str, language: &str, context: Option<&str>) -> String {
        let mut prompt_message = format!("语言: {}\n", language);

        if let Some(context) = context {
            prompt_message.push_str(&format!("上下文: {}\n", context));
        }

        prompt_message.push_str(&format!("要求: {}\n", prompt));

        if self.config.code_generation.include_comments {
            prompt_message.push_str("请包含详细的注释。\n");
        }

        prompt_message.push_str("请只生成代码，不要包含其他解释。");

        prompt_message
    }

    /// 处理生成的代码
    fn process_generated_code(&self, code: &str, language: &str) -> String {
        // 移除Markdown代码块标记
        let mut processed_code = code.trim().to_string();

        // 移除 ```language 和 ``` 标记
        let code_block_start = format!("```{}", language);
        if processed_code.starts_with(&code_block_start) {
            processed_code = processed_code[code_block_start.len()..].trim().to_string();
        }

        if processed_code.ends_with("```") {
            processed_code = processed_code[..processed_code.len() - 3]
                .trim()
                .to_string();
        }

        // 移除通用的 ``` 标记
        if processed_code.starts_with("```") {
            processed_code = processed_code[3..].trim().to_string();
        }

        if processed_code.ends_with("```") {
            processed_code = processed_code[..processed_code.len() - 3]
                .trim()
                .to_string();
        }

        processed_code
    }

    /// 格式化代码
    async fn format_code(
        &self,
        code: &str,
        language: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 根据语言选择格式化工具
        match language.to_lowercase().as_str() {
            "rust" => self.format_rust_code(code).await,
            "python" => self.format_python_code(code).await,
            "javascript" | "js" => self.format_javascript_code(code).await,
            "typescript" | "ts" => self.format_typescript_code(code).await,
            "java" => self.format_java_code(code).await,
            "c" | "cpp" | "c++" => self.format_cpp_code(code).await,
            "go" => self.format_go_code(code).await,
            "ruby" => self.format_ruby_code(code).await,
            "php" => self.format_php_code(code).await,
            "swift" => self.format_swift_code(code).await,
            "kotlin" => self.format_kotlin_code(code).await,
            _ => Ok(code.to_string()),
        }
    }

    /// 格式化Rust代码
    async fn format_rust_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成rustfmt
        Ok(code.to_string())
    }

    /// 格式化Python代码
    async fn format_python_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成black或autopep8
        Ok(code.to_string())
    }

    /// 格式化JavaScript代码
    async fn format_javascript_code(
        &self,
        code: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成prettier
        Ok(code.to_string())
    }

    /// 格式化TypeScript代码
    async fn format_typescript_code(
        &self,
        code: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成prettier
        Ok(code.to_string())
    }

    /// 格式化Java代码
    async fn format_java_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成google-java-format
        Ok(code.to_string())
    }

    /// 格式化C/C++代码
    async fn format_cpp_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成clang-format
        Ok(code.to_string())
    }

    /// 格式化Go代码
    async fn format_go_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成gofmt
        Ok(code.to_string())
    }

    /// 格式化Ruby代码
    async fn format_ruby_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成rubocop
        Ok(code.to_string())
    }

    /// 格式化PHP代码
    async fn format_php_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成php-cs-fixer
        Ok(code.to_string())
    }

    /// 格式化Swift代码
    async fn format_swift_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成swiftformat
        Ok(code.to_string())
    }

    /// 格式化Kotlin代码
    async fn format_kotlin_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 这里可以集成ktlint
        Ok(code.to_string())
    }
}
