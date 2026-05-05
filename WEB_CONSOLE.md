# Web Console

`bilistream` 从 `v0.2.0` 开始内置 Web 控制台，适合部署在服务器后远程访问。

## 访问方式

默认监听地址：

```yaml
WebConsole:
  Addr: 0.0.0.0:9090
```

浏览器访问：

```text
http://<your-server-ip>:9090
```

## 鉴权

控制台使用 `config.yaml` 里的 Basic Auth 配置：

```yaml
WebConsole:
  Addr: 0.0.0.0:9090
  Username: admin
  Password: change-me
```

说明：

- `Password` 为空时，不启用鉴权
- 不建议把无密码控制台直接暴露到公网
- 如果首次启动时没有 `config.yaml`，程序会自动初始化配置
- 如果检测到公网监听且密码为空，程序会自动生成随机密码并写回 `config.yaml`

## 控制台能力

- 查看监控状态、源站状态、B 站状态、FFmpeg 状态
- 查看通知策略与通道开关状态
- 在线编辑 `config.yaml`
- 远程启动监控
- 远程停止监控
- 远程重载配置
- 查看最近运行日志
- 查看版本号与 GitHub 仓库链接

## 通知配置

```yaml
Notification:
  Enabled: true
  Channel: both
  GotifyEnabled: true
  NtfyEnabled: true

Gotify:
  Url: https://gotify.example.com
  Token: your-token

Ntfy:
  Url: https://ntfy.sh
  Topic: bilistream-live
  Token: ""
  Username: ""
  Password: ""
  Priority: default
  Tags: bilistream,live
```

说明：

- `Notification.Channel` 支持 `gotify`、`ntfy`、`both`
- `GotifyEnabled` 和 `NtfyEnabled` 可以分别关闭单个通道
- `Enabled` 是总开关

## Docker 示例

```yaml
services:
  bilistream:
    image: ghcr.io/limitcool/bilistream:v0.2.0
    restart: unless-stopped
    ports:
      - "9090:9090"
    volumes:
      - ./config.yaml:/app/config.yaml
      - ./cookies.txt:/app/cookies.txt:ro
```

注意：

- `config.yaml` 需要可写，因为控制台支持在线保存配置
- 如果你只想本机访问，可以把 `WebConsole.Addr` 改成 `127.0.0.1:9090`

## 前端构建说明

控制台前端已经编译为静态文件，由 Rust 服务直接托管：

- 开发目录：`web/`
- 构建产物：`web/out`
- 后端托管入口：`src/dashboard.rs`

当前 Dockerfile 已经包含前端构建步骤，不需要额外手工处理。
