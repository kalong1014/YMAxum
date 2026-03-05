//! 示例生成器
//! 用于生成各种示例代码

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

/// 示例生成器
pub struct ExampleGenerator;

impl ExampleGenerator {
    /// 生成示例代码
    pub fn generate_examples(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;

        // 生成基本示例
        Self::generate_basic_example(output_dir)?;
        // 生成插件示例
        Self::generate_plugin_example(output_dir)?;
        // 生成场景示例
        Self::generate_scene_example(output_dir)?;
        // 生成命令示例
        Self::generate_command_example(output_dir)?;

        Ok(())
    }

    /// 生成基本示例
    fn generate_basic_example(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example = r#"
use ymaxum::core::state::AppState;
use ymaxum::core::route::create_router;
use ymaxum::core::middleware::create_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用状态
    let state = AppState::new().await?;

    // 创建中间件
    let middleware = create_middleware();

    // 创建路由
    let router = create_router(state);

    // 启动服务器
    let addr = "127.0.0.1:3000".parse()?;
    println!("Server running on https://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
        "#;

        let output_path = output_dir.join("basic_example.rs");
        let mut file = File::create(output_path)?;
        file.write_all(example.as_bytes())?;

        Ok(())
    }

    /// 生成插件示例
    fn generate_plugin_example(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example = r#"
use ymaxum::plugin::manager::{PluginInfo, PluginManager, PluginStatus};
use ymaxum::plugin::format::PluginManifest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化插件管理器
    let mut plugin_manager = PluginManager::new();

    // 安装插件
    let plugin_path = "plugins/output/customer_service.axpl";
    plugin_manager.install_plugin(plugin_path).await?;

    // 启用插件
    plugin_manager.enable_plugin("customer_service").await?;

    // 获取插件列表
    let plugins = plugin_manager.list_plugins().await?;
    println!("Installed plugins: {:?}", plugins);

    // 停用插件
    plugin_manager.disable_plugin("customer_service").await?;

    // 卸载插件
    plugin_manager.uninstall_plugin("customer_service").await?;

    Ok(())
}
        "#;

        let output_path = output_dir.join("plugin_example.rs");
        let mut file = File::create(output_path)?;
        file.write_all(example.as_bytes())?;

        Ok(())
    }

    /// 生成场景示例
    fn generate_scene_example(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example = r#"
use ymaxum::scene::{SceneAdapter, SceneManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化场景管理器
    let mut scene_manager = SceneManager::new();

    // 注册场景适配器
    // 这里需要实现具体的场景适配器
    // scene_manager.register(Box::new(NewbieSceneAdapter::new()));
    // scene_manager.register(Box::new(GameSceneAdapter::new()));
    // scene_manager.register(Box::new(MallSceneAdapter::new()));
    // scene_manager.register(Box::new(SaasSceneAdapter::new()));

    // 初始化所有场景
    scene_manager.init_all()?;

    // 启动所有场景
    scene_manager.start_all()?;

    // 获取场景适配器
    if let Some(adapter) = scene_manager.get_adapter("newbie") {
        println!("Found scene adapter: {}", adapter.name());
    }

    // 停止所有场景
    scene_manager.stop_all()?;

    Ok(())
}
        "#;

        let output_path = output_dir.join("scene_example.rs");
        let mut file = File::create(output_path)?;
        file.write_all(example.as_bytes())?;

        Ok(())
    }

    /// 生成命令示例
    fn generate_command_example(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example = r#"
use ymaxum::command::parser::CommandParser;
use ymaxum::command::executor::CommandExecutor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化命令解析器
    let parser = CommandParser::new();

    // 解析命令
    let command = parser.parse("INIT PROJECT NAME=my_project")?;
    println!("Parsed command: {:?}", command);

    // 初始化命令执行器
    let mut executor = CommandExecutor::new();

    // 执行命令
    let result = executor.execute(&command).await?;
    println!("Command result: {:?}", result);

    // 执行更多命令
    let commands = vec! [
        "SERVICE START",
        "PLUGIN LIST",
        "SCENE LIST",
        "PERFORMANCE ANALYZE",
        "SECURITY SCAN",
        "OPS MONITOR"
    ];

    for cmd in commands {
        let command = parser.parse(cmd)?;
        let result = executor.execute(&command).await?;
        println!("Command: {} Result: {:?}", cmd, result);
    }

    Ok(())
}
        "#;

        let output_path = output_dir.join("command_example.rs");
        let mut file = File::create(output_path)?;
        file.write_all(example.as_bytes())?;

        Ok(())
    }
}
