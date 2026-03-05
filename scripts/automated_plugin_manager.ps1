#!/usr/bin/env pwsh
# 自动化插件管理脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化插件管理工具"
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

# 确保插件目录存在
$pluginsDir = "plugins"
if (-not (Test-Path $pluginsDir)) {
    New-Item -ItemType Directory -Path $pluginsDir | Out-Null
    Write-Output "✓ 创建插件目录：$pluginsDir"
}

# 确保插件输出目录存在
$pluginOutputDir = "$pluginsDir/output"
if (-not (Test-Path $pluginOutputDir)) {
    New-Item -ItemType Directory -Path $pluginOutputDir | Out-Null
    Write-Output "✓ 创建插件输出目录：$pluginOutputDir"
}

# 确保插件备份目录存在
$pluginBackupDir = "$pluginsDir/backup"
if (-not (Test-Path $pluginBackupDir)) {
    New-Item -ItemType Directory -Path $pluginBackupDir | Out-Null
    Write-Output "✓ 创建插件备份目录：$pluginBackupDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "插件管理选项："
    Write-Output "1. 列出已安装插件"
    Write-Output "2. 安装插件"
    Write-Output "3. 更新插件"
    Write-Output "4. 卸载插件"
    Write-Output "5. 启用插件"
    Write-Output "6. 禁用插件"
    Write-Output "7. 备份插件"
    Write-Output "8. 恢复插件"
    Write-Output "9. 验证插件签名"
    Write-Output "10. 生成插件报告"
    Write-Output "11. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-11)"
    return $choice
}

# 列出已安装插件
function List-Plugins {
    Write-Output "========================================"
    Write-Output "已安装插件列表..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描插件目录
    $pluginDirs = Get-ChildItem -Path $pluginsDir -Directory | Where-Object { $_.Name -ne "output" -and $_.Name -ne "backup" }
    
    if ($pluginDirs.Count -eq 0) {
        Write-Output "没有已安装的插件"
        return
    }
    
    foreach ($pluginDir in $pluginDirs) {
        $pluginName = $pluginDir.Name
        $manifestPath = Join-Path $pluginDir.FullName "manifest.json"
        
        Write-Output "插件名称：$pluginName"
        
        # 检查清单文件
        if (Test-Path $manifestPath) {
            try {
                $manifest = Get-Content -Path $manifestPath -Raw | ConvertFrom-Json
                Write-Output "版本：$($manifest.version)"
                Write-Output "作者：$($manifest.author)"
                Write-Output "描述：$($manifest.description)"
                Write-Output "类型：$($manifest.type)"
            } catch {
                Write-Output "警告：清单文件格式错误"
            }
        } else {
            Write-Output "警告：缺少清单文件"
        }
        
        # 检查插件状态
        $pluginStatus = "已安装"
        if (Test-Path (Join-Path $pluginDir.FullName "disabled")) {
            $pluginStatus = "已禁用"
        }
        Write-Output "状态：$pluginStatus"
        
        Write-Output "----------------------------------------"
    }
    
    Write-Output "✓ 插件列表生成完成"
}

