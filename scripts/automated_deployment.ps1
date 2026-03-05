#!/usr/bin/env pwsh
# 自动化部署脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化部署工具"
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

# 检查是否存在部署目录
$deployDir = "deployment"
if (-not (Test-Path $deployDir)) {
    New-Item -ItemType Directory -Path $deployDir | Out-Null
    Write-Output "✓ 创建部署目录：$deployDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "部署选项："
    Write-Output "1. 构建项目"
    Write-Output "2. 部署到开发环境"
    Write-Output "3. 部署到测试环境"
    Write-Output "4. 部署到生产环境"
    Write-Output "5. 回滚部署"
    Write-Output "6. 查看部署历史"
    Write-Output "7. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-7)"
    return $choice
}

# 构建项目
function Build-Project {
    Write-Output "========================================"
    Write-Output "构建项目..."
    Write-Output "========================================"
    Write-Output ""
    
    # 清理之前的构建
    cargo clean
    
    # 构建发布版本
    Write-Output "构建发布版本..."
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ 项目构建成功"
        
        # 复制构建产物到部署目录
        if (Test-Path "target/release/ymaxum.exe") {
            Copy-Item "target/release/ymaxum.exe" "$deployDir/ymaxum-$(Get-Date -Format "yyyyMMdd-HHmmss").exe" -Force
            Write-Output "✓ 构建产物已复制到部署目录"
        }
        
        return $true
    } else {
        Write-Output "✗ 项目构建失败"
        return $false
    }
}

# 部署到指定环境
function Deploy-To-Environment {
    param (
        [string]$Environment
    )
    
    Write-Output "========================================"
    Write-Output "部署到 $Environment 环境..."
    Write-Output "========================================"
    Write-Output ""
    
    # 检查构建产物是否存在
    $latestBuild = Get-ChildItem -Path $deployDir -Filter "ymaxum-*.exe" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    
    if (-not $latestBuild) {
        Write-Output "错误：未找到构建产物"
        Write-Output "请先运行构建项目操作"
        return $false
    }
    
    Write-Output "使用构建产物：$($latestBuild.Name)"
    Write-Output ""
    
    # 检查环境配置文件是否存在
    $envConfig = "config/environment.toml"
    if (-not (Test-Path $envConfig)) {
        Write-Output "错误：未找到环境配置文件"
        Write-Output "请确保 $envConfig 文件存在"
        return $false
    }
    
    # 读取环境配置
    $configContent = Get-Content -Path $envConfig -Raw
    
    # 检查部署目标
    switch ($Environment) {
        "开发" {
            $targetDir = "deployments/dev"
            $port = 3000
        }
        "测试" {
            $targetDir = "deployments/test"
            $port = 3001
        }
        "生产" {
            $targetDir = "deployments/prod"
            $port = 80
        }
    }
    
    # 确保目标目录存在
    if (-not (Test-Path $targetDir)) {
        New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
        Write-Output "✓ 创建部署目标目录：$targetDir"
    }
    
    # 停止当前运行的服务
    Write-Output "停止当前运行的服务..."
    try {
        Get-Process | Where-Object {$_.ProcessName -eq "ymaxum"} | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Output "✓ 服务已停止"
    } catch {
        Write-Output "⚠ 停止服务时出错：$($_.Exception.Message)"
    }
    
    # 复制构建产物到目标目录
    Write-Output "复制构建产物到目标目录..."
    Copy-Item $latestBuild.FullName "$targetDir/ymaxum.exe" -Force
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ 构建产物已复制"
    } else {
        Write-Output "✗ 复制构建产物失败"
        return $false
    }
    
    # 复制配置文件到目标目录
    Write-Output "复制配置文件到目标目录..."
    Copy-Item "config/*.toml" $targetDir -Force
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ 配置文件已复制"
    } else {
        Write-Output "✗ 复制配置文件失败"
        return $false
    }
    
    # 记录部署历史
    $deployHistory = @{
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        environment = $Environment
        buildVersion = $latestBuild.Name
        status = "deployed"
    }
    
    $historyFile = "$deployDir/deployment_history.json"
    if (Test-Path $historyFile) {
        $history = Get-Content -Path $historyFile -Raw | ConvertFrom-Json
        $history += $deployHistory
    } else {
        $history = @($deployHistory)
    }
    
    $history | ConvertTo-Json -Depth 32 | Out-File -FilePath $historyFile -Force
    Write-Output "✓ 部署历史已记录"
    
    # 启动服务
    Write-Output "启动服务..."
    try {
        Start-Process -FilePath "$targetDir/ymaxum.exe" -WorkingDirectory $targetDir -WindowStyle Hidden
        Write-Output "✓ 服务已启动"
        
        # 等待服务启动
        Start-Sleep -Seconds 5
        
        # 检查服务状态
        $serviceStatus = Test-Service-Status -Port $port
        if ($serviceStatus) {
            Write-Output "✓ 服务状态正常"
        } else {
            Write-Output "⚠ 服务状态检查失败"
        }
        
        return $true
    } catch {
        Write-Output "✗ 启动服务失败：$($_.Exception.Message)"
        return $false
    }
}

# 测试服务状态
function Test-Service-Status {
    param (
        [int]$Port
    )
    
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:$Port/health" -TimeoutSec 10
        if ($response.StatusCode -eq 200) {
            return $true
        } else {
            return $false
        }
    } catch {
        return $false
    }
}

