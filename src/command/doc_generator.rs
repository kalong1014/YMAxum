//! 文档生成命令
//! 用于自动化生成项目文档

use clap::Parser;
use log::info;
use std::fs;
use std::path::Path;

/// 文档生成命令参数
#[derive(Parser, Debug)]
pub struct DocGeneratorCommand {
    /// 项目根路径
    #[arg(short, long, default_value = "./")]
    pub project_path: String,

    /// 文档输出目录
    #[arg(short, long, default_value = "./docs/generated")]
    pub output_dir: String,

    /// 是否生成API文档
    #[arg(short, long, default_value = "true")]
    pub api_docs: bool,

    /// 是否生成用户文档
    #[arg(short, long, default_value = "true")]
    pub user_docs: bool,

    /// 是否生成架构文档
    #[arg(short, long, default_value = "true")]
    pub architecture_docs: bool,

    /// 是否生成API参考
    #[arg(short, long, default_value = "true")]
    pub api_reference: bool,

    /// 是否包含示例代码
    #[arg(short, long, default_value = "true")]
    pub include_examples: bool,

    /// 是否使用AI辅助生成
    #[arg(short, long, default_value = "false")]
    pub use_ai: bool,

    /// AI模型名称
    #[arg(short, long, default_value = "gpt-3.5-turbo")]
    pub ai_model: String,
}

impl DocGeneratorCommand {
    /// 执行文档生成命令
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始执行文档生成命令");
        info!("项目路径: {}", self.project_path);
        info!("输出目录: {}", self.output_dir);

        // 创建输出目录
        self.create_output_dir()?;

        // 生成API文档
        if self.api_docs {
            self.generate_api_docs().await?;
        }

        // 生成用户文档
        if self.user_docs {
            self.generate_user_docs().await?;
        }

        // 生成架构文档
        if self.architecture_docs {
            self.generate_architecture_docs().await?;
        }

        // 生成API参考
        if self.api_reference {
            self.generate_api_reference().await?;
        }

        info!("文档生成完成！");
        info!("文档已生成到: {}", self.output_dir);

        Ok(())
    }

    /// 创建输出目录
    fn create_output_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = Path::new(&self.output_dir);
        if !output_path.exists() {
            info!("创建输出目录: {}", self.output_dir);
            fs::create_dir_all(output_path)?;
        }
        Ok(())
    }

    /// 生成API文档
    async fn generate_api_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("生成API文档...");

        // 运行cargo doc生成Rust文档
        let status = std::process::Command::new("cargo")
            .arg("doc")
            .arg("--no-deps")
            .arg("--output")
            .arg(format!("{}/api", self.output_dir))
            .status()?;

        if status.success() {
            info!("API文档生成成功");
        } else {
            info!("API文档生成失败，使用默认路径");
        }

        Ok(())
    }

    /// 生成用户文档
    async fn generate_user_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("生成用户文档...");

        // 创建用户文档目录
        let user_docs_path = format!("{}/user", self.output_dir);
        fs::create_dir_all(&user_docs_path)?;

        // 生成README.md
        let readme_content = r#"# YMAxum 用户文档

## 1. 项目概述

YMAxum是一个基于Rust和Axum的高性能API网关，提供了丰富的功能和灵活的配置选项。

### 1.1 核心特性

- 高性能异步架构
- 服务发现与负载均衡
- 插件系统支持
- Serverless函数执行
- AI辅助开发工具
- GUF集成框架
- 安全防护机制
- 监控与告警

### 1.2 技术栈

- Rust 1.93.0
- Axum 0.8.8
- Tokio 1.x
- SQLx 0.8.6
- Moka + Redis 缓存
- AES-256 GCM + RSA 加密

## 2. 快速开始

### 2.1 安装

```bash
# 从源码构建
cargo build --release

# 或从发布包安装
# 下载对应操作系统的发布包并解压
```

### 2.2 配置

```bash
# 复制示例配置
cp -r config.example config

# 编辑配置文件
vim config/config.toml
```

### 2.3 运行

```bash
# 直接运行
./ymaxum --config config/config.toml

# 或作为服务运行
# Windows: ./ymaxum service install
# Linux: sudo systemctl enable ymaxum && sudo systemctl start ymaxum
```

### 2.4 验证

```bash
curl http://localhost:8080/health
```

## 3. 核心功能

### 3.1 服务管理

- **服务注册**: 注册新的后端服务
- **服务发现**: 发现可用的服务实例
- **负载均衡**: 支持轮询、加权、自定义等负载均衡策略
- **健康检查**: 自动检测服务健康状态

### 3.2 插件系统

- **热插拔**: 支持插件的动态加载和卸载
- **多语言支持**: 支持JavaScript、Python、Go等语言的插件
- **安全沙箱**: 插件运行在安全的沙箱环境中

### 3.3 Serverless 函数

