#!/usr/bin/env powershell

# 增强版自动化部署脚本
# 用于自动化部署YMAxum框架，包括环境检查、依赖安装、配置部署、服务启动和验证

param(
    [string]$Environment = "production",
    [string]$Version = "latest",
    [string]$InstallPath = "C:\ymaxum",
    [switch]$Force,
    [switch]$SkipValidation
)

# 全局变量
$ScriptName = "enhanced_automated_deployment.ps1"
$ScriptVersion = "1.0.0"
$LogFile = "$PSScriptRoot\..\logs\deployment_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
$ErrorActionPreference = "Stop"

# 确保日志目录存在
$LogDir = Split-Path -Parent $LogFile
if (-not (Test-Path $LogDir)) {
    New-Item -ItemType Directory -Path $LogDir -Force | Out-Null
}

# 日志函数
function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "INFO"
    )
    $Timestamp = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    $LogEntry = "[$Timestamp] [$Level] $Message"
    Write-Host $LogEntry
    Add-Content -Path $LogFile -Value $LogEntry
}

# 错误处理函数
function Handle-Error {
    param(
        [string]$Message,
        [Exception]$Exception
    )
    Write-Log "Error: $Message" "ERROR"
    if ($Exception) {
        Write-Log "Exception: $($Exception.Message)" "ERROR"
        Write-Log "Stack Trace: $($Exception.StackTrace)" "ERROR"
    }
    Write-Log "Deployment failed. Please check the logs for details." "ERROR"
    exit 1
}

# 检查系统环境
function Test-SystemRequirements {
    Write-Log "Checking system requirements..."
    
    # 检查Windows版本
    $OSVersion = [System.Environment]::OSVersion.Version
    if ($OSVersion.Major -lt 10) {
        Handle-Error "Windows 10 or later is required. Current version: $($OSVersion.Major).$($OSVersion.Minor)"
    }
    Write-Log "Windows version check passed: $($OSVersion.Major).$($OSVersion.Minor)"
    
    # 检查.NET Framework版本
    try {
        $NetVersion = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full" -Name Version -ErrorAction Stop
        $NetVersionValue = $NetVersion.Version
        if ([Version]$NetVersionValue -lt [Version]"4.7.2") {
            Handle-Error ".NET Framework 4.7.2 or later is required. Current version: $NetVersionValue"
        }
        Write-Log ".NET Framework version check passed: $NetVersionValue"
    } catch {
        Handle-Error ".NET Framework 4.7.2 or later is required. Cannot determine current version."
    }
    
    # 检查PowerShell版本
    $PSVersion = $PSVersionTable.PSVersion
    if ($PSVersion.Major -lt 5) {
        Handle-Error "PowerShell 5.0 or later is required. Current version: $($PSVersion.Major).$($PSVersion.Minor)"
    }
    Write-Log "PowerShell version check passed: $($PSVersion.Major).$($PSVersion.Minor)"
    
    # 检查磁盘空间
    $Drive = Split-Path -Qualifier $InstallPath
    $DriveInfo = Get-WmiObject -Class Win32_LogicalDisk -Filter "DeviceID='$Drive'"
    $FreeSpaceGB = [math]::Round($DriveInfo.FreeSpace / 1GB, 2)
    if ($FreeSpaceGB -lt 10) {
        Handle-Error "At least 10GB of free disk space is required. Current free space: $FreeSpaceGB GB"
    }
    Write-Log "Disk space check passed: $FreeSpaceGB GB free"
    
    Write-Log "System requirements check completed successfully."
}

# 检查并安装依赖
function Install-Dependencies {
    Write-Log "Checking and installing dependencies..."
    
    # 检查是否安装了Rust
    try {
        $RustVersion = rustc --version
        Write-Log "Rust is already installed: $RustVersion"
    } catch {
        Write-Log "Rust is not installed. Please install Rust 1.93.0 or later."
        Write-Log "You can download Rust from https://www.rust-lang.org/tools/install"
        Handle-Error "Rust is required but not installed."
    }
    
    # 检查是否安装了Git
    try {
        $GitVersion = git --version
        Write-Log "Git is already installed: $GitVersion"
    } catch {
        Write-Log "Git is not installed. Please install Git."
        Write-Log "You can download Git from https://git-scm.com/downloads"
        Handle-Error "Git is required but not installed."
    }
    
    # 检查是否安装了数据库（可选）
    try {
        $MySQLVersion = mysql --version 2>$null
        if ($MySQLVersion) {
            Write-Log "MySQL is installed: $MySQLVersion"
        } else {
            Write-Log "MySQL is not installed. Using SQLite as fallback."
        }
    } catch {
        Write-Log "MySQL is not installed. Using SQLite as fallback."
    }
    
    # 检查是否安装了Redis（可选）
    try {
        $RedisVersion = redis-server --version 2>$null
        if ($RedisVersion) {
            Write-Log "Redis is installed: $RedisVersion"
        } else {
            Write-Log "Redis is not installed. Using in-memory cache as fallback."
        }
    } catch {
        Write-Log "Redis is not installed. Using in-memory cache as fallback."
    }
    
    Write-Log "Dependencies check completed."
}

