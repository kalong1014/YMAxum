//! 命令语法检查和关键词补全工具

use super::command_def::CommandType;

/// 语法检查结果
#[derive(Debug, PartialEq)]
pub enum SyntaxCheckResult {
    /// 有效
    Valid,
    /// 无效
    Invalid { line: usize, message: String },
}

/// 语法检查器
pub struct SyntaxChecker {
    /// 支持的命令类型
    _supported_commands: Vec<CommandType>,
}

impl SyntaxChecker {
    /// 创建新的语法检查器
    pub fn new() -> Self {
        Self {
            _supported_commands: vec![
                CommandType::INIT,
                CommandType::PLUGIN,
                CommandType::ROUTE,
                CommandType::RULE,
                CommandType::CONFIG,
                CommandType::ITERATE,
                CommandType::SERVICE,
            ],
        }
    }

    /// 检查命令语法
    pub fn check_syntax(&self, line: &str, line_num: usize) -> SyntaxCheckResult {
        let line = line.trim();

        // 跳过空行和注释行
        if line.is_empty() || line.starts_with('#') {
            return SyntaxCheckResult::Valid;
        }

        // 检查行尾反斜杠
        if line.ends_with('\\') {
            return SyntaxCheckResult::Invalid {
                line: line_num,
                message: "行尾不能有反斜杠".to_string(),
            };
        }

        // 拆分命令部分
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return SyntaxCheckResult::Valid;
        }

        // 检查命令类型
        let cmd_type = super::command_def::command_type_from_str(parts[0]);
        if let CommandType::Unknown(cmd) = cmd_type {
            return SyntaxCheckResult::Invalid {
                line: line_num,
                message: format!("未知命令: {}", cmd),
            };
        }

        // 检查参数格式
        for part in parts.iter().skip(2) {
            if !self.check_param_format(part) {
                return SyntaxCheckResult::Invalid {
                    line: line_num,
                    message: format!("参数格式错误: {}", part),
                };
            }
        }

        SyntaxCheckResult::Valid
    }

    /// 检查参数格式(key=value)
    fn check_param_format(&self, param: &str) -> bool {
        let parts: Vec<&str> = param.split('=').collect();
        parts.len() == 2 && !parts[0].is_empty()
    }
}

impl Default for SyntaxChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// 关键词补全器
pub struct KeywordCompleter {
    // 所有支持的关键词
    keywords: Vec<String>,
    // 命令类型对应的关键词映射
    cmd_keywords: std::collections::HashMap<CommandType, Vec<String>>,
}

impl KeywordCompleter {
    /// 创建新的关键词补全器
    pub fn new() -> Self {
        let mut cmd_keywords = std::collections::HashMap::new();

        // 初始化命令关键词
        cmd_keywords.insert(
            CommandType::INIT,
            vec!["PROJECT".to_string(), "ENV".to_string(), "DB".to_string()],
        );

        cmd_keywords.insert(
            CommandType::PLUGIN,
            vec![
                "INSTALL".to_string(),
                "UNINSTALL".to_string(),
                "ENABLE".to_string(),
                "DISABLE".to_string(),
                "LIST".to_string(),
                "UPDATE".to_string(),
            ],
        );

        cmd_keywords.insert(
            CommandType::ROUTE,
            vec![
                "ADD".to_string(),
                "DELETE".to_string(),
                "LIST".to_string(),
                "UPDATE".to_string(),
            ],
        );

        cmd_keywords.insert(
            CommandType::RULE,
            vec![
                "ADD".to_string(),
                "DELETE".to_string(),
                "LIST".to_string(),
                "UPDATE".to_string(),
            ],
        );

        cmd_keywords.insert(
            CommandType::CONFIG,
            vec![
                "SET".to_string(),
                "GET".to_string(),
                "LIST".to_string(),
                "RESET".to_string(),
            ],
        );

        cmd_keywords.insert(
            CommandType::ITERATE,
            vec![
                "INIT".to_string(),
                "BUILD".to_string(),
                "DEPLOY".to_string(),
                "TEST".to_string(),
            ],
        );

        cmd_keywords.insert(
            CommandType::SERVICE,
            vec![
                "START".to_string(),
                "STOP".to_string(),
                "RESTART".to_string(),
                "STATUS".to_string(),
            ],
        );

        Self {
            keywords: vec![
                "INIT".to_string(),
                "PLUGIN".to_string(),
                "ROUTE".to_string(),
                "RULE".to_string(),
                "CONFIG".to_string(),
                "ITERATE".to_string(),
                "SERVICE".to_string(),
            ],
            cmd_keywords,
        }
    }

    /// 获取建议关键词
    pub fn get_suggestions(&self, input: &str) -> Vec<String> {
        let input = input.trim().to_uppercase();

        if input.is_empty() {
            return self.keywords.clone();
        }

        // 过滤匹配的关键词
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 1 {
            // 命令补全
            return self
                .keywords
                .iter()
                .filter(|&kw| kw.starts_with(&input))
                .cloned()
                .collect();
        } else if parts.len() == 2 {
            // 关键词补全
            let cmd_type = super::command_def::command_type_from_str(parts[0]);
            if let Some(keywords) = self.cmd_keywords.get(&cmd_type) {
                return keywords
                    .iter()
                    .filter(|&kw| kw.starts_with(parts[1]))
                    .cloned()
                    .collect();
            }
        }

        Vec::new()
    }

    /// 获取命令关键词
    pub fn get_command_keywords(&self, cmd_type: &CommandType) -> Vec<String> {
        self.cmd_keywords.get(cmd_type).cloned().unwrap_or_default()
    }
}

impl Default for KeywordCompleter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_syntax() {
        let checker = SyntaxChecker::new();

        // 测试有效命令
        let result = checker.check_syntax("INIT PROJECT NAME=test TYPE=web", 1);
        assert_eq!(result, SyntaxCheckResult::Valid);

        // 测试未知命令
        let result = checker.check_syntax("UNKNOWN COMMAND", 2);
        assert_eq!(
            result,
            SyntaxCheckResult::Invalid {
                line: 2,
                message: "未知命令: UNKNOWN".to_string()
            }
        );

        // 测试参数错误
        let result = checker.check_syntax("INIT PROJECT NAME", 3);
        assert_eq!(
            result,
            SyntaxCheckResult::Invalid {
                line: 3,
                message: "参数格式错误: NAME".to_string()
            }
        );

        // 测试空行
        let result = checker.check_syntax("   ", 4);
        assert_eq!(result, SyntaxCheckResult::Valid);

        // 测试注释行
        let result = checker.check_syntax("# 这是一个注释", 5);
        assert_eq!(result, SyntaxCheckResult::Valid);
    }

    #[test]
    fn test_get_suggestions() {
        let completer = KeywordCompleter::new();

        // 测试获取所有命令
        let suggestions = completer.get_suggestions("");
        assert_eq!(suggestions.len(), 7);

        // 测试命令补全
        let suggestions = completer.get_suggestions("IN");
        assert!(suggestions.contains(&"INIT".to_string()));

        // 测试关键词补全
        let suggestions = completer.get_suggestions("INIT P");
        assert!(suggestions.contains(&"PROJECT".to_string()));

        let suggestions = completer.get_suggestions("PLUGIN I");
        assert!(suggestions.contains(&"INSTALL".to_string()));

        // 测试无效输入
        let suggestions = completer.get_suggestions("INVALID");
        assert!(suggestions.is_empty());
    }
}
