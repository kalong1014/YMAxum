use std::collections::HashSet;

use super::{TxtCommand, CommandParam};

/// 安全扫描结果
#[derive(Debug, Clone, PartialEq)]
pub struct SecurityScanResult {
    /// 是否安全
    pub is_safe: bool,
    /// 安全问题列表
    pub issues: Vec<SecurityIssue>,
}

/// 安全问题
#[derive(Debug, Clone, PartialEq)]
pub struct SecurityIssue {
    /// 问题类型
    pub issue_type: SecurityIssueType,
    /// 问题描述
    pub description: String,
    /// 相关命令
    pub command: String,
    /// 相关参数
    pub parameter: Option<String>,
    /// 严重程度
    pub severity: SecuritySeverity,
}

/// 安全问题类型
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityIssueType {
    /// SQL注入
    SqlInjection,
    /// 命令注入
    CommandInjection,
    /// 路径遍历
    PathTraversal,
    /// 跨站脚本
    Xss,
    /// 不安全的参数
    UnsafeParameter,
    /// 敏感信息泄露
    SensitiveInfoLeak,
    /// 权限提升
    PrivilegeEscalation,
}

/// 安全严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum SecuritySeverity {
    /// 低
    Low,
    /// 中
    Medium,
    /// 高
    High,
    /// 严重
    Critical,
}

/// 安全增强器
pub struct SecurityEnhancer {
    // 敏感参数名
    sensitive_parameters: HashSet<String>,
    // 危险命令模式
    dangerous_patterns: Vec<(String, SecurityIssueType)>,
    // 允许的路径前缀
    allowed_path_prefixes: Vec<String>,
}

impl SecurityEnhancer {
    /// 创建新的安全增强器实例
    pub fn new() -> Self {
        let mut sensitive_parameters = HashSet::new();
        // 添加敏感参数名
        sensitive_parameters.insert("password".to_string());
        sensitive_parameters.insert("token".to_string());
        sensitive_parameters.insert("api_key".to_string());
        sensitive_parameters.insert("secret".to_string());
        sensitive_parameters.insert("key".to_string());

        let dangerous_patterns = vec![
            (r"[';]\s*--".to_string(), SecurityIssueType::SqlInjection),
            (r"\b(SELECT|INSERT|UPDATE|DELETE|DROP|ALTER)\b".to_string(), SecurityIssueType::SqlInjection),
            (r"\b(system|exec|shell_exec|passthru|proc_open|popen)\b".to_string(), SecurityIssueType::CommandInjection),
            (r"\.\./".to_string(), SecurityIssueType::PathTraversal),
            (r"<script[^>]*>.*?</script>".to_string(), SecurityIssueType::Xss),
            (r"javascript:|data:".to_string(), SecurityIssueType::Xss),
        ];

        let allowed_path_prefixes = vec![
            "/".to_string(),
            "./".to_string(),
            "plugins/".to_string(),
            "config/".to_string(),
            "scripts/".to_string(),
        ];

        Self {
            sensitive_parameters,
            dangerous_patterns,
            allowed_path_prefixes,
        }
    }

    /// 扫描命令的安全性
    pub fn scan_command(&self, cmd: &TxtCommand) -> SecurityScanResult {
        let mut issues = Vec::new();

        // 构建命令字符串
        let command_str = self.build_command_string(cmd);

        // 检查命令类型
        self.check_command_type(cmd, &command_str, &mut issues);

        // 检查参数
        for param in &cmd.params {
            self.check_parameter(param, &command_str, &mut issues);
        }

        SecurityScanResult {
            is_safe: issues.is_empty(),
            issues,
        }
    }

    /// 构建命令字符串
    fn build_command_string(&self, cmd: &TxtCommand) -> String {
        let mut parts = vec![format!("{:?}", cmd.cmd_type), cmd.keyword.clone()];
        for param in &cmd.params {
            parts.push(format!("{}={}", param.key, param.value));
        }
        parts.join(" ")
    }

