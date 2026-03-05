use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

use super::command_def::*;

/// 增强的命令解析器
pub struct EnhancedCommandParser {
    // 解析器配置
    pub enable_nested_commands: bool,
    pub enable_complex_expressions: bool,
    pub enable_variable_interpolation: bool,
    // 变量环境
    pub variables: HashMap<String, String>,
}

impl EnhancedCommandParser {
    /// 创建新的解析器实例
    pub fn new() -> Self {
        Self {
            enable_nested_commands: true,
            enable_complex_expressions: true,
            enable_variable_interpolation: true,
            variables: HashMap::new(),
        }
    }

    /// 从文件解析命令
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<TxtCommand>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut commands = Vec::new();
        let mut current_command = String::new();
        let mut in_multiline = false;

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let trimmed_line = line.trim();

            // 处理多行命令
            if trimmed_line.ends_with('\\') {
                current_command.push_str(&trimmed_line[..trimmed_line.len() - 1]);
                current_command.push(' ');
                in_multiline = true;
            } else if in_multiline {
                current_command.push_str(trimmed_line);
                current_command.push(' ');
                if trimmed_line.ends_with(';') || trimmed_line.ends_with('}') {
                    if let Some(cmd) = self.parse_line(&current_command, line_num + 1) {
                        commands.push(cmd);
                    }
                    current_command.clear();
                    in_multiline = false;
                }
            } else {
                // 处理单行命令
                if let Some(cmd) = self.parse_line(&line, line_num + 1) {
                    commands.push(cmd);
                }
            }
        }

        // 处理最后一个多行命令
        if !current_command.is_empty() {
            if let Some(cmd) = self.parse_line(&current_command, 0) {
                commands.push(cmd);
            }
        }

        Ok(commands)
    }

    /// 从字符串解析单行命令
    pub fn parse_line(&self, line: &str, _line_num: usize) -> Option<TxtCommand> {
        // 去除空白字符
        let line = line.trim();

        // 跳过空行和注释行（#开头或/* */包围）
        if line.is_empty() || line.starts_with('#') || line.starts_with("/*") {
            return None;
        }

        // 处理注释
        let line = self.remove_comments(line);

        // 处理变量插值
        let line = if self.enable_variable_interpolation {
            self.interpolate_variables(&line)
        } else {
            line.to_string()
        };

        // 拆分命令部分
        let parts = self.tokenize_line(&line);
        if parts.is_empty() {
            return None;
        }

        // 提取指令类型
        let cmd_type = command_type_from_str(&parts[0]);

        // 提取关键词
        let keyword = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            "".to_string()
        };

        // 解析参数
        let mut params = Vec::new();
        for part in parts.iter().skip(2) {
            if let Some((key, value)) = self.parse_param(part) {
                params.push(CommandParam {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }

        Some(TxtCommand {
            cmd_type,
            keyword,
            params,
        })
    }

    /// 分词处理
    fn tokenize_line(&self, line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_quote = false;
        let mut _quote_char = '"';
        let mut in_bracket = 0;
        let mut in_parenthesis = 0;

        for c in line.chars() {
            match c {
                '"' | '\'' => {
                    in_quote = !in_quote;
                    if in_quote {
                        _quote_char = c;
                    }
                    current_token.push(c);
                }
                '[' => {
                    in_bracket += 1;
                    current_token.push(c);
                }
                ']' => {
                    in_bracket -= 1;
                    current_token.push(c);
                }
                '(' => {
                    in_parenthesis += 1;
                    current_token.push(c);
                }
                ')' => {
                    in_parenthesis -= 1;
                    current_token.push(c);
                }
                ' ' => {
                    if !in_quote && in_bracket == 0 && in_parenthesis == 0 {
                        if !current_token.is_empty() {
                            tokens.push(current_token);
                            current_token = String::new();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
                _ => {
                    current_token.push(c);
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        // 合并参数值中的空格
        let mut merged_tokens = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            if token.contains('=') {
                // 检查是否是参数
                let mut merged_token = token.to_string();
                i += 1;
                // 合并后续的tokens直到遇到下一个参数或结束
                while i < tokens.len() && !tokens[i].contains('=') {
                    merged_token.push(' ');
                    merged_token.push_str(&tokens[i]);
                    i += 1;
                }
                merged_tokens.push(merged_token);
            } else {
                // 命令或关键词
                merged_tokens.push(token.to_string());
                i += 1;
            }
        }

        merged_tokens
    }

    /// 解析单个参数
    fn parse_param(&self, param_str: &str) -> Option<(String, String)> {
        // 处理复杂参数格式
        if param_str.contains('=') {
            let parts: Vec<&str> = param_str.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                // 处理嵌套命令
                let value = if self.enable_nested_commands && value.starts_with('(') && value.ends_with(')') {
                    self.parse_nested_command(&value[1..value.len()-1])
                } else {
                    value
                };
                Some((key, value))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 解析嵌套命令
    fn parse_nested_command(&self, nested_str: &str) -> String {
        // 解析嵌套命令
        if let Some(cmd) = self.parse_line(nested_str, 0) {
            // 这里可以实现命令执行逻辑
            // 目前返回命令的字符串表示
            format!("{:?}", cmd)
        } else {
            nested_str.to_string()
        }
    }

    /// 处理变量插值
    fn interpolate_variables(&self, line: &str) -> String {
        // 实现变量插值，替换 ${var} 形式的变量
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();

        while i < chars.len() {
            let c = chars[i];
            if c == '$' && i + 1 < chars.len() && chars[i + 1] == '{' {
                // 找到变量开始
                i += 2;
                let mut var_name = String::new();
                while i < chars.len() && chars[i] != '}' {
                    var_name.push(chars[i]);
                    i += 1;
                }
                // 跳过 '}'
                if i < chars.len() {
                    i += 1;
                }
                // 从变量环境中查找变量
                if let Some(value) = self.variables.get(&var_name) {
                    result.push_str(value);
                } else {
                    // 如果变量不存在，保持原样
                    result.push_str(&format!("${{{}}}", var_name));
                }
            } else {
                result.push(c);
                i += 1;
            }
        }

        result
    }

    /// 移除注释
    fn remove_comments(&self, line: &str) -> String {
        // 移除行尾注释
        if let Some(pos) = line.find('#') {
            line[..pos].to_string()
        } else if let Some(start) = line.find("/*") {
            if let Some(_end) = line.find("*/") {
                line[..start].to_string()
            } else {
                line[..start].to_string()
            }
        } else {
            line.to_string()
        }
    }

    /// 解析条件判断
    pub fn parse_condition(&self, lines: &[&str], start_line: usize) -> Option<Condition> {
        // 增强的条件解析
        let mut condition = Condition {
            condition_type: ConditionType::NumericComparison {
                left: "".to_string(),
                operator: ComparisonOperator::Eq,
                right: "".to_string(),
            },
            then_commands: Vec::new(),
            else_commands: Vec::new(),
        };

        // 解析条件表达式
        if start_line < lines.len() {
            let condition_line = lines[start_line].trim();
            if condition_line.starts_with("IF") {
                // 解析条件表达式
                let expr = &condition_line[2..].trim();
                
                // 支持更复杂的条件表达式
                if let Some((left, rest)) = self.parse_expression(expr) {
                    if let Some((op, right)) = self.parse_operator(&rest) {
                        // 处理变量插值
                        let left = self.interpolate_variables(left);
                        let right = self.interpolate_variables(&right);
                        
                        condition.condition_type = ConditionType::NumericComparison {
                            left: left,
                            operator: op,
                            right: right,
                        };

                        // 解析 THEN 部分
                        let mut current_line = start_line + 1;
                        let mut in_then = false;
                        let mut in_else = false;
                        
                        while current_line < lines.len() {
                            let line = lines[current_line].trim();
                            
                            if line.starts_with("THEN") {
                                in_then = true;
                                in_else = false;
                                current_line += 1;
                                continue;
                            } else if line.starts_with("ELSE") {
                                in_then = false;
                                in_else = true;
                                current_line += 1;
                                continue;
                            } else if line.starts_with("ENDIF") {
                                break;
                            }
                            
                            if !line.is_empty() {
                                if in_then {
                                    if let Some(cmd) = self.parse_line(line, current_line + 1) {
                                        condition.then_commands.push(cmd);
                                    }
                                } else if in_else {
                                    if let Some(cmd) = self.parse_line(line, current_line + 1) {
                                        condition.else_commands.push(cmd);
                                    }
                                }
                            }
                            
                            current_line += 1;
                        }

                        return Some(condition);
                    }
                }
            }
        }

        None
    }

    /// 解析表达式
    fn parse_expression<'a>(&self, expr: &'a str) -> Option<(&'a str, String)> {
        // 简单实现表达式解析
        let parts: Vec<&'a str> = expr.split_whitespace().collect();
        if parts.len() >= 3 {
            Some((parts[0], parts[1..].join(" ")))
        } else {
            None
        }
    }

    /// 解析运算符
    fn parse_operator(&self, expr: &str) -> Option<(ComparisonOperator, String)> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Some(op) = comparison_operator_from_str(parts[0]) {
                Some((op, parts[1].to_string()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for EnhancedCommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_with_complex_syntax() {
        let mut parser = EnhancedCommandParser::new();
        // 设置变量
        parser.variables.insert("plugin_name".to_string(), "test_plugin".to_string());

        // 测试带变量插值的命令
        let line = "PLUGIN INSTALL NAME=${plugin_name} VERSION=1.0";
        let cmd = parser.parse_line(line, 1).unwrap();
        assert_eq!(cmd.cmd_type, CommandType::PLUGIN);
        assert_eq!(cmd.keyword, "INSTALL");
        assert_eq!(cmd.params.len(), 2);
        assert_eq!(cmd.params[0].key, "NAME");
        assert_eq!(cmd.params[0].value, "test_plugin");
        assert_eq!(cmd.params[1].key, "VERSION");
        assert_eq!(cmd.params[1].value, "1.0");

        // 测试带嵌套命令的参数
        let line = "VARIABLE SET NAME=result VALUE=(PLUGIN LIST)";
        let cmd = parser.parse_line(line, 2).unwrap();
        assert_eq!(cmd.cmd_type, CommandType::VARIABLE);
        assert_eq!(cmd.keyword, "SET");
        assert_eq!(cmd.params.len(), 2);
        assert_eq!(cmd.params[0].key, "NAME");
        assert_eq!(cmd.params[0].value, "result");
        assert_eq!(cmd.params[1].key, "VALUE");
        // 嵌套命令的解析结果应该是一个有效的命令表示
        assert!(!cmd.params[1].value.is_empty());

        // 测试带复杂参数的命令
        let line = "CONFIG SET KEY=api.endpoints VALUE=[/api/v1, /api/v2]";
        let cmd = parser.parse_line(line, 3).unwrap();
        assert_eq!(cmd.cmd_type, CommandType::CONFIG);
        assert_eq!(cmd.keyword, "SET");
        assert_eq!(cmd.params.len(), 2);
        assert_eq!(cmd.params[0].key, "KEY");
        assert_eq!(cmd.params[0].value, "api.endpoints");
        assert_eq!(cmd.params[1].key, "VALUE");
        assert_eq!(cmd.params[1].value, "[/api/v1, /api/v2]");
    }

    #[test]
    fn test_parse_condition() {
        let parser = EnhancedCommandParser::new();
        let lines = [
            "IF x == 5",
            "THEN",
            "  PLUGIN ENABLE NAME=test",
            "ELSE",
            "  PLUGIN DISABLE NAME=test",
            "ENDIF"
        ];

        let condition = parser.parse_condition(&lines, 0).unwrap();
        match condition.condition_type {
            ConditionType::NumericComparison { left, operator, right } => {
                assert_eq!(left, "x");
                assert_eq!(operator, ComparisonOperator::Eq);
                assert_eq!(right, "5");
            }
            _ => assert!(false),
        }
        assert_eq!(condition.then_commands.len(), 1);
        assert_eq!(condition.else_commands.len(), 1);
    }

    #[test]
    fn test_tokenize_line() {
        let parser = EnhancedCommandParser::new();

        // 测试带空格的参数
        let line = "PLUGIN INSTALL NAME=test plugin VERSION=1.0";
        let tokens = parser.tokenize_line(line);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], "PLUGIN");
        assert_eq!(tokens[1], "INSTALL");
        assert_eq!(tokens[2], "NAME=test plugin");
        assert_eq!(tokens[3], "VERSION=1.0");

        // 测试带引号的参数
        let line = "CONFIG SET KEY=message VALUE=\"hello world\"";
        let tokens = parser.tokenize_line(line);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], "CONFIG");
        assert_eq!(tokens[1], "SET");
        assert_eq!(tokens[2], "KEY=message");
        assert_eq!(tokens[3], "VALUE=\"hello world\"");

        // 测试带括号的参数
        let line = "VARIABLE SET NAME=result VALUE=(1 + 2)";
        let tokens = parser.tokenize_line(line);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], "VARIABLE");
        assert_eq!(tokens[1], "SET");
        assert_eq!(tokens[2], "NAME=result");
        assert_eq!(tokens[3], "VALUE=(1 + 2)");
    }
}
