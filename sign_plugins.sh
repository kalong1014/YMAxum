#!/bin/bash

echo "========================================"
echo "插件签名脚本"
echo "========================================"
echo ""

# 设置路径
KEYS_DIR="keys"
DIST_DIR="dist"
SIGNER_SCRIPT="./target/release/plugin_signer"

# 创建密钥目录
mkdir -p "$KEYS_DIR"

echo "正在生成密钥对..."
"$SIGNER_SCRIPT" generate "$KEYS_DIR/private.pem" "$KEYS_DIR/public.pem"

if [ $? -ne 0 ]; then
    echo "错误：密钥对生成失败"
    exit 1
fi

echo "密钥对生成完成"
echo ""

echo "正在签名客服插件..."
"$SIGNER_SCRIPT" sign "scripts/customer_service_sign.toml"

if [ $? -ne 0 ]; then
    echo "错误：客服插件签名失败"
    exit 1
fi

echo "客服插件签名完成"
echo ""

echo "正在签名IM插件..."
"$SIGNER_SCRIPT" sign "scripts/im_sign.toml"

if [ $? -ne 0 ]; then
    echo "错误：IM插件签名失败"
    exit 1
fi

echo "IM插件签名完成"
echo ""

echo "========================================"
echo "所有插件签名完成！"
echo "========================================"
echo ""
echo "签名的插件："
echo "- $DIST_DIR/customer_service.axpl (签名: $DIST_DIR/customer_service_signature.json)"
echo "- $DIST_DIR/im.axpl (签名: $DIST_DIR/im_signature.json)"
echo ""
