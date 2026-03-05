# YMAxum 框架 GUF 集成文档

## 1. 概述

本文档详细描述 YMAxum 框架与 GUF (Godot UI Framework v4.4) 的集成方案，包括集成架构、配置管理、组件交互等内容。本文档旨在帮助开发者了解和使用 YMAxum 框架的 GUF 集成功能，加速应用开发。

## 2. GUF 集成架构

### 2.1 整体架构

YMAxum 框架与 GUF 的集成采用适配器模式，通过 `GufAdapter` 组件实现两者之间的通信和数据交换。整体架构如下：

```
+-------------------+
|  YMAxum 框架      |
|                   |
| +---------------+ |
| | GufAdapter    | | ←→ 双向通信 ←→ | GUF (Godot) |
| +---------------+ |
| | - 服务注册     | |
| | - 方法调用     | |
| | - 事件通知     | |
| +---------------+ |
|                   |
+-------------------+
```

### 2.2 核心组件

- **GufAdapter**：负责与 GUF 进行通信，处理服务注册、方法调用和事件通知
- **GufService**：表示一个 GUF 服务，包含服务名称、配置和方法
- **GufEvent**：表示一个 GUF 事件，包含事件名称和数据

## 3. 集成配置

### 3.1 配置文件

GUF 集成的配置文件位于 `config/guf.toml`，示例配置如下：

```toml
[guf]
enabled = true
host = "localhost"
port = 8080
protocol = "http"
timeout = 30

[guf.services]
java_service = {
    host = "localhost",
    port = 8081,
    protocol = "http",
    timeout = 30
}

python_service = {
    host = "localhost",
    port = 8082,
    protocol = "http",
    timeout = 30
}

node_service = {
    host = "localhost",
    port = 8083,
    protocol = "http",
    timeout = 30
}
```

### 3.2 环境变量

GUF 集成也支持通过环境变量进行配置，优先级高于配置文件：

- `GUF_ENABLED`：是否启用 GUF 集成
- `GUF_HOST`：GUF 主机地址
- `GUF_PORT`：GUF 端口号
- `GUF_PROTOCOL`：GUF 协议
- `GUF_TIMEOUT`：GUF 超时时间

## 4. 服务注册与发现

### 4.1 注册服务

通过 `register_service` 函数注册 GUF 服务：

```rust
use ymaxum::guf::service::register_service;

let config = serde_json::json!({
    "host": "localhost",
    "port": 8080,
    "protocol": "http"
});

register_service("java_service", config).await?;
```

### 4.2 发现服务

通过 `discover_services` 函数发现 GUF 服务：

```rust
use ymaxum::guf::service::discover_services;

let services = discover_services().await?;
```

## 5. 方法调用

### 5.1 同步调用

通过 `call_service` 函数同步调用 GUF 服务方法：

```rust
use ymaxum::guf::service::call_service;

let result = call_service(
    "java_service",
    "get_user",
    serde_json::json!({
        "user_id": "user1"
    })
).await?;
```

### 5.2 异步调用

通过 `call_service_async` 函数异步调用 GUF 服务方法：

```rust
use ymaxum::guf::service::call_service_async;

let task = call_service_async(
    "java_service",
    "get_user",
    serde_json::json!({
        "user_id": "user1"
    })
).await?;

// 稍后获取结果
let result = task.await?;
```

## 6. 事件通知

### 6.1 发布事件

通过 `publish_event` 函数发布 GUF 事件：

```rust
use ymaxum::guf::event::publish_event;

publish_event(
    "user_created",
    serde_json::json!({
        "user_id": "user1",
        "user_name": "John Doe"
    })
).await?;
```

### 6.2 订阅事件

通过 `subscribe_event` 函数订阅 GUF 事件：

```rust
use ymaxum::guf::event::subscribe_event;

subscribe_event("user_created", |event| {
    println!("User created: {:?}", event.data);
}).await?;
```

## 7. 错误处理

### 7.1 错误类型

GUF 集成的错误类型包括：

- `GufError`：GUF 集成错误
- `ServiceError`：服务调用错误
- `EventError`：事件处理错误

### 7.2 错误处理示例

```rust
use ymaxum::guf::error::GufError;

match call_service("java_service", "get_user", params).await {
    Ok(result) => println!("Result: {:?}", result),
    Err(error) => match error {
        GufError::ServiceError(err) => println!("Service error: {:?}", err),
        GufError::EventError(err) => println!("Event error: {:?}", err),
        GufError::ConfigError(err) => println!("Config error: {:?}", err),
        _ => println!("Unknown error: {:?}", error),
    },
}
```

## 8. 性能优化

### 8.1 连接池

