#!/usr/bin/env powershell

# 自动化版本管理工具
# 功能：管理项目版本、更新版本文件、生成版本报告等

function Write-Header {
    param(
        [string]$Title
    )
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "$Title" -ForegroundColor Green
    Write-Host "========================================`n" -ForegroundColor Cyan
}

function Write-Step {
    param(
        [string]$Message
    )
    Write-Host "[STEP] $Message" -ForegroundColor Yellow
}

function Write-Success {
    param(
        [string]$Message
    )
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Error {
    param(
        [string]$Message
    )
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Info {
    param(
        [string]$Message
    )
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Get-CurrentVersion {
    param(
        [string]$ProjectRoot = "."
    )
    Write-Step "获取当前版本信息"
    try {
        # 检查 Cargo.toml 文件
        $cargoTomlPath = Join-Path -Path $ProjectRoot -ChildPath "Cargo.toml"
        if (Test-Path -Path $cargoTomlPath) {
            $cargoContent = Get-Content -Path $cargoTomlPath -Encoding UTF8
            if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
                $version = $matches[1]
                Write-Success "从 Cargo.toml 获取版本成功: $version"
                return $version
            } else {
                Write-Error "无法从 Cargo.toml 解析版本"
                return $null
            }
        } else {
            Write-Error "Cargo.toml 文件不存在"
            return $null
        }
    } catch {
        Write-Error "获取当前版本时出错: $_"
        return $null
    }
}

function Update-Version {
    param(
        [string]$NewVersion,
        [string]$ProjectRoot = "."
    )
    Write-Step "更新版本到: $NewVersion"
    try {
        # 更新 Cargo.toml 文件
        $cargoTomlPath = Join-Path -Path $ProjectRoot -ChildPath "Cargo.toml"
        if (Test-Path -Path $cargoTomlPath) {
            $cargoContent = Get-Content -Path $cargoTomlPath -Encoding UTF8
            $updatedContent = $cargoContent -replace 'version\s*=\s*"([^"]+)"', "version = \"$NewVersion\""
            Set-Content -Path $cargoTomlPath -Value $updatedContent -Encoding UTF8
            Write-Success "Cargo.toml 版本更新成功"
        } else {
            Write-Error "Cargo.toml 文件不存在"
            return $false
        }
        
        # 更新版本配置文件
        $versionTomlPath = Join-Path -Path $ProjectRoot -ChildPath "config\version.toml"
        if (Test-Path -Path $versionTomlPath) {
            $versionContent = Get-Content -Path $versionTomlPath -Encoding UTF8
            $updatedVersionContent = $versionContent -replace 'version\s*=\s*"([^"]+)"', "version = \"$NewVersion\""
            # 更新时间戳
            $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
            $updatedVersionContent = $updatedVersionContent -replace 'timestamp\s*=\s*"([^"]+)"', "timestamp = \"$timestamp\""
            Set-Content -Path $versionTomlPath -Value $updatedVersionContent -Encoding UTF8
            Write-Success "config/version.toml 版本更新成功"
        } else {
            Write-Info "config/version.toml 文件不存在，跳过更新"
        }
        
        # 更新 RELEASE_NOTES.md 文件
        $releaseNotesPath = Join-Path -Path $ProjectRoot -ChildPath "RELEASE_NOTES.md"
        if (Test-Path -Path $releaseNotesPath) {
            $releaseContent = Get-Content -Path $releaseNotesPath -Encoding UTF8
            $timestamp = Get-Date -Format "yyyy-MM-dd"
            $newReleaseEntry = "## Version $NewVersion ($timestamp)`n`n- 版本更新到 $NewVersion`n"
            # 插入到文件开头
            $updatedReleaseContent = $newReleaseEntry + "`n" + $releaseContent
            Set-Content -Path $releaseNotesPath -Value $updatedReleaseContent -Encoding UTF8
            Write-Success "RELEASE_NOTES.md 版本更新成功"
        } else {
            Write-Info "RELEASE_NOTES.md 文件不存在，跳过更新"
        }
        
        Write-Success "版本更新完成: $NewVersion"
        return $true
    } catch {
        Write-Error "更新版本时出错: $_"
        return $false
    }
}

function Increment-Version {
    param(
        [string]$IncrementType = "patch", # major, minor, patch
        [string]$ProjectRoot = "."
    )
    Write-Step "递增版本号: $IncrementType"
    try {
        $currentVersion = Get-CurrentVersion -ProjectRoot $ProjectRoot
        if ($currentVersion) {
            if ($currentVersion -match '^(\d+)\.(\d+)\.(\d+)(.*)$') {
                $major = [int]$matches[1]
                $minor = [int]$matches[2]
                $patch = [int]$matches[3]
                $suffix = $matches[4]
                
                switch ($IncrementType) {
                    "major" {
                        $major++
                        $minor = 0
                        $patch = 0
                    }
                    "minor" {
                        $minor++
                        $patch = 0
                    }
                    "patch" {
                        $patch++
                    }
                    default {
                        Write-Error "无效的递增类型: $IncrementType"
                        return $null
                    }
                }
                
                $newVersion = "$major.$minor.$patch$suffix"
                $result = Update-Version -NewVersion $newVersion -ProjectRoot $ProjectRoot
                if ($result) {
                    Write-Success "版本递增成功: $currentVersion -> $newVersion"
                    return $newVersion
                } else {
                    Write-Error "版本递增失败"
                    return $null
                }
            } else {
                Write-Error "无效的版本格式: $currentVersion"
                return $null
            }
        } else {
            Write-Error "无法获取当前版本"
            return $null
        }
    } catch {
        Write-Error "递增版本时出错: $_"
        return $null
    }
}

function Generate-VersionReport {
    param(
        [string]$OutputDir = "./version_reports",
        [string]$ProjectRoot = "."
    )
    Write-Step "生成版本报告"
    try {
        # 创建输出目录
        if (-not (Test-Path -Path $OutputDir)) {
            New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
        }
        
        # 获取当前版本
        $currentVersion = Get-CurrentVersion -ProjectRoot $ProjectRoot
        
        # 生成版本报告
        $reportContent = @"
# 版本报告

## 当前版本
$currentVersion

## 生成时间
$(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## 项目信息
- 项目路径: $ProjectRoot
- 操作系统: $([System.Environment]::OSVersion.VersionString)

## 版本历史
"@
        
        # 检查 RELEASE_NOTES.md 文件
        $releaseNotesPath = Join-Path -Path $ProjectRoot -ChildPath "RELEASE_NOTES.md"
        if (Test-Path -Path $releaseNotesPath) {
            $releaseContent = Get-Content -Path $releaseNotesPath -Encoding UTF8
            $reportContent += "`n$releaseContent"
        } else {
            $reportContent += "`nRELEASE_NOTES.md 文件不存在"
        }
        
        # 保存报告
        $reportPath = Join-Path -Path $OutputDir -ChildPath "version_report_$(Get-Date -Format "yyyyMMdd_HHmmss").md"
        Set-Content -Path $reportPath -Value $reportContent -Encoding UTF8
        
        Write-Success "版本报告生成成功，保存在: $reportPath"
        return $reportPath
    } catch {
        Write-Error "生成版本报告时出错: $_"
        return $null
    }
}

function Validate-Version {
    param(
        [string]$Version
    )
    Write-Step "验证版本格式: $Version"
    try {
        if ($Version -match '^\d+\.\d+\.\d+.*$') {
            Write-Success "版本格式验证通过"
            return $true
        } else {
            Write-Error "版本格式验证失败"
            return $false
        }
    } catch {
        Write-Error "验证版本时出错: $_"
        return $false
    }
}

function Check-VersionConsistency {
    param(
        [string]$ProjectRoot = "."
    )
    Write-Step "检查版本一致性"
    try {
        $versions = @{}
        
        # 检查 Cargo.toml
        $cargoTomlPath = Join-Path -Path $ProjectRoot -ChildPath "Cargo.toml"
        if (Test-Path -Path $cargoTomlPath) {
            $cargoContent = Get-Content -Path $cargoTomlPath -Encoding UTF8
            if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
                $versions["Cargo.toml"] = $matches[1]
            }
        }
        
        # 检查 config/version.toml
        $versionTomlPath = Join-Path -Path $ProjectRoot -ChildPath "config\version.toml"
        if (Test-Path -Path $versionTomlPath) {
            $versionContent = Get-Content -Path $versionTomlPath -Encoding UTF8
            if ($versionContent -match 'version\s*=\s*"([^"]+)"') {
                $versions["config/version.toml"] = $matches[1]
            }
        }
        
        # 检查源代码中的版本引用
        $srcDir = Join-Path -Path $ProjectRoot -ChildPath "src"
        if (Test-Path -Path $srcDir) {
            $rustFiles = Get-ChildItem -Path $srcDir -Recurse -Filter "*.rs"
            foreach ($file in $rustFiles) {
                $content = Get-Content -Path $file.FullName -Encoding UTF8
                if ($content -match 'version\s*=\s*"([^"]+)"') {
                    $relativePath = $file.FullName.Substring($ProjectRoot.Length + 1)
                    $versions[$relativePath] = $matches[1]
                }
            }
        }
        
        # 检查一致性
        if ($versions.Count -gt 0) {
            $uniqueVersions = $versions.Values | Select-Object -Unique
            if ($uniqueVersions.Count -eq 1) {
                Write-Success "版本一致性检查通过，所有文件版本相同: $($uniqueVersions[0])"
                return $true
            } else {
                Write-Error "版本一致性检查失败，发现不同版本:"
                foreach ($key in $versions.Keys) {
                    Write-Host "- $key: $($versions[$key])" -ForegroundColor Red
                }
                return $false
            }
        } else {
            Write-Error "未找到版本信息"
            return $false
        }
    } catch {
        Write-Error "检查版本一致性时出错: $_"
        return $false
    }
}

function Tag-Version {
    param(
        [string]$Version,
        [string]$ProjectRoot = "."
    )
    Write-Step "为版本创建 Git 标签: $Version"
    try {
        # 检查是否在 Git 仓库中
        $gitStatus = git status
        if ($LASTEXITCODE -eq 0) {
            # 创建标签
            $tagOutput = git tag -a "v$Version" -m "Version $Version"
            if ($LASTEXITCODE -eq 0) {
                Write-Success "Git 标签创建成功: v$Version"
                return $true
            } else {
                Write-Error "Git 标签创建失败"
                return $false
            }
        } else {
            Write-Info "不在 Git 仓库中，跳过标签创建"
            return $true
        }
    } catch {
        Write-Error "创建 Git 标签时出错: $_"
        return $false
    }
}

function Show-Menu {
    Write-Header "自动化版本管理工具"
    Write-Host "1. 获取当前版本信息" -ForegroundColor Yellow
    Write-Host "2. 更新版本"
    Write-Host "3. 递增版本号"
    Write-Host "4. 生成版本报告"
    Write-Host "5. 验证版本格式"
    Write-Host "6. 检查版本一致性"
    Write-Host "7. 为版本创建 Git 标签"
    Write-Host "8. 退出"
    Write-Host "`n请选择操作: " -ForegroundColor Cyan -NoNewline
}

function Main {
    $running = $true
    while ($running) {
        Show-Menu
        $choice = Read-Host
        
        switch ($choice) {
            "1" {
                $result = Get-CurrentVersion
                if ($result) {
                    Write-Host "`n当前版本: $result" -ForegroundColor Green
                }
            }
            "2" {
                $newVersion = Read-Host "请输入新版本号: "
                $validation = Validate-Version -Version $newVersion
                if ($validation) {
                    $result = Update-Version -NewVersion $newVersion
                    if ($result) {
                        Write-Host "`n版本更新成功: $newVersion" -ForegroundColor Green
                    }
                }
            }
            "3" {
                Write-Host "`n选择递增类型:"
                Write-Host "1. Major (主版本)"
                Write-Host "2. Minor (次版本)"
                Write-Host "3. Patch (补丁版本)"
                $incrementChoice = Read-Host "请选择: "
                
                $incrementType = "patch"
                switch ($incrementChoice) {
                    "1" { $incrementType = "major" }
                    "2" { $incrementType = "minor" }
                    "3" { $incrementType = "patch" }
                    default { $incrementType = "patch" }
                }
                
                $result = Increment-Version -IncrementType $incrementType
                if ($result) {
                    Write-Host "`n版本递增成功: $result" -ForegroundColor Green
                }
            }
            "4" {
                $outputDir = Read-Host "请输入报告输出目录 (默认: ./version_reports): "
                if ([string]::IsNullOrEmpty($outputDir)) {
                    $outputDir = "./version_reports"
                }
                $result = Generate-VersionReport -OutputDir $outputDir
                if ($result) {
                    Write-Host "`n版本报告生成成功: $result" -ForegroundColor Green
                }
            }
            "5" {
                $version = Read-Host "请输入要验证的版本号: "
                $result = Validate-Version -Version $version
                if ($result) {
                    Write-Host "`n版本格式验证通过" -ForegroundColor Green
                }
            }
            "6" {
                $result = Check-VersionConsistency
                if ($result) {
                    Write-Host "`n版本一致性检查通过" -ForegroundColor Green
                }
            }
            "7" {
                $version = Read-Host "请输入要标记的版本号 (留空使用当前版本): "
                if ([string]::IsNullOrEmpty($version)) {
                    $version = Get-CurrentVersion
                }
                if ($version) {
                    $result = Tag-Version -Version $version
                    if ($result) {
                        Write-Host "`nGit 标签创建成功: v$version" -ForegroundColor Green
                    }
                }
            }
            "8" {
                $running = $false
                Write-Host "`n退出工具..." -ForegroundColor Green
            }
            default {
                Write-Host "`n无效的选择，请重新输入" -ForegroundColor Red
            }
        }
        
        if ($running) {
            Write-Host "`n按任意键继续..." -ForegroundColor Gray
            $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") | Out-Null
        }
    }
}

# 执行主函数
Main
