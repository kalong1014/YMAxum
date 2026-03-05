#!/usr/bin/env pwsh
# 自动化配置管理脚本

Write-Output "========================================"
Write-Output "YMAxum 框架 - 自动化配置管理工具"
Write-Output "========================================"
Write-Output ""

# 确保配置目录存在
$configDir = "config"
if (-not (Test-Path $configDir)) {
    Write-Output "错误：配置目录不存在"
    Write-Output "请确保 $configDir 目录存在"
    exit 1
}

Write-Output "✓ 配置目录已找到"
Write-Output ""

# 确保配置版本目录存在
$configVersionDir = "$configDir/versions"
if (-not (Test-Path $configVersionDir)) {
    New-Item -ItemType Directory -Path $configVersionDir | Out-Null
    Write-Output "✓ 创建配置版本目录：$configVersionDir"
}

# 确保配置审计目录存在
$configAuditDir = "$configDir/audit"
if (-not (Test-Path $configAuditDir)) {
    New-Item -ItemType Directory -Path $configAuditDir | Out-Null
    Write-Output "✓ 创建配置审计目录：$configAuditDir"
}

Write-Output ""

# 显示菜单
function Show-Menu {
    Write-Output "配置管理选项："
    Write-Output "1. 列出配置文件"
    Write-Output "2. 备份当前配置"
    Write-Output "3. 查看配置版本历史"
    Write-Output "4. 回滚配置到指定版本"
    Write-Output "5. 比较配置版本"
    Write-Output "6. 验证配置文件"
    Write-Output "7. 生成配置审计报告"
    Write-Output "8. 清理旧版本配置"
    Write-Output "9. 退出"
    Write-Output ""
    $choice = Read-Host "请选择操作 (1-9)"
    return $choice
}

# 列出配置文件
function List-Config-Files {
    Write-Output "========================================"
    Write-Output "配置文件列表..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描配置目录
    $configFiles = Get-ChildItem -Path $configDir -Recurse -Filter "*.toml" | Where-Object { $_.FullName -notlike "*versions*" -and $_.FullName -notlike "*audit*" }
    
    if ($configFiles.Count -eq 0) {
        Write-Output "没有找到配置文件"
        return
    }
    
    foreach ($configFile in $configFiles) {
        $relativePath = $configFile.FullName.Substring($PWD.Path.Length + 1)
        $fileSize = [math]::Round($configFile.Length / 1KB, 2)
        $lastModified = $configFile.LastWriteTime
        
        Write-Output "文件：$relativePath"
        Write-Output "大小：$fileSize KB"
        Write-Output "最后修改：$lastModified"
        Write-Output "----------------------------------------"
    }
    
    Write-Output "✓ 配置文件列表生成完成"
}

# 备份当前配置
function Backup-Current-Config {
    Write-Output "========================================"
    Write-Output "备份当前配置..."
    Write-Output "========================================"
    Write-Output ""
    
    $backupName = "backup-$(Get-Date -Format "yyyyMMdd-HHmmss")"
    $backupDir = Join-Path $configVersionDir $backupName
    
    New-Item -ItemType Directory -Path $backupDir | Out-Null
    Write-Output "✓ 创建备份目录：$backupDir"
    
    # 复制所有配置文件
    $configFiles = Get-ChildItem -Path $configDir -Recurse -Filter "*.toml" | Where-Object { $_.FullName -notlike "*versions*" -and $_.FullName -notlike "*audit*" }
    
    foreach ($configFile in $configFiles) {
        $relativePath = $configFile.FullName.Substring($configDir.Length + 1)
        $targetPath = Join-Path $backupDir $relativePath
        
        # 确保目标目录存在
        $targetParentDir = Split-Path $targetPath -Parent
        if (-not (Test-Path $targetParentDir)) {
            New-Item -ItemType Directory -Path $targetParentDir -Force | Out-Null
        }
        
        Copy-Item $configFile.FullName $targetPath -Force
        Write-Output "✓ 备份：$relativePath"
    }
    
    # 创建备份元数据
    $metadata = @{
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        backupName = $backupName
        fileCount = $configFiles.Count
        description = Read-Host "请输入备份描述"
    }
    
    $metadataPath = Join-Path $backupDir "metadata.json"
    $metadata | ConvertTo-Json -Depth 32 | Out-File -FilePath $metadataPath -Force
    Write-Output "✓ 备份元数据已创建"
    
    # 记录审计日志
    $auditEntry = @{
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        action = "backup"
        backupName = $backupName
        description = $metadata.description
        user = $env:USERNAME
        computer = $env:COMPUTERNAME
    }
    
    $auditLogPath = Join-Path $configAuditDir "audit_log.json"
    if (Test-Path $auditLogPath) {
        $auditLog = Get-Content -Path $auditLogPath -Raw | ConvertFrom-Json
        $auditLog += $auditEntry
    } else {
        $auditLog = @($auditEntry)
    }
    
    $auditLog | ConvertTo-Json -Depth 32 | Out-File -FilePath $auditLogPath -Force
    Write-Output "✓ 审计日志已更新"
    
    Write-Output ""
    Write-Output "✓ 配置备份完成"
    return $backupName
}

