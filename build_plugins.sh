#!/bin/bash

echo "========================================"
echo "插件打包脚本"
echo "========================================"
echo ""

# 设置路径
PLUGIN_DIR="plugins"
DIST_DIR="dist"
PACKER_SCRIPT="./target/release/plugin_packer"

# 创建输出目录
mkdir -p "$DIST_DIR"

echo "正在打包客服插件..."
"$PACKER_SCRIPT" "scripts/customer_service_pack.toml"

if [ $? -ne 0 ]; then
    echo "错误：客服插件打包失败"
    exit 1
fi

echo "客服插件打包完成：customer_service.axpl"
echo ""

echo "正在打包IM插件..."
"$PACKER_SCRIPT" "scripts/im_pack.toml"

if [ $? -ne 0 ]; then
    echo "错误：IM插件打包失败"
    exit 1
fi

echo "IM插件打包完成：im.axpl"
echo ""

echo "========================================"
echo "所有插件打包完成！"
echo "========================================"
echo ""
echo "打包的插件："
echo "- $DIST_DIR/customer_service.axpl"
echo "- $DIST_DIR/im.axpl"
echo ""
