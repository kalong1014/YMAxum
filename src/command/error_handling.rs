use std::fmt;

/// 命令解析错误
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// 语法错误
    SyntaxError {
        line: usize,
        message: String,
    },
    /// 缺少必要参数
    MissingParameter {
        line: usize,
        parameter: String,
        command: String,
    },
    /// 无效的参数值
    InvalidParameterValue {
        line: usize,
        parameter: String,
        value: String,
        message: String,
    },
    /// 未知命令
    UnknownCommand {
        line: usize,
        command: String,
    },
    /// 嵌套命令错误
    NestedCommandError {
        line: usize,
        message: String,
    },
    /// 变量插值错误
    VariableInterpolationError {
        line: usize,
        variable: String,
        message: String,
    },
    /// 文件读取错误
    FileReadError {
        path: String,
        message: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SyntaxError { line, message } => {
                write!(f, "Syntax error at line {}: {}", line, message)
            }
            ParseError::MissingParameter { line, parameter, command } => {
                write!(f, "Missing parameter '{}' for command '{}' at line {}", parameter, command, line)
            }
            ParseError::InvalidParameterValue { line, parameter, value, message } => {
                write!(f, "Invalid value '{}' for parameter '{}' at line {}: {}", value, parameter, line, message)
            }
            ParseError::UnknownCommand { line, command } => {
                write!(f, "Unknown command '{}' at line {}", command, line)
            }
            ParseError::NestedCommandError { line, message } => {
                write!(f, "Nested command error at line {}: {}", line, message)
            }
            ParseError::VariableInterpolationError { line, variable, message } => {
                write!(f, "Variable interpolation error for '{}' at line {}: {}", variable, line, message)
            }
            ParseError::FileReadError { path, message } => {
                write!(f, "Error reading file '{}': {}", path, message)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 命令执行错误
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    /// 命令执行失败
    CommandExecutionFailed {
        line: usize,
        command: String,
        message: String,
        error_code: u32,
    },
    /// 权限错误
    PermissionError {
        line: usize,
        command: String,
        message: String,
    },
    /// 资源错误
    ResourceError {
        line: usize,
        command: String,
        resource: String,
        message: String,
    },
    /// 超时错误
    TimeoutError {
        line: usize,
        command: String,
        timeout: u64,
    },
    /// 依赖错误
    DependencyError {
        line: usize,
        command: String,
        dependency: String,
        message: String,
    },
    /// 安全错误
    SecurityError {
        line: usize,
        command: String,
        message: String,
    },
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::CommandExecutionFailed { line, command, message, error_code } => {
                write!(f, "Command '{}' failed at line {} (code {}): {}", command, line, error_code, message)
            }
            ExecutionError::PermissionError { line, command, message } => {
                write!(f, "Permission denied for command '{}' at line {}: {}", command, line, message)
            }
            ExecutionError::ResourceError { line, command, resource, message } => {
                write!(f, "Resource '{}' error for command '{}' at line {}: {}", resource, command, line, message)
            }
            ExecutionError::TimeoutError { line, command, timeout } => {
                write!(f, "Command '{}' timed out after {}ms at line {}", command, timeout, line)
            }
            ExecutionError::DependencyError { line, command, dependency, message } => {
                write!(f, "Dependency '{}' error for command '{}' at line {}: {}", dependency, command, line, message)
            }
            ExecutionError::SecurityError { line, command, message } => {
                write!(f, "Security error for command '{}' at line {}: {}", command, line, message)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

/// 命令错误结果
#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    /// 解析错误
    ParseError(ParseError),
    /// 执行错误
    ExecutionError(ExecutionError),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::ParseError(err) => write!(f, "{}", err),
            CommandError::ExecutionError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for CommandError {}

/// 错误处理工具
pub struct ErrorHandler {
    // 错误日志
    errors: Vec<CommandError>,
    // 是否在遇到第一个错误时停止
    stop_on_first_error: bool,
}

impl ErrorHandler {
    /// 创建新的错误处理实例
    pub fn new(stop_on_first_error: bool) -> Self {
        Self {
            errors: Vec::new(),
            stop_on_first_error,
        }
    }

    /// 添加错误
    pub fn add_error(&mut self, error: CommandError) -> bool {
        self.errors.push(error);
        self.stop_on_first_error
    }

    /// 获取所有错误
    pub fn get_errors(&self) -> &Vec<CommandError> {
        &self.errors
    }

    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 清除错误
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// 生成错误报告
    pub fn generate_report(&self) -> String {
        if self.errors.is_empty() {
            return "No errors".to_string();
        }

        let mut report = format!("Found {} error{}", self.errors.len(), if self.errors.len() > 1 { "s" } else { "" });
        report.push_str("\n");

        for (i, error) in self.errors.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, error));
        }

        report
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let error = ParseError::SyntaxError {
            line: 10,
            message: "Invalid syntax".to_string(),
        };
        assert_eq!(format!("{}", error), "Syntax error at line 10: Invalid syntax");

        let error = ParseError::MissingParameter {
            line: 5,
            parameter: "NAME".to_string(),
            command: "PLUGIN INSTALL".to_string(),
        };
        assert_eq!(format!("{}", error), "Missing parameter 'NAME' for command 'PLUGIN INSTALL' at line 5");
    }

    #[test]
    fn test_execution_error_display() {
        let error = ExecutionError::CommandExecutionFailed {
            line: 20,
            command: "PLUGIN INSTALL".to_string(),
            message: "Plugin not found".to_string(),
            error_code: 404,
        };
        assert_eq!(format!("{}", error), "Command 'PLUGIN INSTALL' failed at line 20 (code 404): Plugin not found");

        let error = ExecutionError::PermissionError {
            line: 15,
            command: "CONFIG SET".to_string(),
            message: "No permission to modify config".to_string(),
        };
        assert_eq!(format!("{}", error), "Permission denied for command 'CONFIG SET' at line 15: No permission to modify config");
    }

    #[test]
    fn test_error_handler() {
        let mut handler = ErrorHandler::new(false);

        // 添加错误
        let error1 = CommandError::ParseError(ParseError::SyntaxError {
            line: 1,
            message: "Invalid syntax".to_string(),
        });
        handler.add_error(error1);

        let error2 = CommandError::ExecutionError(ExecutionError::CommandExecutionFailed {
            line: 2,
            command: "PLUGIN INSTALL".to_string(),
            message: "Plugin not found".to_string(),
            error_code: 404,
        });
        handler.add_error(error2);

        // 检查错误
        assert!(handler.has_errors());
        assert_eq!(handler.get_errors().len(), 2);

        // 生成报告
        let report = handler.generate_report();
        assert!(report.contains("Found 2 errors"));
        assert!(report.contains("Syntax error at line 1: Invalid syntax"));
        assert!(report.contains("Command 'PLUGIN INSTALL' failed at line 2 (code 404): Plugin not found"));

        // 清除错误
        handler.clear_errors();
        assert!(!handler.has_errors());
    }
}
