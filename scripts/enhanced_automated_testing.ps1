#!/usr/bin/env pwsh
# 增强的自动化测试工具脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 增强的自动化测试工具"
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

# 确保测试结果目录存在
$testResultsDir = "test_results"
if (-not (Test-Path $testResultsDir)) {
    New-Item -ItemType Directory -Path $testResultsDir | Out-Null
    Write-Output "✓ 创建测试结果目录：$testResultsDir"
}

# 确保测试覆盖率目录存在
$coverageDir = "$testResultsDir/coverage"
if (-not (Test-Path $coverageDir)) {
    New-Item -ItemType Directory -Path $coverageDir | Out-Null
    Write-Output "✓ 创建测试覆盖率目录：$coverageDir"
}

# 确保测试报告目录存在
$reportDir = "$testResultsDir/reports"
if (-not (Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir | Out-Null
    Write-Output "✓ 创建测试报告目录：$reportDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "测试工具选项："
    Write-Output "1. 生成智能测试用例"
    Write-Output "2. 运行所有测试"
    Write-Output "3. 运行单元测试"
    Write-Output "4. 运行集成测试"
    Write-Output "5. 分析测试结果"
    Write-Output "6. 生成测试覆盖率报告"
    Write-Output "7. 生成综合测试报告"
    Write-Output "8. 清理测试结果"
    Write-Output "9. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-9)"
    return $choice
}

# 生成智能测试用例
function Generate-Intelligent-Test-Cases {
    Write-Output "========================================"
    Write-Output "生成智能测试用例..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描 src 目录，识别所有模块
    $srcDir = "src"
    $modules = Get-ChildItem -Path $srcDir -Recurse -Filter "*.rs" | Where-Object { $_.Name -ne "main.rs" -and $_.Name -ne "lib.rs" -and $_.Name -notlike "*_test.rs" }
    
    Write-Output "发现 $($modules.Count) 个模块文件"
    Write-Output ""
    
    $generatedCount = 0
    
    foreach ($module in $modules) {
        $modulePath = $module.FullName
        $relativePath = $modulePath.Substring($PWD.Path.Length + 1)
        $moduleName = $module.BaseName
        
        Write-Output "处理模块：$moduleName"
        
        # 检查是否已存在测试文件
        $testFileName = "${moduleName}_test.rs"
        $testFilePath = Join-Path "tests" $testFileName
        
        if (Test-Path $testFilePath) {
            Write-Output "  测试文件已存在，跳过"
            continue
        }
        
        # 读取模块内容
        $content = Get-Content -Path $modulePath -Raw
        
        # 提取函数和结构体
        $functions = [regex]::Matches($content, 'pub\s+fn\s+(\w+)\s*\(([^)]*)\)', [System.Text.RegularExpressions.RegexOptions]::Singleline)
        $structs = [regex]::Matches($content, 'pub\s+struct\s+(\w+)', [System.Text.RegularExpressions.RegexOptions]::Singleline)
        
        if ($functions.Count -eq 0 -and $structs.Count -eq 0) {
            Write-Output "  未发现可测试的函数或结构体，跳过"
            continue
        }
        
        # 生成测试文件内容
        $testContent = "//! 自动生成的测试文件
//! 测试 $moduleName 模块的功能

use ymaxum::*;

"
        
        # 为每个函数生成测试
        foreach ($func in $functions) {
            $funcName = $func.Groups[1].Value
            $funcParams = $func.Groups[2].Value
            
            # 生成测试用例
            $testContent += "#[tokio::test]
async fn test_${funcName}() {
    // 测试 $funcName 函数
    // 参数：$funcParams
    
    // TODO: 实现测试逻辑
    // 示例：
    // let result = $funcName($params);
    // assert!(result.is_ok());
}

"
        }
        
        # 为每个结构体生成测试
        foreach ($struct in $structs) {
            $structName = $struct.Groups[1].Value
            
            # 生成测试用例
            $testContent += "#[tokio::test]
async fn test_${structName}_creation() {
    // 测试 $structName 结构体创建
    
    // TODO: 实现测试逻辑
    // 示例：
    // let instance = $structName::new();
    // assert!(instance.is_ok());
}

"
        }
        
        # 写入测试文件
        New-Item -ItemType File -Path $testFilePath -Value $testContent -Force | Out-Null
        Write-Output "  ✓ 测试文件已生成：$testFilePath"
        $generatedCount++
    }
    
    Write-Output ""
    Write-Output "生成结果："
    Write-Output "已生成：$generatedCount 个测试文件"
    Write-Output "跳过：$($modules.Count - $generatedCount) 个模块（已有测试文件或无可测试内容）"
    
    Write-Output ""
    Write-Output "✓ 智能测试用例生成完成"
    return $generatedCount
}

# 运行所有测试
function Run-All-Tests {
    Write-Output "========================================"
    Write-Output "运行所有测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行所有测试
    $testResultFile = "$testResultsDir/all_tests_$(Get-Date -Format "yyyyMMdd-HHmmss").txt"
    
    Write-Output "运行测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output ""
    
    # 运行测试并保存结果
    cargo test | Tee-Object -FilePath $testResultFile
    
    $exitCode = $LASTEXITCODE
    
    Write-Output ""
    Write-Output "测试完成，退出码：$exitCode"
    
    # 分析测试结果
    Analyze-Test-Results -ResultFile $testResultFile
    
    return $exitCode
}

# 运行单元测试
function Run-Unit-Tests {
    Write-Output "========================================"
    Write-Output "运行单元测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行单元测试
    $testResultFile = "$testResultsDir/unit_tests_$(Get-Date -Format "yyyyMMdd-HHmmss").txt"
    
    Write-Output "运行单元测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output ""
    
    # 运行单元测试并保存结果
    cargo test --lib | Tee-Object -FilePath $testResultFile
    
    $exitCode = $LASTEXITCODE
    
    Write-Output ""
    Write-Output "单元测试完成，退出码：$exitCode"
    
    # 分析测试结果
    Analyze-Test-Results -ResultFile $testResultFile
    
    return $exitCode
}

# 运行集成测试
function Run-Integration-Tests {
    Write-Output "========================================"
    Write-Output "运行集成测试..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行集成测试
    $testResultFile = "$testResultsDir/integration_tests_$(Get-Date -Format "yyyyMMdd-HHmmss").txt"
    
    Write-Output "运行集成测试..."
    Write-Output "测试结果将保存到：$testResultFile"
    Write-Output ""
    
    # 运行集成测试并保存结果
    cargo test --test * | Tee-Object -FilePath $testResultFile
    
    $exitCode = $LASTEXITCODE
    
    Write-Output ""
    Write-Output "集成测试完成，退出码：$exitCode"
    
    # 分析测试结果
    Analyze-Test-Results -ResultFile $testResultFile
    
    return $exitCode
}

# 分析测试结果
function Analyze-Test-Results {
    param (
        [string]$ResultFile
    )
    
    Write-Output "========================================"
    Write-Output "分析测试结果..."
    Write-Output "========================================"
    Write-Output ""
    
    if (-not (Test-Path $ResultFile)) {
        Write-Output "错误：测试结果文件不存在"
        return $false
    }
    
    # 读取测试结果
    $content = Get-Content -Path $ResultFile -Raw
    
    # 分析测试结果
    $testSummary = ""
    
    # 提取测试统计信息
    $summaryMatch = [regex]::Match($content, 'test result: (\w+). (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in ([\d.]+s)')
    
    if ($summaryMatch.Success) {
        $status = $summaryMatch.Groups[1].Value
        $passed = $summaryMatch.Groups[2].Value
        $failed = $summaryMatch.Groups[3].Value
        $ignored = $summaryMatch.Groups[4].Value
        $measured = $summaryMatch.Groups[5].Value
        $filtered = $summaryMatch.Groups[6].Value
        $duration = $summaryMatch.Groups[7].Value
        
        $testSummary = "测试结果：$status
通过：$passed
失败：$failed
忽略：$ignored
测量：$measured
过滤：$filtered
耗时：$duration"
        
        Write-Output $testSummary
        
        # 提取失败的测试
        if ($failed -gt 0) {
            Write-Output ""
            Write-Output "失败的测试："
            
            $failedTests = [regex]::Matches($content, 'test (\S+) ... (failed)')
            foreach ($test in $failedTests) {
                $testName = $test.Groups[1].Value
                Write-Output "- $testName"
            }
            
            # 提取失败原因
            Write-Output ""
            Write-Output "失败原因："
            
            # 这里可以添加更详细的失败原因分析
        }
    } else {
        Write-Output "警告：无法提取测试统计信息"
    }
    
    # 生成分析报告
    $analysisReport = "# YMAxum 框架测试分析报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 测试结果

$testSummary

## 失败测试分析

"  
    
    if ($failed -gt 0) {
        $analysisReport += "### 失败的测试：
"
        foreach ($test in $failedTests) {
            $testName = $test.Groups[1].Value
            $analysisReport += "- $testName
"
        }
        
        $analysisReport += "
### 可能的失败原因：
- 代码逻辑错误
- 测试用例设计不当
- 依赖项问题
- 环境配置问题
"
    } else {
        $analysisReport += "所有测试均通过，未发现失败情况。
"
    }
    
    $analysisReport += "
## 改进建议

1. **测试覆盖率**：确保所有关键功能都有相应的测试用例
2. **测试质量**：提高测试用例的质量，覆盖更多边界情况
3. **测试速度**：优化测试执行速度，减少测试运行时间
4. **测试自动化**：将测试集成到 CI/CD 流水线中
"
    
    # 保存分析报告
    $reportFile = "$reportDir/test_analysis_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    $analysisReport | Out-File -FilePath $reportFile -Force
    
    Write-Output ""
    Write-Output "✓ 测试分析报告已生成：$reportFile"
    
    Write-Output ""
    Write-Output "✓ 测试结果分析完成"
    return $true
}

# 生成测试覆盖率报告
function Generate-Coverage-Report {
    Write-Output "========================================"
    Write-Output "生成测试覆盖率报告..."
    Write-Output "========================================"
    Write-Output ""
    
    # 检查 cargo-tarpaulin 是否已安装
    if (-not (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue)) {
        Write-Output "安装 cargo-tarpaulin..."
        cargo install cargo-tarpaulin
        
        if (-not (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue)) {
            Write-Output "错误：cargo-tarpaulin 安装失败"
            Write-Output "请手动安装：cargo install cargo-tarpaulin"
            return $false
        }
    }
    
    Write-Output "运行测试并生成覆盖率报告..."
    
    # 运行测试并生成覆盖率报告
    $coverageOutputDir = "$coverageDir/$(Get-Date -Format "yyyyMMdd-HHmmss")"
    New-Item -ItemType Directory -Path $coverageOutputDir | Out-Null
    
    cargo tarpaulin --out Html --out Xml --out Lcov --output-dir $coverageOutputDir --timeout 300 --run-types Tests --exclude-files "tests/*,examples/*,benches/*" --exclude "tests::*,main"
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output ""
        Write-Output "✓ 测试覆盖率报告生成成功"
        Write-Output "HTML 报告位置：$coverageOutputDir/index.html"
        
        # 提取覆盖率统计信息
        $lcovFile = Get-ChildItem -Path $coverageOutputDir -Filter "*.lcov" | Select-Object -First 1
        if ($lcovFile) {
            $lcovContent = Get-Content -Path $lcovFile.FullName -Raw
            $coverageMatch = [regex]::Match($lcovContent, 'TN:\nSF:.*\nFNF:(\d+)\nFNH:(\d+)\nLF:(\d+)\nLH:(\d+)')
            
            if ($coverageMatch.Success) {
                $functionsTotal = $coverageMatch.Groups[1].Value
                $functionsHit = $coverageMatch.Groups[2].Value
                $linesTotal = $coverageMatch.Groups[3].Value
                $linesHit = $coverageMatch.Groups[4].Value
                
                $functionCoverage = [math]::Round(($functionsHit / $functionsTotal) * 100, 2)
                $lineCoverage = [math]::Round(($linesHit / $linesTotal) * 100, 2)
                
                Write-Output ""
                Write-Output "覆盖率统计："
                Write-Output "函数覆盖率：$functionCoverage% ($functionsHit/$functionsTotal)"
                Write-Output "行覆盖率：$lineCoverage% ($linesHit/$linesTotal)"
            }
        }
        
        return $true
    } else {
        Write-Output ""
        Write-Output "✗ 测试覆盖率报告生成失败"
        return $false
    }
}