# 安装插件
function Install-Plugin {
    Write-Output "========================================"
    Write-Output "安装插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginPath = Read-Host "请输入插件文件路径或目录"
    
    if (-not (Test-Path $pluginPath)) {
        Write-Output "错误：插件路径不存在"
        return $false
    }
    
    # 检查是文件还是目录
    if (Test-Path $pluginPath -PathType Leaf) {
        # 是文件，假设是 .axpl 格式
        if ($pluginPath -notlike "*.axpl") {
            Write-Output "错误：插件文件格式错误，应为 .axpl 文件"
            return $false
        }
        
        # 解析插件名称
        $pluginName = [System.IO.Path]::GetFileNameWithoutExtension($pluginPath)
        $targetDir = Join-Path $pluginsDir $pluginName
        
        # 检查插件是否已存在
        if (Test-Path $targetDir) {
            $overwrite = Read-Host "插件已存在，是否覆盖？ (y/n)"
            if ($overwrite -ne "y") {
                return $false
            }
        }
        
        # 创建目标目录
        if (-not (Test-Path $targetDir)) {
            New-Item -ItemType Directory -Path $targetDir | Out-Null
        }
        
        # 复制插件文件
        Copy-Item $pluginPath $targetDir -Force
        Write-Output "✓ 插件文件已复制"
        
        # 验证插件签名
        if (Test-Path "scripts/sign_plugins.ps1") {
            & "scripts/sign_plugins.ps1" -PluginPath $targetDir
            if ($LASTEXITCODE -eq 0) {
                Write-Output "✓ 插件签名验证通过"
            } else {
                Write-Output "⚠ 插件签名验证失败"
            }
        }
        
    } else {
        # 是目录，假设是插件源代码
        $pluginDirName = Split-Path $pluginPath -Leaf
        $targetDir = Join-Path $pluginsDir $pluginDirName
        
        # 检查插件是否已存在
        if (Test-Path $targetDir) {
            $overwrite = Read-Host "插件已存在，是否覆盖？ (y/n)"
            if ($overwrite -ne "y") {
                return $false
            }
        }
        
        # 复制插件目录
        Copy-Item $pluginPath $targetDir -Recurse -Force
        Write-Output "✓ 插件目录已复制"
        
        # 构建插件
        Write-Output "构建插件..."
        Push-Location $targetDir
        cargo build --release
        
        if ($LASTEXITCODE -eq 0) {
            Write-Output "✓ 插件构建成功"
            
            # 复制构建产物到输出目录
            $pluginExe = Get-ChildItem -Path "target/release" -Filter "*.exe" | Select-Object -First 1
            if ($pluginExe) {
                $pluginOutput = Join-Path $pluginOutputDir "$pluginDirName.axpl"
                Copy-Item $pluginExe.FullName $pluginOutput -Force
                Write-Output "✓ 构建产物已复制到输出目录"
            }
        } else {
            Write-Output "✗ 插件构建失败"
            Pop-Location
            return $false
        }
        
        Pop-Location
    }
    
    Write-Output ""
    Write-Output "✓ 插件安装完成"
    return $true
}

# 更新插件
function Update-Plugin {
    Write-Output "========================================"
    Write-Output "更新插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称"
    $pluginDir = Join-Path $pluginsDir $pluginName
    
    if (-not (Test-Path $pluginDir)) {
        Write-Output "错误：插件不存在"
        return $false
    }
    
    # 备份插件
    $backupName = "$pluginName-$(Get-Date -Format "yyyyMMdd-HHmmss")"
    $backupDir = Join-Path $pluginBackupDir $backupName
    Copy-Item $pluginDir $backupDir -Recurse -Force
    Write-Output "✓ 插件已备份到 $backupDir"
    
    # 获取更新源
    $updateSource = Read-Host "请输入更新源路径"
    
    if (-not (Test-Path $updateSource)) {
        Write-Output "错误：更新源路径不存在"
        return $false
    }
    
    # 复制更新文件
    if (Test-Path $updateSource -PathType Leaf) {
        # 是文件
        Copy-Item $updateSource $pluginDir -Force
    } else {
        # 是目录
        Get-ChildItem -Path $updateSource -Recurse | Copy-Item -Destination $pluginDir -Recurse -Force
    }
    
    Write-Output "✓ 更新文件已复制"
    
    # 重新构建插件（如果是源代码）
    if (Test-Path (Join-Path $pluginDir "Cargo.toml")) {
        Write-Output "重新构建插件..."
        Push-Location $pluginDir
        cargo build --release
        
        if ($LASTEXITCODE -eq 0) {
            Write-Output "✓ 插件构建成功"
        } else {
            Write-Output "✗ 插件构建失败"
            Pop-Location
            return $false
        }
        
        Pop-Location
    }
    
    Write-Output ""
    Write-Output "✓ 插件更新完成"
    return $true
}

# 卸载插件
function Uninstall-Plugin {
    Write-Output "========================================"
    Write-Output "卸载插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称"
    $pluginDir = Join-Path $pluginsDir $pluginName
    
    if (-not (Test-Path $pluginDir)) {
        Write-Output "错误：插件不存在"
        return $false
    }
    
    # 备份插件
    $backupName = "$pluginName-$(Get-Date -Format "yyyyMMdd-HHmmss")"
    $backupDir = Join-Path $pluginBackupDir $backupName
    Copy-Item $pluginDir $backupDir -Recurse -Force
    Write-Output "✓ 插件已备份到 $backupDir"
    
    # 删除插件目录
    Remove-Item $pluginDir -Recurse -Force
    Write-Output "✓ 插件目录已删除"
    
    # 从输出目录删除
    $pluginOutput = Join-Path $pluginOutputDir "$pluginName.axpl"
    if (Test-Path $pluginOutput) {
        Remove-Item $pluginOutput -Force
        Write-Output "✓ 插件输出文件已删除"
    }
    
    Write-Output ""
    Write-Output "✓ 插件卸载完成"
    return $true
}

