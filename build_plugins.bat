@echo off
echo ========================================
echo 插件打包脚本
echo ========================================
echo.

REM 设置路径
set PLUGIN_DIR=plugins
set DIST_DIR=dist
set PACKER_SCRIPT=target\release\plugin_packer.exe

REM 创建输出目录
if not exist "%DIST_DIR%" mkdir "%DIST_DIR%"

echo 正在打包客服插件...
"%PACKER_SCRIPT%" "scripts\customer_service_pack.toml"

if %ERRORLEVEL% NEQ 0 (
    echo 错误：客服插件打包失败
    exit /b 1
)

echo 客服插件打包完成：customer_service.axpl
echo.

echo 正在打包IM插件...
"%PACKER_SCRIPT%" "scripts\im_pack.toml"

if %ERRORLEVEL% NEQ 0 (
    echo 错误：IM插件打包失败
    exit /b 1
)

echo IM插件打包完成：im.axpl
echo.

echo ========================================
echo 所有插件打包完成！
echo ========================================
echo.
echo 打包的插件：
echo - %DIST_DIR%\customer_service.axpl
echo - %DIST_DIR%\im.axpl
echo.
pause
