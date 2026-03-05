//! 插件模板生成器
//! 用于生成插件模板

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

/// 插件模板生成器
pub struct PluginTemplateGenerator;

impl PluginTemplateGenerator {
    /// 生成插件模板
    pub fn generate_plugin_template(
        output_dir: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_dir = output_dir.join(plugin_name);
        create_dir_all(&plugin_dir)?;

        // 生成插件配置文件
        Self::generate_plugin_toml(&plugin_dir, plugin_name)?;
        // 生成插件源代码
        Self::generate_plugin_src(&plugin_dir, plugin_name)?;
        // 生成插件打包配置
        Self::generate_pack_toml(&plugin_dir, plugin_name)?;
        // 生成插件签名配置
        Self::generate_sign_toml(&plugin_dir, plugin_name)?;

        Ok(())
    }

    /// 生成插件配置文件
    fn generate_plugin_toml(
        plugin_dir: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_toml = format! {
            r#"
[package]
name = "{plugin_name}"
version = "1.0.0"
edition = "2024"

[lib]
name = "{plugin_name}"
path = "src/lib.rs"
crate-type = ["cdylib"]

[dependencies]
ymaxum = {{ path = "../../" }}
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
log = "0.4"

[features]
default = []
        "#
        };

        let output_path = plugin_dir.join("Cargo.toml");
        let mut file = File::create(output_path)?;
        file.write_all(plugin_toml.as_bytes())?;

        Ok(())
    }

    /// 生成插件源代码
    fn generate_plugin_src(
        plugin_dir: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src_dir = plugin_dir.join("src");
        create_dir_all(&src_dir)?;

        let plugin_lib = format! {
            r#"
//! {plugin_name} 插件
//! 提供 {plugin_name} 相关功能

use ymaxum::plugin::manager::{{PluginLifecycle, PluginInfo}};
use std::sync::Arc;

/// {plugin_name} 插件
pub struct {plugin_name}Plugin;

impl Default for {plugin_name}Plugin {{
    fn default() -> Self {{
        Self {{}}
    }}
}}

impl PluginLifecycle for {plugin_name}Plugin {{
    /// 初始化插件
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Initializing {plugin_name} plugin...");
        // 初始化插件资源
        Ok(())
    }}

    /// 启动插件
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Starting {plugin_name} plugin...");
        // 启动插件服务
        Ok(())
    }}

    /// 停止插件
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Stopping {plugin_name} plugin...");
        // 停止插件服务
        Ok(())
    }}
}}

/// 获取插件信息
pub fn get_plugin_info() -> PluginInfo {{
    PluginInfo {{
        name: "{plugin_name}".to_string(),
        version: "1.0.0".to_string(),
        author: "YMAxum Team".to_string(),
        description: format!("{plugin_name} plugin for YMAxum framework"),
        dependencies: Vec::new(),
        routes: Vec::new(),
        permissions: Vec::new(),
    }}
}}

/// 插件入口点
#[no_mangle]
pub extern "C" fn plugin_entry() -> *mut dyn PluginLifecycle {{
    let plugin = Box::new({plugin_name}Plugin::default());
    Box::into_raw(plugin)
}}
        "#
        };

        let output_path = src_dir.join("lib.rs");
        let mut file = File::create(output_path)?;
        file.write_all(plugin_lib.as_bytes())?;

        Ok(())
    }

    /// 生成插件打包配置
    fn generate_pack_toml(
        plugin_dir: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pack_toml = format! {
            r#"
[plugin]
name = "{plugin_name}"
version = "1.0.0"
author = "YMAxum Team"
description = "{plugin_name} plugin for YMAxum framework"

[build]
target = "release"
out_dir = "output"

[files]
src = ["src/**/*"]
config = ["Cargo.toml"]
        "#
        };

        let output_path = plugin_dir.join(format!("{}_pack.toml", plugin_name));
        let mut file = File::create(output_path)?;
        file.write_all(pack_toml.as_bytes())?;

        Ok(())
    }

    /// 生成插件签名配置
    fn generate_sign_toml(
        plugin_dir: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sign_toml = format! {
            r#"
[signature]
private_key = "keys/{plugin_name}_private.pem"
public_key = "keys/{plugin_name}_public.pem"

[plugin]
name = "{plugin_name}"
version = "1.0.0"
        "#
        };

        let output_path = plugin_dir.join(format!("{}_sign.toml", plugin_name));
        let mut file = File::create(output_path)?;
        file.write_all(sign_toml.as_bytes())?;

        Ok(())
    }
}