# 生成综合测试报告
function Generate-Composite-Test-Report {
    Write-Output "========================================"
    Write-Output "生成综合测试报告..."
    Write-Output "========================================"
    Write-Output ""
    
    # 运行所有测试
    $testExitCode = Run-All-Tests
    
    # 生成覆盖率报告
    $coverageSuccess = Generate-Coverage-Report
    
    # 生成综合报告
    $compositeReport = "# YMAxum 框架综合测试报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 测试执行结果

### 测试状态
$($testExitCode -eq 0 ? "通过" : "失败")

### 覆盖率报告
$($coverageSuccess ? "已生成" : "生成失败")

## 测试详情

请参考以下报告获取详细信息：

- **测试分析报告**：查看 `test_results/reports/` 目录下的分析报告
- **覆盖率报告**：查看 `test_results/coverage/` 目录下的覆盖率报告

## 总结

YMAxum 框架的测试状态：$($testExitCode -eq 0 ? "良好" : "需要关注")

$($coverageSuccess ? "测试覆盖率报告已生成，可以用于评估测试质量。" : "覆盖率报告生成失败，无法评估测试覆盖率。")

## 建议

1. **定期运行测试**：确保代码变更不会破坏现有功能
2. **提高测试覆盖率**：为未覆盖的代码添加测试用例
3. **优化测试速度**：减少测试运行时间，提高开发效率
4. **集成 CI/CD**：将测试集成到持续集成流程中
"
    
    # 保存综合报告
    $compositeReportFile = "$reportDir/composite_test_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    $compositeReport | Out-File -FilePath $compositeReportFile -Force
    
    Write-Output ""
    Write-Output "✓ 综合测试报告已生成：$compositeReportFile"
    
    Write-Output ""
    Write-Output "✓ 综合测试报告生成完成"
    return $testExitCode -eq 0 -and $coverageSuccess
}