    /// 检查命令类型
    fn check_command_type(&self, cmd: &TxtCommand, command_str: &str, issues: &mut Vec<SecurityIssue>) {
        // 检查危险命令
        match cmd.cmd_type {
            super::CommandType::SCRIPT => {
                issues.push(SecurityIssue {
                    issue_type: SecurityIssueType::CommandInjection,
                    description: "Script commands can execute arbitrary code".to_string(),
                    command: command_str.to_string(),
                    parameter: None,
                    severity: SecuritySeverity::High,
                });
            }
            super::CommandType::DATABASE => {
                // 数据库命令需要特别检查
                self.check_database_command(cmd, command_str, issues);
            }
            _ => {}
        }
    }

    /// 检查数据库命令
    fn check_database_command(&self, cmd: &TxtCommand, command_str: &str, issues: &mut Vec<SecurityIssue>) {
        // 检查SQL注入
        for param in &cmd.params {
            if param.key.to_lowercase() == "table" || param.key.to_lowercase() == "condition" {
                for (pattern, issue_type) in &self.dangerous_patterns {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if regex.is_match(&param.value) {
                            issues.push(SecurityIssue {
                                issue_type: issue_type.clone(),
                                description: format!("Potential SQL injection in parameter '{}'", param.key),
                                command: command_str.to_string(),
                                parameter: Some(param.key.clone()),
                                severity: SecuritySeverity::Critical,
                            });
                        }
                    }
                }
            }
        }
    }

    /// 检查参数
    fn check_parameter(&self, param: &CommandParam, command_str: &str, issues: &mut Vec<SecurityIssue>) {
        // 检查敏感参数，排除CONFIG命令的KEY参数
        if self.sensitive_parameters.contains(&param.key.to_lowercase()) && !(command_str.starts_with("CONFIG") && param.key.to_lowercase() == "key") {
            issues.push(SecurityIssue {
                issue_type: SecurityIssueType::SensitiveInfoLeak,
                description: format!("Sensitive parameter '{}' may leak sensitive information", param.key),
                command: command_str.to_string(),
                parameter: Some(param.key.clone()),
                severity: SecuritySeverity::Medium,
            });
        }

        // 检查危险模式
        for (pattern, issue_type) in &self.dangerous_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(&param.value) {
                    issues.push(SecurityIssue {
                        issue_type: issue_type.clone(),
                        description: format!("Potential {} in parameter '{}'", self.issue_type_to_string(issue_type.clone()), param.key),
                        command: command_str.to_string(),
                        parameter: Some(param.key.clone()),
                        severity: self.get_severity(issue_type.clone()),
                    });
                }
            }
        }

        // 检查路径参数
        if param.key.to_lowercase().contains("path") || param.key.to_lowercase().contains("file") {
            self.check_path_parameter(param, command_str, issues);
        }
    }

    /// 检查路径参数
    fn check_path_parameter(&self, param: &CommandParam, command_str: &str, issues: &mut Vec<SecurityIssue>) {
        // 检查路径遍历
        if param.value.contains("../") {
            issues.push(SecurityIssue {
                issue_type: SecurityIssueType::PathTraversal,
                description: format!("Potential path traversal in parameter '{}'", param.key),
                command: command_str.to_string(),
                parameter: Some(param.key.clone()),
                severity: SecuritySeverity::High,
            });
        }

        // 检查路径前缀
        let mut is_allowed = false;
        for prefix in &self.allowed_path_prefixes {
            if param.value.starts_with(prefix) {
                is_allowed = true;
                break;
            }
        }

        if !is_allowed && !param.value.is_empty() {
            issues.push(SecurityIssue {
                issue_type: SecurityIssueType::PathTraversal,
                description: format!("Path in parameter '{}' is not in allowed prefixes", param.key),
                command: command_str.to_string(),
                parameter: Some(param.key.clone()),
                severity: SecuritySeverity::Medium,
            });
        }
    }

    /// 问题类型转字符串
    fn issue_type_to_string(&self, issue_type: SecurityIssueType) -> String {
        match issue_type {
            SecurityIssueType::SqlInjection => "SQL injection",
            SecurityIssueType::CommandInjection => "command injection",
            SecurityIssueType::PathTraversal => "path traversal",
            SecurityIssueType::Xss => "XSS",
            SecurityIssueType::UnsafeParameter => "unsafe parameter",
            SecurityIssueType::SensitiveInfoLeak => "sensitive info leak",
            SecurityIssueType::PrivilegeEscalation => "privilege escalation",
        }
        .to_string()
    }

    /// 获取严重程度
    fn get_severity(&self, issue_type: SecurityIssueType) -> SecuritySeverity {
        match issue_type {
            SecurityIssueType::SqlInjection => SecuritySeverity::Critical,
            SecurityIssueType::CommandInjection => SecuritySeverity::Critical,
            SecurityIssueType::PathTraversal => SecuritySeverity::High,
            SecurityIssueType::Xss => SecuritySeverity::Medium,
            SecurityIssueType::UnsafeParameter => SecuritySeverity::Low,
            SecurityIssueType::SensitiveInfoLeak => SecuritySeverity::Medium,
            SecurityIssueType::PrivilegeEscalation => SecuritySeverity::High,
        }
    }

    /// 清理参数值，防止注入攻击
    pub fn sanitize_parameter(&self, param: &mut CommandParam) {
        // 清理SQL注入
        if param.key.to_lowercase() == "table" || param.key.to_lowercase() == "condition" {
            // 只允许字母、数字和下划线
            param.value = param.value.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != ' ' && c != '=' && c != '<' && c != '>' && c != '!' && c != '(' && c != ')', "");
        }

        // 清理命令注入
        if param.key.to_lowercase() == "command" || param.key.to_lowercase() == "script" {
            // 移除危险函数
            let dangerous_functions = ["system", "exec", "shell_exec", "passthru", "proc_open", "popen"];
            for func in &dangerous_functions {
                param.value = param.value.replace(func, "");
            }
        }

        // 清理路径遍历
        if param.key.to_lowercase().contains("path") || param.key.to_lowercase().contains("file") {
            // 移除 ../
            while param.value.contains("../") {
                param.value = param.value.replace("../", "");
            }
        }

        // 清理XSS
        if param.key.to_lowercase().contains("html") || param.key.to_lowercase().contains("content") {
            // 转义HTML特殊字符
            param.value = param.value
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace(">", "&gt;")
                .replace("\"", "&quot;");
        }
    }
}

