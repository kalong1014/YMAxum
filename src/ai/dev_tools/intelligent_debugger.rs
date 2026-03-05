//! 智能调试器模块
//! 提供基于AI的智能调试功能，分析错误信息和堆栈跟踪，并提供修复建议

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::AiDevToolConfig;

/// 智能调试请求
#[derive(Debug, Serialize)]
struct IntelligentDebugRequest {
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

/// 智能调试响应
#[derive(Debug, Deserialize)]
struct IntelligentDebugResponse {
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

/// 智能调试器
#[derive(Debug, Clone)]
pub struct IntelligentDebugger {
    /// 配置
    config: Arc<AiDevToolConfig>,
    /// HTTP客户端
    client: Client,
}

impl IntelligentDebugger {
    /// 创建新的智能调试器
    pub fn new(config: Arc<AiDevToolConfig>) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// 初始化智能调试器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("智能调试器初始化完成");
        Ok(())
    }

    /// 智能调试
    pub async fn debug_code(
        &self,
        code: &str,
        error_message: &str,
        stack_trace: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.intelligent_debugging.enabled {
            return Err("智能调试功能未启用".into());
        }

        info!("分析错误: {}", error_message);

        // 构建提示
        let prompt_message = self.build_prompt(code, error_message, stack_trace);

        // 构建请求
        let request = IntelligentDebugRequest {
            model: self.config.model_name.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "你是一个专业的调试专家，擅长分析代码错误并提供详细的修复建议。请分析错误信息和堆栈跟踪，找出问题所在，并提供具体的修复方案。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt_message,
                },
            ],
            max_tokens: 2000,
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
            .json::<IntelligentDebugResponse>()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 提取调试结果
        let debug_result = response_json.choices[0].message.content.trim().to_string();

        info!("智能调试完成");
        Ok(debug_result)
    }

    /// 构建提示
    fn build_prompt(&self, code: &str, error_message: &str, stack_trace: Option<&str>) -> String {
        let mut prompt_message = "# 智能调试请求\n".to_string();

        prompt_message.push_str(&format!("## 错误信息\n{}\n\n", error_message));

        if let Some(stack_trace) = stack_trace
            && self.config.intelligent_debugging.analyze_stack_trace
        {
            prompt_message.push_str(&format!("## 堆栈跟踪\n{}\n\n", stack_trace));
        }

        prompt_message.push_str(&format!("## 代码\n```rust\n{}\n```\n\n", code));

        prompt_message.push_str("## 要求\n");
        prompt_message.push_str("1. 详细分析错误原因\n");
        prompt_message.push_str("2. 提供具体的修复方案\n");
        prompt_message.push_str("3. 如果需要，提供修复后的代码\n");
        prompt_message.push_str("4. 解释修复原理\n");
        prompt_message.push_str("5. 提供预防类似错误的建议\n");

        prompt_message
    }

    /// 分析错误模式
    pub fn analyze_error_pattern(&self, error_message: &str) -> String {
        // 分析错误模式，识别常见错误类型
        let error_lower = error_message.to_lowercase();

        if error_lower.contains("panic") {
            "Rust panic 错误".to_string()
        } else if error_lower.contains("type") && error_lower.contains("mismatch") {
            "类型不匹配错误".to_string()
        } else if error_lower.contains("borrow") {
            "借用检查器错误".to_string()
        } else if error_lower.contains("overflow") {
            "数值溢出错误".to_string()
        } else if error_lower.contains("deadlock") {
            "死锁错误".to_string()
        } else if error_lower.contains("timeout") {
            "超时错误".to_string()
        } else if error_lower.contains("database") {
            "数据库错误".to_string()
        } else if error_lower.contains("network") {
            "网络错误".to_string()
        } else {
            "未知错误类型".to_string()
        }
    }

    /// 提供修复建议
    pub async fn provide_fix_suggestions(
        &self,
        code: &str,
        error_type: &str,
        error_message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.intelligent_debugging.provide_fix_suggestions {
            return Err("修复建议功能未启用".into());
        }

        info!("为{}提供修复建议", error_type);

        // 构建提示
        let prompt_message = format!(
            "# 修复建议请求\n\n## 错误类型\n{}\n\n## 错误信息\n{}\n\n## 代码\n```rust\n{}\n```\n\n## 要求\n1. 提供具体的修复方案\n2. 提供修复后的代码\n3. 解释修复原理\n4. 提供预防类似错误的建议\n",
            error_type, error_message, code
        );

        // 构建请求
        let request = IntelligentDebugRequest {
            model: self.config.model_name.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "你是一个专业的代码修复专家，擅长修复各种类型的代码错误。请提供具体、实用的修复方案。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt_message,
                },
            ],
            max_tokens: 1500,
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
            .json::<IntelligentDebugResponse>()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 提取修复建议
        let fix_suggestion = response_json.choices[0].message.content.trim().to_string();

        Ok(fix_suggestion)
    }

    /// 自动应用修复
    pub async fn auto_apply_fix(
        &self,
        code: &str,
        fix_suggestion: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.intelligent_debugging.auto_apply_fixes {
            return Err("自动应用修复功能未启用".into());
        }

        info!("自动应用修复");

        // 构建提示
        let prompt_message = format!(
            "# 自动应用修复请求\n\n## 原始代码\n```rust\n{}\n```\n\n## 修复建议\n{}\n\n## 要求\n1. 基于修复建议，生成修复后的完整代码\n2. 保持代码的原有结构和风格\n3. 只生成代码，不要包含其他解释\n",
            code, fix_suggestion
        );

        // 构建请求
        let request = IntelligentDebugRequest {
            model: self.config.model_name.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "你是一个专业的代码修复专家，擅长根据修复建议生成修复后的代码。请只生成修复后的完整代码，不要包含其他解释。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt_message,
                },
            ],
            max_tokens: 2000,
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
            .json::<IntelligentDebugResponse>()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 提取修复后的代码
        let fixed_code = response_json.choices[0].message.content.trim().to_string();

        // 处理代码（移除Markdown标记等）
        let processed_code = self.process_fixed_code(&fixed_code);

        Ok(processed_code)
    }

    /// 处理修复后的代码
    fn process_fixed_code(&self, code: &str) -> String {
        // 移除Markdown代码块标记
        let mut processed_code = code.trim().to_string();

        // 移除 ```rust 和 ``` 标记
        if processed_code.starts_with("```rust") {
            processed_code = processed_code[7..].trim().to_string();
        } else if processed_code.starts_with("```") {
            processed_code = processed_code[3..].trim().to_string();
        }

        if processed_code.ends_with("```") {
            processed_code = processed_code[..processed_code.len() - 3]
                .trim()
                .to_string();
        }

        processed_code
    }

    /// 生成调试报告
    pub fn generate_debug_report(
        &self,
        error_message: &str,
        stack_trace: Option<&str>,
        fix_suggestion: &str,
    ) -> String {
        let mut report = "# 智能调试报告\n\n".to_string();

        report.push_str(&format!("## 错误信息\n{}\n\n", error_message));

        if let Some(stack_trace) = stack_trace {
            report.push_str(&format!("## 堆栈跟踪\n```\n{}\n```\n\n", stack_trace));
        }

        report.push_str(&format!("## 修复建议\n{}\n\n", fix_suggestion));

        report.push_str("## 调试时间\n");
        report.push_str(&format!("{}\n", chrono::Local::now()));

        report
    }
}

use chrono;
