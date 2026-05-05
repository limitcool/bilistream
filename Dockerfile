# Frontend build stage
FROM node:22-alpine AS web-builder

WORKDIR /usr/src/bilistream/web
COPY web/package.json web/pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile
COPY web ./
RUN pnpm run build

# Rust build stage
FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    pkgconfig

WORKDIR /usr/src/bilistream

COPY . .
COPY --from=web-builder /usr/src/bilistream/web/out ./web/out

RUN cargo build --release && \
    strip target/release/bilistream

# Python dependency stage
FROM alpine:latest AS python-deps

RUN apk add --no-cache python3 py3-pip && \
    python3 -m venv /opt/venv && \
    /opt/venv/bin/pip install --no-cache-dir yt-dlp

# Final runtime stage
FROM ghcr.io/jrottenberg/ffmpeg:7.1-scratch

ENTRYPOINT []

COPY --from=python-deps /usr/bin/python3 /usr/bin/
COPY --from=python-deps /usr/lib/python3.* /usr/lib/python3.*/
COPY --from=python-deps /opt/venv /opt/venv

WORKDIR /app

COPY --from=builder /usr/src/bilistream/target/release/bilistream /app/
COPY --from=builder /usr/src/bilistream/web/out /app/web/out

RUN chmod +x /app/bilistream

ENV PATH="/opt/venv/bin:$PATH"
ENV PYTHONPATH="/opt/venv/lib/python3.12/site-packages"

CMD ["/app/bilistream"]
