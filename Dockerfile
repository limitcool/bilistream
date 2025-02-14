# 构建阶段
FROM rust:alpine AS builder

# 安装构建依赖
RUN apk add --no-cache \
    musl-dev \
    pkgconfig

# 设置工作目录
WORKDIR /usr/src/bilistream

# 复制项目文件
COPY . .

# 构建项目
RUN cargo build --release && \
    strip target/release/bilistream

# Python 依赖安装阶段
FROM alpine:latest AS python-deps

# 安装 Python 和创建虚拟环境
RUN apk add --no-cache python3 py3-pip && \
    python3 -m venv /opt/venv && \
    /opt/venv/bin/pip install --no-cache-dir yt-dlp

# 最终运行阶段
FROM ghcr.io/jrottenberg/ffmpeg:7.1-scratch

# 清除默认的 ENTRYPOINT
ENTRYPOINT []

# 从 Alpine 复制 Python 运行时和虚拟环境
COPY --from=python-deps /usr/bin/python3 /usr/bin/
COPY --from=python-deps /usr/lib/python3.* /usr/lib/python3.*/
COPY --from=python-deps /opt/venv /opt/venv

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/bilistream/target/release/bilistream /app/

# 设置权限
RUN chmod +x /app/bilistream

# 设置环境变量
ENV PATH="/opt/venv/bin:$PATH"
ENV PYTHONPATH="/opt/venv/lib/python3.12/site-packages"

# 设置容器启动命令
CMD ["/app/bilistream"]
