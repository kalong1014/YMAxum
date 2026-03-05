//! CLI管理器模块
//! 
//! 提供命令行工具的定义、执行和帮助文档生成等功能

use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// CLI命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub arguments: Vec<CliArgument>,
    pub options: Vec<CliOption>,
    pub subcommands: Vec<CliCommand>,
    pub handler: Option<String>,
    pub examples: Vec<String>,
}

/// CLI参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_name: String,
}

/// CLI选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOption {
    pub short: Option<char>,
    pub long: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_name: Option<String>,
}

/// CLI执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliExecutionResult {
    pub command: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub exit_code: i32,
    pub duration_ms: u64,
}

/// CLI配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub commands: Vec<CliCommand>,
}

/// CLI管理器
#[derive(Debug, Clone)]
pub struct CliManager {
    config: CliConfig,
    commands: HashMap<String, CliCommand>,
}

impl CliManager {
    /// 创建新的CLI管理器
    pub fn new() -> Self {
        let config = CliConfig {
            name: "ymaxum-cli".to_string(),
            version: "1.0.0".to_string(),
            description: "YMAxum Command Line Interface".to_string(),
            author: "YMAxum Team".to_string(),
            homepage: Some("https://ymaxum.com".to_string()),
            repository: Some("https://github.com/ymaxum/ymaxum-cli".to_string()),
            license: Some("MIT".to_string()),
            commands: Vec::new(),
        };

        Self {
            config,
            commands: HashMap::new(),
        }
    }

