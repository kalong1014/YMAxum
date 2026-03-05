#!/bin/bash

# YMAxum框架Linux版本打包脚本
# 使用cross工具进行交叉编译

set -e

echo "========================================"
echo "YMAxum框架Linux版本打包"
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
mkdir -p dist/linux
echo "✓ 创建输出目录：dist/linux"
echo ""

# 编译Linux x64版本
echo "开始编译Linux x64版本..."
cross build --release --target x86_64-unknown-linux-gnu

if [ $? -ne 0 ]; then
    echo "错误：Linux x64版本编译失败"
    exit 1
fi

echo "✓ Linux x64版本编译成功"
echo ""

# 复制可执行文件
cp target/x86_64-unknown-linux-gnu/release/ymaxum dist/linux/ymaxum-x86_64

if [ $? -ne 0 ]; then
    echo "错误：复制可执行文件失败"
    exit 1
fi

echo "✓ 可执行文件已复制：dist/linux/ymaxum-x86_64"
echo ""

# 编译Linux ARM64版本
echo "开始编译Linux ARM64版本..."
cross build --release --target aarch64-unknown-linux-gnu

if [ $? -ne 0 ]; then
    echo "错误：Linux ARM64版本编译失败"
    exit 1
fi

echo "✓ Linux ARM64版本编译成功"
echo ""

# 复制可执行文件
cp target/aarch64-unknown-linux-gnu/release/ymaxum dist/linux/ymaxum-aarch64

if [ $? -ne 0 ]; then
    echo "错误：复制可执行文件失败"
    exit 1
fi

echo "✓ 可执行文件已复制：dist/linux/ymaxum-aarch64"
echo ""

# 检查文件大小
x64_size=$(du -h dist/linux/ymaxum-x86_64 | cut -f1)
aarch64_size=$(du -h dist/linux/ymaxum-aarch64 | cut -f1)

echo "✓ Linux x64版本文件大小：$x64_size"
echo "✓ Linux ARM64版本文件大小：$aarch64_size"
echo ""

# 检查文件大小是否超过10MB
x64_size_bytes=$(stat -f%s dist/linux/ymaxum-x86_64)
aarch64_size_bytes=$(stat -f%s dist/linux/ymaxum-aarch64)
max_size=$((10 * 1024 * 1024))

if [ $x64_size_bytes -gt $max_size ]; then
    echo "警告：Linux x64版本文件大小超过10MB"
fi

if [ $aarch64_size_bytes -gt $max_size ]; then
    echo "警告：Linux ARM64版本文件大小超过10MB"
fi

# 设置可执行权限
chmod +x dist/linux/ymaxum-x86_64
chmod +x dist/linux/ymaxum-aarch64

echo "✓ 可执行权限已设置"
echo ""

# 创建压缩包
echo "开始创建压缩包..."
cd dist/linux
tar -czf ymaxum-linux-x86_64.tar.gz ymaxum-x86_64
tar -czf ymaxum-linux-aarch64.tar.gz ymaxum-aarch64
cd ../..

if [ $? -ne 0 ]; then
    echo "错误：创建压缩包失败"
    exit 1
fi

echo "✓ 压缩包已创建"
echo ""

# 显示最终结果
echo "========================================"
echo "打包完成！"
echo "========================================"
echo ""
echo "输出文件："
echo "  - dist/linux/ymaxum-x86_64"
echo "  - dist/linux/ymaxum-aarch64"
echo "  - dist/linux/ymaxum-linux-x86_64.tar.gz"
echo "  - dist/linux/ymaxum-linux-aarch64.tar.gz"
echo ""
echo "文件大小："
echo "  - Linux x64: $x64_size"
echo "  - Linux ARM64: $aarch64_size"
echo ""
echo "使用方法："
echo "  # 解压并运行（x64）"
echo "  tar -xzf ymaxum-linux-x86_64.tar.gz"
echo "  ./ymaxum-x86_64"
echo ""
echo "  # 解压并运行（ARM64）"
echo "  tar -xzf ymaxum-linux-aarch64.tar.gz"
echo "  ./ymaxum-aarch64"
echo ""
