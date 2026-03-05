@echo off
REM YMAxum框架Windows版本优化打包脚本

setlocal enabledelayedexpansion

echo ========================================
echo YMAxum框架Windows版本优化打包
echo ========================================
echo.

REM 检查Rust是否安装
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo 错误：未找到Rust
    echo 请先安装Rust：https://rustup.rs/
    exit /b 1
)

echo ✓ Rust已安装
echo.

REM 检查是否安装了sccache（可选的构建缓存工具）
where sccache >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo ✓ sccache已安装，将使用缓存加速构建
    set RUSTC_WRAPPER=sccache
) else (
    echo ℹ sccache未安装，使用默认构建方式
)
echo.

REM 创建输出目录
if not exist dist\windows mkdir dist\windows
echo ✓ 创建输出目录：dist\windows
echo.

REM 清理旧的构建文件（可选，用于解决构建问题）
echo 清理旧的构建文件...
cargo clean --target x86_64-pc-windows-msvc
echo ✓ 清理完成
echo.

REM 编译Windows x64版本（使用并行构建）
echo 开始编译Windows x64版本...
echo 启用并行构建，使用所有可用CPU核心...

REM 设置构建参数
set build_flags=--release --target x86_64-pc-windows-msvc
set build_flags=%build_flags% --jobs %NUMBER_OF_PROCESSORS%

REM 执行构建
cargo build %build_flags%

if %ERRORLEVEL% NEQ 0 (
    echo 错误：Windows x64版本编译失败
    exit /b 1
)

echo ✓ Windows x64版本编译成功
echo.

REM 复制可执行文件
copy target\x86_64-pc-windows-msvc\release\ymaxum.exe dist\windows\ymaxum-x86_64.exe

if %ERRORLEVEL% NEQ 0 (
    echo 错误：复制可执行文件失败
    exit /b 1
)

echo ✓ 可执行文件已复制：dist\windows\ymaxum-x86_64.exe
echo.

REM 检查文件大小
for %%A in (dist\windows\ymaxum-x86_64.exe) do set size=%%~zA

echo ✓ Windows x64版本文件大小：%size% 字节

REM 检查文件大小是否超过10MB
set max_size=10485760
if %size% GTR %max_size% (
    echo 警告：Windows x64版本文件大小超过10MB
)

REM 设置可执行权限
echo ✓ 可执行文件已准备
echo.

REM 创建压缩包（并行处理）
echo 开始创建压缩包...
cd dist\windows

REM 使用PowerShell进行并行压缩，提高速度
powershell -Command "
    $files = @('ymaxum-x86_64.exe')
    $tasks = @()
    
    # 创建tar.gz压缩包
    $tasks += Start-Job -ScriptBlock {
        param($file)
        tar -czf "$($file).tar.gz" $file
    } -ArgumentList "ymaxum-x86_64.exe"
    
    # 创建zip压缩包
    $tasks += Start-Job -ScriptBlock {
        param($file)
        tar -czf "$($file).zip" $file
    } -ArgumentList "ymaxum-x86_64.exe"
    
    # 等待所有任务完成
    Wait-Job -Job $tasks
    
    # 获取任务结果
    foreach ($task in $tasks) {
        Receive-Job -Job $task
    }
"

cd ..\..

if %ERRORLEVEL% NEQ 0 (
    echo 错误：创建压缩包失败
    exit /b 1
)

echo ✓ 压缩包已创建
echo.

REM 显示最终结果
echo ========================================
echo 打包完成！
echo ========================================
echo.
echo 输出文件：
echo   - dist\windows\ymaxum-x86_64.exe
echo   - dist\windows\ymaxum-x86_64.exe.tar.gz
echo   - dist\windows\ymaxum-x86_64.exe.zip
echo.
echo 文件大小：%size% 字节
echo.
echo 优化措施：
echo   1. 启用并行构建，使用所有可用CPU核心
echo   2. 支持sccache构建缓存（如果安装）
echo   3. 清理旧的构建文件，避免构建冲突
echo   4. 并行创建压缩包，提高打包速度
echo.
echo 使用方法：
echo   # 直接运行
echo   dist\windows\ymaxum-x86_64.exe
echo.
echo   # 或解压运行
echo   tar -xzf ymaxum-x86_64.exe.tar.gz
echo   ymaxum-x86_64.exe
echo.
pause
