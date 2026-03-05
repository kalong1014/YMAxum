use crate::command::{executor::CommandExecutor, parser::parse_command, CommandType, TxtCommand};
use crate::core::state::AppState;
use std::sync::Arc;

#[tokio::test]
async fn test_command_executor_creation() {
    let app_state = Arc::new(AppState::new());
    let executor = CommandExecutor::new(app_state);
    assert!(!executor.context.app_state.is_none());
}

#[tokio::test]
async fn test_set_execution_mode() {
    let app_state = Arc::new(AppState::new());
    let mut executor = CommandExecutor::new(app_state);
    executor.set_execution_mode(crate::command::executor::ExecutionMode::BatchUpload);
    match executor.context.execution_mode {
        crate::command::executor::ExecutionMode::BatchUpload => assert!(true),
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Failure { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
        crate::command::executor::ExecuteResult::Success { message, .. } => {
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
    let (results, error_report) = executor.execute_batch(commands).await;
    assert_eq!(results.len(), 2);
    assert!(error_report.contains("No errors"));
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
        crate::command::executor::ExecuteResult::Failure { message, error_code, .. } => {
            assert!(message.contains("Unknown command: UNKNOWN"));
            assert_eq!(error_code, 400);
        }
        _ => assert!(false),
    }
}