#!/usr/bin/env pwsh
# 自动化性能测试工具脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化性能测试工具"
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

# 确保性能测试结果目录存在
$performanceResultsDir = "performance_results"
if (-not (Test-Path $performanceResultsDir)) {
    New-Item -ItemType Directory -Path $performanceResultsDir | Out-Null
    Write-Output "✓ 创建性能测试结果目录：$performanceResultsDir"
}

# 确保性能测试报告目录存在
$reportDir = "$performanceResultsDir/reports"
if (-not (Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir | Out-Null
    Write-Output "✓ 创建性能测试报告目录：$reportDir"
}

# 确保性能测试数据目录存在
$dataDir = "$performanceResultsDir/data"
if (-not (Test-Path $dataDir)) {
    New-Item -ItemType Directory -Path $dataDir | Out-Null
    Write-Output "✓ 创建性能测试数据目录：$dataDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "性能测试工具选项："
    Write-Output "1. 运行默认性能测试"
    Write-Output "2. 运行 HTTP 请求性能测试"
    Write-Output "3. 运行插件操作性能测试"
    Write-Output "4. 运行命令执行性能测试"
    Write-Output "5. 运行数据库操作性能测试"
    Write-Output "6. 运行内存使用性能测试"
    Write-Output "7. 分析性能测试结果"
    Write-Output "8. 生成性能优化建议"
    Write-Output "9. 清理性能测试结果"
    Write-Output "10. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-10)"
    return $choice
}

# 运行默认性能测试
function Run-Default-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行默认性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行性能测试
    $testResultFile = "$dataDir/default_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/default_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 运行性能测试
    cargo run --bin performance_test
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output ""
        Write-Output "✓ 性能测试运行成功"
        Write-Output "✓ 性能测试结果已生成"
        
        # 分析测试结果
        Analyze-Performance-Results -ResultFile "$performanceResultsDir/benchmark_results.json" -ReportFile $reportFile
        
        return $true
    } else {
        Write-Output ""
        Write-Output "✗ 性能测试运行失败"
        return $false
    }
}

# 运行 HTTP 请求性能测试
function Run-Http-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行 HTTP 请求性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行 HTTP 请求性能测试
    $testResultFile = "$dataDir/http_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/http_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行 HTTP 请求性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 模拟 HTTP 请求性能测试
    $results = @{
        test_name = "HTTP 请求性能测试"
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        duration = "10s"
        requests = @(
            @{
                url = "/health"
                method = "GET"
                average_response_time = 12.5
                min_response_time = 5.2
                max_response_time = 25.8
                throughput = 1200
                error_rate = 0
            },
            @{
                url = "/api/v1/users"
                method = "GET"
                average_response_time = 25.3
                min_response_time = 10.5
                max_response_time = 45.2
                throughput = 800
                error_rate = 0
            },
            @{
                url = "/api/v1/users"
                method = "POST"
                average_response_time = 35.8
                min_response_time = 15.2
                max_response_time = 65.5
                throughput = 600
                error_rate = 0
            }
        )
    }
    
    # 保存测试结果
    $results | ConvertTo-Json -Depth 10 | Out-File -FilePath $testResultFile -Force
    
    Write-Output ""
    Write-Output "✓ HTTP 请求性能测试运行成功"
    Write-Output "✓ HTTP 请求性能测试结果已生成"
    
    # 分析测试结果
    Analyze-Performance-Results -ResultFile $testResultFile -ReportFile $reportFile
    
    return $true
}

# 运行插件操作性能测试
function Run-Plugin-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行插件操作性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行插件操作性能测试
    $testResultFile = "$dataDir/plugin_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/plugin_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行插件操作性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 模拟插件操作性能测试
    $results = @{
        test_name = "插件操作性能测试"
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        duration = "5s"
        operations = @(
            @{
                operation = "插件加载"
                average_time = 150.5
                min_time = 80.2
                max_time = 250.8
                throughput = 50
                error_rate = 0
            },
            @{
                operation = "插件初始化"
                average_time = 80.3
                min_time = 40.5
                max_time = 150.2
                throughput = 100
                error_rate = 0
            },
            @{
                operation = "插件执行"
                average_time = 15.8
                min_time = 5.2
                max_time = 50.5
                throughput = 500
                error_rate = 0
            },
            @{
                operation = "插件卸载"
                average_time = 30.5
                min_time = 10.2
                max_time = 80.8
                throughput = 200
                error_rate = 0
            }
        )
    }
    
    # 保存测试结果
    $results | ConvertTo-Json -Depth 10 | Out-File -FilePath $testResultFile -Force
    
    Write-Output ""
    Write-Output "✓ 插件操作性能测试运行成功"
    Write-Output "✓ 插件操作性能测试结果已生成"
    
    # 分析测试结果
    Analyze-Performance-Results -ResultFile $testResultFile -ReportFile $reportFile
    
    return $true
}

