# GUF 插件模板

这是一个标准化的 GUF (Godot UI Framework v4.4) 插件开发模板，用于快速创建兼容 GUF 生态系统的插件。

## 目录结构

```
guf_plugin_template/
├── Cargo.toml          # 项目配置文件
├── README.md           # 插件开发指南
├── src/
│   ├── lib.rs          # 插件核心实现
│   └── bin/
│       └── main.rs     # 插件示例
└── signature.json      # 插件签名文件（构建时生成）
```

## 开发环境要求

- Rust 1.70+（推荐使用最新稳定版本）
- Cargo（Rust 包管理器）
- Tokio 运行时
- YMAxum 框架

## 快速开始

### 1. 复制模板

将此模板复制到新的目录中，并修改以下文件：

- `Cargo.toml`：修改插件名称、版本、作者等信息
- `src/lib.rs`：修改插件实现逻辑
- `src/bin/main.rs`：修改示例代码（可选）

### 2. 配置依赖

确保 `Cargo.toml` 中的依赖项正确配置，特别是 YMAxum 框架的路径。

### 3. 实现插件逻辑

在 `src/lib.rs` 中，您需要：

- 修改 `PluginManifest` 结构体中的插件信息
- 实现 `GufPluginTemplate` 结构体的方法
- 自定义事件处理和服务调用逻辑

### 4. 构建插件

```bash
# 构建插件库
cargo build --release

# 构建示例（可选）
cargo run --bin guf_plugin_template
```

### 5. 签名插件

插件需要使用 RSA2048 签名才能被加载：

```bash
# 生成签名文件
target/release/ymaxum --sign-plugin path/to/plugin
```

### 6. 安装插件

将构建好的插件和签名文件复制到 YMAxum 的插件目录：

```bash
# 复制插件到插件目录
cp target/release/libguf_plugin_template.so plugins/
cp signature.json plugins/

# 安装插件
target/release/ymaxum --install-plugin plugins/guf_plugin_template
```

## API 文档

### 插件生命周期方法

- `initialize()`：初始化插件，设置 GUF 集成
- `start()`：启动插件，开始处理事件和服务调用
- `stop()`：停止插件，清理资源
- `uninstall()`：卸载插件，完全清理

### GUF 集成方法

- `check_guf_status()`：检查 GUF 集成状态
- `handle_guf_event()`：处理 GUF 事件
- `call_guf_service()`：调用 GUF 服务

### 插件入口点

插件必须实现以下 C 导出函数：

- `plugin_create()`：创建插件实例
- `plugin_initialize()`：初始化插件
- `plugin_start()`：启动插件
- `plugin_stop()`：停止插件
- `plugin_uninstall()`：卸载插件
- `plugin_get_info()`：获取插件信息

## 最佳实践

1. **错误处理**：使用 `anyhow` 库进行错误处理，确保插件在遇到错误时能够优雅降级
2. **日志记录**：使用适当的日志级别记录插件的运行状态和错误信息
3. **性能优化**：
   - 避免在事件处理中执行耗时操作
   - 使用异步处理提高并发性能
   - 合理使用缓存减少重复计算
4. **安全考虑**：
   - 不要在插件中硬编码敏感信息
   - 验证所有输入参数
   - 遵循最小权限原则
5. **兼容性**：
   - 确保插件与指定版本的 GUF 兼容
   - 处理版本差异和向后兼容性

## 示例插件

此模板包含一个完整的示例插件，展示了如何：

- 初始化和启动 GUF 集成
- 处理 GUF 事件
- 调用 GUF 服务
- 管理插件生命周期

您可以运行示例来测试插件的基本功能：

```bash
cargo run --bin guf_plugin_template
```

## 调试技巧

1. **日志调试**：在插件中添加详细的日志输出，帮助定位问题
2. **断点调试**：使用 Rust 的调试器（如 gdb 或 lldb）设置断点
3. **测试模式**：在开发过程中使用测试模式运行插件，以便快速迭代
4. **GUF 集成测试**：使用 YMAxum 的 GUF 集成测试工具测试插件与 GUF 的交互

## 常见问题

### 1. 插件无法加载

- 检查插件是否已签名
- 验证签名文件是否与插件匹配
- 确保插件与 YMAxum 版本兼容

### 2. GUF 集成失败

- 检查 GUF 配置文件是否正确
- 验证网络连接是否正常
- 查看 GUF 服务是否可用

### 3. 性能问题

- 优化事件处理逻辑
- 减少网络调用次数
- 使用批处理和缓存

## 联系方式

如果您在开发过程中遇到问题，请参考以下资源：

- YMAxum 框架文档
- GUF 官方文档
- 插件开发社区

## 许可证

此模板使用 MIT 许可证，您可以自由修改和分发。
