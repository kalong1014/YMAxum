# 插件系统开发指南

## 1. 插件系统架构

YMAxum框架的插件系统采用模块化设计，支持以下核心功能：

- **插件生命周期管理**：安装、启用、停用、卸载、更新
- **安全隔离**：CPU、内存、磁盘I/O、网络I/O限制
- **签名验证**：RSA2048签名验证和可信白名单
- **跨语言支持**：Rust、JavaScript、Python、Go、Java、C#
- **通信总线**：插件间安全高效通信
- **市场集成**：插件搜索、下载、评分和反馈

## 2. 插件开发流程

### 2.1 插件结构

每个插件应包含以下文件：

- `manifest.json`：插件清单文件
- `signature.json`：插件签名文件
- 插件代码文件（根据语言不同而不同）

### 2.2 插件清单格式

```json
{
  "name": "plugin_name",
  "version": "1.0.0",
  "author": "Author Name",
  "description": "Plugin description",
  "plugin_type": "basic",
  "dependencies": [],
  "core_version": "1.0.0",
  "entry_file": "main.rs",
  "config_file": null,
  "signature_file": "signature.json",
  "license": "MIT",
  "routes": null
}
```

### 2.3 插件签名

插件必须使用RSA2048算法签名，签名文件格式：

```json
{
  "algorithm": "RSA2048",
  "signer": "YMAxum Framework",
  "timestamp": 1620000000,
  "signature": "base64_encoded_signature",
  "plugin_hash": "base64_encoded_hash",
  "signature_id": "unique_id"
}
```

## 3. 各语言插件示例

### 3.1 Rust插件示例

```rust
// src/lib.rs
use ymaxum::plugin::PluginLifecycle;

struct MyPlugin;

impl PluginLifecycle for MyPlugin {
    fn init(&self) -> Result<(), String> {
        println!("Plugin initialized");
        Ok(())
    }
    
    fn start(&self) -> Result<(), String> {
        println!("Plugin started");
        Ok(())
    }
    
    fn stop(&self) -> Result<(), String> {
        println!("Plugin stopped");
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn PluginLifecycle> {
    Box::new(MyPlugin)
}
```

### 3.2 JavaScript插件示例

```javascript
// plugin.js
class MyPlugin {
    init() {
        console.log('Plugin initialized');
        return Promise.resolve();
    }
    
    start() {
        console.log('Plugin started');
        return Promise.resolve();
    }
    
    stop() {
        console.log('Plugin stopped');
        return Promise.resolve();
    }
}

module.exports = MyPlugin;
```

### 3.3 Python插件示例

```python
# plugin.py
class MyPlugin:
    def init(self):
        print('Plugin initialized')
        return True
    
    def start(self):
        print('Plugin started')
        return True
    
    def stop(self):
        print('Plugin stopped')
        return True

plugin = MyPlugin()
```

### 3.4 Go插件示例

```go
// main.go
package main

import "fmt"

type MyPlugin struct{}

func (p *MyPlugin) Init() error {
    fmt.Println("Plugin initialized")
    return nil
}

func (p *MyPlugin) Start() error {
    fmt.Println("Plugin started")
    return nil
}

func (p *MyPlugin) Stop() error {
    fmt.Println("Plugin stopped")
    return nil
}

func CreatePlugin() interface{} {
    return &MyPlugin{}
}
```

### 3.5 Java插件示例

```java
// MyPlugin.java
public class MyPlugin {
    public boolean init() {
        System.out.println("Plugin initialized");
        return true;
    }
    
    public boolean start() {
        System.out.println("Plugin started");
        return true;
    }
    
    public boolean stop() {
        System.out.println("Plugin stopped");
        return true;
    }
}
```

### 3.6 C#插件示例

```csharp
// MyPlugin.cs
public class MyPlugin {
    public bool Init() {
        Console.WriteLine("Plugin initialized");
        return true;
    }
    
    public bool Start() {
        Console.WriteLine("Plugin started");
        return true;
    }
    
    public bool Stop() {
        Console.WriteLine("Plugin stopped");
        return true;
    }
}
```

## 4. 插件配置和部署

### 4.1 插件配置

插件可以通过`config.toml`文件进行配置：

```toml
[plugin]
name = "my_plugin"
version = "1.0.0"
author = "Author Name"

[plugin.settings]
enabled = true
debug = false

[plugin.resources]
max_cpu = 20
max_memory = 50
```

### 4.2 插件部署

1. **打包插件**：将插件文件打包为`.axpl`压缩包
2. **签名插件**：使用`sign_plugins.bat`或`sign_plugins.sh`脚本签名
3. **安装插件**：使用插件管理器安装

```bash
# 签名插件
bash sign_plugins.sh my_plugin

# 安装插件
./ymaxum plugin install my_plugin.axpl
```

## 5. 插件通信

### 5.1 发送消息

```rust
let message = PluginMessage {
    message_type: "test_message".to_string(),
    data: serde_json::json!({ "key": "value" }),
    sender: "plugin_a".to_string(),
    target: Some("plugin_b".to_string()),
    message_id: "unique_id".to_string(),
    timestamp: chrono::Utc::now().timestamp() as u64,
    signature: None,
    priority: 128,
};

bus.send_message(message).await?;
```

### 5.2 订阅消息

```rust
let mut receiver = bus.subscribe("plugin_b", "test_message").await?;

while let Ok(message) = receiver.recv().await {
    println!("Received message: {:?}", message);
}
```

## 6. 插件市场

### 6.1 发布插件

1. **准备插件**：确保插件符合市场要求
2. **提交插件**：通过市场API提交插件
3. **验证插件**：市场管理员验证插件
4. **发布插件**：插件上线

### 6.2 搜索和下载插件

```rust
// 搜索插件
let result = marketplace.search_plugins("analytics", None, 1, 10).await;

// 下载插件
let plugin_data = marketplace.download_plugin("analytics_plugin").await?;
```

## 7. 最佳实践

### 7.1 安全最佳实践

- **最小权限原则**：只申请必要的权限
- **输入验证**：验证所有用户输入
- **错误处理**：妥善处理错误，不暴露敏感信息
- **资源管理**：合理使用系统资源

### 7.2 性能最佳实践

- **懒加载**：按需加载插件资源
- **缓存**：合理使用缓存
- **异步处理**：使用异步操作处理耗时任务
- **批量操作**：使用批量API减少网络请求

### 7.3 开发最佳实践

- **代码质量**：遵循语言编码规范
- **测试**：编写单元测试和集成测试
- **文档**：提供详细的文档
- **版本控制**：使用语义化版本

## 8. 故障排除

### 8.1 常见问题

- **签名验证失败**：检查签名文件和公钥配置
- **资源限制**：检查插件资源使用情况
- **依赖缺失**：确保所有依赖已安装
- **权限不足**：检查插件权限配置

### 8.2 日志和监控

- **日志**：插件应使用标准日志接口
- **监控**：定期检查插件状态和资源使用
- **错误报告**：及时报告和处理错误

## 9. 版本兼容性

- **核心版本**：插件应指定兼容的核心版本
- **API变更**：关注API变更公告
- **向后兼容**：尽量保持向后兼容

## 10. 结语

YMAxum插件系统为开发者提供了一个灵活、安全、高效的插件开发框架。通过遵循本指南，您可以开发出高质量的插件，为YMAxum生态系统贡献力量。