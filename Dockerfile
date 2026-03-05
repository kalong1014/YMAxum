# Dockerfile for YMAxum
# 优化版：更小的镜像大小和更快的构建速度

# 阶段1：构建器
FROM rust:1.93.0-alpine as builder

WORKDIR /app

# 安装构建依赖
RUN apk add --no-cache \
    pkgconfig \
    openssl-dev \
    build-base

# 复制Cargo文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟的src目录以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖（缓存层）
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src
COPY config ./config
COPY templates ./templates

# 构建应用
RUN touch src/main.rs && cargo build --release

# 阶段2：运行时
FROM alpine:3.20

WORKDIR /app

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    openssl3 \
    curl

# 创建非root用户
RUN addgroup -g 1000 ymaxum && \
    adduser -u 1000 -G ymaxum -s /bin/sh -D ymaxum

# 从构建器复制二进制文件
COPY --from=builder /app/target/release/ymaxum /app/ymaxum

# 复制配置和模板
COPY --from=builder /app/config /app/config
COPY --from=builder /app/templates /app/templates

# 创建数据目录并设置权限
RUN mkdir -p /app/data /app/logs /app/plugins && \
    chown -R ymaxum:ymaxum /app

# 切换到非root用户
USER ymaxum

# 暴露端口
EXPOSE 3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# 设置环境变量
ENV RUST_LOG=info
ENV APP_ENV=production

# 运行应用
CMD ["./ymaxum"]