# 部署应用程序
function Deploy-Application {
    Write-Log "Deploying application..."
    
    # 确保安装目录存在
    if (-not (Test-Path $InstallPath)) {
        Write-Log "Creating installation directory: $InstallPath"
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    } else {
        if ($Force) {
            Write-Log "Force mode enabled. Removing existing files in $InstallPath"
            Remove-Item -Path "$InstallPath\*" -Recurse -Force | Out-Null
        } else {
            Write-Log "Installation directory already exists. Using existing directory."
        }
    }
    
    # 复制发布文件
    $ReleaseDir = "$PSScriptRoot\..\target\release"
    if (Test-Path $ReleaseDir) {
        Write-Log "Copying release files from $ReleaseDir to $InstallPath"
        Copy-Item -Path "$ReleaseDir\ymaxum.exe" -Destination "$InstallPath\" -Force
        Copy-Item -Path "$ReleaseDir\*.pdb" -Destination "$InstallPath\" -Force -ErrorAction SilentlyContinue
    } else {
        Handle-Error "Release directory not found at $ReleaseDir. Please run 'cargo build --release' first."
    }
    
    # 复制配置文件
    $ConfigDir = "$PSScriptRoot\..\config"
    if (Test-Path $ConfigDir) {
        $TargetConfigDir = "$InstallPath\config"
        if (-not (Test-Path $TargetConfigDir)) {
            New-Item -ItemType Directory -Path $TargetConfigDir -Force | Out-Null
        }
        Write-Log "Copying configuration files from $ConfigDir to $TargetConfigDir"
        Copy-Item -Path "$ConfigDir\*" -Destination $TargetConfigDir -Recurse -Force
    } else {
        Handle-Error "Config directory not found at $ConfigDir."
    }
    
    # 复制插件
    $PluginsDir = "$PSScriptRoot\..\plugins"
    if (Test-Path $PluginsDir) {
        $TargetPluginsDir = "$InstallPath\plugins"
        if (-not (Test-Path $TargetPluginsDir)) {
            New-Item -ItemType Directory -Path $TargetPluginsDir -Force | Out-Null
        }
        Write-Log "Copying plugins from $PluginsDir to $TargetPluginsDir"
        Copy-Item -Path "$PluginsDir\*" -Destination $TargetPluginsDir -Recurse -Force
    }
    
    # 复制密钥
    $KeysDir = "$PSScriptRoot\..\keys"
    if (Test-Path $KeysDir) {
        $TargetKeysDir = "$InstallPath\keys"
        if (-not (Test-Path $TargetKeysDir)) {
            New-Item -ItemType Directory -Path $TargetKeysDir -Force | Out-Null
        }
        Write-Log "Copying keys from $KeysDir to $TargetKeysDir"
        Copy-Item -Path "$KeysDir\*" -Destination $TargetKeysDir -Recurse -Force
    }
    
    Write-Log "Application deployment completed."
}

