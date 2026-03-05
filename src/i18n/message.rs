//! 消息处理模块
//! 提供消息定义和处理功能

use serde::{Deserialize, Serialize};

/// 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息键
    pub key: String,
    /// 消息内容
    pub value: String,
    /// 消息描述（可选）
    pub description: Option<String>,
    /// 消息参数（可选）
    pub parameters: Option<Vec<String>>,
}

impl Message {
    /// 创建新消息
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            description: None,
            parameters: None,
        }
    }

    /// 创建带描述的消息
    pub fn with_description(key: String, value: String, description: String) -> Self {
        Self {
            key,
            value,
            description: Some(description),
            parameters: None,
        }
    }

    /// 创建带参数的消息
    pub fn with_parameters(key: String, value: String, parameters: Vec<String>) -> Self {
        Self {
            key,
            value,
            description: None,
            parameters: Some(parameters),
        }
    }

    /// 创建完整消息
    pub fn with_all(
        key: String,
        value: String,
        description: String,
        parameters: Vec<String>,
    ) -> Self {
        Self {
            key,
            value,
            description: Some(description),
            parameters: Some(parameters),
        }
    }

    /// 格式化消息（替换参数）
    pub fn format(&self, args: &[&str]) -> String {
        let mut result = self.value.clone();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        result
    }

    /// 检查消息是否包含参数
    pub fn has_parameters(&self) -> bool {
        self.parameters.is_some() && !self.parameters.as_ref().unwrap().is_empty()
    }

    /// 获取参数数量
    pub fn parameter_count(&self) -> usize {
        self.parameters.as_ref().map(|p| p.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_new() {
        let message = Message::new("test.key".to_string(), "Test message".to_string());
        assert_eq!(message.key, "test.key");
        assert_eq!(message.value, "Test message");
        assert!(message.description.is_none());
        assert!(message.parameters.is_none());
    }

    #[test]
    fn test_message_with_description() {
        let message = Message::with_description(
            "test.key".to_string(),
            "Test message".to_string(),
            "Test description".to_string(),
        );
        assert_eq!(message.key, "test.key");
        assert_eq!(message.value, "Test message");
        assert_eq!(message.description, Some("Test description".to_string()));
        assert!(message.parameters.is_none());
    }

    #[test]
    fn test_message_with_parameters() {
        let message = Message::with_parameters(
            "test.key".to_string(),
            "Hello, {0}!".to_string(),
            vec!["name".to_string()],
        );
        assert_eq!(message.key, "test.key");
        assert_eq!(message.value, "Hello, {0}!");
        assert!(message.description.is_none());
        assert_eq!(message.parameters, Some(vec!["name".to_string()]));
    }

    #[test]
    fn test_message_format() {
        let message = Message::new(
            "test.key".to_string(),
            "Hello, {0}! You are {1} years old.".to_string(),
        );
        let formatted = message.format(&["John", "30"]);
        assert_eq!(formatted, "Hello, John! You are 30 years old.");
    }

    #[test]
    fn test_message_has_parameters() {
        let message1 = Message::new("test.key".to_string(), "Test message".to_string());
        assert!(!message1.has_parameters());

        let message2 = Message::with_parameters(
            "test.key".to_string(),
            "Hello, {0}!".to_string(),
            vec!["name".to_string()],
        );
        assert!(message2.has_parameters());
    }

    #[test]
    fn test_message_parameter_count() {
        let message1 = Message::new("test.key".to_string(), "Test message".to_string());
        assert_eq!(message1.parameter_count(), 0);

        let message2 = Message::with_parameters(
            "test.key".to_string(),
            "Hello, {0}! You are {1} years old.".to_string(),
            vec!["name".to_string(), "age".to_string()],
        );
        assert_eq!(message2.parameter_count(), 2);
    }
}