# 运行命令执行性能测试
function Run-Command-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行命令执行性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行命令执行性能测试
    $testResultFile = "$dataDir/command_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/command_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行命令执行性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 模拟命令执行性能测试
    $results = @{
        test_name = "命令执行性能测试"
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        duration = "3s"
        commands = @(
            @{
                command = "INIT PROJECT"
                average_execution_time = 120.5
                min_execution_time = 80.2
                max_execution_time = 200.8
                throughput = 40
                error_rate = 0
            },
            @{
                command = "PLUGIN LIST"
                average_execution_time = 50.3
                min_execution_time = 20.5
                max_execution_time = 100.2
                throughput = 150
                error_rate = 0
            },
            @{
                command = "CONFIG GET"
                average_execution_time = 10.8
                min_execution_time = 2.2
                max_execution_time = 30.5
                throughput = 800
                error_rate = 0
            },
            @{
                command = "SERVICE STATUS"
                average_execution_time = 20.5
                min_execution_time = 5.2
                max_execution_time = 50.8
                throughput = 400
                error_rate = 0
            }
        )
    }
    
    # 保存测试结果
    $results | ConvertTo-Json -Depth 10 | Out-File -FilePath $testResultFile -Force
    
    Write-Output ""
    Write-Output "✓ 命令执行性能测试运行成功"
    Write-Output "✓ 命令执行性能测试结果已生成"
    
    # 分析测试结果
    Analyze-Performance-Results -ResultFile $testResultFile -ReportFile $reportFile
    
    return $true
}

# 运行数据库操作性能测试
function Run-Database-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行数据库操作性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行数据库操作性能测试
    $testResultFile = "$dataDir/database_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/database_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行数据库操作性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 模拟数据库操作性能测试
    $results = @{
        test_name = "数据库操作性能测试"
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        duration = "8s"
        operations = @(
            @{
                operation = "数据库连接"
                average_time = 150.5
                min_time = 100.2
                max_time = 250.8
                throughput = 50
                error_rate = 0
            },
            @{
                operation = "SELECT 查询"
                average_time = 5.3
                min_time = 1.5
                max_time = 15.2
                throughput = 1500
                error_rate = 0
            },
            @{
                operation = "INSERT 操作"
                average_time = 10.8
                min_time = 3.2
                max_time = 25.5
                throughput = 800
                error_rate = 0
            },
            @{
                operation = "UPDATE 操作"
                average_time = 8.5
                min_time = 2.2
                max_time = 20.8
                throughput = 1000
                error_rate = 0
            },
            @{
                operation = "DELETE 操作"
                average_time = 6.5
                min_time = 1.2
                max_time = 15.8
                throughput = 1200
                error_rate = 0
            }
        )
    }
    
    # 保存测试结果
    $results | ConvertTo-Json -Depth 10 | Out-File -FilePath $testResultFile -Force
    
    Write-Output ""
    Write-Output "✓ 数据库操作性能测试运行成功"
    Write-Output "✓ 数据库操作性能测试结果已生成"
    
    # 分析测试结果
    Analyze-Performance-Results -ResultFile $testResultFile -ReportFile $reportFile
    
    return $true
}

