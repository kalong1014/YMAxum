@echo off
REM YMAxum框架简化构建脚本

echo ========================================
echo YMAxum框架简化构建
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

REM 创建输出目录
if not exist dist mkdir dist
if not exist dist\windows mkdir dist\windows
if not exist dist\linux mkdir dist\linux
if not exist dist\macos mkdir dist\macos

echo ✓ 输出目录已创建
echo.

REM 构建Windows x64版本
echo 开始构建Windows x64版本...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo 错误：Windows x64版本编译失败
    exit /b 1
)

echo ✓ Windows x64版本编译成功
echo.

REM 复制Windows可执行文件
copy target\release\ymaxum.exe dist\windows\ymaxum-x86_64.exe

if %ERRORLEVEL% NEQ 0 (
    echo 错误：复制Windows可执行文件失败
    exit /b 1
)

echo ✓ Windows可执行文件已复制
echo.

REM 构建Linux x64版本（如果安装了cross）
echo 检查cross工具是否安装...
where cross >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo ✓ cross工具已安装
    echo 开始构建Linux x64版本...
    cross build --release --target x86_64-unknown-linux-gnu
    
    if %ERRORLEVEL% EQU 0 (
        echo ✓ Linux x64版本编译成功
        copy target\x86_64-unknown-linux-gnu\release\ymaxum dist\linux\ymaxum-x86_64
        if %ERRORLEVEL% EQU 0 (
            echo ✓ Linux可执行文件已复制
        ) else (
            echo 警告：复制Linux可执行文件失败
        )
    ) else (
        echo 警告：Linux x64版本编译失败
    )
) else (
    echo 警告：未找到cross工具，跳过Linux版本构建
    echo 请安装cross：cargo install cross
)

echo.

REM 构建macOS x64版本（如果安装了cross）
where cross >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo 开始构建macOS x64版本...
    cross build --release --target x86_64-apple-darwin
    
    if %ERRORLEVEL% EQU 0 (
        echo ✓ macOS x64版本编译成功
        copy target\x86_64-apple-darwin\release\ymaxum dist\macos\ymaxum-x86_64
        if %ERRORLEVEL% EQU 0 (
            echo ✓ macOS可执行文件已复制
        ) else (
            echo 警告：复制macOS可执行文件失败
        )
    ) else (
        echo 警告：macOS x64版本编译失败
    )
) else (
    echo 警告：未找到cross工具，跳过macOS版本构建
)

echo.

REM 显示构建结果
echo ========================================
echo 构建完成！
echo ========================================
echo.
echo 输出文件：
echo   - Windows: dist\windows\ymaxum-x86_64.exe
if exist dist\linux\ymaxum-x86_64 (
    echo   - Linux: dist\linux\ymaxum-x86_64
)
if exist dist\macos\ymaxum-x86_64 (
    echo   - macOS: dist\macos\ymaxum-x86_64
)
echo.
echo 使用方法：
echo   # Windows
   dist\windows\ymaxum-x86_64.exe
echo.
if exist dist\linux\ymaxum-x86_64 (
    echo   # Linux
    echo   ./dist/linux/ymaxum-x86_64
    echo.
)
if exist dist\macos\ymaxum-x86_64 (
    echo   # macOS
    echo   ./dist/macos/ymaxum-x86_64
    echo.
)

pause