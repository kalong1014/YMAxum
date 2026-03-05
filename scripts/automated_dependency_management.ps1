#!/usr/bin/env powershell

# 自动化依赖管理工具
# 功能：管理项目依赖、解决冲突、分析依赖树、生成依赖报告等

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

function Get-DependencyStatus {
    Write-Step "检查依赖状态"
    try {
        $output = cargo tree --all
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖状态检查成功"
            return $output
        } else {
            Write-Error "依赖状态检查失败"
            return $null
        }
    } catch {
        Write-Error "检查依赖状态时出错: $_"
        return $null
    }
}

function Update-Dependencies {
    Write-Step "更新依赖版本"
    try {
        $output = cargo update
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖更新成功"
            return $output
        } else {
            Write-Error "依赖更新失败"
            return $null
        }
    } catch {
        Write-Error "更新依赖时出错: $_"
        return $null
    }
}

function Update-SpecificDependency {
    param(
        [string]$Dependency
    )
    Write-Step "更新特定依赖: $Dependency"
    try {
        $output = cargo update -p $Dependency
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖 $Dependency 更新成功"
            return $output
        } else {
            Write-Error "依赖 $Dependency 更新失败"
            return $null
        }
    } catch {
        Write-Error "更新依赖 $Dependency 时出错: $_"
        return $null
    }
}

function Resolve-DependencyConflicts {
    Write-Step "解决依赖冲突"
    try {
        # 首先检查是否有冲突
        $conflicts = cargo tree --duplicates
        if ($LASTEXITCODE -eq 0) {
            if ($conflicts -match "Duplicate dependencies found:") {
                Write-Info "发现依赖冲突，尝试解决..."
                # 尝试通过更新依赖来解决冲突
                $updateOutput = cargo update
                if ($LASTEXITCODE -eq 0) {
                    # 再次检查冲突是否解决
                    $newConflicts = cargo tree --duplicates
                    if (-not ($newConflicts -match "Duplicate dependencies found:")) {
                        Write-Success "依赖冲突已解决"
                        return "依赖冲突已解决"
                    } else {
                        Write-Info "仍存在依赖冲突，生成冲突报告..."
                        return $newConflicts
                    }
                } else {
                    Write-Error "更新依赖失败，无法解决冲突"
                    return $null
                }
            } else {
                Write-Success "未发现依赖冲突"
                return "未发现依赖冲突"
            }
        } else {
            Write-Error "检查依赖冲突失败"
            return $null
        }
    } catch {
        Write-Error "解决依赖冲突时出错: $_"
        return $null
    }
}

function Analyze-DependencyTree {
    Write-Step "分析依赖树"
    try {
        $output = cargo tree --invert
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖树分析成功"
            return $output
        } else {
            Write-Error "依赖树分析失败"
            return $null
        }
    } catch {
        Write-Error "分析依赖树时出错: $_"
        return $null
    }
}

function Get-DependencyVersions {
    param(
        [string]$Dependency
    )
    Write-Step "获取依赖版本: $Dependency"
    try {
        $output = cargo tree -p $Dependency
        if ($LASTEXITCODE -eq 0) {
            Write-Success "获取依赖版本成功"
            return $output
        } else {
            Write-Error "获取依赖版本失败"
            return $null
        }
    } catch {
        Write-Error "获取依赖版本时出错: $_"
        return $null
    }
}

function Generate-DependencyReport {
    param(
        [string]$OutputDir = "./dependency_reports"
    )
    Write-Step "生成依赖报告"
    try {
        # 创建输出目录
        if (-not (Test-Path -Path $OutputDir)) {
            New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
        }
        
        # 生成依赖树报告
        $treeOutput = cargo tree
        $treeOutput | Out-File -FilePath "$OutputDir/dependency_tree.txt" -Encoding UTF8
        
        # 生成重复依赖报告
        $duplicatesOutput = cargo tree --duplicates
        $duplicatesOutput | Out-File -FilePath "$OutputDir/duplicate_dependencies.txt" -Encoding UTF8
        
        # 生成依赖版本报告
        $versionsOutput = cargo pkgid
        $versionsOutput | Out-File -FilePath "$OutputDir/dependency_versions.txt" -Encoding UTF8
        
        Write-Success "依赖报告生成成功，保存在 $OutputDir 目录"
        return "依赖报告生成成功"
    } catch {
        Write-Error "生成依赖报告时出错: $_"
        return $null
    }
}