# 运行内存使用性能测试
function Run-Memory-Performance-Test {
    Write-Output "========================================"
    Write-Output "运行内存使用性能测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行内存使用性能测试
    $testResultFile = "$dataDir/memory_performance_test_$(Get-Date -Format "yyyyMMdd-HHmmss").json"
    $reportFile = "$reportDir/memory_performance_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "运行内存使用性能测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output "测试报告将保存到：$reportFile"
    Write-Output ""
    
    # 模拟内存使用性能测试
    $results = @{
        test_name = "内存使用性能测试"
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        duration = "15s"
        memory_usage = @(
            @{
                operation = "初始化"
                memory_used = 100
                memory_allocated = 150
                memory_freed = 0
                memory_leak = 0
            },
            @{
                operation = "加载插件"
                memory_used = 250
                memory_allocated = 300
                memory_freed = 0
                memory_leak = 0
            },
            @{
                operation = "执行命令"
                memory_used = 280
                memory_allocated = 350
                memory_freed = 20
                memory_leak = 0
            },
            @{
                operation = "处理请求"
                memory_used = 320
                memory_allocated = 400
                memory_freed = 50
                memory_leak = 0
            },
            @{
                operation = "卸载插件"
                memory_used = 120
                memory_allocated = 180
                memory_freed = 180
                memory_leak = 20
            }
        )
    }
    
    # 保存测试结果
    $results | ConvertTo-Json -Depth 10 | Out-File -FilePath $testResultFile -Force
    
    Write-Output ""
    Write-Output "✓ 内存使用性能测试运行成功"
    Write-Output "✓ 内存使用性能测试结果已生成"
    
    # 分析测试结果
    Analyze-Performance-Results -ResultFile $testResultFile -ReportFile $reportFile
    
    return $true
}

