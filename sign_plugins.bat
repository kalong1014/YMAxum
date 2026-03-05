@echo off
echo ========================================
echo 插件签名脚本
echo ========================================
echo.

REM 设置路径
set KEYS_DIR=keys
set DIST_DIR=dist
set SIGNER_SCRIPT=target\release\plugin_signer.exe

REM 创建密钥目录
if not exist "%KEYS_DIR%" mkdir "%KEYS_DIR%"

echo 正在生成密钥对...
"%SIGNER_SCRIPT%" generate "%KEYS_DIR%\private.pem" "%KEYS_DIR%\public.pem"

if %ERRORLEVEL% NEQ 0 (
    echo 错误：密钥对生成失败
    exit /b 1
)

echo 密钥对生成完成
echo.

echo 正在签名客服插件...
"%SIGNER_SCRIPT%" sign "scripts\customer_service_sign.toml"

if %ERRORLEVEL% NEQ 0 (
    echo 错误：客服插件签名失败
    exit /b 1
)

echo 客服插件签名完成
echo.

echo 正在签名IM插件...
"%SIGNER_SCRIPT%" sign "scripts\im_sign.toml"

if %ERRORLEVEL% NEQ 0 (
    echo 错误：IM插件签名失败
    exit /b 1
)

echo IM插件签名完成
echo.

echo ========================================
echo 所有插件签名完成！
echo ========================================
echo.
echo 签名的插件：
echo - %DIST_DIR%\customer_service.axpl (签名: %DIST_DIR%\customer_service_signature.json)
echo - %DIST_DIR%\im.axpl (签名: %DIST_DIR%\im_signature.json)
echo.
pause