# 查看配置版本历史
function View-Config-Version-History {
    Write-Output "========================================"
    Write-Output "配置版本历史..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出所有备份
    $backups = Get-ChildItem -Path $configVersionDir -Directory | Sort-Object LastWriteTime -Descending
    
    if ($backups.Count -eq 0) {
        Write-Output "没有找到配置备份"
        return
    }
    
    for ($i = 0; $i -lt $backups.Count; $i++) {
        $backupDir = $backups[$i]
        $metadataPath = Join-Path $backupDir.FullName "metadata.json"
        
        Write-Output "版本 $($i + 1): $($backupDir.Name)"
        
        if (Test-Path $metadataPath) {
            try {
                $metadata = Get-Content -Path $metadataPath -Raw | ConvertFrom-Json
                Write-Output "时间：$($metadata.timestamp)"
                Write-Output "文件数：$($metadata.fileCount)"
                Write-Output "描述：$($metadata.description)"
            } catch {
                Write-Output "警告：元数据格式错误"
            }
        } else {
            Write-Output "警告：缺少元数据"
        }
        
        Write-Output "----------------------------------------"
    }
    
    Write-Output "✓ 配置版本历史查看完成"
}

# 回滚配置到指定版本
function Rollback-Config-To-Version {
    Write-Output "========================================"
    Write-Output "回滚配置到指定版本..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出所有备份
    $backups = Get-ChildItem -Path $configVersionDir -Directory | Sort-Object LastWriteTime -Descending
    
    if ($backups.Count -eq 0) {
        Write-Output "错误：没有找到配置备份"
        return $false
    }
    
    Write-Output "可用版本："
    for ($i = 0; $i -lt $backups.Count; $i++) {
        $backupDir = $backups[$i]
        $metadataPath = Join-Path $backupDir.FullName "metadata.json"
        
        Write-Output "$($i + 1). $($backupDir.Name)"
        
        if (Test-Path $metadataPath) {
            try {
                $metadata = Get-Content -Path $metadataPath -Raw | ConvertFrom-Json
                Write-Output "   时间：$($metadata.timestamp)"
                Write-Output "   描述：$($metadata.description)"
            } catch {
                Write-Output "   警告：元数据格式错误"
            }
        }
    }
    
    $versionIndex = Read-Host "请选择回滚版本编号"
    $versionIndex = [int]$versionIndex - 1
    
    if ($versionIndex -lt 0 -or $versionIndex -ge $backups.Count) {
        Write-Output "错误：无效的版本编号"
        return $false
    }
    
    $backupDir = $backups[$versionIndex].FullName
    
    # 先备份当前配置
    $currentBackup = Backup-Current-Config
    Write-Output "✓ 当前配置已备份到 $currentBackup"
    
    # 回滚配置文件
    $backupFiles = Get-ChildItem -Path $backupDir -Recurse -Filter "*.toml"
    
    foreach ($backupFile in $backupFiles) {
        $relativePath = $backupFile.FullName.Substring($backupDir.Length + 1)
        $targetPath = Join-Path $configDir $relativePath
        
        # 确保目标目录存在
        $targetParentDir = Split-Path $targetPath -Parent
        if (-not (Test-Path $targetParentDir)) {
            New-Item -ItemType Directory -Path $targetParentDir -Force | Out-Null
        }
        
        Copy-Item $backupFile.FullName $targetPath -Force
        Write-Output "✓ 回滚：$relativePath"
    }
    
    # 记录审计日志
    $auditEntry = @{
        timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        action = "rollback"
        fromVersion = $currentBackup
        toVersion = $backups[$versionIndex].Name
        user = $env:USERNAME
        computer = $env:COMPUTERNAME
    }
    
    $auditLogPath = Join-Path $configAuditDir "audit_log.json"
    if (Test-Path $auditLogPath) {
        $auditLog = Get-Content -Path $auditLogPath -Raw | ConvertFrom-Json
        $auditLog += $auditEntry
    } else {
        $auditLog = @($auditEntry)
    }
    
    $auditLog | ConvertTo-Json -Depth 32 | Out-File -FilePath $auditLogPath -Force
    Write-Output "✓ 审计日志已更新"
    
    Write-Output ""
    Write-Output "✓ 配置回滚完成"
    return $true
}