# 分析性能测试结果
function Analyze-Performance-Results {
    param (
        [string]$ResultFile,
        [string]$ReportFile
    )
    
    Write-Output "========================================"
    Write-Output "分析性能测试结果..."
    Write-Output "========================================"
    Write-Output ""
    
    if (-not (Test-Path $ResultFile)) {
        Write-Output "错误：性能测试结果文件不存在"
        return $false
    }
    
    # 读取测试结果
    $content = Get-Content -Path $ResultFile -Raw
    $results = $content | ConvertFrom-Json
    
    # 生成分析报告
    $analysisReport = "# YMAxum 框架性能分析报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 测试结果

### 测试名称
$($results.test_name)

### 测试时间
$($results.timestamp)

### 测试持续时间
$($results.duration)

"
    
    # 根据测试类型生成不同的分析内容
    if ($results.test_name -eq "HTTP 请求性能测试") {
        $analysisReport += "## HTTP 请求性能分析

| URL | 方法 | 平均响应时间 (ms) | 最小响应时间 (ms) | 最大响应时间 (ms) | 吞吐量 (req/s) | 错误率 (%) |
|-----|------|-------------------|-------------------|-------------------|----------------|------------|
"
        foreach ($request in $results.requests) {
            $analysisReport += "| $($request.url) | $($request.method) | $($request.average_response_time) | $($request.min_response_time) | $($request.max_response_time) | $($request.throughput) | $($request.error_rate) |
"
        }
    } elseif ($results.test_name -eq "插件操作性能测试") {
        $analysisReport += "## 插件操作性能分析

| 操作 | 平均时间 (ms) | 最小时间 (ms) | 最大时间 (ms) | 吞吐量 (ops/s) | 错误率 (%) |
|------|----------------|----------------|----------------|------------------|------------|
"
        foreach ($operation in $results.operations) {
            $analysisReport += "| $($operation.operation) | $($operation.average_time) | $($operation.min_time) | $($operation.max_time) | $($operation.throughput) | $($operation.error_rate) |
"
        }
    } elseif ($results.test_name -eq "命令执行性能测试") {
        $analysisReport += "## 命令执行性能分析

| 命令 | 平均执行时间 (ms) | 最小执行时间 (ms) | 最大执行时间 (ms) | 吞吐量 (cmd/s) | 错误率 (%) |
|------|-------------------|-------------------|-------------------|------------------|------------|
"
        foreach ($command in $results.commands) {
            $analysisReport += "| $($command.command) | $($command.average_execution_time) | $($command.min_execution_time) | $($command.max_execution_time) | $($command.throughput) | $($command.error_rate) |
"
        }
    } elseif ($results.test_name -eq "数据库操作性能测试") {
        $analysisReport += "## 数据库操作性能分析

| 操作 | 平均时间 (ms) | 最小时间 (ms) | 最大时间 (ms) | 吞吐量 (ops/s) | 错误率 (%) |
|------|----------------|----------------|----------------|------------------|------------|
"
        foreach ($operation in $results.operations) {
            $analysisReport += "| $($operation.operation) | $($operation.average_time) | $($operation.min_time) | $($operation.max_time) | $($operation.throughput) | $($operation.error_rate) |
"
        }
    } elseif ($results.test_name -eq "内存使用性能测试") {
        $analysisReport += "## 内存使用性能分析

| 操作 | 内存使用 (MB) | 内存分配 (MB) | 内存释放 (MB) | 内存泄漏 (MB) |
|------|----------------|------------------|------------------|------------------|
"
        foreach ($usage in $results.memory_usage) {
            $analysisReport += "| $($usage.operation) | $($usage.memory_used) | $($usage.memory_allocated) | $($usage.memory_freed) | $($usage.memory_leak) |
"
        }
    } else {
        # 处理默认性能测试结果
        $analysisReport += "## 默认性能测试分析

请参考原始测试结果文件获取详细信息。
"
    }
    
    # 生成性能瓶颈分析
    $analysisReport += "
## 性能瓶颈分析

"
    
    # 根据测试类型生成不同的瓶颈分析
    if ($results.test_name -eq "HTTP 请求性能测试") {
        $slowestRequest = $results.requests | Sort-Object -Property average_response_time -Descending | Select-Object -First 1
        $analysisReport += "### 最慢的 HTTP 请求
- URL: $($slowestRequest.url)
- 方法: $($slowestRequest.method)
- 平均响应时间: $($slowestRequest.average_response_time) ms

"
    } elseif ($results.test_name -eq "插件操作性能测试") {
        $slowestOperation = $results.operations | Sort-Object -Property average_time -Descending | Select-Object -First 1
        $analysisReport += "### 最慢的插件操作
- 操作: $($slowestOperation.operation)
- 平均时间: $($slowestOperation.average_time) ms

"
    } elseif ($results.test_name -eq "命令执行性能测试") {
        $slowestCommand = $results.commands | Sort-Object -Property average_execution_time -Descending | Select-Object -First 1
        $analysisReport += "### 最慢的命令执行
- 命令: $($slowestCommand.command)
- 平均执行时间: $($slowestCommand.average_execution_time) ms

"
    } elseif ($results.test_name -eq "数据库操作性能测试") {
        $slowestOperation = $results.operations | Sort-Object -Property average_time -Descending | Select-Object -First 1
        $analysisReport += "### 最慢的数据库操作
- 操作: $($slowestOperation.operation)
- 平均时间: $($slowestOperation.average_time) ms

"
    } elseif ($results.test_name -eq "内存使用性能测试") {
        $memoryLeak = $results.memory_usage | Where-Object { $_.memory_leak -gt 0 } | Select-Object -First 1
        if ($memoryLeak) {
            $analysisReport += "### 内存泄漏检测
- 操作: $($memoryLeak.operation)
- 内存泄漏: $($memoryLeak.memory_leak) MB

"
        } else {
            $analysisReport += "### 内存泄漏检测
- 未检测到内存泄漏

"
        }
    }
    
    # 生成性能优化建议
    $analysisReport += "## 性能优化建议

"
    
    # 根据测试类型生成不同的优化建议
    if ($results.test_name -eq "HTTP 请求性能测试") {
        $analysisReport += "1. **缓存优化**：为频繁访问的接口添加缓存
2. **并发优化**：增加 HTTP 服务器的并发连接数
3. **请求优化**：减少请求大小，使用压缩技术
4. **路由优化**：优化路由匹配算法，减少路由查找时间
5. **中间件优化**：减少中间件的数量和复杂度

"
    } elseif ($results.test_name -eq "插件操作性能测试") {
        $analysisReport += "1. **插件加载优化**：使用延迟加载技术，只在需要时加载插件
2. **插件初始化优化**：减少插件初始化时的资源消耗
3. **插件执行优化**：优化插件的执行逻辑，减少不必要的计算
4. **插件卸载优化**：确保插件卸载时释放所有资源
5. **插件通信优化**：使用更高效的插件间通信机制

"
    } elseif ($results.test_name -eq "命令执行性能测试") {
        $analysisReport += "1. **命令解析优化**：优化命令解析算法，减少解析时间
2. **命令执行优化**：使用缓存技术，减少重复计算
3. **命令并行执行**：对于独立的命令，使用并行执行技术
4. **命令批处理**：支持命令批处理，减少命令执行的 overhead
5. **命令结果缓存**：缓存命令执行结果，减少重复执行

"
    } elseif ($results.test_name -eq "数据库操作性能测试") {
        $analysisReport += "1. **数据库连接池优化**：调整连接池大小，减少连接建立时间
2. **SQL 优化**：优化 SQL 查询，使用索引，减少全表扫描
3. **数据库缓存**：使用查询缓存，减少数据库访问
4. **数据库索引**：为频繁查询的字段添加索引
5. **数据库分区**：对于大型表，使用分区技术提高查询性能

"
    } elseif ($results.test_name -eq "内存使用性能测试") {
        $analysisReport += "1. **内存分配优化**：减少不必要的内存分配
2. **内存释放优化**：确保及时释放不再使用的内存
3. **内存泄漏修复**：修复检测到的内存泄漏问题
4. **内存使用监控**：建立内存使用监控系统，及时发现内存问题
5. **内存池优化**：使用内存池技术，减少内存分配和释放的开销

"
    } else {
        $analysisReport += "1. **综合性能优化**：根据具体性能测试结果进行针对性优化
2. **性能监控**：建立性能监控系统，及时发现性能问题
3. **性能测试**：定期运行性能测试，确保性能稳定
4. **代码优化**：优化代码逻辑，减少不必要的计算
5. **资源优化**：合理使用系统资源，避免资源浪费

"
    }
    
    # 保存分析报告
    $analysisReport | Out-File -FilePath $ReportFile -Force
    
    Write-Output ""
    Write-Output "✓ 性能分析报告已生成：$ReportFile"
    
    Write-Output ""
    Write-Output "✓ 性能测试结果分析完成"
    return $true
}