# 配置服务
function Configure-Service {
    Write-Log "Configuring service..."
    
    # 创建服务配置
    $ServiceName = "YMAxum"
    $ServiceDisplayName = "YMAxum Framework"
    $ServiceDescription = "High-performance web framework with GUF integration"
    
    # 检查服务是否存在
    $ServiceExists = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if ($ServiceExists) {
        if ($Force) {
            Write-Log "Service $ServiceName already exists. Stopping and removing..."
            Stop-Service -Name $ServiceName -Force -ErrorAction SilentlyContinue
            Start-Sleep -Seconds 2
            sc.exe delete $ServiceName | Out-Null
            Start-Sleep -Seconds 2
        } else {
            Write-Log "Service $ServiceName already exists. Using existing service."
            return
        }
    }
    
    # 创建服务
    Write-Log "Creating Windows service for $ServiceName"
    try {
        # 使用NSSM创建服务（如果可用）
        $NSSM = "$PSScriptRoot\nssm.exe"
        if (Test-Path $NSSM) {
            & $NSSM install $ServiceName "$InstallPath\ymaxum.exe"
            & $NSSM set $ServiceName DisplayName $ServiceDisplayName
            & $NSSM set $ServiceName Description $ServiceDescription
            & $NSSM set $ServiceName Start SERVICE_AUTO_START
            & $NSSM set $ServiceName AppDirectory $InstallPath
            & $NSSM set $ServiceName AppEnvironmentExtra "RUST_LOG=info"
            Write-Log "Service created successfully using NSSM."
        } else {
            # 使用sc.exe创建服务
            $BinaryPath = "`"$InstallPath\ymaxum.exe`""
            sc.exe create $ServiceName binPath= $BinaryPath start= auto displayname= "$ServiceDisplayName" | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-Log "Service created successfully using sc.exe."
            } else {
                Write-Log "Warning: Failed to create service using sc.exe. You may need to run this script as administrator."
                Write-Log "Continuing deployment without service creation."
            }
        }
    } catch {
        Write-Log "Warning: Failed to create service: $($_.Exception.Message)"
        Write-Log "Continuing deployment without service creation."
    }
    
    # 配置防火墙
    try {
        Write-Log "Configuring firewall rules..."
        New-NetFirewallRule -DisplayName "YMAxum Framework" -Direction Inbound -Program "$InstallPath\ymaxum.exe" -Action Allow -Profile Any | Out-Null
        Write-Log "Firewall rule created successfully."
    } catch {
        Write-Log "Warning: Failed to configure firewall: $($_.Exception.Message)"
    }
    
    Write-Log "Service configuration completed."
}

# 启动服务
function Start-Service {
    param(
        [string]$ServiceName = "YMAxum"
    )
    
    Write-Log "Starting service $ServiceName..."
    
    # 检查服务是否存在
    $ServiceExists = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if ($ServiceExists) {
        try {
            # 启动服务
            Start-Service -Name $ServiceName
            Write-Log "Service $ServiceName started successfully."
            
            # 等待服务启动
            Write-Log "Waiting for service to fully start..."
            Start-Sleep -Seconds 5
            
            # 检查服务状态
            $ServiceStatus = Get-Service -Name $ServiceName
            if ($ServiceStatus.Status -eq "Running") {
                Write-Log "Service $ServiceName is now running."
            } else {
                Write-Log "Warning: Service $ServiceName status is $($ServiceStatus.Status)."
            }
        } catch {
            Write-Log "Warning: Failed to start service: $($_.Exception.Message)"
            Write-Log "Attempting to run application directly..."
            
            # 尝试直接运行应用程序
            try {
                Start-Process -FilePath "$InstallPath\ymaxum.exe" -WorkingDirectory $InstallPath -WindowStyle Hidden
                Write-Log "Application started directly."
            } catch {
                Handle-Error "Failed to start application: $($_.Exception.Message)"
            }
        }
    } else {
        # 直接运行应用程序
        try {
            Write-Log "Service not found. Running application directly..."
            Start-Process -FilePath "$InstallPath\ymaxum.exe" -WorkingDirectory $InstallPath -WindowStyle Hidden
            Write-Log "Application started directly."
        } catch {
            Handle-Error "Failed to start application: $($_.Exception.Message)"
        }
    }
}

# 验证部署
function Test-Deployment {
    if ($SkipValidation) {
        Write-Log "Skipping deployment validation."
        return
    }
    
    Write-Log "Validating deployment..."
    
    # 检查文件是否存在
    if (-not (Test-Path "$InstallPath\ymaxum.exe")) {
        Handle-Error "ymaxum.exe not found at $InstallPath\ymaxum.exe"
    }
    
    # 检查配置文件是否存在
    if (-not (Test-Path "$InstallPath\config")) {
        Handle-Error "Config directory not found at $InstallPath\config"
    }
    
    # 测试健康检查接口
    try {
        Write-Log "Testing health check endpoint..."
        $HealthCheckUrl = "http://localhost:3000/health"
        $Response = Invoke-WebRequest -Uri $HealthCheckUrl -TimeoutSec 30
        if ($Response.StatusCode -eq 200) {
            Write-Log "Health check passed: $($Response.StatusCode)"
            $ResponseContent = $Response.Content | ConvertFrom-Json
            Write-Log "Version: $($ResponseContent.version)"
            Write-Log "Status: $($ResponseContent.status)"
        } else {
            Write-Log "Warning: Health check returned status code: $($Response.StatusCode)"
        }
    } catch {
        Write-Log "Warning: Failed to test health check endpoint: $($_.Exception.Message)"
        Write-Log "Application may still be starting up. Please check the logs."
    }
    
    # 检查日志
    $LogFile = "$InstallPath\logs\application.log"
    if (Test-Path $LogFile) {
        $LastLogLines = Get-Content -Path $LogFile -Tail 10
        Write-Log "Last 10 lines of application log:"
        $LastLogLines | ForEach-Object { Write-Log $_ "DEBUG" }
    }
    
    Write-Log "Deployment validation completed."
}

# 主函数
function Main {
    Write-Log "Starting enhanced automated deployment..."
    Write-Log "Script: $ScriptName v$ScriptVersion"
    Write-Log "Environment: $Environment"
    Write-Log "Version: $Version"
    Write-Log "Install Path: $InstallPath"
    Write-Log "Force: $Force"
    Write-Log "Skip Validation: $SkipValidation"
    
    try {
        # 1. 检查系统环境
        Test-SystemRequirements
        
        # 2. 安装依赖
        Install-Dependencies
        
        # 3. 部署应用程序
        Deploy-Application
        
        # 4. 配置服务
        Configure-Service
        
        # 5. 启动服务
        Start-Service
        
        # 6. 验证部署
        Test-Deployment
        
        Write-Log "Enhanced automated deployment completed successfully!"
        Write-Log "Application is now running at http://localhost:3000"
        Write-Log "You can access the health check endpoint at http://localhost:3000/health"
        Write-Log "For more information, please check the logs at $LogFile"
        
    } catch {
        Handle-Error "Deployment failed: $($_.Exception.Message)" $_
    }
}

# 运行主函数
Main
