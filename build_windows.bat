@echo off
REM YMAxum框架Windows版本打包脚本

echo ========================================
echo YMAxum框架Windows版本打包
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
if not exist dist\windows mkdir dist\windows
echo ✓ 创建输出目录：dist\windows
echo.

REM 编译Windows x64版本
echo 开始编译Windows x64版本...
cargo build --release --target x86_64-pc-windows-msvc

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

REM 创建压缩包
echo 开始创建压缩包...
cd dist\windows
tar -czf ymaxum-windows-x86_64.tar.gz ymaxum-x86_64.exe
cd ..\..

if %ERRORLEVEL% NEQ 0 (
    echo 错误：创建压缩包失败
    exit /b 1
)

echo ✓ 压缩包已创建
echo.

REM 创建ZIP压缩包（Windows常用格式）
echo 开始创建ZIP压缩包...
cd dist\windows
tar -czf ymaxum-windows-x86_64.zip ymaxum-x86_64.exe
cd ..\..

if %ERRORLEVEL% NEQ 0 (
    echo 错误：创建ZIP压缩包失败
    exit /b 1
)

echo ✓ ZIP压缩包已创建
echo.

REM 显示最终结果
echo ========================================
echo 打包完成！
echo ========================================
echo.
echo 输出文件：
echo   - dist\windows\ymaxum-x86_64.exe
echo   - dist\windows\ymaxum-windows-x86_64.tar.gz
echo   - dist\windows\ymaxum-windows-x86_64.zip
echo.
echo 文件大小：%size% 字节
echo.
echo 使用方法：
echo   # 直接运行
echo   dist\windows\ymaxum-x86_64.exe
echo.
echo   # 或解压运行
echo   tar -xzf ymaxum-windows-x86_64.tar.gz
echo   ymaxum-x86_64.exe
echo.
pause