    /// 初始化CLI管理器
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化CLI管理器
        self.register_default_commands().await?;
        // 注册扩展命令
        self.register_extended_commands().await?;
        Ok(())
    }

    /// 注册默认命令
    async fn register_default_commands(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 注册默认命令
        let commands = vec![
            CliCommand {
                name: "help".to_string(),
                description: "显示帮助信息".to_string(),
                usage: "help [command]".to_string(),
                arguments: vec![CliArgument {
                    name: "command".to_string(),
                    description: "命令名称".to_string(),
                    required: false,
                    default_value: None,
                    value_name: "COMMAND".to_string(),
                }],
                options: vec![],
                subcommands: vec![],
                handler: Some("help_handler".to_string()),
                examples: vec!["help", "help build"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
            },
            CliCommand {
                name: "version".to_string(),
                description: "显示版本信息".to_string(),
                usage: "version".to_string(),
                arguments: vec![],
                options: vec![],
                subcommands: vec![],
                handler: Some("version_handler".to_string()),
                examples: vec!["version"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
            },
            CliCommand {
                name: "build".to_string(),
                description: "构建项目".to_string(),
                usage: "build [options]".to_string(),
                arguments: vec![],
                options: vec![
                    CliOption {
                        short: Some('r'),
                        long: "release".to_string(),
                        description: "发布模式构建".to_string(),
                        required: false,
                        default_value: Some("false".to_string()),
                        value_name: None,
                    },
                    CliOption {
                        short: Some('t'),
                        long: "target".to_string(),
                        description: "目标平台".to_string(),
                        required: false,
                        default_value: None,
                        value_name: Some("TARGET".to_string()),
                    },
                ],
                subcommands: vec![],
                handler: Some("build_handler".to_string()),
                examples: vec!["build", "build --release"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
            },
            CliCommand {
                name: "test".to_string(),
                description: "运行测试".to_string(),
                usage: "test [options]".to_string(),
                arguments: vec![],
                options: vec![
                    CliOption {
                        short: Some('v'),
                        long: "verbose".to_string(),
                        description: "详细输出".to_string(),
                        required: false,
                        default_value: Some("false".to_string()),
                        value_name: None,
                    },
                    CliOption {
                        short: Some('p'),
                        long: "pattern".to_string(),
                        description: "测试模式".to_string(),
                        required: false,
                        default_value: None,
                        value_name: Some("PATTERN".to_string()),
                    },
                ],
                subcommands: vec![],
                handler: Some("test_handler".to_string()),
                examples: vec!["test", "test --verbose"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
            },
            CliCommand {
                name: "plugin".to_string(),
                description: "插件管理".to_string(),
                usage: "plugin [subcommand]".to_string(),
                arguments: vec![],
                options: vec![],
                subcommands: vec![
                    CliCommand {
                        name: "list".to_string(),
                        description: "列出所有插件".to_string(),
                        usage: "plugin list".to_string(),
                        arguments: vec![],
                        options: vec![],
                        subcommands: vec![],
                        handler: Some("plugin_list_handler".to_string()),
                        examples: vec!["plugin list"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                    },
                    CliCommand {
                        name: "install".to_string(),
                        description: "安装插件".to_string(),
                        usage: "plugin install <name>".to_string(),
                        arguments: vec![CliArgument {
                            name: "name".to_string(),
                            description: "插件名称".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "NAME".to_string(),
                        }],
                        options: vec![],
                        subcommands: vec![],
                        handler: Some("plugin_install_handler".to_string()),
                        examples: vec!["plugin install auth"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                    },
                    CliCommand {
                        name: "uninstall".to_string(),
                        description: "卸载插件".to_string(),
                        usage: "plugin uninstall <name>".to_string(),
                        arguments: vec![CliArgument {
                            name: "name".to_string(),
                            description: "插件名称".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "NAME".to_string(),
                        }],
                        options: vec![],
                        subcommands: vec![],
                        handler: Some("plugin_uninstall_handler".to_string()),
                        examples: vec!["plugin uninstall auth"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                    },
                ],
                handler: None,
                examples: vec!["plugin list", "plugin install auth"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
            },
        ];

        for command in commands {
            self.register_command(command).await?;
        }

        Ok(())
    }

    /// 注册命令
    pub async fn register_command(&mut self, command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.commands.insert(command.name.clone(), command);
        Ok(())
    }

    /// 执行命令
    pub async fn execute_command(
        &self,
        command_name: &str,
        args: Vec<String>,
        options: HashMap<String, String>,
    ) -> Result<CliExecutionResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 查找命令
        if let Some(command) = self.commands.get(command_name) {
            // 模拟命令执行
            let result = self.execute_command_handler(command, args, options).await?;
            let duration = start_time.elapsed().as_millis() as u64;

            Ok(CliExecutionResult {
                command: command_name.to_string(),
                success: result.success,
                output: result.output,
                error: result.error,
                exit_code: result.exit_code,
                duration_ms: duration,
            })
        } else {
            // 改进错误提示
            let mut similar_commands = Vec::new();
            for cmd_name in self.commands.keys() {
                if cmd_name.starts_with(command_name) || self.levenshtein_distance(command_name, cmd_name) <= 2 {
                    similar_commands.push(cmd_name);
                }
            }

            let mut error_msg = format!("Command not found: {}", command_name);
            if !similar_commands.is_empty() {
                error_msg.push_str("\nDid you mean:");
                for cmd in similar_commands {
                    error_msg.push_str(&format!("\n  {}", cmd));
                }
            }
            error_msg.push_str(&format!("\n\nUse '{} help' to see all available commands.", self.config.name));

            Err(error_msg.into())
        }
    }

    /// 计算字符串编辑距离（Levenshtein距离）
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        let mut matrix = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..=s1.len() {
            matrix[i][0] = i;
        }

        for j in 0..=s2.len() {
            matrix[0][j] = j;
        }

        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                let cost = if s1[i-1] == s2[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(matrix[i-1][j] + 1, matrix[i][j-1] + 1),
                    matrix[i-1][j-1] + cost
                );
            }
        }

        matrix[s1.len()][s2.len()]
    }

    /// 执行命令处理器
    async fn execute_command_handler(
        &self,
        command: &CliCommand,
        args: Vec<String>,
        options: HashMap<String, String>,
    ) -> Result<CliExecutionResult, Box<dyn std::error::Error>> {
        // 模拟命令执行
        match command.name.as_str() {
            "help" => {
                let output = if args.is_empty() {
                    self.generate_help_text().await
                } else {
                    self.generate_command_help_text(args[0].as_str()).await
                };
                Ok(CliExecutionResult {
                    command: command.name.clone(),
                    success: true,
                    output,
                    error: None,
                    exit_code: 0,
                    duration_ms: 0,
                })
            }
            "version" => {
                let output = format!("{} {}\n{}\nAuthor: {}", self.config.name, self.config.version, self.config.description, self.config.author);
                Ok(CliExecutionResult {
                    command: command.name.clone(),
                    success: true,
                    output,
                    error: None,
                    exit_code: 0,
                    duration_ms: 0,
                })
            }
            "build" => {
                let release = options.get("release").unwrap_or(&"false".to_string()) == "true";
                let target = options.get("target");

                let mut cmd = Command::new("cargo");
                cmd.arg("build");
                if release {
                    cmd.arg("--release");
                }
                if let Some(target) = target {
                    // 验证target参数，防止命令注入
                    if target.contains(' ') || target.contains('|') || target.contains('&') || target.contains(';') {
                        return Err("Invalid target parameter".into());
                    }
                    cmd.arg("--target").arg(target);
                }

                let output = cmd.output()?;
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                Ok(CliExecutionResult {
                    command: command.name.clone(),
                    success: output.status.success(),
                    output: stdout,
                    error: if stderr.is_empty() { None } else { Some(stderr) },
                    exit_code: output.status.code().unwrap_or(1),
                    duration_ms: 0,
                })
            }
            "test" => {
                let verbose = options.get("verbose").unwrap_or(&"false".to_string()) == "true";
                let pattern = options.get("pattern");

                let mut cmd = Command::new("cargo");
                cmd.arg("test");
                if verbose {
                    cmd.arg("--verbose");
                }
                if let Some(pattern) = pattern {
                    // 验证pattern参数，防止命令注入
                    if pattern.contains(' ') || pattern.contains('|') || pattern.contains('&') || pattern.contains(';') {
                        return Err("Invalid pattern parameter".into());
                    }
                    cmd.arg(pattern);
                }

                let output = cmd.output()?;
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                Ok(CliExecutionResult {
                    command: command.name.clone(),
                    success: output.status.success(),
                    output: stdout,
                    error: if stderr.is_empty() { None } else { Some(stderr) },
                    exit_code: output.status.code().unwrap_or(1),
                    duration_ms: 0,
                })
            }
            "code" => {
                if args.len() > 0 {
                    match args[0].as_str() {
                        "generate" => {
                            let template = args.get(1).ok_or("Template is required")?;
                            let output = args.get(2).ok_or("Output path is required")?;
                            let overwrite = options.get("overwrite").unwrap_or(&"false".to_string()) == "true";
                            let format = options.get("format").unwrap_or(&"false".to_string()) == "true";

                            let output = format!("Generating code from template '{}' to '{}'\nOverwrite: {} | Format: {}", template, output, overwrite, format);
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output,
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        "list" => {
                            let output = "Available templates:\n- rust-api: Rust API template\n- rust-model: Rust Model template\n- rust-service: Rust Service template\n- rust-middleware: Rust Middleware template\n- rust-config: Rust Config template\n- rust-test: Rust Test template\n- rust-controller: Rust Controller template\n- js-api: JavaScript API template\n- js-model: JavaScript Model template\n- python-api: Python API template\n- python-model: Python Model template\n- ts-api: TypeScript API template\n- ts-model: TypeScript Model template";
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: output.to_string(),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        _ => {
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: format!("Command {} executed successfully", command.name),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                    }
                } else {
                    Ok(CliExecutionResult {
                        command: command.name.clone(),
                        success: true,
                        output: format!("Command {} executed successfully", command.name),
                        error: None,
                        exit_code: 0,
                        duration_ms: 0,
                    })
                }
            }
            "doc" => {
                if args.len() > 0 {
                    match args[0].as_str() {
                        "generate" => {
                            let doc_id = args.get(1).ok_or("Documentation ID is required")?;
                            let output = args.get(2).ok_or("Output path is required")?;
                            let format = options.get("format").unwrap_or(&"markdown".to_string());
                            let overwrite = options.get("overwrite").unwrap_or(&"false".to_string()) == "true";

                            let output = format!("Generating documentation '{}' to '{}'\nFormat: {} | Overwrite: {}", doc_id, output, format, overwrite);
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output,
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        "list" => {
                            let output = "Available documentations:\n- api-docs: API Documentation\n- user-guide: User Guide\n- developer-guide: Developer Guide";
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: output.to_string(),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        _ => {
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: format!("Command {} executed successfully", command.name),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                    }
                } else {
                    Ok(CliExecutionResult {
                        command: command.name.clone(),
                        success: true,
                        output: format!("Command {} executed successfully", command.name),
                        error: None,
                        exit_code: 0,
                        duration_ms: 0,
                    })
                }
            }
            "dev" => {
                if args.len() > 0 {
                    match args[0].as_str() {
                        "serve" => {
                            let port = options.get("port").unwrap_or(&"3000".to_string());
                            let host = options.get("host").unwrap_or(&"localhost".to_string());

                            let output = format!("Starting development server at http://{}:{}\nPress Ctrl+C to stop", host, port);
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output,
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        "watch" => {
                            let output = "Watching files for changes...\nPress Ctrl+C to stop";
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: output.to_string(),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        _ => {
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: format!("Command {} executed successfully", command.name),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                    }
                } else {
                    Ok(CliExecutionResult {
                        command: command.name.clone(),
                        success: true,
                        output: format!("Command {} executed successfully", command.name),
                        error: None,
                        exit_code: 0,
                        duration_ms: 0,
                    })
                }
            }
            "deploy" => {
                if args.len() > 0 {
                    match args[0].as_str() {
                        "build" => {
                            let release = options.get("release").unwrap_or(&"false".to_string()) == "true";
                            let target = options.get("target");

                            let mut output = format!("Building project\nRelease mode: {}", release);
                            if let Some(target) = target {
                                output.push_str(&format!("\nTarget: {}", target));
                            }

                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output,
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        "publish" => {
                            let environment = options.get("environment").unwrap_or(&"production".to_string());

                            let output = format!("Publishing project to {} environment", environment);
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output,
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                        _ => {
                            Ok(CliExecutionResult {
                                command: command.name.clone(),
                                success: true,
                                output: format!("Command {} executed successfully", command.name),
                                error: None,
                                exit_code: 0,
                                duration_ms: 0,
                            })
                        }
                    }
                } else {
                    Ok(CliExecutionResult {
                        command: command.name.clone(),
                        success: true,
                        output: format!("Command {} executed successfully", command.name),
                        error: None,
                        exit_code: 0,
                        duration_ms: 0,
                    })
                }
            }
            _ => {
                Ok(CliExecutionResult {
                    command: command.name.clone(),
                    success: true,
                    output: format!("Command {} executed successfully", command.name),
                    error: None,
                    exit_code: 0,
                    duration_ms: 0,
                })
            }
        }
    }

    /// 生成帮助文本
    async fn generate_help_text(&self) -> String {
        let mut help_text = format!("{}\n{}\nVersion: {}\nAuthor: {}\n\nUsage:\n  {} [command] [arguments] [options]\n\nCommands:\n",
            self.config.name,
            self.config.description,
            self.config.version,
            self.config.author,
            self.config.name
        );

        for (name, command) in &self.commands {
            help_text.push_str(&format!("  {:12} {}\n", name, command.description));
        }

        help_text.push_str(&format!("\nUse '{} help <command>' to get help for a specific command.\n", self.config.name));
        help_text
    }

    /// 生成命令帮助文本
    async fn generate_command_help_text(&self, command_name: &str) -> String {
        if let Some(command) = self.commands.get(command_name) {
            let mut help_text = format!("Usage:\n  {} {}\n\nDescription:\n  {}\n",
                self.config.name,
                command.usage,
                command.description
            );

            if !command.arguments.is_empty() {
                help_text.push_str("\nArguments:\n");
                for arg in &command.arguments {
                    help_text.push_str(&format!("  {:12} {}\n", arg.name, arg.description));
                    if let Some(default) = &arg.default_value {
                        help_text.push_str(&format!("                  Default: {}\n", default));
                    }
                }
            }

            if !command.options.is_empty() {
                help_text.push_str("\nOptions:\n");
                for opt in &command.options {
                    let mut opt_text = "  ".to_string();
                    if let Some(short) = opt.short {
                        opt_text.push_str(&format!("-{short}, "));
                    } else {
                        opt_text.push_str("    ");
                    }
                    opt_text.push_str(&format!("--{}", opt.long));
                    if let Some(value_name) = &opt.value_name {
                        opt_text.push_str(&format!(" <{}}", value_name));
                    }
                    opt_text.push_str(&format!("{:12} {}", "", opt.description));
                    help_text.push_str(&format!("{}\n", opt_text));
                    if let Some(default) = &opt.default_value {
                        help_text.push_str(&format!("                  Default: {}\n", default));
                    }
                }
            }

            if !command.examples.is_empty() {
                help_text.push_str("\nExamples:\n");
                for example in &command.examples {
                    help_text.push_str(&format!("  {}\n", example));
                }
            }

            help_text
        } else {
            format!("Command not found: {}", command_name)
        }
    }

    /// 获取命令
    pub async fn get_command(&self, command_name: &str) -> Option<CliCommand> {
        self.commands.get(command_name).cloned()
    }

    /// 获取所有命令
    pub async fn get_all_commands(&self) -> Vec<CliCommand> {
        self.commands.values().cloned().collect()
    }

    /// 更新命令
    pub async fn update_command(&mut self, command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
        if self.commands.contains_key(&command.name) {
            self.commands.insert(command.name.clone(), command);
            Ok(())
        } else {
            Err(format!("Command not found: {}", command.name).into())
        }
    }

    /// 删除命令
    pub async fn delete_command(&mut self, command_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.commands.remove(command_name).is_some() {
            Ok(())
        } else {
            Err(format!("Command not found: {}", command_name).into())
        }
    }

    /// 生成命令行工具
    pub async fn generate_cli_tool(
        &self,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 生成命令行工具
        debug!("Generating CLI tool at {}", output_path);
        Ok(())
    }

    /// 注册扩展命令
    async fn register_extended_commands(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 注册代码生成命令
        let code_command = CliCommand {
            name: "code".to_string(),
            description: "代码生成相关命令".to_string(),
            usage: "code [subcommand]".to_string(),
            arguments: vec![],
            options: vec![],
            subcommands: vec![
                CliCommand {
                    name: "generate".to_string(),
                    description: "生成代码".to_string(),
                    usage: "code generate <template> <output> [options]".to_string(),
                    arguments: vec![
                        CliArgument {
                            name: "template".to_string(),
                            description: "模板ID".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "TEMPLATE".to_string(),
                        },
                        CliArgument {
                            name: "output".to_string(),
                            description: "输出路径".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "OUTPUT".to_string(),
                        },
                    ],
                    options: vec![
                        CliOption {
                            short: Some('o'),
                            long: "overwrite".to_string(),
                            description: "覆盖现有文件".to_string(),
                            required: false,
                            default_value: Some("false".to_string()),
                            value_name: None,
                        },
                        CliOption {
                            short: Some('f'),
                            long: "format".to_string(),
                            description: "格式化代码".to_string(),
                            required: false,
                            default_value: Some("false".to_string()),
                            value_name: None,
                        },
                    ],
                    subcommands: vec![],
                    handler: Some("code_generate_handler".to_string()),
                    examples: vec!["code generate rust-api ./src/api.rs", "code generate js-model ./src/models/User.js"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
                CliCommand {
                    name: "list".to_string(),
                    description: "列出所有模板".to_string(),
                    usage: "code list".to_string(),
                    arguments: vec![],
                    options: vec![],
                    subcommands: vec![],
                    handler: Some("code_list_handler".to_string()),
                    examples: vec!["code list"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
            ],
            handler: None,
            examples: vec!["code generate rust-api ./src/api.rs", "code list"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
        };

        // 注册文档生成命令
        let doc_command = CliCommand {
            name: "doc".to_string(),
            description: "文档生成相关命令".to_string(),
            usage: "doc [subcommand]".to_string(),
            arguments: vec![],
            options: vec![],
            subcommands: vec![
                CliCommand {
                    name: "generate".to_string(),
                    description: "生成文档".to_string(),
                    usage: "doc generate <doc_id> <output> [options]".to_string(),
                    arguments: vec![
                        CliArgument {
                            name: "doc_id".to_string(),
                            description: "文档ID".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "DOC_ID".to_string(),
                        },
                        CliArgument {
                            name: "output".to_string(),
                            description: "输出路径".to_string(),
                            required: true,
                            default_value: None,
                            value_name: "OUTPUT".to_string(),
                        },
                    ],
                    options: vec![
                        CliOption {
                            short: Some('f'),
                            long: "format".to_string(),
                            description: "文档格式 (markdown, html, json, yaml)".to_string(),
                            required: false,
                            default_value: Some("markdown".to_string()),
                            value_name: Some("FORMAT".to_string()),
                        },
                        CliOption {
                            short: Some('o'),
                            long: "overwrite".to_string(),
                            description: "覆盖现有文件".to_string(),
                            required: false,
                            default_value: Some("false".to_string()),
                            value_name: None,
                        },
                    ],
                    subcommands: vec![],
                    handler: Some("doc_generate_handler".to_string()),
                    examples: vec!["doc generate api-docs ./docs/api.md", "doc generate user-guide ./docs/user.html --format html"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
                CliCommand {
                    name: "list".to_string(),
                    description: "列出所有文档".to_string(),
                    usage: "doc list".to_string(),
                    arguments: vec![],
                    options: vec![],
                    subcommands: vec![],
                    handler: Some("doc_list_handler".to_string()),
                    examples: vec!["doc list"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
            ],
            handler: None,
            examples: vec!["doc generate api-docs ./docs/api.md", "doc list"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
        };

        // 注册开发命令
        let dev_command = CliCommand {
            name: "dev".to_string(),
            description: "开发相关命令".to_string(),
            usage: "dev [subcommand]".to_string(),
            arguments: vec![],
            options: vec![],
            subcommands: vec![
                CliCommand {
                    name: "serve".to_string(),
                    description: "启动开发服务器".to_string(),
                    usage: "dev serve [options]".to_string(),
                    arguments: vec![],
                    options: vec![
                        CliOption {
                            short: Some('p'),
                            long: "port".to_string(),
                            description: "服务器端口".to_string(),
                            required: false,
                            default_value: Some("3000".to_string()),
                            value_name: Some("PORT".to_string()),
                        },
                        CliOption {
                            short: Some('h'),
                            long: "host".to_string(),
                            description: "服务器主机".to_string(),
                            required: false,
                            default_value: Some("localhost".to_string()),
                            value_name: Some("HOST".to_string()),
                        },
                    ],
                    subcommands: vec![],
                    handler: Some("dev_serve_handler".to_string()),
                    examples: vec!["dev serve", "dev serve --port 8080"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
                CliCommand {
                    name: "watch".to_string(),
                    description: "监视文件变化".to_string(),
                    usage: "dev watch".to_string(),
                    arguments: vec![],
                    options: vec![],
                    subcommands: vec![],
                    handler: Some("dev_watch_handler".to_string()),
                    examples: vec!["dev watch"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
            ],
            handler: None,
            examples: vec!["dev serve", "dev watch"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
        };

        // 注册部署命令
        let deploy_command = CliCommand {
            name: "deploy".to_string(),
            description: "部署相关命令".to_string(),
            usage: "deploy [subcommand]".to_string(),
            arguments: vec![],
            options: vec![],
            subcommands: vec![
                CliCommand {
                    name: "build".to_string(),
                    description: "构建项目".to_string(),
                    usage: "deploy build [options]".to_string(),
                    arguments: vec![],
                    options: vec![
                        CliOption {
                            short: Some('r'),
                            long: "release".to_string(),
                            description: "发布模式构建".to_string(),
                            required: false,
                            default_value: Some("false".to_string()),
                            value_name: None,
                        },
                        CliOption {
                            short: Some('t'),
                            long: "target".to_string(),
                            description: "目标平台".to_string(),
                            required: false,
                            default_value: None,
                            value_name: Some("TARGET".to_string()),
                        },
                    ],
                    subcommands: vec![],
                    handler: Some("deploy_build_handler".to_string()),
                    examples: vec!["deploy build", "deploy build --release"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
                CliCommand {
                    name: "publish".to_string(),
                    description: "发布项目".to_string(),
                    usage: "deploy publish [options]".to_string(),
                    arguments: vec![],
                    options: vec![
                        CliOption {
                            short: Some('e'),
                            long: "environment".to_string(),
                            description: "环境 (production, staging, development)".to_string(),
                            required: false,
                            default_value: Some("production".to_string()),
                            value_name: Some("ENV".to_string()),
                        },
                    ],
                    subcommands: vec![],
                    handler: Some("deploy_publish_handler".to_string()),
                    examples: vec!["deploy publish", "deploy publish --environment staging"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
                },
            ],
            handler: None,
            examples: vec!["deploy build", "deploy publish"].into_iter().map(|s| format!("{} {}", self.config.name, s)).collect(),
        };

        // 注册所有扩展命令
        self.register_command(code_command).await?;
        self.register_command(doc_command).await?;
        self.register_command(dev_command).await?;
        self.register_command(deploy_command).await?;

        Ok(())
    }
}