# 启用插件
function Enable-Plugin {
    Write-Output "========================================"
    Write-Output "启用插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称"
    $pluginDir = Join-Path $pluginsDir $pluginName
    
    if (-not (Test-Path $pluginDir)) {
        Write-Output "错误：插件不存在"
        return $false
    }
    
    # 检查是否已禁用
    $disabledFile = Join-Path $pluginDir "disabled"
    if (Test-Path $disabledFile) {
        Remove-Item $disabledFile -Force
        Write-Output "✓ 插件禁用标记已移除"
    }
    
    Write-Output ""
    Write-Output "✓ 插件已启用"
    return $true
}

# 禁用插件
function Disable-Plugin {
    Write-Output "========================================"
    Write-Output "禁用插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称"
    $pluginDir = Join-Path $pluginsDir $pluginName
    
    if (-not (Test-Path $pluginDir)) {
        Write-Output "错误：插件不存在"
        return $false
    }
    
    # 创建禁用标记
    $disabledFile = Join-Path $pluginDir "disabled"
    New-Item -ItemType File -Path $disabledFile -Force | Out-Null
    Write-Output "✓ 插件禁用标记已创建"
    
    Write-Output ""
    Write-Output "✓ 插件已禁用"
    return $true
}

# 备份插件
function Backup-Plugin {
    Write-Output "========================================"
    Write-Output "备份插件..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称（留空备份所有插件）"
    
    if ([string]::IsNullOrEmpty($pluginName)) {
        # 备份所有插件
        $backupName = "all-$(Get-Date -Format "yyyyMMdd-HHmmss")"
        $backupDir = Join-Path $pluginBackupDir $backupName
        New-Item -ItemType Directory -Path $backupDir | Out-Null
        
        # 复制所有插件目录
        $pluginDirs = Get-ChildItem -Path $pluginsDir -Directory | Where-Object { $_.Name -ne "output" -and $_.Name -ne "backup" }
        foreach ($dir in $pluginDirs) {
            $targetDir = Join-Path $backupDir $dir.Name
            Copy-Item $dir.FullName $targetDir -Recurse -Force
        }
        
        Write-Output "✓ 所有插件已备份到 $backupDir"
    } else {
        # 备份单个插件
        $pluginDir = Join-Path $pluginsDir $pluginName
        
        if (-not (Test-Path $pluginDir)) {
            Write-Output "错误：插件不存在"
            return $false
        }
        
        $backupName = "$pluginName-$(Get-Date -Format "yyyyMMdd-HHmmss")"
        $backupDir = Join-Path $pluginBackupDir $backupName
        Copy-Item $pluginDir $backupDir -Recurse -Force
        
        Write-Output "✓ 插件已备份到 $backupDir"
    }
    
    Write-Output ""
    Write-Output "✓ 插件备份完成"
    return $true
}

# 恢复插件
function Restore-Plugin {
    Write-Output "========================================"
    Write-Output "恢复插件..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出可用备份
    $backups = Get-ChildItem -Path $pluginBackupDir -Directory | Sort-Object LastWriteTime -Descending
    
    if ($backups.Count -eq 0) {
        Write-Output "错误：没有可用的备份"
        return $false
    }
    
    Write-Output "可用备份："
    for ($i = 0; $i -lt $backups.Count; $i++) {
        Write-Output "$($i + 1). $($backups[$i].Name) ($($backups[$i].LastWriteTime))"
    }
    
    $backupIndex = Read-Host "请选择备份编号"
    $backupIndex = [int]$backupIndex - 1
    
    if ($backupIndex -lt 0 -or $backupIndex -ge $backups.Count) {
        Write-Output "错误：无效的备份编号"
        return $false
    }
    
    $backupDir = $backups[$backupIndex].FullName
    
    # 检查备份内容
    $backupContents = Get-ChildItem -Path $backupDir -Directory
    
    if ($backupContents.Count -eq 0) {
        Write-Output "错误：备份内容为空"
        return $false
    }
    
    # 恢复插件
    foreach ($pluginDir in $backupContents) {
        $pluginName = $pluginDir.Name
        $targetDir = Join-Path $pluginsDir $pluginName
        
        # 覆盖目标目录
        if (Test-Path $targetDir) {
            Remove-Item $targetDir -Recurse -Force
        }
        
        Copy-Item $pluginDir.FullName $targetDir -Recurse -Force
        Write-Output "✓ 插件 $pluginName 已恢复"
    }
    
    Write-Output ""
    Write-Output "✓ 插件恢复完成"
    return $true
}

