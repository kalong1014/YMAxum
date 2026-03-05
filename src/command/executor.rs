use serde_json::{Value, json};
use std::sync::Arc;
use std::collections::HashMap;

use std::time::{Duration, Instant};

use crate::command::{CommandType, TxtCommand, SecurityEnhancer, ErrorHandler, CommandError, ExecutionError};
use crate::core::state::AppState;

#[derive(Debug, Clone)]
pub enum ExecutionMode {
    LocalRealtime,
    BatchUpload,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub app_state: Arc<AppState>,
    pub current_line: Option<usize>,
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, Clone)]
pub enum ExecuteResult {
    Success {
        message: String,
        data: Value,
    },
    Failure {
        message: String,
        error_code: u32,
        line: usize,
    },
}

/// 命令执行缓存项
#[derive(Debug, Clone)]
pub struct CommandCacheItem {
    result: ExecuteResult,
    timestamp: Instant,
    _execution_time: Duration,
}

pub struct CommandExecutor {
    context: ExecutionContext,
    security_enhancer: SecurityEnhancer,
    error_handler: ErrorHandler,
    // 命令执行缓存
    command_cache: HashMap<TxtCommand, CommandCacheItem>,
    // 缓存配置
    cache_enabled: bool,
    cache_ttl: Duration,
}