impl Default for SecurityEnhancer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_scan() {
        use crate::command::CommandType;
        let enhancer = SecurityEnhancer::new();

        // 测试SQL注入
        let cmd = TxtCommand {
            cmd_type: CommandType::DATABASE,
            keyword: "SELECT".to_string(),
            params: vec![
                CommandParam { key: "TABLE".to_string(), value: "users; DROP TABLE users".to_string() },
            ],
        };

        let result = enhancer.scan_command(&cmd);
        assert!(!result.is_safe);
        assert!(!result.issues.is_empty());

        // 测试路径遍历
        let cmd = TxtCommand {
            cmd_type: CommandType::PLUGIN,
            keyword: "INSTALL".to_string(),
            params: vec![
                CommandParam { key: "PATH".to_string(), value: "../../etc/passwd".to_string() },
            ],
        };

        let result = enhancer.scan_command(&cmd);
        assert!(!result.is_safe);
        assert!(!result.issues.is_empty());

        // 测试敏感信息
        let cmd = TxtCommand {
            cmd_type: CommandType::CONFIG,
            keyword: "SET".to_string(),
            params: vec![
                CommandParam { key: "API_KEY".to_string(), value: "secret123".to_string() },
            ],
        };

        let result = enhancer.scan_command(&cmd);
        assert!(!result.is_safe);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_sanitize_parameter() {
        let enhancer = SecurityEnhancer::new();

        // 测试SQL注入清理
        let mut param = CommandParam { key: "TABLE".to_string(), value: "users; DROP TABLE users".to_string() };
        enhancer.sanitize_parameter(&mut param);
        assert_eq!(param.value, "users DROP TABLE users");

        // 测试路径遍历清理
        let mut param = CommandParam { key: "PATH".to_string(), value: "../../etc/passwd".to_string() };
        enhancer.sanitize_parameter(&mut param);
        assert_eq!(param.value, "etc/passwd");

        // 测试XSS清理
        let mut param = CommandParam { key: "HTML".to_string(), value: "<script>alert('XSS')</script>".to_string() };
        enhancer.sanitize_parameter(&mut param);
        assert_eq!(param.value, "&lt;script&gt;alert('XSS')&lt;/script&gt;");
    }
}