# 验证插件签名
function Verify-Plugin-Signature {
    Write-Output "========================================"
    Write-Output "验证插件签名..."
    Write-Output "========================================"
    Write-Output ""
    
    $pluginName = Read-Host "请输入插件名称"
    $pluginDir = Join-Path $pluginsDir $pluginName
    
    if (-not (Test-Path $pluginDir)) {
        Write-Output "错误：插件不存在"
        return $false
    }
    
    # 检查是否存在签名验证工具
    if (Test-Path "scripts/sign_plugins.ps1") {
        Write-Output "运行签名验证..."
        & "scripts/sign_plugins.ps1" -PluginPath $pluginDir -VerifyOnly
        
        if ($LASTEXITCODE -eq 0) {
            Write-Output ""
            Write-Output "✓ 插件签名验证通过"
        } else {
            Write-Output ""
            Write-Output "✗ 插件签名验证失败"
        }
    } else {
        Write-Output "错误：签名验证工具不存在"
        return $false
    }
    
    return $true
}

# 生成插件报告
function Generate-Plugin-Report {
    Write-Output "========================================"
    Write-Output "生成插件报告..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描插件目录
    $pluginDirs = Get-ChildItem -Path $pluginsDir -Directory | Where-Object { $_.Name -ne "output" -and $_.Name -ne "backup" }
    
    # 生成报告内容
    $reportContent = "# YMAxum 框架插件报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 插件统计

### 已安装插件数量
$($pluginDirs.Count)

## 插件详情

"
    
    foreach ($pluginDir in $pluginDirs) {
        $pluginName = $pluginDir.Name
        $manifestPath = Join-Path $pluginDir.FullName "manifest.json"
        
        $reportContent += "### $pluginName

"
        
        # 检查清单文件
        if (Test-Path $manifestPath) {
            try {
                $manifest = Get-Content -Path $manifestPath -Raw | ConvertFrom-Json
                $reportContent += "- 版本：$($manifest.version)
"
                $reportContent += "- 作者：$($manifest.author)
"
                $reportContent += "- 描述：$($manifest.description)
"
                $reportContent += "- 类型：$($manifest.type)
"
            } catch {
                $reportContent += "- 警告：清单文件格式错误
"
            }
        } else {
            $reportContent += "- 警告：缺少清单文件
"
        }
        
        # 检查插件状态
        $pluginStatus = "已安装"
        if (Test-Path (Join-Path $pluginDir.FullName "disabled")) {
            $pluginStatus = "已禁用"
        }
        $reportContent += "- 状态：$pluginStatus
"
        
        # 检查插件大小
        $pluginSize = (Get-ChildItem -Path $pluginDir -Recurse | Measure-Object -Property Length -Sum).Sum
        $pluginSizeMB = [math]::Round($pluginSize / 1MB, 2)
        $reportContent += "- 大小：$pluginSizeMB MB
"
        
        $reportContent += "
"
    }
    
    # 保存报告
    $reportFile = "$pluginOutputDir/plugin_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    $reportContent | Out-File -FilePath $reportFile -Force
    
    Write-Output "✓ 插件报告已生成：$reportFile"
    Write-Output ""
    Write-Output "✓ 插件报告生成完成"
    return $true
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            List-Plugins
        }
        "2" {
            Install-Plugin
        }
        "3" {
            Update-Plugin
        }
        "4" {
            Uninstall-Plugin
        }
        "5" {
            Enable-Plugin
        }
        "6" {
            Disable-Plugin
        }
        "7" {
            Backup-Plugin
        }
        "8" {
            Restore-Plugin
        }
        "9" {
            Verify-Plugin-Signature
        }
        "10" {
            Generate-Plugin-Report
        }
        "11" {
            Write-Output "退出插件管理工具..."
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