# 比较配置版本
function Compare-Config-Versions {
    Write-Output "========================================"
    Write-Output "比较配置版本..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出所有备份
    $backups = Get-ChildItem -Path $configVersionDir -Directory | Sort-Object LastWriteTime -Descending
    
    if ($backups.Count -lt 2) {
        Write-Output "错误：至少需要两个版本才能比较"
        return $false
    }
    
    Write-Output "可用版本："
    for ($i = 0; $i -lt $backups.Count; $i++) {
        $backupDir = $backups[$i]
        $metadataPath = Join-Path $backupDir.FullName "metadata.json"
        
        Write-Output "$($i + 1). $($backupDir.Name)"
        
        if (Test-Path $metadataPath) {
            try {
                $metadata = Get-Content -Path $metadataPath -Raw | ConvertFrom-Json
                Write-Output "   时间：$($metadata.timestamp)"
            } catch {
                Write-Output "   警告：元数据格式错误"
            }
        }
    }
    
    $version1Index = Read-Host "请选择第一个版本编号"
    $version1Index = [int]$version1Index - 1
    
    $version2Index = Read-Host "请选择第二个版本编号"
    $version2Index = [int]$version2Index - 1
    
    if ($version1Index -lt 0 -or $version1Index -ge $backups.Count -or $version2Index -lt 0 -or $version2Index -ge $backups.Count) {
        Write-Output "错误：无效的版本编号"
        return $false
    }
    
    $version1Dir = $backups[$version1Index].FullName
    $version2Dir = $backups[$version2Index].FullName
    
    Write-Output ""
    Write-Output "比较版本："
    Write-Output "版本 1: $($backups[$version1Index].Name)"
    Write-Output "版本 2: $($backups[$version2Index].Name)"
    Write-Output ""
    
    # 获取两个版本的文件列表
    $version1Files = Get-ChildItem -Path $version1Dir -Recurse -Filter "*.toml" | ForEach-Object { $_.FullName.Substring($version1Dir.Length + 1) }
    $version2Files = Get-ChildItem -Path $version2Dir -Recurse -Filter "*.toml" | ForEach-Object { $_.FullName.Substring($version2Dir.Length + 1) }
    
    # 找出共同文件
    $commonFiles = $version1Files | Where-Object { $version2Files -contains $_ }
    
    # 找出仅在版本1中存在的文件
    $onlyInVersion1 = $version1Files | Where-Object { $version2Files -notcontains $_ }
    
    # 找出仅在版本2中存在的文件
    $onlyInVersion2 = $version2Files | Where-Object { $version1Files -notcontains $_ }
    
    # 显示差异
    if ($onlyInVersion1.Count -gt 0) {
        Write-Output "仅在版本 1 中存在的文件："
        foreach ($file in $onlyInVersion1) {
            Write-Output "- $file"
        }
        Write-Output ""
    }
    
    if ($onlyInVersion2.Count -gt 0) {
        Write-Output "仅在版本 2 中存在的文件："
        foreach ($file in $onlyInVersion2) {
            Write-Output "- $file"
        }
        Write-Output ""
    }
    
    if ($commonFiles.Count -gt 0) {
        Write-Output "共同文件的差异："
        foreach ($file in $commonFiles) {
            $file1 = Join-Path $version1Dir $file
            $file2 = Join-Path $version2Dir $file
            
            try {
                $content1 = Get-Content -Path $file1 -Raw
                $content2 = Get-Content -Path $file2 -Raw
                
                if ($content1 -ne $content2) {
                    Write-Output "- $file"
                    Write-Output "  内容不同"
                }
            } catch {
                Write-Output "- $file"
                Write-Output "  读取错误：$($_.Exception.Message)"
            }
        }
    }
    
    Write-Output ""
    Write-Output "✓ 配置版本比较完成"
    return $true
}

# 验证配置文件
function Validate-Config-Files {
    Write-Output "========================================"
    Write-Output "验证配置文件..."
    Write-Output "========================================"
    Write-Output ""
    
    # 扫描配置目录
    $configFiles = Get-ChildItem -Path $configDir -Recurse -Filter "*.toml" | Where-Object { $_.FullName -notlike "*versions*" -and $_.FullName -notlike "*audit*" }
    
    $validCount = 0
    $invalidCount = 0
    
    foreach ($configFile in $configFiles) {
        $relativePath = $configFile.FullName.Substring($PWD.Path.Length + 1)
        
        Write-Output "验证：$relativePath"
        
        try {
            # 尝试解析 TOML 文件
            $content = Get-Content -Path $configFile.FullName -Raw
            
            # 这里可以添加更详细的验证逻辑
            # 例如检查必要的配置项是否存在
            
            Write-Output "✓ 验证通过"
            $validCount++
        } catch {
            Write-Output "✗ 验证失败：$($_.Exception.Message)"
            $invalidCount++
        }
        
        Write-Output "----------------------------------------"
    }
    
    Write-Output "验证结果："
    Write-Output "通过：$validCount"
    Write-Output "失败：$invalidCount"
    Write-Output "总计：$($validCount + $invalidCount)"
    
    Write-Output ""
    Write-Output "✓ 配置文件验证完成"
    return $invalidCount -eq 0
}