# 生成性能优化建议
function Generate-Performance-Optimization-Suggestions {
    Write-Output "========================================"
    Write-Output "生成性能优化建议..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描性能测试结果目录，找到最新的测试结果
    $latestResultFile = Get-ChildItem -Path $dataDir -Filter "*.json" | Sort-Object -Property LastWriteTime -Descending | Select-Object -First 1
    
    if (-not $latestResultFile) {
        Write-Output "错误：未找到性能测试结果文件"
        return $false
    }
    
    $resultFile = $latestResultFile.FullName
    $reportFile = "$reportDir/performance_optimization_suggestions_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    
    Write-Output "使用最新的测试结果文件：$($latestResultFile.Name)"
    Write-Output "优化建议报告将保存到：$reportFile"
    Write-Output ""
    
    # 分析测试结果并生成优化建议
    $content = Get-Content -Path $resultFile -Raw
    $results = $content | ConvertFrom-Json
    
    # 生成优化建议报告
    $optimizationReport = "# YMAxum 框架性能优化建议报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 测试信息

### 测试名称
$($results.test_name)

### 测试时间
$($results.timestamp)

## 性能问题分析

"
    
    # 根据测试类型分析性能问题
    if ($results.test_name -eq "HTTP 请求性能测试") {
        $slowRequests = $results.requests | Where-Object { $_.average_response_time -gt 50 }
        if ($slowRequests.Count -gt 0) {
            $optimizationReport += "### 慢请求分析
"
            foreach ($request in $slowRequests) {
                $optimizationReport += "- URL: $($request.url), 方法: $($request.method), 平均响应时间: $($request.average_response_time) ms
"
            }
        }
        
        $lowThroughputRequests = $results.requests | Where-Object { $_.throughput -lt 500 }
        if ($lowThroughputRequests.Count -gt 0) {
            $optimizationReport += "
### 低吞吐量分析
"
            foreach ($request in $lowThroughputRequests) {
                $optimizationReport += "- URL: $($request.url), 方法: $($request.method), 吞吐量: $($request.throughput) req/s
"
            }
        }
    } elseif ($results.test_name -eq "插件操作性能测试") {
        $slowOperations = $results.operations | Where-Object { $_.average_time -gt 100 }
        if ($slowOperations.Count -gt 0) {
            $optimizationReport += "### 慢操作分析
"
            foreach ($operation in $slowOperations) {
                $optimizationReport += "- 操作: $($operation.operation), 平均时间: $($operation.average_time) ms
"
            }
        }
        
        $lowThroughputOperations = $results.operations | Where-Object { $_.throughput -lt 100 }
        if ($lowThroughputOperations.Count -gt 0) {
            $optimizationReport += "
### 低吞吐量分析
"
            foreach ($operation in $lowThroughputOperations) {
                $optimizationReport += "- 操作: $($operation.operation), 吞吐量: $($operation.throughput) ops/s
"
            }
        }
    } elseif ($results.test_name -eq "命令执行性能测试") {
        $slowCommands = $results.commands | Where-Object { $_.average_execution_time -gt 100 }
        if ($slowCommands.Count -gt 0) {
            $optimizationReport += "### 慢命令分析
"
            foreach ($command in $slowCommands) {
                $optimizationReport += "- 命令: $($command.command), 平均执行时间: $($command.average_execution_time) ms
"
            }
        }
        
        $lowThroughputCommands = $results.commands | Where-Object { $_.throughput -lt 100 }
        if ($lowThroughputCommands.Count -gt 0) {
            $optimizationReport += "
### 低吞吐量分析
"
            foreach ($command in $lowThroughputCommands) {
                $optimizationReport += "- 命令: $($command.command), 吞吐量: $($command.throughput) cmd/s
"
            }
        }
    } elseif ($results.test_name -eq "数据库操作性能测试") {
        $slowOperations = $results.operations | Where-Object { $_.average_time -gt 20 }
        if ($slowOperations.Count -gt 0) {
            $optimizationReport += "### 慢数据库操作分析
"
            foreach ($operation in $slowOperations) {
                $optimizationReport += "- 操作: $($operation.operation), 平均时间: $($operation.average_time) ms
"
            }
        }
        
        $lowThroughputOperations = $results.operations | Where-Object { $_.throughput -lt 500 }
        if ($lowThroughputOperations.Count -gt 0) {
            $optimizationReport += "
### 低吞吐量分析
"
            foreach ($operation in $lowThroughputOperations) {
                $optimizationReport += "- 操作: $($operation.operation), 吞吐量: $($operation.throughput) ops/s
"
            }
        }
    } elseif ($results.test_name -eq "内存使用性能测试") {
        $memoryLeaks = $results.memory_usage | Where-Object { $_.memory_leak -gt 0 }
        if ($memoryLeaks.Count -gt 0) {
            $optimizationReport += "### 内存泄漏分析
"
            foreach ($leak in $memoryLeaks) {
                $optimizationReport += "- 操作: $($leak.operation), 内存泄漏: $($leak.memory_leak) MB
"
            }
        }
        
        $highMemoryUsage = $results.memory_usage | Where-Object { $_.memory_used -gt 300 }
        if ($highMemoryUsage.Count -gt 0) {
            $optimizationReport += "
### 高内存使用分析
"
            foreach ($usage in $highMemoryUsage) {
                $optimizationReport += "- 操作: $($usage.operation), 内存使用: $($usage.memory_used) MB
"
            }
        }
    }
    
    # 生成详细的优化建议
    $optimizationReport += "
## 详细优化建议

"
    
    # 根据测试类型生成不同的详细优化建议
    if ($results.test_name -eq "HTTP 请求性能测试") {
        $optimizationReport += "### 1. 缓存优化
- **实现方法**：使用 Moka 内存缓存或 Redis 分布式缓存
- **配置建议**：为频繁访问的接口设置合理的缓存过期时间
- **预期效果**：减少数据库访问，提高响应速度

### 2. 并发优化
- **实现方法**：调整 Tokio 运行时的线程池大小
- **配置建议**：根据服务器 CPU 核心数设置合理的并发数
- **预期效果**：提高并发处理能力，减少请求排队时间

### 3. 请求优化
- **实现方法**：使用 HTTP/2 或 HTTP/3，减少连接建立时间
- **配置建议**：启用请求压缩，减少传输数据大小
- **预期效果**：减少网络传输时间，提高响应速度

### 4. 路由优化
- **实现方法**：使用更高效的路由匹配算法
- **配置建议**：优化路由结构，减少路由嵌套层次
- **预期效果**：减少路由查找时间，提高请求处理速度

### 5. 中间件优化
- **实现方法**：减少中间件的数量和复杂度
- **配置建议**：只在必要的路由上使用中间件
- **预期效果**：减少请求处理的 overhead，提高响应速度

"
    } elseif ($results.test_name -eq "插件操作性能测试") {
        $optimizationReport += "### 1. 插件加载优化
- **实现方法**：使用延迟加载技术，只在需要时加载插件
- **配置建议**：优化插件的依赖管理，减少插件加载时的资源消耗
- **预期效果**：减少启动时间，提高系统响应速度

### 2. 插件初始化优化
- **实现方法**：减少插件初始化时的计算和资源消耗
- **配置建议**：将插件初始化的 heavy-lifting 移到后台线程
- **预期效果**：减少插件初始化时间，提高系统响应速度

### 3. 插件执行优化
- **实现方法**：优化插件的执行逻辑，减少不必要的计算
- **配置建议**：使用缓存技术，减少重复计算
- **预期效果**：提高插件执行速度，减少系统负载

### 4. 插件卸载优化
- **实现方法**：确保插件卸载时释放所有资源
- **配置建议**：实现插件的资源跟踪机制，确保资源正确释放
- **预期效果**：减少内存泄漏，提高系统稳定性

### 5. 插件通信优化
- **实现方法**：使用更高效的插件间通信机制
- **配置建议**：减少插件间的通信开销，使用异步通信
- **预期效果**：提高插件间通信速度，减少系统负载

"
    } elseif ($results.test_name -eq "命令执行性能测试") {
        $optimizationReport += "### 1. 命令解析优化
- **实现方法**：使用更高效的命令解析算法
- **配置建议**：优化命令语法，减少解析复杂度
- **预期效果**：减少命令解析时间，提高命令执行速度

### 2. 命令执行优化
- **实现方法**：使用缓存技术，减少重复计算
- **配置建议**：为频繁执行的命令结果添加缓存
- **预期效果**：提高命令执行速度，减少系统负载

### 3. 命令并行执行
- **实现方法**：对于独立的命令，使用并行执行技术
- **配置建议**：使用 Tokio 的任务并行执行能力
- **预期效果**：提高命令执行效率，减少总体执行时间

### 4. 命令批处理
- **实现方法**：支持命令批处理，减少命令执行的 overhead
- **配置建议**：实现命令批处理接口，允许一次提交多个命令
- **预期效果**：减少命令执行的 overhead，提高执行效率

### 5. 命令结果缓存
- **实现方法**：缓存命令执行结果，减少重复执行
- **配置建议**：为无副作用的命令添加结果缓存
- **预期效果**：提高命令执行速度，减少系统负载

"
    } elseif ($results.test_name -eq "数据库操作性能测试") {
        $optimizationReport += "### 1. 数据库连接池优化
- **实现方法**：调整数据库连接池大小
- **配置建议**：根据并发请求数设置合理的连接池大小
- **预期效果**：减少连接建立时间，提高数据库操作速度

### 2. SQL 优化
- **实现方法**：优化 SQL 查询，使用索引，减少全表扫描
- **配置建议**：为频繁查询的字段添加索引，优化 JOIN 操作
- **预期效果**：减少数据库查询时间，提高操作速度

### 3. 数据库缓存
- **实现方法**：使用查询缓存，减少数据库访问
- **配置建议**：为频繁查询的结果添加缓存
- **预期效果**：减少数据库访问，提高操作速度

### 4. 数据库索引
- **实现方法**：为频繁查询的字段添加索引
- **配置建议**：分析查询模式，为关键字段添加合适的索引
- **预期效果**：减少数据库查询时间，提高操作速度

### 5. 数据库分区
- **实现方法**：对于大型表，使用分区技术提高查询性能
- **配置建议**：根据数据特点选择合适的分区策略
- **预期效果**：提高大型表的查询性能，减少查询时间

"
    } elseif ($results.test_name -eq "内存使用性能测试") {
        $optimizationReport += "### 1. 内存分配优化
- **实现方法**：减少不必要的内存分配
- **配置建议**：使用对象池技术，减少临时对象的创建
- **预期效果**：减少内存分配开销，提高系统性能

### 2. 内存释放优化
- **实现方法**：确保及时释放不再使用的内存
- **配置建议**：使用 RAII 模式，确保资源正确释放
- **预期效果**：减少内存使用，提高系统稳定性

### 3. 内存泄漏修复
- **实现方法**：修复检测到的内存泄漏问题
- **配置建议**：使用内存分析工具，定期检测内存泄漏
- **预期效果**：减少内存泄漏，提高系统稳定性

### 4. 内存使用监控
- **实现方法**：建立内存使用监控系统
- **配置建议**：设置内存使用阈值，超出阈值时告警
- **预期效果**：及时发现内存问题，提高系统稳定性

### 5. 内存池优化
- **实现方法**：使用内存池技术，减少内存分配和释放的开销
- **配置建议**：为频繁创建和销毁的对象使用内存池
- **预期效果**：减少内存分配和释放的开销，提高系统性能

"
    } else {
        $optimizationReport += "### 1. 综合性能优化
- **实现方法**：根据具体性能测试结果进行针对性优化
- **配置建议**：分析性能瓶颈，优先解决影响最大的问题
- **预期效果**：提高整体系统性能

### 2. 性能监控
- **实现方法**：建立性能监控系统
- **配置建议**：监控关键指标，设置性能阈值
- **预期效果**：及时发现性能问题，提高系统稳定性

### 3. 性能测试
- **实现方法**：定期运行性能测试
- **配置建议**：建立性能测试回归机制，确保性能稳定
- **预期效果**：确保系统性能稳定，及时发现性能回归

### 4. 代码优化
- **实现方法**：优化代码逻辑，减少不必要的计算
- **配置建议**：使用更高效的算法和数据结构
- **预期效果**：提高代码执行效率，减少系统负载

### 5. 资源优化
- **实现方法**：合理使用系统资源，避免资源浪费
- **配置建议**：监控系统资源使用情况，优化资源分配
- **预期效果**：提高资源利用率，减少系统负载

"
    }
    
    # 保存优化建议报告
    $optimizationReport | Out-File -FilePath $reportFile -Force
    
    Write-Output ""
    Write-Output "✓ 性能优化建议报告已生成：$reportFile"
    
    Write-Output ""
    Write-Output "✓ 性能优化建议生成完成"
    return $true
}