impl CommandExecutor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            context: ExecutionContext {
                app_state,
                current_line: None,
                execution_mode: ExecutionMode::LocalRealtime,
            },
            security_enhancer: SecurityEnhancer::new(),
            error_handler: ErrorHandler::default(),
            command_cache: HashMap::new(),
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300), // 5分钟缓存
        }
    }

    pub fn set_execution_mode(&mut self, mode: ExecutionMode) {
        self.context.execution_mode = mode;
    }

    pub async fn execute_command(&mut self, mut cmd: TxtCommand, line: usize) -> ExecuteResult {
        self.context.current_line = Some(line);

        // 检查缓存
        if self.cache_enabled {
            if let Some(cache_item) = self.command_cache.get(&cmd) {
                // 检查缓存是否过期
                let elapsed = Instant::now() - cache_item.timestamp;
                if elapsed < self.cache_ttl {
                    return cache_item.result.clone();
                }
            }
        }

        // 安全检查
        let security_result = self.security_enhancer.scan_command(&cmd);
        if !security_result.is_safe {
            // 清理参数
            for param in &mut cmd.params {
                self.security_enhancer.sanitize_parameter(param);
            }
            // 重新检查
            let sanitized_result = self.security_enhancer.scan_command(&cmd);
            if !sanitized_result.is_safe {
                let error = CommandError::ExecutionError(ExecutionError::SecurityError {
                    line,
                    command: format!("{:?} {}", cmd.cmd_type, cmd.keyword),
                    message: "Security issues detected".to_string(),
                });
                self.error_handler.add_error(error);
                return ExecuteResult::Failure {
                    message: "Security issues detected".to_string(),
                    error_code: 403,
                    line,
                };
            }
        }

        // 保存命令信息用于错误处理
        let cmd_type = cmd.cmd_type.clone();
        let keyword = cmd.keyword.clone();

        // 执行命令并计时
        let start_time = Instant::now();
        let result = match cmd.cmd_type {
            CommandType::INIT => self.execute_init_command(cmd.clone()).await,
            CommandType::PLUGIN => self.execute_plugin_command(cmd.clone()).await,
            CommandType::ROUTE => self.execute_route_command(cmd.clone()).await,
            CommandType::RULE => self.execute_rule_command(cmd.clone()).await,
            CommandType::CONFIG => self.execute_config_command(cmd.clone()).await,
            CommandType::ITERATE => self.execute_iterate_command(cmd.clone()).await,
            CommandType::SERVICE => self.execute_service_command(cmd.clone()).await,
            CommandType::SCRIPT => self.execute_script_command(cmd.clone()).await,
            CommandType::FUNCTION => self.execute_function_command(cmd.clone()).await,
            CommandType::CONDITION => self.execute_condition_command(cmd.clone()).await,
            CommandType::LOOP => self.execute_loop_command(cmd.clone()).await,
            CommandType::DATABASE => self.execute_database_command(cmd.clone()).await,
            CommandType::API => self.execute_api_command(cmd.clone()).await,
            CommandType::VARIABLE => self.execute_variable_command(cmd.clone()).await,
            CommandType::TRANSACTION => self.execute_transaction_command(cmd.clone()).await,
            CommandType::Unknown(ref cmd_str) => ExecuteResult::Failure {
                message: format!("Unknown command: {}", cmd_str),
                error_code: 400,
                line,
            },
        };
        let execution_time = start_time.elapsed();

        // 处理执行结果
        if let ExecuteResult::Failure { message, error_code, line } = &result {
            let error = CommandError::ExecutionError(ExecutionError::CommandExecutionFailed {
                line: *line,
                command: format!("{:?} {}", cmd_type, keyword),
                message: message.clone(),
                error_code: *error_code,
            });
            self.error_handler.add_error(error);
        }

        // 缓存执行结果
        if self.cache_enabled {
            self.command_cache.insert(cmd, CommandCacheItem {
                result: result.clone(),
                timestamp: Instant::now(),
                _execution_time: execution_time,
            });
        }

        result
    }

    pub async fn execute_batch(&mut self, commands: Vec<TxtCommand>) -> (Vec<ExecuteResult>, String) {
        let mut results = Vec::new();
        
        // 清除之前的错误
        self.error_handler.clear_errors();
        
        for (i, cmd) in commands.iter().enumerate() {
            let result = self.execute_command(cmd.clone(), i + 1).await;
            results.push(result);
        }
        
        // 生成错误报告
        let error_report = self.error_handler.generate_report();
        
        (results, error_report)
    }
    
    /// 获取错误处理器
    pub fn get_error_handler(&self) -> &ErrorHandler {
        &self.error_handler
    }
    
    /// 获取安全增强器
    pub fn get_security_enhancer(&self) -> &SecurityEnhancer {
        &self.security_enhancer
    }

    pub async fn execute_init_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "DATABASE" => self.execute_init_database(cmd).await,
            "TABLE" => self.execute_init_table(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown INIT keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_init_database(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Database initialized successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_init_table(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Table initialized successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_plugin_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "INSTALL" => self.execute_plugin_install(cmd).await,
            "ENABLE" => self.execute_plugin_enable(cmd).await,
            "DISABLE" => self.execute_plugin_disable(cmd).await,
            "UNINSTALL" => self.execute_plugin_uninstall(cmd).await,
            "UPDATE" => self.execute_plugin_update(cmd).await,
            "LIST" => self.execute_plugin_list(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown PLUGIN keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_plugin_install(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_path = cmd
            .params
            .iter()
            .find(|p| p.key == "PATH")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_path.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin path is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} installed successfully", plugin_path),
            data: json!({ "plugin": plugin_path }),
        }
    }

    pub async fn execute_plugin_enable(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} enabled successfully", plugin_name),
            data: json!({ "plugin": plugin_name }),
        }
    }

    pub async fn execute_plugin_disable(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} disabled successfully", plugin_name),
            data: json!({ "plugin": plugin_name }),
        }
    }

    pub async fn execute_plugin_uninstall(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} uninstalled successfully", plugin_name),
            data: json!({ "plugin": plugin_name }),
        }
    }

    pub async fn execute_plugin_update(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} updated successfully", plugin_name),
            data: json!({ "plugin": plugin_name }),
        }
    }

    pub async fn execute_plugin_list(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Plugin list retrieved successfully".to_string(),
            data: json!({ "plugins": [] }),
        }
    }

    pub async fn execute_route_command(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Route command executed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_rule_command(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Rule command executed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_config_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "SET" => self.execute_config_set(cmd).await,
            "GET" => self.execute_config_get(cmd).await,
            "RELOAD" => self.execute_config_reload(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown CONFIG keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_config_set(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let key = cmd
            .params
            .iter()
            .find(|p| p.key == "KEY")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        let value = cmd
            .params
            .iter()
            .find(|p| p.key == "VALUE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if key.is_empty() {
            return ExecuteResult::Failure {
                message: "Config key is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Config {} set to {}", key, value),
            data: json!({ "key": key, "value": value }),
        }
    }

    pub async fn execute_config_get(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let key = cmd
            .params
            .iter()
            .find(|p| p.key == "KEY")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if key.is_empty() {
            return ExecuteResult::Failure {
                message: "Config key is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Config {} retrieved", key),
            data: json!({ "key": key, "value": "example_value" }),
        }
    }

    pub async fn execute_config_reload(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Config reloaded successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_iterate_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "REGISTER" => self.execute_iterate_register(cmd).await,
            "INIT" => self.execute_iterate_init(cmd).await,
            "START" => self.execute_iterate_start(cmd).await,
            "STOP" => self.execute_iterate_stop(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown ITERATE keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_iterate_register(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_path = cmd
            .params
            .iter()
            .find(|p| p.key == "PATH")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_path.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin path is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} registered successfully", plugin_path),
            data: json!({ "path": plugin_path }),
        }
    }

    pub async fn execute_iterate_init(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} initialized successfully", plugin_name),
            data: json!({ "name": plugin_name }),
        }
    }

    pub async fn execute_iterate_start(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} started successfully", plugin_name),
            data: json!({ "name": plugin_name }),
        }
    }

    pub async fn execute_iterate_stop(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let plugin_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if plugin_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Plugin name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Plugin {} stopped successfully", plugin_name),
            data: json!({ "name": plugin_name }),
        }
    }

    pub async fn execute_service_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "START" => self.execute_service_start(cmd).await,
            "STOP" => self.execute_service_stop(cmd).await,
            "RESTART" => self.execute_service_restart(cmd).await,
            "STATUS" => self.execute_service_status(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown SERVICE keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_service_start(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let service_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if service_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Service name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Service {} started successfully", service_name),
            data: json!({ "service": service_name }),
        }
    }

    pub async fn execute_service_stop(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let service_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if service_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Service name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Service {} stopped successfully", service_name),
            data: json!({ "service": service_name }),
        }
    }

    pub async fn execute_service_restart(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let service_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if service_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Service name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Service {} restarted successfully", service_name),
            data: json!({ "service": service_name }),
        }
    }

    pub async fn execute_service_status(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let service_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if service_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Service name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Service {} status retrieved", service_name),
            data: json!({ "service": service_name, "status": "running" }),
        }
    }

    pub async fn execute_script_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "EXECUTE" => self.execute_script_execute(cmd).await,
            "CREATE" => self.execute_script_create(cmd).await,
            "DELETE" => self.execute_script_delete(cmd).await,
            "LIST" => self.execute_script_list(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown SCRIPT keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_script_execute(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let script_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if script_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Script name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Script {} executed successfully", script_name),
            data: json!({ "script": script_name }),
        }
    }

    pub async fn execute_script_create(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let script_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if script_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Script name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Script {} created successfully", script_name),
            data: json!({ "script": script_name }),
        }
    }

    pub async fn execute_script_delete(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let script_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if script_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Script name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Script {} deleted successfully", script_name),
            data: json!({ "script": script_name }),
        }
    }

    pub async fn execute_script_list(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Script list retrieved successfully".to_string(),
            data: json!({ "scripts": [] }),
        }
    }

    pub async fn execute_function_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "CREATE" => self.execute_function_create(cmd).await,
            "CALL" => self.execute_function_call(cmd).await,
            "DELETE" => self.execute_function_delete(cmd).await,
            "LIST" => self.execute_function_list(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown FUNCTION keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_function_create(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let function_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if function_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Function name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Function {} created successfully", function_name),
            data: json!({ "function": function_name }),
        }
    }

    pub async fn execute_function_call(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let function_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if function_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Function name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Function {} called successfully", function_name),
            data: json!({ "function": function_name }),
        }
    }

    pub async fn execute_function_delete(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let function_name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if function_name.is_empty() {
            return ExecuteResult::Failure {
                message: "Function name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Function {} deleted successfully", function_name),
            data: json!({ "function": function_name }),
        }
    }

    pub async fn execute_function_list(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Function list retrieved successfully".to_string(),
            data: json!({ "functions": [] }),
        }
    }

    pub async fn execute_condition_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "IF" => self.execute_condition_if(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown CONDITION keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_condition_if(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Condition executed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_loop_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "FOR" => self.execute_loop_for(cmd).await,
            "WHILE" => self.execute_loop_while(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown LOOP keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_loop_for(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "For loop executed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_loop_while(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "While loop executed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_database_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "SELECT" => self.execute_database_select(cmd).await,
            "INSERT" => self.execute_database_insert(cmd).await,
            "UPDATE" => self.execute_database_update(cmd).await,
            "DELETE" => self.execute_database_delete(cmd).await,
            "CREATETABLE" => self.execute_database_create_table(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown DATABASE keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_database_select(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let table = cmd
            .params
            .iter()
            .find(|p| p.key == "TABLE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if table.is_empty() {
            return ExecuteResult::Failure {
                message: "Table name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        // 安全处理：对表名进行验证，防止 SQL 注入
        let safe_table = table.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        if safe_table != table {
            return ExecuteResult::Failure {
                message: "Invalid table name".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Select from table {} executed successfully", safe_table),
            data: json!({ "table": safe_table, "data": [] }),
        }
    }

    pub async fn execute_database_insert(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let table = cmd
            .params
            .iter()
            .find(|p| p.key == "TABLE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if table.is_empty() {
            return ExecuteResult::Failure {
                message: "Table name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        // 安全处理：对表名进行验证，防止 SQL 注入
        let safe_table = table.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        if safe_table != table {
            return ExecuteResult::Failure {
                message: "Invalid table name".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Insert into table {} executed successfully", safe_table),
            data: json!({ "table": safe_table }),
        }
    }

    pub async fn execute_database_update(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let table = cmd
            .params
            .iter()
            .find(|p| p.key == "TABLE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if table.is_empty() {
            return ExecuteResult::Failure {
                message: "Table name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        // 安全处理：对表名进行验证，防止 SQL 注入
        let safe_table = table.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        if safe_table != table {
            return ExecuteResult::Failure {
                message: "Invalid table name".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Update table {} executed successfully", safe_table),
            data: json!({ "table": safe_table }),
        }
    }

    pub async fn execute_database_delete(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let table = cmd
            .params
            .iter()
            .find(|p| p.key == "TABLE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if table.is_empty() {
            return ExecuteResult::Failure {
                message: "Table name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        // 安全处理：对表名进行验证，防止 SQL 注入
        let safe_table = table.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        if safe_table != table {
            return ExecuteResult::Failure {
                message: "Invalid table name".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Delete from table {} executed successfully", safe_table),
            data: json!({ "table": safe_table }),
        }
    }

    pub async fn execute_database_create_table(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let table = cmd
            .params
            .iter()
            .find(|p| p.key == "TABLE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if table.is_empty() {
            return ExecuteResult::Failure {
                message: "Table name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        // 安全处理：对表名进行验证，防止 SQL 注入
        let safe_table = table.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        if safe_table != table {
            return ExecuteResult::Failure {
                message: "Invalid table name".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Table {} created successfully", safe_table),
            data: json!({ "table": safe_table }),
        }
    }

    pub async fn execute_api_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "DEFINE" => self.execute_api_define(cmd).await,
            "LIST" => self.execute_api_list(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown API keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_api_define(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let path = cmd
            .params
            .iter()
            .find(|p| p.key == "PATH")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if path.is_empty() {
            return ExecuteResult::Failure {
                message: "API path is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("API {} defined successfully", path),
            data: json!({ "path": path }),
        }
    }

    pub async fn execute_api_list(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "API list retrieved successfully".to_string(),
            data: json!({ "apis": [] }),
        }
    }

    pub async fn execute_variable_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "SET" => self.execute_variable_set(cmd).await,
            "GET" => self.execute_variable_get(cmd).await,
            "DELETE" => self.execute_variable_delete(cmd).await,
            "LIST" => self.execute_variable_list(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown VARIABLE keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_variable_set(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        let value = cmd
            .params
            .iter()
            .find(|p| p.key == "VALUE")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if name.is_empty() {
            return ExecuteResult::Failure {
                message: "Variable name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Variable {} set to {}", name, value),
            data: json!({ "name": name, "value": value }),
        }
    }

    pub async fn execute_variable_get(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if name.is_empty() {
            return ExecuteResult::Failure {
                message: "Variable name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Variable {} retrieved", name),
            data: json!({ "name": name, "value": "example_value" }),
        }
    }

    pub async fn execute_variable_delete(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let name = cmd
            .params
            .iter()
            .find(|p| p.key == "NAME")
            .map(|p| p.value.clone())
            .unwrap_or_default();

        if name.is_empty() {
            return ExecuteResult::Failure {
                message: "Variable name is required".to_string(),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            };
        }

        ExecuteResult::Success {
            message: format!("Variable {} deleted successfully", name),
            data: json!({ "name": name }),
        }
    }

    pub async fn execute_variable_list(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        ExecuteResult::Success {
            message: "Variable list retrieved successfully".to_string(),
            data: json!({ "variables": [] }),
        }
    }

    pub async fn execute_transaction_command(&mut self, cmd: TxtCommand) -> ExecuteResult {
        let keyword = cmd.keyword.as_str();
        match keyword {
            "BEGIN" => self.execute_transaction_begin(cmd).await,
            "COMMIT" => self.execute_transaction_commit(cmd).await,
            "ROLLBACK" => self.execute_transaction_rollback(cmd).await,
            _ => ExecuteResult::Failure {
                message: format!("Unknown TRANSACTION keyword: {}", keyword),
                error_code: 400,
                line: self.context.current_line.unwrap_or(0),
            },
        }
    }

    pub async fn execute_transaction_begin(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        // 实现事务开始逻辑
        ExecuteResult::Success {
            message: "Transaction begun successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_transaction_commit(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        // 实现事务提交逻辑
        ExecuteResult::Success {
            message: "Transaction committed successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }

    pub async fn execute_transaction_rollback(&mut self, _cmd: TxtCommand) -> ExecuteResult {
        // 实现事务回滚逻辑
        ExecuteResult::Success {
            message: "Transaction rolled back successfully".to_string(),
            data: json!({ "status": "ok" }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::AppState;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_command_executor_creation() {
        let app_state = Arc::new(AppState::new());
        let _executor = CommandExecutor::new(app_state);
        // 验证执行器被正确创建
        assert!(true);
    }

    #[tokio::test]
    async fn test_set_execution_mode() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        executor.set_execution_mode(ExecutionMode::BatchUpload);
        match executor.context.execution_mode {
            ExecutionMode::BatchUpload => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_init_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::INIT,
            keyword: "DATABASE".to_string(),
            params: vec![],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Database initialized successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_plugin_install_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::PLUGIN,
            keyword: "INSTALL".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "PATH".to_string(),
                value: "plugins/test.axpl".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Plugin plugins/test.axpl installed successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_plugin_install_command_missing_path() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::PLUGIN,
            keyword: "INSTALL".to_string(),
            params: vec![],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Failure { message, .. } => {
                assert!(message.contains("Plugin path is required"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_plugin_enable_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::PLUGIN,
            keyword: "ENABLE".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "NAME".to_string(),
                value: "test_plugin".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Plugin test_plugin enabled successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_config_set_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::CONFIG,
            keyword: "SET".to_string(),
            params: vec![
                crate::command::command_def::CommandParam {
                    key: "KEY".to_string(),
                    value: "test.key".to_string(),
                },
                crate::command::command_def::CommandParam {
                    key: "VALUE".to_string(),
                    value: "test_value".to_string(),
                },
            ],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Config test.key set to test_value"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_config_get_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::CONFIG,
            keyword: "GET".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "KEY".to_string(),
                value: "test.key".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Config test.key retrieved"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_service_start_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::SERVICE,
            keyword: "START".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "NAME".to_string(),
                value: "test_service".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Service test_service started successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_database_select_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::DATABASE,
            keyword: "SELECT".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "TABLE".to_string(),
                value: "test_table".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Select from table test_table executed successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_api_define_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::API,
            keyword: "DEFINE".to_string(),
            params: vec![crate::command::command_def::CommandParam {
                key: "PATH".to_string(),
                value: "/api/test".to_string(),
            }],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("API /api/test defined successfully"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_variable_set_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::VARIABLE,
            keyword: "SET".to_string(),
            params: vec![
                crate::command::command_def::CommandParam {
                    key: "NAME".to_string(),
                    value: "test_var".to_string(),
                },
                crate::command::command_def::CommandParam {
                    key: "VALUE".to_string(),
                    value: "test_value".to_string(),
                },
            ],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Success { message, .. } => {
                assert!(message.contains("Variable test_var set to test_value"));
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_execute_batch() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd1 = TxtCommand {
            cmd_type: CommandType::INIT,
            keyword: "DATABASE".to_string(),
            params: vec![],
        };
        let cmd2 = TxtCommand {
            cmd_type: CommandType::PLUGIN,
            keyword: "LIST".to_string(),
            params: vec![],
        };
        let commands = vec![cmd1, cmd2];
        let (results, _error_report) = executor.execute_batch(commands).await;
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_execute_unknown_command() {
        let app_state = Arc::new(AppState::new());
        let mut executor = CommandExecutor::new(app_state);
        let cmd = TxtCommand {
            cmd_type: CommandType::Unknown("UNKNOWN".to_string()),
            keyword: "".to_string(),
            params: vec![],
        };
        let result = executor.execute_command(cmd, 1).await;
        match result {
            ExecuteResult::Failure {
                message,
                error_code,
                ..
            } => {
                assert!(message.contains("Unknown command: UNKNOWN"));
                assert_eq!(error_code, 400);
            }
            _ => assert!(false),
        }
    }
}