- **函数部署**: 部署Serverless函数
- **函数执行**: 执行Serverless函数
- **自动扩缩容**: 根据负载自动调整实例数
- **多提供商支持**: 支持本地、AWS Lambda、Azure Functions等

### 3.4 AI 辅助工具

- **代码生成**: 自动生成代码
- **智能调试**: 智能分析和修复bug
- **性能优化**: 自动分析和优化性能
- **智能路由**: 基于AI的智能路由决策

### 3.5 GUF 集成

- **统一框架**: 集成GUF通用框架
- **插件生态**: 接入GUF插件生态
- **配置同步**: 与GUF配置中心同步

## 4. API 参考

### 4.1 健康检查

```bash
GET /health
```

### 4.2 服务管理

```bash
# 注册服务
POST /api/services

# 列出服务
GET /api/services

# 获取服务详情
GET /api/services/{name}

# 更新服务
PUT /api/services/{name}

# 删除服务
DELETE /api/services/{name}
```

### 4.3 插件管理

```bash
# 安装插件
POST /api/plugins

# 列出插件
GET /api/plugins

# 启用插件
POST /api/plugins/{name}/enable

# 禁用插件
POST /api/plugins/{name}/disable

# 删除插件
DELETE /api/plugins/{name}
```

### 4.4 Serverless 函数

```bash
# 部署函数
POST /api/serverless/functions

# 列出函数
GET /api/serverless/functions

# 执行函数
POST /api/serverless/functions/{name}

# 删除函数
DELETE /api/serverless/functions/{name}
```

## 5. 配置指南

### 5.1 主配置文件

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[logging]
level = "info"
file = "logs/ymaxum.log"
rotation = "daily"

[security]
enable_https = false
cert_file = ""
key_file = ""

[ai]
enabled = true
model_path = "models/"

[serverless]
enabled = true
max_instances = 10
instance_timeout = 300
```

### 5.2 数据库配置

```toml
[mysql]
enabled = false
host = "localhost"
port = 3306
username = "root"
password = "password"
database = "ymaxum"

[postgresql]
enabled = false
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database = "ymaxum"

[sqlite]
enabled = true
file = "data/ymaxum.db"
```

### 5.3 Redis 配置

```toml
[redis]
enabled = false
host = "localhost"
port = 6379
password = ""
database = 0
```

## 6. 部署指南

### 6.1 开发环境

- 使用默认配置文件
- 启用详细日志
- 禁用HTTPS
- 使用SQLite数据库

### 6.2 测试环境

- 使用独立的数据库实例
- 启用基本的安全配置
- 配置监控和告警

### 6.3 生产环境

- 使用HTTPS
- 配置高可用数据库
- 启用Redis缓存
- 配置详细的安全策略
- 启用监控和告警
- 定期备份数据

## 7. 监控与维护

### 7.1 日志管理

- **日志文件**: `logs/ymaxum.log`
- **日志级别**: debug, info, warn, error
- **日志轮转**: 每天自动轮转，保留7天

### 7.2 监控指标

- **CPU使用率**
- **内存使用率**
- **请求量**
- **响应时间**
- **错误率**
- **服务健康状态**

### 7.3 定期维护

- **备份配置文件和数据**
- **更新依赖包**
- **检查安全漏洞**
- **优化数据库性能**
- **清理日志文件**

## 8. 故障排除

### 8.1 服务无法启动

- 检查端口是否被占用
- 检查配置文件是否正确
- 检查数据库连接是否正常

### 8.2 响应时间过长

- 检查后端服务是否正常
- 检查数据库性能
- 检查网络连接

### 8.3 500 错误

- 查看详细日志
- 检查后端服务状态
- 检查数据库连接

## 9. 安全最佳实践

### 9.1 配置安全

- 使用强密码和密钥
- 定期更新密钥
- 限制配置文件访问权限
- 使用环境变量存储敏感信息

### 9.2 网络安全

- 启用HTTPS
- 使用防火墙限制访问
- 配置正确的CORS策略
- 定期进行安全扫描

### 9.3 数据安全

- 加密敏感数据
- 定期备份数据
- 限制数据库访问权限
- 实现数据访问审计

### 9.4 插件安全

- 只使用可信的插件
- 验证插件签名
- 限制插件权限
- 定期更新插件

## 10. 升级指南

### 10.1 从旧版本升级

1. **备份数据和配置**
2. **停止服务**
3. **替换可执行文件**
4. **更新配置文件**
5. **启动服务**
6. **验证升级**

### 10.2 回滚策略

1. **停止服务**
2. **恢复旧版本**
3. **启动服务**

## 11. 联系支持

- **GitHub Issues**: [https://github.com/yourusername/ymaxum/issues](https://github.com/yourusername/ymaxum/issues)
- **邮件支持**: support@ymaxum.com
- **文档**: [https://docs.ymaxum.com](https://docs.ymaxum.com)

---

**版本**: 1.6.0  
**最后更新**: 2026-02-06  
**作者**: YMAxum Team
"#;

        fs::write(format!("{}/README.md", user_docs_path), readme_content)?;
        info!("用户文档生成成功");

        Ok(())
    }

    /// 生成架构文档
    async fn generate_architecture_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("生成架构文档...");

        // 创建架构文档目录
        let arch_docs_path = format!("{}/architecture", self.output_dir);
        fs::create_dir_all(&arch_docs_path)?;

        // 生成架构文档
        let arch_content = r#"# YMAxum 架构文档

## 1. 系统架构

YMAxum 采用多层架构设计，提供高性能、可扩展的API网关服务。

### 1.1 架构层次

```
┌─────────────────────────────────────────────────────────────┐
│                        客户端层                            │
├─────────────────────────────────────────────────────────────┤
│                        API 层                              │
│  - 路由管理                                               │
│  - 请求处理                                               │
│  - 响应处理                                               │
├─────────────────────────────────────────────────────────────┤
│                      服务层                                │
│  - 服务发现                                               │
│  - 负载均衡                                               │
│  - 健康检查                                               │
├─────────────────────────────────────────────────────────────┤
│                      插件层                                │
│  - 插件管理                                               │
│  - 插件执行                                               │
│  - 沙箱环境                                               │
├─────────────────────────────────────────────────────────────┤
│                    Serverless 层                           │
│  - 函数管理                                               │
│  - 实例池管理                                             │
│  - 多提供商支持                                           │
├─────────────────────────────────────────────────────────────┤
│                        AI 层                               │
│  - 代码生成                                               │
│  - 智能调试                                               │
│  - 性能优化                                               │
│  - 智能路由                                               │
├─────────────────────────────────────────────────────────────┤
│                      GUF 层                                │
│  - 统一框架集成                                           │
│  - 插件生态接入                                           │
│  - 配置同步                                               │
├─────────────────────────────────────────────────────────────┤
│                      存储层                                │
│  - 数据库 (MySQL/PostgreSQL/SQLite)                       │
│  - 缓存 (Moka/Redis)                                       │
│  - 配置存储                                               │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 核心组件

