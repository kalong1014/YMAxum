#!/usr/bin/env pwsh
# 自动化测试用例生成脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化测试用例生成"
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

# 检查是否存在测试生成工具
if (-not (Test-Path "src/testing/test_generation.rs")) {
    Write-Output "错误：未找到测试生成工具"
    Write-Output "请确保测试生成模块已正确实现"
    exit 1
}

Write-Output "✓ 测试生成工具已找到"
Write-Output ""

# 读取项目结构
Write-Output "读取项目结构..."
$srcDir = "src"
$testDir = "tests"

# 确保测试目录存在
if (-not (Test-Path $testDir)) {
    New-Item -ItemType Directory -Path $testDir | Out-Null
    Write-Output "✓ 创建测试目录：$testDir"
}

# 收集所有 Rust 源文件
$rustFiles = Get-ChildItem -Path $srcDir -Recurse -Filter "*.rs" | Where-Object { $_.Name -notlike "*_test.rs" }

Write-Output "✓ 找到 $($rustFiles.Count) 个源文件"
Write-Output ""

# 为每个源文件生成测试用例
foreach ($file in $rustFiles) {
    $relativePath = $file.FullName.Substring($PWD.Path.Length + 1)
    Write-Output "处理文件：$relativePath"
    
    # 提取模块名称
    $moduleName = $file.BaseName
    
    # 生成测试文件路径
    $testFileName = "${moduleName}_test.rs"
    $testFilePath = Join-Path $testDir $testFileName
    
    # 如果测试文件已存在，跳过
    if (Test-Path $testFilePath) {
        Write-Output "  - 测试文件已存在，跳过"
        continue
    }
    
    # 生成测试用例
    Write-Output "  - 生成测试用例..."
    
    # 读取源文件内容
    $content = Get-Content -Path $file.FullName -Raw
    
    # 提取函数和结构体
    $functions = [regex]::Matches($content, 'pub\s+fn\s+(\w+)\s*\(', [System.Text.RegularExpressions.RegexOptions]::Multiline)
    $structs = [regex]::Matches($content, 'pub\s+struct\s+(\w+)', [System.Text.RegularExpressions.RegexOptions]::Multiline)
    
    # 生成测试文件内容
    $testContent = "//! 自动生成的测试文件
//! 测试 $moduleName 模块的功能

use super::*;

"
    
    # 为每个函数生成测试
    foreach ($func in $functions) {
        $funcName = $func.Groups[1].Value
        $testContent += "#[tokio::test]
async fn test_${funcName}() {
    // TODO: 实现测试逻辑
    // 示例：assert!(true);
}

"
    }
    
    # 为每个结构体生成测试
    foreach ($struct in $structs) {
        $structName = $struct.Groups[1].Value
        $testContent += "#[tokio::test]
async fn test_${structName}_creation() {
    // TODO: 实现结构体创建测试
    // 示例：let _instance = ${structName}::new();
}

"
    }
    
    # 写入测试文件
    New-Item -ItemType File -Path $testFilePath -Value $testContent -Force | Out-Null
    Write-Output "  - 测试文件生成成功：$testFilePath"
}

Write-Output ""
Write-Output "========================================"
Write-Output "测试用例生成完成！"
Write-Output "========================================"
Write-Output ""
Write-Output "生成的测试文件位于：$testDir"
Write-Output "请根据需要修改测试逻辑，确保测试覆盖率达到要求。"
Write-Output ""