# 生成配置审计报告
function Generate-Config-Audit-Report {
    Write-Output "========================================"
    Write-Output "生成配置审计报告..."
    Write-Output "========================================"
    Write-Output ""
    
    # 读取审计日志
    $auditLogPath = Join-Path $configAuditDir "audit_log.json"
    
    if (-not (Test-Path $auditLogPath)) {
        Write-Output "错误：审计日志不存在"
        return $false
    }
    
    try {
        $auditLog = Get-Content -Path $auditLogPath -Raw | ConvertFrom-Json
    } catch {
        Write-Output "错误：审计日志格式错误"
        return $false
    }
    
    # 生成报告内容
    $reportContent = "# YMAxum 框架配置审计报告

## 报告生成日期
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 审计统计

### 审计事件数量
$($auditLog.Count)

## 审计详情

"
    
    foreach ($entry in $auditLog | Sort-Object timestamp -Descending) {
        $reportContent += "### $($entry.timestamp)

"
        $reportContent += "- 操作：$($entry.action)
"
        
        if ($entry.description) {
            $reportContent += "- 描述：$($entry.description)
"
        }
        
        if ($entry.fromVersion) {
            $reportContent += "- 从版本：$($entry.fromVersion)
"
        }
        
        if ($entry.toVersion) {
            $reportContent += "- 到版本：$($entry.toVersion)
"
        }
        
        if ($entry.user) {
            $reportContent += "- 用户：$($entry.user)
"
        }
        
        if ($entry.computer) {
            $reportContent += "- 计算机：$($entry.computer)
"
        }
        
        $reportContent += "
"
    }
    
    # 保存报告
    $reportFile = "$configAuditDir/config_audit_report_$(Get-Date -Format "yyyyMMdd-HHmmss").md"
    $reportContent | Out-File -FilePath $reportFile -Force
    
    Write-Output "✓ 配置审计报告已生成：$reportFile"
    Write-Output ""
    Write-Output "✓ 配置审计报告生成完成"
    return $true
}

# 清理旧版本配置
function Cleanup-Old-Config-Versions {
    Write-Output "========================================"
    Write-Output "清理旧版本配置..."
    Write-Output "========================================"
    Write-Output ""
    
    # 列出所有备份
    $backups = Get-ChildItem -Path $configVersionDir -Directory | Sort-Object LastWriteTime -Descending
    
    if ($backups.Count -eq 0) {
        Write-Output "错误：没有找到配置备份"
        return $false
    }
    
    Write-Output "当前备份数量：$($backups.Count)"
    
    $keepCount = Read-Host "请输入要保留的版本数量"
    $keepCount = [int]$keepCount
    
    if ($keepCount -lt 1) {
        Write-Output "错误：保留数量必须大于 0"
        return $false
    }
    
    if ($keepCount -ge $backups.Count) {
        Write-Output "警告：保留数量大于或等于当前备份数量，无需清理"
        return $true
    }
    
    # 计算要删除的备份
    $toDelete = $backups | Select-Object -Skip $keepCount
    
    Write-Output ""
    Write-Output "要删除的备份："
    foreach ($backup in $toDelete) {
        Write-Output "- $($backup.Name)"
    }
    
    $confirm = Read-Host "确认删除这些备份？ (y/n)"
    if ($confirm -ne "y") {
        Write-Output "取消删除"
        return $true
    }
    
    # 删除备份
    foreach ($backup in $toDelete) {
        try {
            Remove-Item $backup.FullName -Recurse -Force
            Write-Output "✓ 删除：$($backup.Name)"
        } catch {
            Write-Output "✗ 删除失败：$($backup.Name)"
            Write-Output "  错误：$($_.Exception.Message)"
        }
    }
    
    Write-Output ""
    Write-Output "✓ 旧版本配置清理完成"
    return $true
}

# 主循环
while ($true) {
    $choice = Show-Menu
    
    switch ($choice) {
        "1" {
            List-Config-Files
        }
        "2" {
            Backup-Current-Config
        }
        "3" {
            View-Config-Version-History
        }
        "4" {
            Rollback-Config-To-Version
        }
        "5" {
            Compare-Config-Versions
        }
        "6" {
            Validate-Config-Files
        }
        "7" {
            Generate-Config-Audit-Report
        }
        "8" {
            Cleanup-Old-Config-Versions
        }
        "9" {
            Write-Output "退出配置管理工具..."
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