#### 1.2.1 API 层
- **路由管理器**: 管理API路由，处理请求分发
- **请求处理器**: 处理客户端请求，进行参数验证和转换
- **响应处理器**: 处理服务响应，进行格式转换和错误处理

#### 1.2.2 服务层
- **服务注册表**: 存储服务信息和实例状态
- **负载均衡器**: 实现多种负载均衡策略
- **健康检查器**: 定期检查服务健康状态

#### 1.2.3 插件层
- **插件管理器**: 管理插件的生命周期
- **插件执行器**: 执行插件代码
- **沙箱管理器**: 管理插件的安全沙箱环境

#### 1.2.4 Serverless 层
- **函数管理器**: 管理Serverless函数
- **实例池**: 管理函数实例
- **提供商适配器**: 适配不同的Serverless提供商

#### 1.2.5 AI 层
- **代码生成器**: 自动生成代码
- **智能调试器**: 智能分析和修复bug
- **性能优化器**: 自动分析和优化性能
- **智能路由器**: 基于AI的智能路由决策

#### 1.2.6 GUF 层
- **GUF 集成器**: 集成GUF通用框架
- **插件适配器**: 适配GUF插件
- **配置同步器**: 与GUF配置中心同步

#### 1.2.7 存储层
- **数据库管理器**: 管理数据库连接和操作
- **缓存管理器**: 管理缓存操作
- **配置管理器**: 管理配置文件和环境变量

## 2. 数据流

### 2.1 请求处理流程

1. **客户端发送请求** → API层
2. **API层路由请求** → 服务层
3. **服务层选择实例** → 后端服务
4. **后端服务处理请求** → 服务层
5. **服务层处理响应** → API层
6. **API层返回响应** → 客户端

### 2.2 插件执行流程

1. **请求到达API层** → 插件层
2. **插件层执行前置插件** → 服务层
3. **服务层处理请求** → 插件层
4. **插件层执行后置插件** → API层
5. **API层返回响应** → 客户端

### 2.3 Serverless 函数执行流程

1. **客户端发送请求** → API层
2. **API层路由请求** → Serverless层
3. **Serverless层获取或创建实例** → 函数执行
4. **函数执行完成** → Serverless层
5. **Serverless层处理响应** → API层
6. **API层返回响应** → 客户端

## 3. 关键模块

### 3.1 路由模块

- **功能**: 管理API路由，处理请求分发
- **设计**: 基于Axum的路由系统，支持动态路由和参数提取
- **实现**: `src/core/routing.rs`

### 3.2 服务发现模块

- **功能**: 发现和管理后端服务实例
- **设计**: 基于注册表的服务发现机制，支持健康检查
- **实现**: `src/core/service_discovery.rs`

### 3.3 负载均衡模块

- **功能**: 在多个服务实例之间分配请求
- **设计**: 支持轮询、加权、自定义等负载均衡策略
- **实现**: `src/core/load_balancer.rs`

