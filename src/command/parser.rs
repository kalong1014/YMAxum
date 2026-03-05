use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use super::command_def::*;

/// TXT命令解析器
pub struct CommandParser {
    // 解析器配置
}

impl CommandParser {
    /// 创建新的解析器实例
    pub fn new() -> Self {
        Self {}
    }

    /// 从文件解析命令
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<TxtCommand>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut commands = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if let Some(cmd) = self.parse_line(&line, line_num + 1) {
                commands.push(cmd);
            }
        }

        Ok(commands)
    }

    /// 从字符串解析单行命令
    pub fn parse_line(&self, line: &str, _line_num: usize) -> Option<TxtCommand> {
        // 去除空白字符
        let line = line.trim();

        // 跳过空行和注释行（#开头）
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        // 处理换行符转义
        let line = line.replace('\\', "");

        // 拆分命令部分
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        // 提取指令类型
        let cmd_type = command_type_from_str(parts[0]);

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

    /// 解析单个参数
    fn parse_param<'a>(&self, param_str: &'a str) -> Option<(&'a str, &'a str)> {
        let parts: Vec<&str> = param_str.split('=').collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    }

    /// 解析条件判断
    pub fn parse_condition(&self, _lines: &[&str], _start_line: usize) -> Option<Condition> {
        // 简单实现，后续可扩展
        None
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 从字符串解析命令的便捷函数
pub fn parse_command(line: &str) -> Option<TxtCommand> {
    let parser = CommandParser::new();
    parser.parse_line(line, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parser = CommandParser::new();

        // 测试正常命令
        let line = "INIT PROJECT NAME=test_project TYPE=web";
        let cmd = parser.parse_line(line, 1).unwrap();
        assert_eq!(cmd.cmd_type, CommandType::INIT);
        assert_eq!(cmd.keyword, "PROJECT");
        assert_eq!(cmd.params.len(), 2);
        assert_eq!(cmd.params[0].key, "NAME");
        assert_eq!(cmd.params[0].value, "test_project");
        assert_eq!(cmd.params[1].key, "TYPE");
        assert_eq!(cmd.params[1].value, "web");

        // 测试注释行
        let line = "# 这是一个注释";
        assert!(parser.parse_line(line, 2).is_none());

        // 测试空行
        let line = "   ";
        assert!(parser.parse_line(line, 3).is_none());

        // 测试带转义的命令
        let line = "PLUGIN INSTALL NAME=test_plugin\\";
        let cmd = parser.parse_line(line, 4).unwrap();
        assert_eq!(cmd.cmd_type, CommandType::PLUGIN);
        assert_eq!(cmd.keyword, "INSTALL");
        assert_eq!(cmd.params.len(), 1);
        assert_eq!(cmd.params[0].key, "NAME");
        assert_eq!(cmd.params[0].value, "test_plugin");
    }

    #[test]
    fn test_parse_param() {
        let parser = CommandParser::new();

        // 测试正常参数
        let param = "KEY=VALUE";
        let (key, value) = parser.parse_param(param).unwrap();
        assert_eq!(key, "KEY");
        assert_eq!(value, "VALUE");

        // 测试没有值的参数
        let param = "KEY=";
        let (key, value) = parser.parse_param(param).unwrap();
        assert_eq!(key, "KEY");
        assert_eq!(value, "");

        // 测试无效参数
        let param = "KEY";
        assert!(parser.parse_param(param).is_none());
    }
}
