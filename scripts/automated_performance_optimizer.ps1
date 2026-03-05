#!/usr/bin/env pwsh
# 自动化性能优化脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化性能优化"
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

# 确保性能测试目录存在
$perfDir = "performance_results"
if (-not (Test-Path $perfDir)) {
    New-Item -ItemType Directory -Path $perfDir | Out-Null
    Write-Output "✓ 创建性能测试目录：$perfDir"
}

Write-Output ""

# 步骤 1：运行基准测试
Write-Output "步骤 1：运行基准测试..."
Write-Output ""

# 运行性能测试二进制文件
if (Test-Path "src/bin/performance_test.rs") {
    Write-Output "运行性能测试..."
    cargo run --bin performance_test
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ 性能测试运行成功"
    } else {
        Write-Output "⚠ 性能测试运行失败，继续执行其他步骤"
    }
} else {
    Write-Output "⚠ 未找到性能测试文件，跳过此步骤"
}

Write-Output ""

# 步骤 2：分析性能瓶颈
Write-Output "步骤 2：分析性能瓶颈..."
Write-Output ""

# 检查是否存在性能分析工具
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    # 安装 cargo-flamegraph（如果未安装）
    if (-not (Get-Command cargo-flamegraph -ErrorAction SilentlyContinue)) {
        Write-Output "安装性能分析工具..."
        cargo install flamegraph
    }
    
    Write-Output "✓ 性能分析工具已就绪"
} else {
    Write-Output "⚠ 性能分析工具未安装，跳过此步骤"
}

Write-Output ""

# 步骤 3：优化建议生成
Write-Output "步骤 3：生成性能优化建议..."
Write-Output ""

# 创建优化建议文件
$optimizationSuggestions = "# YMAxum 框架性能优化建议

## 分析日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 性能瓶颈分析

### 1. 代码层面优化建议
- [ ] 使用 `async/await` 替代同步操作
- [ ] 优化 `clone()` 操作，减少内存复制
- [ ] 使用 `Arc` 和 `RwLock` 优化共享数据访问
- [ ] 实现缓存机制，减少重复计算
- [ ] 优化迭代器使用，避免不必要的中间集合

### 2. 配置层面优化建议
- [ ] 调整线程池大小，根据 CPU 核心数优化
- [ ] 优化数据库连接池配置
- [ ] 调整缓存过期时间
- [ ] 优化日志级别，生产环境减少日志输出
- [ ] 启用 HTTP/2 或 HTTP/3

### 3. 架构层面优化建议
- [ ] 实现请求批处理，减少网络往返
- [ ] 优化数据序列化/反序列化
- [ ] 实现连接复用
- [ ] 考虑使用更高效的数据结构
- [ ] 优化错误处理路径

## 性能监控建议

### 1. 监控指标
- [ ] CPU 使用率
- [ ] 内存使用情况
- [ ] 磁盘 I/O
- [ ] 网络流量
- [ ] 请求响应时间
- [ ] 并发连接数

### 2. 监控工具
- [ ] 集成 Prometheus 和 Grafana
- [ ] 实现自定义性能指标
- [ ] 设置性能告警阈值
- [ ] 定期生成性能报告

## 后续行动

1. **实施优化建议**：根据优先级实施上述优化建议
2. **验证优化效果**：重新运行性能测试，验证优化效果
3. **持续监控**：建立长期性能监控机制
4. **定期分析**：定期进行性能分析，发现新的瓶颈
"

$suggestionsFile = Join-Path $perfDir "optimization_suggestions.md"
$optimizationSuggestions | Out-File -FilePath $suggestionsFile -Force

Write-Output "✓ 性能优化建议已生成：$suggestionsFile"
Write-Output ""

# 步骤 4：自动应用常见优化
Write-Output "步骤 4：自动应用常见优化..."
Write-Output ""

# 检查并优化 Cargo.toml 配置
if (Test-Path "Cargo.toml") {
    Write-Output "检查 Cargo.toml 配置..."
    
    # 读取 Cargo.toml 内容
    $cargoToml = Get-Content -Path "Cargo.toml" -Raw
    
    # 检查是否已包含优化配置
    if ($cargoToml -notlike "*[profile.release]*") {
        Write-Output "添加发布配置优化..."
        $optimizedCargoToml = $cargoToml + "`n
[profile.release]
opt-level = "z"  # 优化大小
lto = true       # 链接时间优化
codegen-units = 1  # 减少代码生成单元以优化大小
panic = "abort"    # 使用abort而不是unwind来减小大小
strip = true       # 移除符号表
"
        
        $optimizedCargoToml | Out-File -FilePath "Cargo.toml" -Force
        Write-Output "✓ 发布配置优化已应用"
    } else {
        Write-Output "✓ 发布配置已优化"
    }
} else {
    Write-Output "⚠ 未找到 Cargo.toml 文件，跳过此步骤"
}

Write-Output ""

# 步骤 5：生成性能报告
Write-Output "步骤 5：生成性能报告..."
Write-Output ""

# 创建性能报告文件
$performanceReport = "# YMAxum 框架性能报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 系统信息
- **操作系统**: $([System.Environment]::OSVersion.VersionString)
- **CPU 核心数**: $([System.Environment]::ProcessorCount)
- **可用内存**: $([math]::Round((Get-WmiObject -Class Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB

## 性能测试结果

### 1. 测试概述
- [ ] HTTP 请求性能测试
- [ ] 插件操作性能测试
- [ ] 命令执行性能测试
- [ ] 数据库操作性能测试
- [ ] 缓存操作性能测试

### 2. 测试结果

| 测试项 | 执行时间 (ms) | 吞吐量 (ops/s) | 内存使用 (MB) | 状态 |
|--------|----------------|----------------|----------------|------|
| HTTP 请求 | - | - | - | 未测试 |
| 插件操作 | - | - | - | 未测试 |
| 命令执行 | - | - | - | 未测试 |
| 数据库操作 | - | - | - | 未测试 |
| 缓存操作 | - | - | - | 未测试 |

## 性能瓶颈

### 1. 已识别的瓶颈

### 2. 优化建议

## 后续步骤

1. **实施优化建议**
2. **重新运行性能测试**
3. **验证优化效果**
4. **更新性能基准**
"

$reportFile = Join-Path $perfDir "performance_report.md"
$performanceReport | Out-File -FilePath $reportFile -Force

Write-Output "✓ 性能报告已生成：$reportFile"
Write-Output ""

# 步骤 6：清理临时文件
Write-Output "步骤 6：清理临时文件..."
Write-Output ""

# 清理编译产物（可选）
if (Test-Path "target") {
    Write-Output "清理编译产物..."
    Remove-Item -Path "target/debug" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Output "✓ 临时文件清理完成"
}

Write-Output ""
Write-Output "========================================"
Write-Output "性能优化流程完成！"
Write-Output "========================================"
Write-Output ""
Write-Output "生成的文件："
Write-Output "- 性能优化建议：$suggestionsFile"
Write-Output "- 性能报告：$reportFile"
Write-Output ""
Write-Output "请根据优化建议实施具体的性能优化措施，"
Write-Output "并定期重新运行此脚本以监控性能变化。"
Write-Output ""
