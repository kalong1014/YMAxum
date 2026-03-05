// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 配置生成器
//! 用于生成各种配置文件

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 配置生成器
pub struct ConfigGenerator;

impl ConfigGenerator {
    /// 生成服务器配置
    pub fn generate_server_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfig {
            port: 3000,
            host: "127.0.0.1".to_string(),
            workers: 4,
        };

        let content = toml::to_string_pretty(&config)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// 生成数据库配置
    pub fn generate_database_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config = DatabaseConfig {
            url: "sqlite://./database.db".to_string(),
            max_connections: 10,
            min_connections: 1,
        };

        let content = toml::to_string_pretty(&config)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

/// 服务器配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub workers: usize,
}

/// 数据库配置
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

