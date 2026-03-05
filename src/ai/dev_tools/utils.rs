//! 工具函数模块
//! 提供AI开发工具的通用工具函数

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 通用消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// 通用AI请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// 通用AI响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// 响应选择
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

/// 响应使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 发送AI请求
pub async fn send_ai_request(
    endpoint: &str,
    api_key: &str,
    request: &AiRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(request)
        .send()
        .await?;

    let ai_response: AiResponse = response.json().await?;

    Ok(ai_response.choices[0].message.content.clone())
}

/// 验证API密钥
pub fn validate_api_key(api_key: &str) -> bool {
    !api_key.is_empty() && api_key.len() > 10
}

/// 验证API端点
pub fn validate_api_endpoint(endpoint: &str) -> bool {
    endpoint.starts_with("http://") || endpoint.starts_with("https://")
}