function Clean-DependencyCache {
    Write-Step "清理依赖缓存"
    try {
        $output = cargo clean
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖缓存清理成功"
            return $output
        } else {
            Write-Error "依赖缓存清理失败"
            return $null
        }
    } catch {
        Write-Error "清理依赖缓存时出错: $_"
        return $null
    }
}

function Check-DependencyUpdates {
    Write-Step "检查依赖更新"
    try {
        $output = cargo outdated
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖更新检查成功"
            return $output
        } else {
            Write-Info "cargo outdated 未安装，尝试安装..."
            # 尝试安装 cargo-outdated
            $installOutput = cargo install cargo-outdated
            if ($LASTEXITCODE -eq 0) {
                Write-Success "cargo outdated 安装成功，重新检查更新"
                $output = cargo outdated
                if ($LASTEXITCODE -eq 0) {
                    Write-Success "依赖更新检查成功"
                    return $output
                } else {
                    Write-Error "依赖更新检查失败"
                    return $null
                }
            } else {
                Write-Error "cargo outdated 安装失败"
                return $null
            }
        }
    } catch {
        Write-Error "检查依赖更新时出错: $_"
        return $null
    }
}

function Export-Dependencies {
    param(
        [string]$OutputFile = "./dependencies.json"
    )
    Write-Step "导出依赖信息"
    try {
        # 生成依赖树并解析
        $treeOutput = cargo tree --format "{p} {v}"
        if ($LASTEXITCODE -eq 0) {
            # 解析依赖信息
            $dependencies = @()
            $lines = $treeOutput -split "`n"
            foreach ($line in $lines) {
                if ($line -match "^(.*)\s+(\d+\.\d+\.\d+.*)$") {
                    $name = $matches[1]
                    $version = $matches[2]
                    $dependencies += @{
                        name = $name
                        version = $version
                    }
                }
            }
            
            # 转换为JSON并导出
            $jsonOutput = $dependencies | ConvertTo-Json -Depth 3
            $jsonOutput | Out-File -FilePath $OutputFile -Encoding UTF8
            
            Write-Success "依赖信息导出成功，保存在 $OutputFile"
            return "依赖信息导出成功"
        } else {
            Write-Error "导出依赖信息失败"
            return $null
        }
    } catch {
        Write-Error "导出依赖信息时出错: $_"
        return $null
    }
}

function Import-Dependencies {
    param(
        [string]$InputFile = "./dependencies.json"
    )
    Write-Step "导入依赖信息"
    try {
        if (Test-Path -Path $InputFile) {
            $jsonContent = Get-Content -Path $InputFile -Encoding UTF8
            $dependencies = $jsonContent | ConvertFrom-Json
            
            Write-Info "导入的依赖信息:"
            foreach ($dep in $dependencies) {
                Write-Info "- $($dep.name): $($dep.version)"
            }
            
            Write-Success "依赖信息导入成功"
            return $dependencies
        } else {
            Write-Error "依赖信息文件不存在: $InputFile"
            return $null
        }
    } catch {
        Write-Error "导入依赖信息时出错: $_"
        return $null
    }
}

function Test-DependencyBuild {
    Write-Step "测试依赖构建"
    try {
        $output = cargo build --quiet
        if ($LASTEXITCODE -eq 0) {
            Write-Success "依赖构建测试成功"
            return "依赖构建测试成功"
        } else {
            Write-Error "依赖构建测试失败"
            return $null
        }
    } catch {
        Write-Error "测试依赖构建时出错: $_"
        return $null
    }
}

