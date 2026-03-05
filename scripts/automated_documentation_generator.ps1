#!/usr/bin/env pwsh
# 自动化文档生成脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化文档生成工具"
Write-Output "========================================"
Write-Output ""

# 检查 Rust 是否安装
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Output "错误：未找到 Rust"
    Write-Output "请先安装 Rust：https://rustup.rs/"
    exit 1
}

Write-Output "✓ Rust 已安装"
Write-Output ""

# 确保文档目录存在
$docsDir = "docs"
if (-not (Test-Path $docsDir)) {
    New-Item -ItemType Directory -Path $docsDir | Out-Null
    Write-Output "✓ 创建文档目录：$docsDir"
}

# 确保 API 文档目录存在
$apiDocsDir = "$docsDir/api"
if (-not (Test-Path $apiDocsDir)) {
    New-Item -ItemType Directory -Path $apiDocsDir | Out-Null
    Write-Output "✓ 创建 API 文档目录：$apiDocsDir"
}

# 确保模块文档目录存在
$moduleDocsDir = "$docsDir/modules"
if (-not (Test-Path $moduleDocsDir)) {
    New-Item -ItemType Directory -Path $moduleDocsDir | Out-Null
    Write-Output "✓ 创建模块文档目录：$moduleDocsDir"
}

# 确保使用指南目录存在
$guideDocsDir = "$docsDir/guides"
if (-not (Test-Path $guideDocsDir)) {
    New-Item -ItemType Directory -Path $guideDocsDir | Out-Null
    Write-Output "✓ 创建使用指南目录：$guideDocsDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "文档生成选项："
    Write-Output "1. 生成 API 文档"
    Write-Output "2. 生成模块文档"
    Write-Output "3. 生成使用指南"
    Write-Output "4. 生成完整文档套件"
    Write-Output "5. 检查文档与代码同步状态"
    Write-Output "6. 更新文档"
    Write-Output "7. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-7)"
    return $choice
}

# 生成 API 文档
function Generate-API-Docs {
    Write-Output "========================================"
    Write-Output "生成 API 文档..."
    Write-Output "========================================"
    Write-Output ""
    
    # 使用 cargo doc 生成 API 文档
    Write-Output "运行 cargo doc..."
    cargo doc --no-deps --document-private-items
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ API 文档生成成功"
        
        # 复制文档到 docs/api 目录
        Write-Output "复制 API 文档到目标目录..."
        if (Test-Path "target/doc") {
            Get-ChildItem -Path "target/doc" -Recurse | Copy-Item -Destination $apiDocsDir -Recurse -Force
            Write-Output "✓ API 文档已复制到 $apiDocsDir"
        }
        
        return $true
    } else {
        Write-Output "✗ API 文档生成失败"
        return $false
    }
}

# 生成模块文档
function Generate-Module-Docs {
    Write-Output "========================================"
    Write-Output "生成模块文档..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描 src 目录，识别所有模块
    $srcDir = "src"
    $modules = Get-ChildItem -Path $srcDir -Recurse -Filter "*.rs" | Where-Object { $_.Name -ne "main.rs" -and $_.Name -ne "lib.rs" }
    
    Write-Output "发现 $($modules.Count) 个模块文件"
    Write-Output ""
    
    # 为每个模块生成文档
    foreach ($module in $modules) {
        $modulePath = $module.FullName
        $relativePath = $modulePath.Substring($PWD.Path.Length + 1)
        $moduleName = $module.BaseName
        
        Write-Output "处理模块：$moduleName"
        
        # 读取模块内容
        $content = Get-Content -Path $modulePath -Raw
        
        # 提取模块注释
        $moduleComment = ""
        $commentMatch = [regex]::Match($content, '^//\!.*?(?=^(?:[^/])|$)', [System.Text.RegularExpressions.RegexOptions]::Singleline)
        if ($commentMatch.Success) {
            $moduleComment = $commentMatch.Value -replace '^//\!\s*', '' -replace '\n//\!\s*', "`n"
        }
        
        # 提取函数和结构体
        $functions = [regex]::Matches($content, 'pub\s+fn\s+(\w+)\s*\([^)]*\)[^\{]*\{[^\}]*\}', [System.Text.RegularExpressions.RegexOptions]::Singleline)
        $structs = [regex]::Matches($content, 'pub\s+struct\s+(\w+)[^\{]*\{[^\}]*\}', [System.Text.RegularExpressions.RegexOptions]::Singleline)
        
        # 生成模块文档
        $moduleDoc = "# $moduleName 模块文档

## 模块路径
$relativePath

## 模块描述
$moduleComment

## 函数列表

"
        
        foreach ($func in $functions) {
            $funcMatch = [regex]::Match($func.Value, 'pub\s+fn\s+(\w+)\s*\(([^)]*)\)')
            if ($funcMatch.Success) {
                $funcName = $funcMatch.Groups[1].Value
                $funcParams = $funcMatch.Groups[2].Value
                $moduleDoc += "### $funcName($funcParams)

"
            }
        }
        
        $moduleDoc += "## 结构体列表

"
        
        foreach ($struct in $structs) {
            $structMatch = [regex]::Match($struct.Value, 'pub\s+struct\s+(\w+)')
            if ($structMatch.Success) {
                $structName = $structMatch.Groups[1].Value
                $moduleDoc += "### $structName

"
            }
        }
        
        # 生成模块文档文件
        $moduleDocPath = "$moduleDocsDir/${moduleName}.md"
        $moduleDoc | Out-File -FilePath $moduleDocPath -Force
        Write-Output "✓ 模块文档已生成：$moduleDocPath"
    }
    
    Write-Output ""
    Write-Output "✓ 所有模块文档生成完成"
    return $true
}

