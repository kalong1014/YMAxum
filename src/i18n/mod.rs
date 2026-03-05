//! 国际化支持模块
//! 提供多语言支持和国际化适配功能

mod language;
mod manager;
mod message;

pub use language::Language;
pub use manager::I18nManager;
pub use message::Message;

/// 国际化错误
#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    /// 语言包未找到
    #[error("语言包未找到: {0}")]
    LanguagePackNotFound(String),
    /// 消息未找到
    #[error("消息未找到: {0}")]
    MessageNotFound(String),
    /// 语言不支持
    #[error("语言不支持: {0}")]
    LanguageNotSupported(String),
    /// 语言包加载失败
    #[error("语言包加载失败: {0}")]
    LanguagePackLoadFailed(String),
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    /// IO错误
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    /// JSON错误
    #[error("JSON错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// 国际化结果类型
pub type I18nResult<T> = Result<T, I18nError>;