### 3.4 插件系统模块

- **功能**: 管理和执行插件
- **设计**: 基于多语言支持的插件系统，运行在安全沙箱中
- **实现**: `src/plugin/`

### 3.5 Serverless 模块

- **功能**: 管理和执行Serverless函数
- **设计**: 支持多提供商的Serverless执行环境
- **实现**: `src/serverless/`

### 3.6 AI 辅助模块

- **功能**: 提供AI辅助开发工具
- **设计**: 集成多种AI模型，提供代码生成、智能调试等功能
- **实现**: `src/ai/`

### 3.7 GUF 集成模块

- **功能**: 集成GUF通用框架
- **设计**: 与GUF框架无缝集成，接入GUF插件生态
- **实现**: `src/guf/`

### 3.8 安全模块

- **功能**: 提供安全防护机制
- **设计**: 实现加密、认证、授权等安全功能
- **实现**: `src/security/`

### 3.9 监控模块

- **功能**: 监控系统运行状态
- **设计**: 收集和分析监控指标，提供告警功能
- **实现**: `src/ops/monitoring.rs`

## 4. 技术栈

### 4.1 核心技术

| 技术 | 版本 | 用途 | 来源 |
|------|------|------|------|
| Rust | 1.93.0 | 核心编程语言 | `Cargo.toml` |
| Axum | 0.8.8 | Web框架 | `Cargo.toml` |
| Tokio | 1.x | 异步运行时 | `Cargo.toml` |
| SQLx | 0.8.6 | 异步SQL驱动 | `Cargo.toml` |
| Moka | 0.12.13 | 内存缓存 | `Cargo.toml` |
| Redis | 0.26.1 | 分布式缓存 | `Cargo.toml` |
| AES-256 GCM | - | 对称加密 | `Cargo.toml` |
| RSA | 0.9.6 | 非对称加密 | `Cargo.toml` |
| GUF | 1.0.0 | 通用集成框架 | `Cargo.toml` |

### 4.2 插件技术

| 技术 | 版本 | 用途 | 来源 |
|------|------|------|------|
| QuickJS | - | JavaScript运行时 | `Cargo.toml` |
| PyO3 | - | Python绑定 | `Cargo.toml` |
| Go | 1.20+ | Go插件支持 | 系统依赖 |

### 4.3 部署技术

| 技术 | 版本 | 用途 | 来源 |
|------|------|------|------|
| Windows服务 | - | Windows部署 | `Cargo.toml` |
| systemd | - | Linux部署 | 系统依赖 |
| Docker | 20.10+ | 容器化部署 | 外部依赖 |
| Kubernetes | 1.20+ | 容器编排 | 外部依赖 |

## 5. 扩展性设计

### 5.1 插件系统

- **热插拔**: 支持插件的动态加载和卸载
- **多语言支持**: 支持JavaScript、Python、Go等语言的插件
- **标准接口**: 插件通过标准接口与核心系统交互

### 5.2 Serverless 函数

- **多提供商支持**: 支持本地、AWS Lambda、Azure Functions等
- **自动扩缩容**: 根据负载自动调整实例数
- **标准函数接口**: 函数通过标准接口与核心系统交互

### 5.3 存储层

- **多数据库支持**: 支持MySQL、PostgreSQL、SQLite
- **多缓存支持**: 支持Moka内存缓存、Redis分布式缓存
- **可插拔存储**: 存储后端可通过插件扩展

### 5.4 集成能力

- **GUF集成**: 集成GUF通用框架
- **第三方服务集成**: 通过插件集成第三方服务
- **API集成**: 提供标准API接口供其他系统集成

## 6. 性能优化

### 6.1 异步设计

- **全异步架构**: 基于Tokio的全异步设计
- **非阻塞IO**: 所有IO操作都是非阻塞的
- **并发处理**: 支持高并发请求处理

### 6.2 缓存策略

- **多级缓存**: 内存缓存 + 分布式缓存
- **智能缓存**: 基于访问模式的智能缓存策略
- **缓存预热**: 启动时预热缓存

### 6.3 负载均衡

- **多种策略**: 支持轮询、加权、自定义等负载均衡策略
- **智能路由**: 基于AI的智能路由决策
- **健康感知**: 基于健康状态的路由决策

### 6.4 资源管理

- **连接池**: 数据库连接池、Redis连接池
- **线程池**: 优化的线程池配置
- **内存管理**: 高效的内存使用

## 7. 高可用性设计

### 7.1 冗余设计

- **多实例部署**: 支持部署多个实例
- **负载均衡**: 前端使用负载均衡器分发请求
- **故障转移**: 自动检测和转移故障实例

### 7.2 容错机制

- **服务降级**: 当后端服务不可用时，返回降级响应
- **熔断机制**: 当服务错误率过高时，暂时停止请求
- **限流机制**: 防止过载，保护系统

### 7.3 数据一致性