# 生成使用指南
function Generate-Usage-Guides {
    Write-Output "========================================"
    Write-Output "生成使用指南..."
    Write-Output "========================================"
    Write-Output ""
    
    # 生成快速开始指南
    $quickStartGuide = "# 快速开始指南

## 环境搭建

1. **安装 Rust**
   访问 [Rust 官方网站](https://www.rust-lang.org/) 下载并安装最新版本的 Rust。

2. **克隆项目**
   ```bash
   git clone https://github.com/ymaxum/ymaxum.git
   cd ymaxum
   ```

3. **安装依赖**
   ```bash
   cargo build
   ```

## 项目初始化

1. **创建配置文件**
   ```bash
   cp config/server.toml.example config/server.toml
   ```

2. **编辑配置文件**
   根据您的环境需求，编辑 `config/server.toml` 文件。

3. **启动服务**
   ```bash
   cargo run
   ```

4. **检查服务状态**
   访问 `http://localhost:3000/health` 检查服务是否正常运行。

## 基本命令

### 初始化项目
```bash
INIT PROJECT NAME=my_project
```

### 安装插件
```bash
PLUGIN INSTALL PATH=plugins/output/customer_service.axpl
```

### 启用插件
```bash
PLUGIN ENABLE NAME=customer_service
```

### 启动服务
```bash
SERVICE START
```

### 停止服务
```bash
SERVICE STOP
```

## 开发流程

1. **创建新模块**
   在 `src` 目录下创建新的模块文件。

2. **编写代码**
   实现模块功能，添加适当的注释。

3. **运行测试**
   ```bash
   cargo test
   ```

4. **构建项目**
   ```bash
   cargo build --release
   ```

5. **部署项目**
   使用自动化部署脚本进行部署。
"
    
    $quickStartGuidePath = "$guideDocsDir/quick_start.md"
    $quickStartGuide | Out-File -FilePath $quickStartGuidePath -Force
    Write-Output "✓ 快速开始指南已生成：$quickStartGuidePath"
    
    # 生成插件开发指南
    $pluginGuide = "# 插件开发指南

## 插件结构

一个标准的 YMAxum 插件包含以下文件：

```
plugin_name/
├── src/
│   ├── bin/
│   │   └── main.rs       # 插件入口点
│   └── lib.rs            # 插件库
├── Cargo.toml           # 插件依赖
└── manifest.json        # 插件清单
```

## 创建插件

1. **使用插件模板**
   ```bash
   ./scripts/generate_plugin_template.ps1 -PluginName my_plugin -PluginVersion 1.0.0 -PluginAuthor "Your Name" -PluginDescription "My awesome plugin" -PluginType "general" -OutputDir plugins
   ```

2. **实现 PluginLifecycle 接口**
   ```rust
   use ymaxum::plugin::PluginLifecycle;
   
   #[derive(Debug, Clone)]
   pub struct MyPlugin;
   
   #[async_trait]
   impl PluginLifecycle for MyPlugin {
       async fn init(&self, context: &PluginContext) -> Result<(), Box<dyn std::error::Error>> {
           // 初始化插件
           Ok(())
       }
       
       async fn start(&self, context: &PluginContext) -> Result<(), Box<dyn std::error::Error>> {
           // 启动插件
           Ok(())
       }
       
       async fn stop(&self, context: &PluginContext) -> Result<(), Box<dyn std::error::Error>> {
           // 停止插件
           Ok(())
       }
   }
   ```

3. **构建插件**
   ```bash
   cd plugins/my_plugin
   cargo build --release
   ```

4. **签名插件**
   ```bash
   ./scripts/sign_plugins.ps1 -PluginPath plugins/my_plugin/target/release/my_plugin.axpl
   ```

## 安装和启用插件

1. **安装插件**
   ```bash
   PLUGIN INSTALL PATH=plugins/my_plugin/target/release/my_plugin.axpl
   ```

2. **启用插件**
   ```bash
   PLUGIN ENABLE NAME=my_plugin
   ```

3. **验证插件状态**
   ```bash
   PLUGIN STATUS NAME=my_plugin
   ```

## 插件最佳实践

1. **保持插件小巧**
   每个插件应该专注于一个特定的功能。

2. **使用依赖注入**
   通过 PluginContext 获取框架服务，而不是直接依赖。

3. **处理错误**
   适当处理和记录错误，确保插件的稳定性。

4. **遵循安全最佳实践**
   不要访问敏感资源，使用框架提供的安全接口。

5. **提供清晰的文档**
   为插件提供详细的使用文档。
"
    
    $pluginGuidePath = "$guideDocsDir/plugin_development.md"
    $pluginGuide | Out-File -FilePath $pluginGuidePath -Force
    Write-Output "✓ 插件开发指南已生成：$pluginGuidePath"
    
    # 生成 GUF 集成指南
    $gufGuide = "# GUF 集成指南

## GUF 简介

GUF (Godot UI Framework v4.4) 是一个通用的跨语言、跨平台、跨生态系统的集成框架。它提供了统一的组件模型、服务发现机制、事件总线、配置管理等核心功能，使得不同技术栈的系统能够无缝集成。

## YMAxum 中的 GUF 集成

YMAxum 框架通过以下模块实现与 GUF 的集成：

- **GUF 核心适配器**：实现与 GUF 框架的核心连接和通信
- **GUF 组件管理器**：管理 GUF 组件的生命周期和状态
- **GUF 配置同步**：实现与 GUF 配置系统的双向同步
- **GUF 事件总线**：处理 GUF 生态系统的事件通知和响应

## 配置 GUF 集成

1. **编辑 GUF 配置文件**
   ```bash
   cp config/guf.toml.example config/guf.toml
   ```

2. **配置 GUF 连接信息**
   ```toml
   [guf]
   enabled = true
   host = "localhost"
   port = 8080
   api_key = "your_api_key"
   ```

3. **启动 GUF 服务**
   如果您还没有运行 GUF 服务，请按照 GUF 官方文档启动服务。

4. **启动 YMAxum 服务**
   ```bash
   cargo run
   ```

## 使用 GUF 组件

1. **注册 GUF 组件**
   ```rust
   use ymaxum::guf::component_manager::GufComponentManager;
   
   async fn register_guf_component() {
       let component_manager = GufComponentManager::new();
       component_manager.register_component("my_component", "1.0.0").await;
   }
   ```

2. **调用 GUF 组件**
   ```rust
   use ymaxum::guf::component_manager::GufComponentManager;
   
   async fn call_guf_component() {
       let component_manager = GufComponentManager::new();
       let result = component_manager.call_component("my_component", "method", serde_json::json!({"param": "value"})).await;
   }
   ```

3. **订阅 GUF 事件**
   ```rust
   use ymaxum::guf::event_bus::GufEventBus;
   
   async fn subscribe_to_guf_events() {
       let event_bus = GufEventBus::new();
       event_bus.subscribe("my_event", |event| {
           println!("Received event: {:?}", event);
       }).await;
   }
   ```

## GUF 模板使用

YMAxum 框架提供了多种 GUF 行业模板，您可以通过以下方式使用：

1. **列出可用模板**
   ```rust
   use ymaxum::guf::templates::GufTemplateLibrary;
   
   async fn list_guf_templates() {
       let template_library = GufTemplateLibrary::new();
       let templates = template_library.list_templates().await;
       for template in templates {
           println!("Template: {} - {}", template.id, template.name);
       }
   }
   ```

2. **使用模板创建应用**
   ```rust
   use ymaxum::guf::templates::GufTemplateLibrary;
   
   async fn create_app_from_template() {
       let template_library = GufTemplateLibrary::new();
       let app = template_library.create_from_template("finance_dashboard", serde_json::json!({
           "app_name": "My Finance App",
           "theme": "dark"
       })).await;
   }
   ```

## 故障排除

1. **检查 GUF 服务状态**
   确保 GUF 服务正在运行，并且可以通过配置的地址访问。

2. **检查网络连接**
   确保 YMAxum 服务可以连接到 GUF 服务。

3. **检查 API 密钥**
   确保配置文件中的 API 密钥正确。

4. **查看日志**
   查看 YMAxum 服务的日志，了解 GUF 集成的详细信息。
"
    
    $gufGuidePath = "$guideDocsDir/guf_integration.md"
    $gufGuide | Out-File -FilePath $gufGuidePath -Force
    Write-Output "✓ GUF 集成指南已生成：$gufGuidePath"
    
    Write-Output ""
    Write-Output "✓ 所有使用指南生成完成"
    return $true
}

# 检查文档与代码同步状态
function Check-Docs-Code-Sync {
    Write-Output "========================================"
    Write-Output "检查文档与代码同步状态..."
    Write-Output "========================================"
    Write-Output ""
    
    # 检查 API 文档是否存在
    if (Test-Path "$apiDocsDir/index.html") {
        Write-Output "✓ API 文档存在"
    } else {
        Write-Output "✗ API 文档不存在"
    }
    
    # 检查模块文档是否存在
    $moduleDocs = Get-ChildItem -Path $moduleDocsDir -Filter "*.md"
    if ($moduleDocs.Count -gt 0) {
        Write-Output "✓ 模块文档存在 ($($moduleDocs.Count) 个)"
    } else {
        Write-Output "✗ 模块文档不存在"
    }
    
    # 检查使用指南是否存在
    $guideDocs = Get-ChildItem -Path $guideDocsDir -Filter "*.md"
    if ($guideDocs.Count -gt 0) {
        Write-Output "✓ 使用指南存在 ($($guideDocs.Count) 个)"
    } else {
        Write-Output "✗ 使用指南不存在"
    }
    
    # 检查文档更新时间
    $docsFiles = Get-ChildItem -Path $docsDir -Recurse -Filter "*.md" | Sort-Object LastWriteTime -Descending
    if ($docsFiles.Count -gt 0) {
        $latestDoc = $docsFiles[0]
        $latestDocTime = $latestDoc.LastWriteTime
        $now = Get-Date
        $timeDiff = $now - $latestDocTime
        
        if ($timeDiff.TotalDays -lt 7) {
            Write-Output "✓ 文档最近已更新 ($($timeDiff.TotalHours.ToString("0.0")) 小时前)"
        } else {
            Write-Output "⚠ 文档可能需要更新 ($($timeDiff.TotalDays.ToString("0.0")) 天前更新)"
        }
    }
    
    Write-Output ""
    Write-Output "✓ 文档与代码同步状态检查完成"
    return $true
}

# 更新文档
function Update-Docs {
    Write-Output "========================================"
    Write-Output "更新文档..."
    Write-Output "========================================"
    Write-Output ""
    
    # 生成完整文档套件
    $apiDocsResult = Generate-API-Docs
    $moduleDocsResult = Generate-Module-Docs
    $guideDocsResult = Generate-Usage-Guides
    
    if ($apiDocsResult -and $moduleDocsResult -and $guideDocsResult) {
        Write-Output ""
        Write-Output "✓ 所有文档更新完成"
        return $true
    } else {
        Write-Output ""
        Write-Output "✗ 部分文档更新失败"
        return $false
    }
}

# 生成完整文档套件
function Generate-Complete-Docs {
    Write-Output "========================================"
    Write-Output "生成完整文档套件..."
    Write-Output "========================================"
    Write-Output ""
    
    # 生成所有类型的文档
    $apiDocsResult = Generate-API-Docs
    $moduleDocsResult = Generate-Module-Docs
    $guideDocsResult = Generate-Usage-Guides
    
    if ($apiDocsResult -and $moduleDocsResult -and $guideDocsResult) {
        Write-Output ""
        Write-Output "✓ 完整文档套件生成完成"
        return $true
    } else {
        Write-Output ""
        Write-Output "✗ 完整文档套件生成失败"
        return $false
    }
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            Generate-API-Docs
        }
        "2" {
            Generate-Module-Docs
        }
        "3" {
            Generate-Usage-Guides
        }
        "4" {
            Generate-Complete-Docs
        }
        "5" {
            Check-Docs-Code-Sync
        }
        "6" {
            Update-Docs
        }
        "7" {
            Write-Output "退出文档生成工具..."
            break
        }
        default {
            Write-Output "无效选择，请重新输入"
        }
    }
    
    Write-Output ""
    Read-Host "按 Enter 键继续..."
    Write-Output ""
}
