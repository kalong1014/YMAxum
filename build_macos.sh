#!/bin/bash

# YMAxum框架macOS版本打包脚本
# 使用cross工具进行交叉编译

set -e

echo "========================================"
echo "YMAxum框架macOS版本打包"
echo "========================================"
echo ""

# 检查cross工具是否安装
if ! command -v cross &> /dev/null; then
    echo "错误：未找到cross工具"
    echo "请先安装cross：cargo install cross"
    exit 1
fi

echo "✓ cross工具已安装"
echo ""

# 检查Docker是否运行
if ! docker info &> /dev/null; then
    echo "错误：Docker未运行"
    echo "请先启动Docker"
    exit 1
fi

echo "✓ Docker正在运行"
echo ""

# 创建输出目录
mkdir -p dist/macos
echo "✓ 创建输出目录：dist/macos"
echo ""

# 编译macOS x64版本
echo "开始编译macOS x64版本..."
cross build --release --target x86_64-apple-darwin

if [ $? -ne 0 ]; then
    echo "错误：macOS x64版本编译失败"
    exit 1
fi

echo "✓ macOS x64版本编译成功"
echo ""

# 复制可执行文件
cp target/x86_64-apple-darwin/release/ymaxum dist/macos/ymaxum-x86_64

if [ $? -ne 0 ]; then
    echo "错误：复制可执行文件失败"
    exit 1
fi

echo "✓ 可执行文件已复制：dist/macos/ymaxum-x86_64"
echo ""

# 检查文件大小
file_size=$(du -h dist/macos/ymaxum-x86_64 | cut -f1)

echo "✓ macOS x64版本文件大小：$file_size"
echo ""

# 检查文件大小是否超过10MB
file_size_bytes=$(stat -f%s dist/macos/ymaxum-x86_64)
max_size=$((10 * 1024 * 1024))

if [ $file_size_bytes -gt $max_size ]; then
    echo "警告：macOS x64版本文件大小超过10MB"
fi

# 设置可执行权限
chmod +x dist/macos/ymaxum-x86_64

echo "✓ 可执行权限已设置"
echo ""

# 创建压缩包
echo "开始创建压缩包..."
cd dist/macos
tar -czf ymaxum-macos-x86_64.tar.gz ymaxum-x86_64
cd ../..

if [ $? -ne 0 ]; then
    echo "错误：创建压缩包失败"
    exit 1
fi

echo "✓ 压缩包已创建"
echo ""

# 创建DMG安装包（macOS标准格式）
echo "开始创建DMG安装包..."

if ! command -v hdiutil &> /dev/null; then
    echo "警告：未找到hdiutil工具，跳过DMG创建"
else
    # 创建临时目录
    mkdir -p dist/macos/dmg_temp
    
    # 复制可执行文件
    cp dist/macos/ymaxum-x86_64 dist/macos/dmg_temp/ymaxum
    
    # 创建DMG
    hdiutil create -volname "YMAxum" -srcfolder dist/macos/dmg_temp -ov -format UDZO dist/macos/ymaxum.dmg
    
    # 清理临时目录
    rm -rf dist/macos/dmg_temp
    
    echo "✓ DMG安装包已创建"
fi

echo ""

# 显示最终结果
echo "========================================"
echo "打包完成！"
echo "========================================"
echo ""
echo "输出文件："
echo "  - dist/macos/ymaxum-x86_64"
echo "  - dist/macos/ymaxum-macos-x86_64.tar.gz"
if [ -f dist/macos/ymaxum.dmg ]; then
    echo "  - dist/macos/ymaxum.dmg"
fi
echo ""
echo "文件大小："
echo "  - macOS x64: $file_size"
echo ""
echo "使用方法："
echo "  # 解压并运行"
echo "  tar -xzf ymaxum-macos-x86_64.tar.gz"
echo "  ./ymaxum-x86_64"
echo ""
echo "  # 或使用DMG安装包"
echo "  open ymaxum.dmg"
echo ""
