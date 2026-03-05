// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 文档生成器
//! 用于生成API文档和系统文档

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

/// 文档生成器
pub struct DocGenerator;

impl DocGenerator {
    /// 生成API文档
    pub fn generate_api_docs(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;

        let api_docs = r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>YMAxum API文档</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1 { color: #333; }
        h2 { color: #666; }
        .endpoint { margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .method { display: inline-block; padding: 5px 10px; margin-right: 10px; border-radius: 3px; font-weight: bold; }
        .get { background-color: #4CAF50; color: white; }
        .post { background-color: #2196F3; color: white; }
        .put { background-color: #FF9800; color: white; }
        .delete { background-color: #f44336; color: white; }
        .path { font-family: monospace; font-size: 14px; }
    </style>
</head>
<body>
    <h1>YMAxum API文档</h1>
    
    <h2>健康检查</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/health</span>
        <p>返回系统健康状态</p>
    </div>
    
    <h2>命令执行</h2>
    <div class="endpoint">
        <span class="method post">POST</span>
        <span class="path">/command/execute</span>
        <p>执行命令</p>
    </div>
    
    <h2>插件管理</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/plugin/list</span>
        <p>获取插件列表</p>
    </div>
    <div class="endpoint">
        <span class="method post">POST</span>
        <span class="path">/plugin/install</span>
        <p>安装插件</p>
    </div>
    <div class="endpoint">
        <span class="method post">POST</span>
        <span class="path">/plugin/enable</span>
        <p>启用插件</p>
    </div>
    <div class="endpoint">
        <span class="method post">POST</span>
        <span class="path">/plugin/disable</span>
        <p>停用插件</p>
    </div>
    
    <h2>场景管理</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/scene/list</span>
        <p>获取场景列表</p>
    </div>
    <div class="endpoint">
        <span class="method post">POST</span>
        <span class="path">/scene/switch</span>
        <p>切换场景</p>
    </div>
    
    <h2>性能监控</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/performance/metrics</span>
        <p>获取性能指标</p>
    </div>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/performance/bottlenecks</span>
        <p>获取性能瓶颈</p>
    </div>
    
    <h2>安全管理</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/security/scan</span>
        <p>执行安全扫描</p>
    </div>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/security/assess</span>
        <p>执行安全评估</p>
    </div>
    
    <h2>运维管理</h2>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/ops/logs</span>
        <p>获取日志</p>
    </div>
    <div class="endpoint">
        <span class="method get">GET</span>
        <span class="path">/ops/monitor</span>
        <p>获取监控信息</p>
    </div>
</body>
</html>
        "#;

        let output_path = output_dir.join("index.html");
        let mut file = File::create(output_path)?;
        file.write_all(api_docs.as_bytes())?;

        Ok(())
    }

    /// 生成系统文档
    pub fn generate_system_docs(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;

        let system_docs = r#"
# YMAxum 系统文档

## 1. 项目概述

YMAxum 是基于 Rust Axum 0.8.8 + Tokio 1.x 技术栈开发的轻量级 Web 开发框架，专注于零代码命令驱动、插件化架构和多场景适配。

### 核心特性
- **零代码命令引擎**：通过 TXT 命令完成项目配置和管理
- **插件生态系统**：支持插件的安装、启用、停用、卸载、更新全生命周期管理
- **多场景适配**：原生支持新手场景、游戏场景、电商场景、SaaS 场景等
- **安全防护**：内置数据加密、HTTPS 支持、漏洞扫描等安全功能
- **性能优化**：提供并发优化、内存优化、缓存优化等性能提升工具
- **跨平台支持**：支持 Windows、Linux、macOS 三大平台

## 2. 快速开始

### 2.1 环境搭建
1. 安装 Rust 工具链
2. 克隆项目代码
3. 运行 `setup_dev_env.bat`（Windows）或 `setup_dev_env.sh`（Linux/macOS）

### 2.2 项目初始化
1. 创建配置文件 `config/server.toml`
2. 执行 `INIT PROJECT NAME=my_project` 命令
3. 配置数据库连接

### 2.3 插件安装
1. 执行 `PLUGIN INSTALL PATH=plugins/output/customer_service.axpl` 命令
2. 执行 `PLUGIN ENABLE NAME=customer_service` 命令

### 2.4 启动服务
1. 执行 `SERVICE START` 命令
2. 访问 `https://localhost:3000/health` 检查服务状态

## 3. 命令参考

### 3.1 系统命令
- `INIT PROJECT NAME=<name>`：初始化项目
- `SERVICE START`：启动服务
- `SERVICE STOP`：停止服务
- `SERVICE RESTART`：重启服务
- `STATUS`：查看系统状态

### 3.2 插件命令
- `PLUGIN LIST`：列出所有插件
- `PLUGIN INSTALL PATH=<path>`：安装插件
- `PLUGIN ENABLE NAME=<name>`：启用插件
- `PLUGIN DISABLE NAME=<name>`：停用插件
- `PLUGIN UNINSTALL NAME=<name>`：卸载插件
- `PLUGIN UPDATE NAME=<name>`：更新插件

### 3.3 场景命令
- `SCENE LIST`：列出所有场景
- `SCENE SWITCH NAME=<name>`：切换场景

### 3.4 性能命令
- `PERFORMANCE ANALYZE`：分析性能
- `PERFORMANCE OPTIMIZE`：优化性能

### 3.5 安全命令
- `SECURITY SCAN`：执行安全扫描
- `SECURITY ASSESS`：执行安全评估

### 3.6 运维命令
- `OPS LOGS LEVEL=<level>`：查看日志
- `OPS MONITOR`：查看监控信息
- `OPS CONFIG HOT RELOAD`：热更新配置

## 4. 插件开发

### 4.1 插件结构
- `plugin.toml`：插件配置文件
- `src/`：插件源代码
- `resources/`：插件资源文件

### 4.2 插件接口
```rust
pub trait PluginLifecycle {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
```

### 4.3 插件打包
1. 配置 `plugin_pack.toml`
2. 运行 `plugin_packer.rs` 脚本
3. 运行 `plugin_signer.rs` 脚本签名

## 5. 场景适配

### 5.1 新手场景
- 一键配置数据库
- 生成默认路由
- 提供基础功能

### 5.2 游戏场景
- TCP/UDP连接管理
- 玩家数据存储
- 背包系统
- 交易系统

### 5.3 电商场景
- 商户管理
- 订单管理
- 分润规则
- 结算管理

### 5.4 SaaS场景
- 分站管理
- 域名管理
- 配置管理

## 6. 性能优化

### 6.1 并发优化
- 调整线程池大小
- 使用异步IO
- 优化锁竞争

### 6.2 内存优化
- 减少内存分配
- 使用对象池
- 优化数据结构

### 6.3 缓存优化
- 使用内存缓存
- 配置缓存策略
- 优化缓存命中率

### 6.4 数据库优化
- 使用连接池
- 优化SQL查询
- 配置索引

## 7. 安全防护

### 7.1 数据加密
- 使用AES-256 GCM加密敏感数据
- 定期轮换密钥

### 7.2 HTTPS配置
- 强制启用HTTPS
- 配置证书
- 拦截非HTTPS请求

### 7.3 漏洞扫描
- 定期执行安全扫描
- 检测SQL注入、XSS等漏洞
- 生成安全报告

### 7.4 权限控制
- 实现基于角色的权限控制
- 限制API访问权限
- 审计权限变更

## 8. 运维管理

### 8.1 日志管理
- 分级日志
- 日志切割
- 日志分析

### 8.2 监控告警
- 监控CPU、内存、请求量、响应时间
- 配置告警阈值
- 多渠道告警

### 8.3 配置管理
- 支持环境变量和配置文件
- 配置热更新
- 配置验证

### 8.4 健康检查
- 定期检查系统状态
- 检查依赖服务
- 生成健康报告

## 9. 故障排查

### 9.1 常见问题
- 服务启动失败：检查配置文件和端口占用
- 插件安装失败：检查插件格式和签名
- 数据库连接失败：检查数据库配置和网络连接
- 性能问题：使用性能分析工具

### 9.2 日志分析
- 查看错误日志
- 分析请求日志
- 监控系统日志

### 9.3 调试工具
- 使用 `RUST_LOG=debug` 启用调试日志
- 使用 `cargo test` 运行测试
- 使用性能分析工具

## 10. 部署方案

### 10.1 本地部署
- 直接运行可执行文件
- 使用 `systemd` 管理服务

### 10.2 Docker部署
- 使用 `docker-compose` 编排服务
- 配置环境变量
- 挂载配置文件和数据目录

### 10.3 云服务部署
- 配置云服务器
- 设置安全组
- 配置负载均衡

## 11. 版本管理

### 11.1 版本号格式
- `MAJOR.MINOR.PATCH`
- `MAJOR`：不兼容的API变更
- `MINOR`：向后兼容的功能添加
- `PATCH`：向后兼容的bug修复

### 11.2 版本升级
- 查看版本变更日志
- 测试兼容性
- 备份数据
- 执行升级

## 12. 贡献指南

### 12.1 开发流程
1. Fork 仓库
2. 创建分支
3. 提交代码
4. 运行测试
5. 提交PR

### 12.2 代码规范
- 遵循 Rust 代码风格
- 编写单元测试
- 提交清晰的 commit 信息

### 12.3 文档规范
- 更新相关文档
- 保持文档与代码同步
- 提供使用示例

## 13. 联系我们

- 项目地址：https://github.com/ymaxum/ymaxum
- 问题反馈：https://github.com/ymaxum/ymaxum/issues
- 邮件：contact@ymaxum.com
        "#;

        let output_path = output_dir.join("system_docs.md");
        let mut file = File::create(output_path)?;
        file.write_all(system_docs.as_bytes())?;

        Ok(())
    }
}