- **配置同步**: 多实例间配置同步
- **状态管理**: 分布式状态管理
- **数据备份**: 定期备份关键数据

## 8. 安全设计

### 8.1 分层安全

- **网络安全**: HTTPS、防火墙
- **应用安全**: 认证、授权、输入验证
- **数据安全**: 加密、脱敏、审计
- **插件安全**: 沙箱隔离、权限控制

### 8.2 安全特性

- **加密通信**: 支持HTTPS
- **身份认证**: 支持JWT、OAuth2等认证方式
- **访问控制**: 基于角色的访问控制
- **输入验证**: 严格的输入验证和参数检查
- **安全审计**: 详细的安全审计日志
- **插件沙箱**: 插件运行在安全的沙箱环境中

### 8.3 安全最佳实践

- **最小权限原则**: 只授予必要的权限
- **防御纵深**: 多层安全防护
- **定期安全审计**: 定期进行安全审计和漏洞扫描
- **安全更新**: 及时更新依赖和修复安全漏洞

## 9. 部署架构

### 9.1 单节点部署

```
┌───────────────────────┐
│      客户端          │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│    YMAxum 实例      │
├─────────────────────┤
│  - API 网关         │
│  - 服务管理         │
│  - 插件系统         │
│  - Serverless       │
│  - AI 辅助工具      │
├─────────────────────┤
│    存储层           │
│  - 数据库          │
│  - 缓存            │
└─────────────────────┘
```

### 9.2 多节点部署

```
┌───────────────────────┐
│      客户端          │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│    负载均衡器        │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│┌───────────────────┐│
││  YMAxum 实例 1    ││
│├───────────────────┤│
││  - API 网关       ││
││  - 服务管理       ││
││  - 插件系统       ││
││  - Serverless     ││
││  - AI 辅助工具    ││
│└───────────────────┘│
│┌───────────────────┐│
││  YMAxum 实例 2    ││
│├───────────────────┤│
││  - API 网关       ││
││  - 服务管理       ││
││  - 插件系统       ││
││  - Serverless     ││
││  - AI 辅助工具    ││
│└───────────────────┘│
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│    存储层           │
│┌───────────────────┐│
││  高可用数据库集群  ││
│└───────────────────┘│
│┌───────────────────┐│
││  Redis 集群       ││
│└───────────────────┘│
└─────────────────────┘
```

### 9.3 容器化部署

```
┌───────────────────────┐
│      客户端          │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│    Kubernetes       │
├─────────────────────┤
│┌───────────────────┐│
││  YMAxum Pod 1     ││
│├───────────────────┤│
││  - API 网关       ││
││  - 服务管理       ││
││  - 插件系统       ││
││  - Serverless     ││
││  - AI 辅助工具    ││
│└───────────────────┘│
│┌───────────────────┐│
││  YMAxum Pod 2     ││
│├───────────────────┤│
││  - API 网关       ││
││  - 服务管理       ││
││  - 插件系统       ││
││  - Serverless     ││
││  - AI 辅助工具    ││
│└───────────────────┘│
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│    存储层           │
│┌───────────────────┐│
││  数据库 StatefulSet││
│└───────────────────┘│
│┌───────────────────┐│
││  Redis StatefulSet││
│└───────────────────┘│
└─────────────────────┘
```

## 10. 未来发展

### 10.1 功能增强

- **更多协议支持**: gRPC、WebSocket等
- **更丰富的插件生态**: 提供更多官方和社区插件
- **更智能的AI辅助**: 增强AI能力，提供更多智能功能
- **更完善的GUF集成**: 深度集成GUF框架，提供更统一的体验

### 10.2 性能优化

- **更高效的异步设计**: 优化异步代码，提高性能
- **更智能的缓存策略**: 基于机器学习的智能缓存策略
- **更高效的负载均衡**: 基于实时监控数据的动态负载均衡

### 10.3 安全性提升

- **更强大的安全防护**: 增强安全功能，抵御新型攻击
- **更完善的安全审计**: 提供更详细的安全审计日志和分析
- **更严格的插件安全**: 增强插件沙箱的安全性

### 10.4 可观测性提升

- **更丰富的监控指标**: 提供更详细的系统运行指标
- **更智能的告警机制**: 基于机器学习的智能告警
- **更完善的分布式追踪**: 支持全链路分布式追踪

### 10.5 生态系统建设

- **更完善的文档**: 提供更详细、更易懂的文档
- **更丰富的示例**: 提供更多使用示例和最佳实践
- **更活跃的社区**: 建立和维护活跃的社区
- **更广泛的集成**: 与更多第三方服务和框架集成

---