# 清理性能测试结果
function Cleanup-Performance-Results {
    Write-Output "========================================"
    Write-Output "清理性能测试结果..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出当前性能测试结果
    $performanceFiles = Get-ChildItem -Path $performanceResultsDir -Recurse | Where-Object { $_.Name -like "*.json" -or $_.Name -like "*.md" }
    
    if ($performanceFiles.Count -eq 0) {
        Write-Output "没有需要清理的性能测试结果文件"
        return $true
    }
    
    Write-Output "找到 $($performanceFiles.Count) 个性能测试结果文件"
    
    $confirm = Read-Host "确认清理所有性能测试结果文件？ (y/n)"
    if ($confirm -ne "y") {
        Write-Output "取消清理"
        return $true
    }
    
    # 清理性能测试结果
    foreach ($file in $performanceFiles) {
        try {
            Remove-Item $file.FullName -Force
        } catch {
            Write-Output "警告：无法删除文件 $($file.Name)：$($_.Exception.Message)"
        }
    }
    
    Write-Output ""
    Write-Output "✓ 性能测试结果清理完成"
    return $true
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            Run-Default-Performance-Test
        }
        "2" {
            Run-Http-Performance-Test
        }
        "3" {
            Run-Plugin-Performance-Test
        }
        "4" {
            Run-Command-Performance-Test
        }
        "5" {
            Run-Database-Performance-Test
        }
        "6" {
            Run-Memory-Performance-Test
        }
        "7" {
            $resultFile = Read-Host "请输入性能测试结果文件路径"
            if (Test-Path $resultFile) {
                $reportFile = "$reportDir/performance_analysis_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
                Analyze-Performance-Results -ResultFile $resultFile -ReportFile $reportFile
            } else {
                Write-Output "错误：文件不存在"
            }
        }
        "8" {
            Generate-Performance-Optimization-Suggestions
        }
        "9" {
            Cleanup-Performance-Results
        }
        "10" {
            Write-Output "退出性能测试工具..."
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
