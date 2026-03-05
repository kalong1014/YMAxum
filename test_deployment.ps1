#!/usr/bin/env pwsh
# 测试部署流程脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 部署流程测试"
Write-Output "========================================"
Write-Output ""

# 测试1: 检查构建产物是否存在
Write-Output "测试1: 检查构建产物是否存在"
if (Test-Path "dist/windows/ymaxum-x86_64.exe") {
    Write-Output "✓ 构建产物存在: dist/windows/ymaxum-x86_64.exe"
} else {
    Write-Output "✗ 构建产物不存在"
}
Write-Output ""

# 测试2: 检查插件和配置文件包是否存在
Write-Output "测试2: 检查插件和配置文件包是否存在"
if (Test-Path "dist/windows/ymaxum-plugins-config.zip") {
    Write-Output "✓ 插件和配置文件包存在: dist/windows/ymaxum-plugins-config.zip"
} else {
    Write-Output "✗ 插件和配置文件包不存在"
}
Write-Output ""

# 测试3: 创建部署目录
Write-Output "测试3: 创建部署目录"
$deployDir = "test_deployment"
if (-not (Test-Path $deployDir)) {
    New-Item -ItemType Directory -Path $deployDir | Out-Null
    Write-Output "✓ 创建部署目录: $deployDir"
} else {
    Write-Output "✓ 部署目录已存在: $deployDir"
}
Write-Output ""

# 测试4: 复制构建产物到部署目录
Write-Output "测试4: 复制构建产物到部署目录"
if (Test-Path "dist/windows/ymaxum-x86_64.exe") {
    Copy-Item "dist/windows/ymaxum-x86_64.exe" "$deployDir/ymaxum.exe" -Force
    if (Test-Path "$deployDir/ymaxum.exe") {
        Write-Output "✓ 构建产物已复制到部署目录"
    } else {
        Write-Output "✗ 复制构建产物失败"
    }
} else {
    Write-Output "✗ 构建产物不存在，无法复制"
}
Write-Output ""

# 测试5: 复制配置文件到部署目录
Write-Output "测试5: 复制配置文件到部署目录"
if (Test-Path "config") {
    New-Item -ItemType Directory -Path "$deployDir/config" -Force | Out-Null
    Copy-Item "config/*.toml" "$deployDir/config" -Force
    $configFiles = Get-ChildItem "$deployDir/config" -Filter "*.toml"
    if ($configFiles.Count -gt 0) {
        Write-Output "✓ 配置文件已复制到部署目录 ($($configFiles.Count) 个文件)"
    } else {
        Write-Output "✗ 复制配置文件失败"
    }
} else {
    Write-Output "✗ 配置目录不存在，无法复制"
}
Write-Output ""

# 测试6: 检查部署目录结构
Write-Output "测试6: 检查部署目录结构"
Write-Output "部署目录内容:"
Get-ChildItem -Path $deployDir -Recurse | ForEach-Object {
    Write-Output "  $($_.FullName.Substring($PWD.Path.Length + 1))"
}
Write-Output ""

# 测试7: 清理测试目录
Write-Output "测试7: 清理测试目录"
if (Test-Path $deployDir) {
    Remove-Item -Path $deployDir -Recurse -Force
    Write-Output "✓ 测试目录已清理"
} else {
    Write-Output "✓ 测试目录不存在，无需清理"
}
Write-Output ""

Write-Output "========================================"
Write-Output "部署流程测试完成"
Write-Output "========================================"