**版本**: 1.6.0  
**最后更新**: 2026-02-06  
**作者**: YMAxum Team
"#;

        fs::write(format!("{}/architecture.md", arch_docs_path), arch_content)?;
        info!("架构文档生成成功");

        Ok(())
    }

    /// 生成API参考文档
    async fn generate_api_reference(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("生成API参考文档...");

        // 创建API参考目录
        let api_ref_path = format!("{}/api-reference", self.output_dir);
        fs::create_dir_all(&api_ref_path)?;

        // 生成API参考文档
        let api_ref_content = r#"# YMAxum API 参考

## 1. 健康检查

### 1.1 检查服务健康状态

```bash
GET /health
```

**响应**:

```json
{
  "status": "healthy",
  "version": "1.6.0",
  "uptime": "24h 30m 15s",
  "dependencies": {
    "database": "connected",
    "redis": "connected",
    "guf": "integrated"
  }
}
```

## 2. 服务管理

### 2.1 注册服务

```bash
POST /api/services
```

**请求体**:

```json
{
  "name": "user-service",
  "version": "1.0.0",
  "url": "http://localhost:3000",
  "health_check": "/health",
  "timeout": 5000,
  "retries": 3
}
```

**响应**:

```json
{
  "id": "user-service-1.0.0",
  "name": "user-service",
  "version": "1.0.0",
  "url": "http://localhost:3000",
  "status": "healthy",
  "created_at": "2026-02-06T10:00:00Z"
}
```

### 2.2 列出服务

```bash
GET /api/services
```

**响应**:

```json
[
  {
    "id": "user-service-1.0.0",
    "name": "user-service",
    "version": "1.0.0",
    "url": "http://localhost:3000",
    "status": "healthy",
    "created_at": "2026-02-06T10:00:00Z"
  },
  {
    "id": "order-service-1.0.0",
    "name": "order-service",
    "version": "1.0.0",
    "url": "http://localhost:3001",
    "status": "healthy",
    "created_at": "2026-02-06T10:05:00Z"
  }
]
```

### 2.3 获取服务详情

```bash
GET /api/services/{name}
```

**响应**:

```json
{
  "id": "user-service-1.0.0",
  "name": "user-service",
  "version": "1.0.0",
  "url": "http://localhost:3000",
  "health_check": "/health",
  "timeout": 5000,
  "retries": 3,
  "status": "healthy",
  "instances": [
    {
      "id": "user-service-instance-1",
      "url": "http://localhost:3000",
      "status": "healthy",
      "load": 0.5
    }
  ],
  "created_at": "2026-02-06T10:00:00Z",
  "updated_at": "2026-02-06T10:00:00Z"
}
```

### 2.4 更新服务

```bash
PUT /api/services/{name}
```

**请求体**:

```json
{
  "version": "1.1.0",
  "url": "http://localhost:3000",
  "health_check": "/health",
  "timeout": 10000,
  "retries": 5
}
```

**响应**:

```json
{
  "id": "user-service-1.1.0",
  "name": "user-service",
  "version": "1.1.0",
  "url": "http://localhost:3000",
  "status": "healthy",
  "updated_at": "2026-02-06T11:00:00Z"
}
```

### 2.5 删除服务

```bash
DELETE /api/services/{name}
```

**响应**:

```json
{
  "message": "Service deleted successfully"
}
```

## 3. 插件管理

### 3.1 安装插件

```bash
POST /api/plugins
```

**请求体**:

```json
{
  "name": "customer-service",
  "version": "1.0.0",
  "type": "javascript",
  "path": "/path/to/plugin",
  "config": {
    "api_key": "your-api-key"
  }
}
```

**响应**:

```json
{
  "id": "customer-service-1.0.0",
  "name": "customer-service",
  "version": "1.0.0",
  "type": "javascript",
  "status": "installed",
  "created_at": "2026-02-06T10:00:00Z"
}
```

### 3.2 列出插件

```bash
GET /api/plugins
```

**响应**:

```json
[
  {
    "id": "customer-service-1.0.0",
    "name": "customer-service",
    "version": "1.0.0",
    "type": "javascript",
    "status": "enabled",
    "created_at": "2026-02-06T10:00:00Z"
  },
  {
    "id": "payment-service-1.0.0",
    "name": "payment-service",
    "version": "1.0.0",
    "type": "python",
    "status": "installed",
    "created_at": "2026-02-06T10:05:00Z"
  }
]
```

### 3.3 启用插件

```bash
POST /api/plugins/{name}/enable
```

**响应**:

```json
{
  "id": "customer-service-1.0.0",
  "name": "customer-service",
  "status": "enabled"
}
```

### 3.4 禁用插件

```bash
POST /api/plugins/{name}/disable
```

**响应**:

```json
{
  "id": "customer-service-1.0.0",
  "name": "customer-service",
  "status": "disabled"
}
```

### 3.5 删除插件

```bash
DELETE /api/plugins/{name}
```

**响应**:

```json
{
  "message": "Plugin deleted successfully"
}
```

## 4. Serverless 函数

### 4.1 部署函数

```bash
POST /api/serverless/functions
```

**请求体**:

```json
{
  "name": "hello-world",
  "runtime": "nodejs18.x",
  "handler": "index.handler",
  "code": {
    "zipfile": "base64-encoded-zip-file"
  },
  "memory": 128,
  "timeout": 30
}
```

**响应**:

```json
{
  "id": "hello-world",
  "name": "hello-world",
  "runtime": "nodejs18.x",
  "status": "deployed",
  "created_at": "2026-02-06T10:00:00Z"
}
```

### 4.2 列出函数

```bash
GET /api/serverless/functions
```

**响应**:

```json
[
  {
    "id": "hello-world",
    "name": "hello-world",
    "runtime": "nodejs18.x",
    "status": "deployed",
    "created_at": "2026-02-06T10:00:00Z"
  },
  {
    "id": "calculate",
    "name": "calculate",
    "runtime": "python3.9",
    "status": "deployed",
    "created_at": "2026-02-06T10:05:00Z"
  }
]
```

### 4.3 执行函数

```bash
POST /api/serverless/functions/{name}
```

**请求体**:

```json
{
  "name": "John",
  "age": 30
}
```

**响应**:

```json
{
  "statusCode": 200,
  "body": {
    "message": "Hello, John! You are 30 years old."
  }
}
```

### 4.4 获取函数详情

```bash
GET /api/serverless/functions/{name}
```

**响应**:

```json
{
  "id": "hello-world",
  "name": "hello-world",
  "runtime": "nodejs18.x",
  "handler": "index.handler",
  "memory": 128,
  "timeout": 30,
  "status": "deployed",
  "instances": 2,
  "created_at": "2026-02-06T10:00:00Z",
  "updated_at": "2026-02-06T10:00:00Z"
}
```

### 4.5 删除函数

```bash
DELETE /api/serverless/functions/{name}
```

**响应**:

```json
{
  "message": "Function deleted successfully"
}
```

## 5. AI 辅助工具

### 5.1 生成代码

```bash
POST /api/ai/code-generate
```

**请求体**:

```json
{
  "prompt": "生成一个用户注册的API端点",
  "language": "rust",
  "framework": "axum",
  "use_ai": true
}
```

**响应**:

```json
{
  "code": "// 用户注册API端点\n#[derive(Debug, Deserialize)]\nstruct RegisterRequest {\n    username: String,\n    email: String,\n    password: String,\n}\n\n#[derive(Debug, Serialize)]\nstruct RegisterResponse {\n    id: String,\n    username: String,\n    email: String,\n}\n\nasync fn register(\n    Json(req): Json<RegisterRequest>,\n) -> impl IntoResponse {\n    // 实现注册逻辑\n    let user = User {\n        id: Uuid::new_v4().to_string(),\n        username: req.username,\n        email: req.email,\n        password: hash_password(&req.password),\n    };\n    \n    // 保存用户到数据库\n    // ...\n    \n    Json(RegisterResponse {\n        id: user.id,\n        username: user.username,\n        email: user.email,\n    })\n}",
  "language": "rust",
  "framework": "axum"
}
```

### 5.2 智能调试

```bash
POST /api/ai/debug
```

**请求体**:

```json
{
  "code": "fn divide(a: i32, b: i32) -> i32 {\n    a / b\n}",
  "error": "thread 'main' panicked at 'attempt to divide by zero'",
  "language": "rust"
}
```

**响应**:

```json
{
  "issue": "除零错误",
  "fix": "fn divide(a: i32, b: i32) -> Result<i32, String> {\n    if b == 0 {\n        return Err(\"除数不能为零\".to_string());\n    }\n    Ok(a / b)\n}",
  "explanation": "当除数为零时，会触发除零错误。需要添加除数检查，当除数为零时返回错误。"
}
```

### 5.3 性能优化

```bash
POST /api/ai/optimize
```

**请求体**:

```json
{
  "code": "fn fibonacci(n: u32) -> u32 {\n    if n <= 1 {\n        return n;\n    }\n    fibonacci(n - 1) + fibonacci(n - 2)\n}",
  "language": "rust",
  "target": "performance"
}
```

**响应**:

```json
{
  "optimized_code": "fn fibonacci(n: u32) -> u32 {\n    if n <= 1 {\n        return n;\n    }\n    let mut a = 0;\n    let mut b = 1;\n    for _ in 2..=n {\n        let c = a + b;\n        a = b;\n        b = c;\n    }\n    b\n}",
  "explanation": "原实现使用递归，时间复杂度为O(2^n)。优化后使用迭代，时间复杂度为O(n)，空间复杂度为O(1)。",
  "improvement": "时间复杂度从O(2^n)降低到O(n)，空间复杂度从O(n)降低到O(1)"
}
```

## 6. GUF 集成

### 6.1 获取GUF状态

```bash
GET /api/guf/status
```

**响应**:

```json
{
  "status": "integrated",
  "version": "1.0.0",
  "components": {
    "plugin_system": "active",
    "config_center": "synced",
    "service_registry": "connected"
  }
}
```

### 6.2 列出GUF插件

```bash
GET /api/guf/plugins
```

**响应**:

```json
[
  {
    "id": "guf-auth-plugin",
    "name": "guf-auth-plugin",
    "version": "1.0.0",
    "status": "enabled"
  },
  {
    "id": "guf-logging-plugin",
    "name": "guf-logging-plugin",
    "version": "1.0.0",
    "status": "enabled"
  }
]
```

### 6.3 同步GUF配置

```bash
POST /api/guf/sync-config
```

**响应**:

```json
{
  "status": "success",
  "message": "GUF配置同步成功"
}
```

## 7. 监控与管理

### 7.1 获取系统状态

```bash
GET /api/ops/status
```

**响应**:

```json
{
  "system": {
    "cpu": {
      "usage": 0.3,
      "cores": 4
    },
    "memory": {
      "used": 1024,
      "total": 4096
    },
    "disk": {
      "used": 5120,
      "total": 20480
    }
  },
  "application": {
    "requests": {
      "total": 1000,
      "per_second": 10
    },
    "errors": {
      "total": 10,
      "per_second": 0.1
    },
    "response_time": {
      "average": 100,
      "p95": 200,
      "p99": 500
    }
  }
}
```

### 7.2 获取日志

```bash
GET /api/ops/logs
```

**查询参数**:
- `level`: 日志级别 (debug, info, warn, error)
- `start_time`: 开始时间
- `end_time`: 结束时间
- `limit`: 限制条数

**响应**:

```json
{
  "logs": [
    {
      "timestamp": "2026-02-06T10:00:00Z",
      "level": "info",
      "message": "Service started successfully",
      "component": "server"
    },
    {
      "timestamp": "2026-02-06T10:01:00Z",
      "level": "warn",
      "message": "Service health check failed",
      "component": "service_discovery"
    }
  ],
  "total": 2
}
```

### 7.3 执行命令

```bash
POST /api/ops/command
```

**请求体**:

```json
{
  "command": "restart",
  "service": "user-service"
}
```

**响应**:

```json
{
  "status": "success",
  "message": "Command executed successfully"
}
```

## 8. 安全管理

### 8.1 生成密钥

```bash
POST /api/security/generate-key
```

**请求体**:

```json
{
  "type": "rsa",
  "size": 2048
}
```

**响应**:

```json
{
  "public_key": "-----BEGIN PUBLIC KEY-----...",
  "private_key": "-----BEGIN PRIVATE KEY-----..."
}
```

### 8.2 加密数据

```bash
POST /api/security/encrypt
```

**请求体**:

```json
{
  "data": "sensitive data",
  "key_id": "primary"
}
```

**响应**:

```json
{
  "encrypted_data": "base64-encoded-encrypted-data"
}
```

### 8.3 解密数据

```bash
POST /api/security/decrypt
```

**请求体**:

```json
{
  "encrypted_data": "base64-encoded-encrypted-data",
  "key_id": "primary"
}
```

**响应**:

```json
{
  "data": "sensitive data"
}
```

### 8.4 安全扫描

```bash
POST /api/security/scan
```

**请求体**:

```json
{
  "scope": "full",
  "target": "./src"
}
```

**响应**:

```json
{
  "status": "completed",
  "findings": [
    {
      "severity": "low",
      "issue": "弱密码策略",
      "location": "src/security/auth.rs:42",
      "fix": "使用更强的密码策略"
    }
  ],
  "total_findings": 1
}
```

## 9. 错误代码

### 9.1 常见错误代码

| 代码 | 描述 | 解决方案 |
|------|------|--------|
| 400 | 请求参数错误 | 检查请求参数 |
| 401 | 未授权 | 检查认证信息 |
| 403 | 禁止访问 | 检查权限配置 |
| 404 | 资源不存在 | 检查资源路径 |
| 500 | 内部服务器错误 | 查看详细日志 |
| 502 | 网关错误 | 检查后端服务 |
| 503 | 服务不可用 | 检查服务状态 |
| 504 | 网关超时 | 检查后端服务响应时间 |

### 9.2 错误响应格式

```json
{
  "error": {
    "code": 400,
    "message": "请求参数错误",
    "details": "缺少必要的参数 'name'"
  }
}
```

## 10. 速率限制

### 10.1 速率限制策略

- **默认限制**: 每IP每分钟100个请求
- **API级别限制**: 不同API可能有不同的限制
- **认证用户限制**: 认证用户可能有更高的限制

### 10.2 速率限制响应头

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1644163200
```

### 10.3 速率限制错误

```json
{
  "error": {
    "code": 429,
    "message": "速率限制 exceeded",
    "details": "请稍后再试"
  }
}
```

---

**版本**: 1.6.0  
**最后更新**: 2026-02-06  
**作者**: YMAxum Team
"#;

        fs::write(
            format!("{}/api-reference.md", api_ref_path),
            api_ref_content,
        )?;
        info!("API参考文档生成成功");

        Ok(())
    }
}
