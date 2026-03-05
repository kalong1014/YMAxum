//! 语言类型定义
//! 提供语言枚举和相关功能

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 支持的语言枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Language {
    /// 中文
    #[serde(rename = "zh-CN")]
    #[default]
    ChineseSimplified,
    /// 英文
    #[serde(rename = "en-US")]
    EnglishUnitedStates,
    /// 日语
    #[serde(rename = "ja-JP")]
    Japanese,
    /// 韩语
    #[serde(rename = "ko-KR")]
    Korean,
    /// 法语
    #[serde(rename = "fr-FR")]
    French,
    /// 德语
    #[serde(rename = "de-DE")]
    German,
    /// 西班牙语
    #[serde(rename = "es-ES")]
    Spanish,
    /// 俄语
    #[serde(rename = "ru-RU")]
    Russian,
    /// 葡萄牙语
    #[serde(rename = "pt-BR")]
    PortugueseBrazil,
    /// 意大利语
    #[serde(rename = "it-IT")]
    Italian,
}

impl Language {
    /// 获取语言代码
    pub fn code(&self) -> &'static str {
        match self {
            Language::ChineseSimplified => "zh-CN",
            Language::EnglishUnitedStates => "en-US",
            Language::Japanese => "ja-JP",
            Language::Korean => "ko-KR",
            Language::French => "fr-FR",
            Language::German => "de-DE",
            Language::Spanish => "es-ES",
            Language::Russian => "ru-RU",
            Language::PortugueseBrazil => "pt-BR",
            Language::Italian => "it-IT",
        }
    }

    /// 获取语言名称
    pub fn name(&self) -> &'static str {
        match self {
            Language::ChineseSimplified => "中文",
            Language::EnglishUnitedStates => "English",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::Russian => "Русский",
            Language::PortugueseBrazil => "Português",
            Language::Italian => "Italiano",
        }
    }

    /// 获取所有支持的语言
    pub fn all() -> Vec<Self> {
        vec![
            Language::ChineseSimplified,
            Language::EnglishUnitedStates,
            Language::Japanese,
            Language::Korean,
            Language::French,
            Language::German,
            Language::Spanish,
            Language::Russian,
            Language::PortugueseBrazil,
            Language::Italian,
        ]
    }

    /// 检查语言是否支持
    pub fn is_supported(code: &str) -> bool {
        Self::from_str(code).is_ok()
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zh-CN" => Ok(Language::ChineseSimplified),
            "en-US" => Ok(Language::EnglishUnitedStates),
            "ja-JP" => Ok(Language::Japanese),
            "ko-KR" => Ok(Language::Korean),
            "fr-FR" => Ok(Language::French),
            "de-DE" => Ok(Language::German),
            "es-ES" => Ok(Language::Spanish),
            "ru-RU" => Ok(Language::Russian),
            "pt-BR" => Ok(Language::PortugueseBrazil),
            "it-IT" => Ok(Language::Italian),
            _ => Err(format!("不支持的语言代码: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code() {
        assert_eq!(Language::ChineseSimplified.code(), "zh-CN");
        assert_eq!(Language::EnglishUnitedStates.code(), "en-US");
        assert_eq!(Language::Japanese.code(), "ja-JP");
    }

    #[test]
    fn test_language_name() {
        assert_eq!(Language::ChineseSimplified.name(), "中文");
        assert_eq!(Language::EnglishUnitedStates.name(), "English");
        assert_eq!(Language::Japanese.name(), "日本語");
    }

    #[test]
    fn test_language_from_str() {
        assert_eq!(
            Language::from_str("zh-CN").unwrap(),
            Language::ChineseSimplified
        );
        assert_eq!(
            Language::from_str("en-US").unwrap(),
            Language::EnglishUnitedStates
        );
        assert_eq!(Language::from_str("ja-JP").unwrap(), Language::Japanese);
        assert!(Language::from_str("invalid").is_err());
    }

    #[test]
    fn test_language_is_supported() {
        assert!(Language::is_supported("zh-CN"));
        assert!(Language::is_supported("en-US"));
        assert!(!Language::is_supported("invalid"));
    }

    #[test]
    fn test_language_all() {
        let all_languages = Language::all();
        assert_eq!(all_languages.len(), 10);
    }

    #[test]
    fn test_language_default() {
        assert_eq!(Language::default(), Language::ChineseSimplified);
    }
}