function Show-Menu {
    Write-Header "自动化依赖管理工具"
    Write-Host "1. 检查依赖状态" -ForegroundColor Yellow
    Write-Host "2. 更新所有依赖"
    Write-Host "3. 更新特定依赖"
    Write-Host "4. 解决依赖冲突"
    Write-Host "5. 分析依赖树"
    Write-Host "6. 获取特定依赖版本"
    Write-Host "7. 生成依赖报告"
    Write-Host "8. 清理依赖缓存"
    Write-Host "9. 检查依赖更新"
    Write-Host "10. 导出依赖信息"
    Write-Host "11. 导入依赖信息"
    Write-Host "12. 测试依赖构建"
    Write-Host "13. 退出"
    Write-Host "`n请选择操作: " -ForegroundColor Cyan -NoNewline
}

function Main {
    $running = $true
    while ($running) {
        Show-Menu
        $choice = Read-Host
        
        switch ($choice) {
            "1" {
                $result = Get-DependencyStatus
                if ($result) {
                    Write-Host "`n依赖状态:"
                    Write-Host $result
                }
            }
            "2" {
                $result = Update-Dependencies
                if ($result) {
                    Write-Host "`n更新结果:"
                    Write-Host $result
                }
            }
            "3" {
                $dependency = Read-Host "请输入要更新的依赖名称: "
                $result = Update-SpecificDependency -Dependency $dependency
                if ($result) {
                    Write-Host "`n更新结果:"
                    Write-Host $result
                }
            }
            "4" {
                $result = Resolve-DependencyConflicts
                if ($result) {
                    Write-Host "`n冲突解决结果:"
                    Write-Host $result
                }
            }
            "5" {
                $result = Analyze-DependencyTree
                if ($result) {
                    Write-Host "`n依赖树分析:"
                    Write-Host $result
                }
            }
            "6" {
                $dependency = Read-Host "请输入要查询的依赖名称: "
                $result = Get-DependencyVersions -Dependency $dependency
                if ($result) {
                    Write-Host "`n依赖版本:"
                    Write-Host $result
                }
            }
            "7" {
                $outputDir = Read-Host "请输入报告输出目录 (默认: ./dependency_reports): "
                if ([string]::IsNullOrEmpty($outputDir)) {
                    $outputDir = "./dependency_reports"
                }
                $result = Generate-DependencyReport -OutputDir $outputDir
                if ($result) {
                    Write-Host "`n报告生成结果:"
                    Write-Host $result
                }
            }
            "8" {
                $result = Clean-DependencyCache
                if ($result) {
                    Write-Host "`n清理结果:"
                    Write-Host $result
                }
            }
            "9" {
                $result = Check-DependencyUpdates
                if ($result) {
                    Write-Host "`n更新检查结果:"
                    Write-Host $result
                }
            }
            "10" {
                $outputFile = Read-Host "请输入导出文件路径 (默认: ./dependencies.json): "
                if ([string]::IsNullOrEmpty($outputFile)) {
                    $outputFile = "./dependencies.json"
                }
                $result = Export-Dependencies -OutputFile $outputFile
                if ($result) {
                    Write-Host "`n导出结果:"
                    Write-Host $result
                }
            }
            "11" {
                $inputFile = Read-Host "请输入导入文件路径 (默认: ./dependencies.json): "
                if ([string]::IsNullOrEmpty($inputFile)) {
                    $inputFile = "./dependencies.json"
                }
                $result = Import-Dependencies -InputFile $inputFile
                if ($result) {
                    Write-Host "`n导入结果:"
                    Write-Host $result
                }
            }
            "12" {
                $result = Test-DependencyBuild
                if ($result) {
                    Write-Host "`n构建测试结果:"
                    Write-Host $result
                }
            }
            "13" {
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