# 清理测试结果
function Cleanup-Test-Results {
    Write-Output "========================================"
    Write-Output "清理测试结果..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出当前测试结果
    $testFiles = Get-ChildItem -Path $testResultsDir -Recurse | Where-Object { $_.Name -like "*.txt" -or $_.Name -like "*.md" -or $_.Name -like "*.html" -or $_.Name -like "*.xml" -or $_.Name -like "*.lcov" }
    
    if ($testFiles.Count -eq 0) {
        Write-Output "没有需要清理的测试结果文件"
        return $true
    }
    
    Write-Output "找到 $($testFiles.Count) 个测试结果文件"
    
    $confirm = Read-Host "确认清理所有测试结果文件？ (y/n)"
    if ($confirm -ne "y") {
        Write-Output "取消清理"
        return $true
    }
    
    # 清理测试结果
    foreach ($file in $testFiles) {
        try {
            Remove-Item $file.FullName -Force
        } catch {
            Write-Output "警告：无法删除文件 $($file.Name)：$($_.Exception.Message)"
        }
    }
    
    # 清理空目录
    Get-ChildItem -Path $testResultsDir -Recurse -Directory | Sort-Object FullName -Descending | ForEach-Object {
        if ((Get-ChildItem -Path $_.FullName | Measure-Object).Count -eq 0) {
            Remove-Item $_.FullName -Force
        }
    }
    
    Write-Output ""
    Write-Output "✓ 测试结果清理完成"
    return $true
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            Generate-Intelligent-Test-Cases
        }
        "2" {
            Run-All-Tests
        }
        "3" {
            Run-Unit-Tests
        }
        "4" {
            Run-Integration-Tests
        }
        "5" {
            $resultFile = Read-Host "请输入测试结果文件路径"
            if (Test-Path $resultFile) {
                Analyze-Test-Results -ResultFile $resultFile
            } else {
                Write-Output "错误：文件不存在"
            }
        }
        "6" {
            Generate-Coverage-Report
        }
        "7" {
            Generate-Composite-Test-Report
        }
        "8" {
            Cleanup-Test-Results
        }
        "9" {
            Write-Output "退出测试工具..."
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