# 回滚部署
function Rollback-Deployment {
    param (
        [string]$Environment
    )
    
    Write-Output "========================================"
    Write-Output "回滚 $Environment 环境部署..."
    Write-Output "========================================"
    Write-Output ""
    
    # 检查部署历史
    $historyFile = "$deployDir/deployment_history.json"
    if (-not (Test-Path $historyFile)) {
        Write-Output "错误：未找到部署历史"
        return $false
    }
    
    $history = Get-Content -Path $historyFile -Raw | ConvertFrom-Json
    $envHistory = $history | Where-Object {$_.environment -eq $Environment} | Sort-Object timestamp -Descending
    
    if ($envHistory.Count -lt 2) {
        Write-Output "错误：没有足够的部署历史进行回滚"
        return $false
    }
    
    # 获取前一次成功的部署
    $previousDeploy = $envHistory[1]
    Write-Output "回滚到部署：$($previousDeploy.timestamp)"
    Write-Output "使用构建版本：$($previousDeploy.buildVersion)"
    Write-Output ""
    
    # 确定目标目录
    switch ($Environment) {
        "开发" {
            $targetDir = "deployments/dev"
            $port = 3000
        }
        "测试" {
            $targetDir = "deployments/test"
            $port = 3001
        }
        "生产" {
            $targetDir = "deployments/prod"
            $port = 80
        }
    }
    
    # 检查构建产物是否存在
    $previousBuild = Join-Path $deployDir $previousDeploy.buildVersion
    if (-not (Test-Path $previousBuild)) {
        Write-Output "错误：未找到回滚目标构建产物"
        return $false
    }
    
    # 停止当前运行的服务
    Write-Output "停止当前运行的服务..."
    try {
        Get-Process | Where-Object {$_.ProcessName -eq "ymaxum"} | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Output "✓ 服务已停止"
    } catch {
        Write-Output "⚠ 停止服务时出错：$($_.Exception.Message)"
    }
    
    # 复制构建产物到目标目录
    Write-Output "复制回滚构建产物到目标目录..."
    Copy-Item $previousBuild "$targetDir/ymaxum.exe" -Force
    
    if ($LASTEXITCODE -eq 0) {
        Write-Output "✓ 构建产物已复制"
    } else {
        Write-Output "✗ 复制构建产物失败"
        return $false
    }
    
    # 记录回滚历史
    $rollbackHistory = @{
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        environment = $Environment
        buildVersion = $previousDeploy.buildVersion
        status = "rolled_back"
        rolledBackFrom = $envHistory[0].buildVersion
    }
    
    $history += $rollbackHistory
    $history | ConvertTo-Json -Depth 32 | Out-File -FilePath $historyFile -Force
    Write-Output "✓ 回滚历史已记录"
    
    # 启动服务
    Write-Output "启动服务..."
    try {
        Start-Process -FilePath "$targetDir/ymaxum.exe" -WorkingDirectory $targetDir -WindowStyle Hidden
        Write-Output "✓ 服务已启动"
        
        # 等待服务启动
        Start-Sleep -Seconds 5
        
        # 检查服务状态
        $serviceStatus = Test-Service-Status -Port $port
        if ($serviceStatus) {
            Write-Output "✓ 服务状态正常"
        } else {
            Write-Output "⚠ 服务状态检查失败"
        }
        
        return $true
    } catch {
        Write-Output "✗ 启动服务失败：$($_.Exception.Message)"
        return $false
    }
}

# 查看部署历史
function Show-Deployment-History {
    Write-Output "========================================"
    Write-Output "部署历史..."
    Write-Output "========================================"
    Write-Output ""
    
    $historyFile = "$deployDir/deployment_history.json"
    if (-not (Test-Path $historyFile)) {
        Write-Output "错误：未找到部署历史"
        return
    }
    
    $history = Get-Content -Path $historyFile -Raw | ConvertFrom-Json
    
    if ($history.Count -eq 0) {
        Write-Output "没有部署历史记录"
        return
    }
    
    foreach ($item in $history | Sort-Object timestamp -Descending) {
        Write-Output "时间：$($item.timestamp)"
        Write-Output "环境：$($item.environment)"
        Write-Output "构建版本：$($item.buildVersion)"
        Write-Output "状态：$($item.status)"
        if ($item.rolledBackFrom) {
            Write-Output "回滚自：$($item.rolledBackFrom)"
        }
        Write-Output "----------------------------------------"
    }
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            Build-Project
        }
        "2" {
            Deploy-To-Environment -Environment "开发"
        }
        "3" {
            Deploy-To-Environment -Environment "测试"
        }
        "4" {
            Deploy-To-Environment -Environment "生产"
        }
        "5" {
            $envChoice = Read-Host "请选择回滚环境 (1. 开发 2. 测试 3. 生产)"
            switch ($envChoice) {
                "1" {
                    Rollback-Deployment -Environment "开发"
                }
                "2" {
                    Rollback-Deployment -Environment "测试"
                }
                "3" {
                    Rollback-Deployment -Environment "生产"
                }
                default {
                    Write-Output "无效选择"
                }
            }
        }
        "6" {
            Show-Deployment-History
        }
        "7" {
            Write-Output "退出部署工具..."
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