GUF 集成使用连接池来管理与 GUF 的连接，减少连接建立的开销：

```toml
[guf.connection_pool]
size = 10
max_idle_time = 60
```

### 8.2 缓存

GUF 集成支持缓存服务调用结果，提高响应速度：

```toml
[guf.cache]
enabled = true
size = 1000
expiration = 300
```

### 8.3 批量调用

通过 `batch_call` 函数批量调用 GUF 服务方法，减少网络开销：

```rust
use ymaxum::guf::service::batch_call;

let calls = vec!(
    ("java_service", "get_user", serde_json::json!({
        "user_id": "user1"
    })),
    ("java_service", "get_user", serde_json::json!({
        "user_id": "user2"
    }))
);

let results = batch_call(calls).await?;
```

## 9. 安全防护

### 9.1 认证授权

GUF 集成支持认证授权机制，确保服务调用的安全性：

```toml
[guf.security]
enabled = true
token = "your-secret-token"
```

### 9.2 加密通信

GUF 集成支持 HTTPS 加密通信，保护数据传输安全：

```toml
[guf]
protocol = "https"

[guf.ssl]
cert_path = "path/to/cert.pem"
key_path = "path/to/key.pem"
```

## 10. 监控与日志

### 10.1 监控指标

GUF 集成提供以下监控指标：

- `guf_service_calls_total`：服务调用总数
- `guf_service_call_duration_seconds`：服务调用持续时间
- `guf_service_call_errors_total`：服务调用错误总数
- `guf_event_published_total`：事件发布总数
- `guf_event_subscribed_total`：事件订阅总数

### 10.2 日志

GUF 集成的日志级别可以在配置文件中设置：

```toml
[guf.log]
level = "info"
file = "logs/guf.log"
```

## 11. 示例代码

### 11.1 基本示例

```rust
use ymaxum::guf::adapter::GufAdapter;
use ymaxum::guf::service::register_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 GUF 适配器
    let adapter = GufAdapter::new();
    
    // 初始化 GUF 适配器
    let config = serde_json::json!({
        "host": "localhost",
        "port": 8080
    });
    adapter.init(config).await?;
    
    // 注册服务
    let java_service_config = serde_json::json!({
        "host": "localhost",
        "port": 8081,
        "protocol": "http"
    });
    register_service("java_service", java_service_config).await?;
    
    // 调用服务方法
    let result = adapter.call_service(
        "java_service",
        "get_user",
        serde_json::json!({
            "user_id": "user1"
        })
    ).await?;
    
    println!("Result: {:?}", result);
    
    Ok(())
}
```

### 11.2 事件处理示例

```rust
use ymaxum::guf::event::publish_event;
use ymaxum::guf::event::subscribe_event;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 订阅事件
    subscribe_event("user_created", |event| {
        println!("User created: {:?}", event.data);
    }).await?;
    
    // 发布事件
    publish_event(
        "user_created",
        serde_json::json!({
            "user_id": "user1",
            "user_name": "John Doe"
        })
    ).await?;
    
    Ok(())
}
```

## 12. 常见问题

### 12.1 服务调用失败

**问题**：服务调用失败，返回错误信息。

**解决方案**：
- 检查服务是否正常运行
- 检查服务地址和端口是否正确
- 检查网络连接是否正常
- 检查服务认证信息是否正确

### 12.2 事件发布失败

**问题**：事件发布失败，返回错误信息。

**解决方案**：
- 检查 GUF 适配器是否初始化成功
- 检查事件名称和数据是否符合要求
- 检查网络连接是否正常

### 12.3 性能问题

**问题**：服务调用响应缓慢。

**解决方案**：
- 启用连接池和缓存
- 使用批量调用减少网络开销
- 优化服务实现，提高响应速度
- 增加服务器资源，提高处理能力

## 13. 版本历史

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| 1.0.0 | 2026-01-20 | 初始版本，支持基本的 GUF 集成功能 |
| 1.1.0 | 2026-01-31 | 添加连接池和缓存支持 |
| 1.2.0 | 2026-02-04 | 添加批量调用和异步调用支持 |
| 1.3.0 | 2026-02-04 | 添加事件处理和监控支持 |
| 1.4.0 | 2026-02-05 | 添加安全防护和错误处理优化 |

## 14. 联系方式

如果您在使用 YMAxum 框架的 GUF 集成功能过程中遇到问题，可以通过以下方式联系我们：

- **GitHub Issues**：https://github.com/ymaxum/ymaxum/issues
- **官方文档**：https://docs.ymaxum.com
- **社区论坛**：https://forum.ymaxum.com
- **电子邮件**：support@ymaxum.com

我们会及时回复您的问题，并不断完善 YMAxum 框架的 GUF 集成功能。