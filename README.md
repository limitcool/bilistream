# bilistream

`bilistream` 是一个面向 self-host 场景的 B 站自动转播工具，用于无人值守地监控 Twitch、YouTube、YouTube 预告直播，并在源站开播时自动推流到 B 站。

当前版本：`v0.2.0`

## 功能概览

- 自动监控 `Twitch`、`YouTube`、`YouTubePreviewLive`
- 源站开播后自动启动 FFmpeg 推流到 B 站
- 源站下播后自动结束 B 站直播
- 支持 `Gotify`、`ntfy`、`both` 三种通知策略
- 支持通知总开关、Gotify 单独开关、ntfy 单独开关
- 内置 Web 控制台，支持远程访问、配置编辑、状态查看、日志查看
- 所有配置统一写入 `config.yaml`，不依赖环境变量

## 运行依赖

二进制运行模式下需要：

- `ffmpeg`
- `python3`
- `yt-dlp`

Docker 镜像已内置这些依赖。

## Docker 部署

推荐直接使用 Docker：

```yaml
services:
  bilistream:
    image: ghcr.io/limitcool/bilistream:v0.2.0
    container_name: bilistream
    restart: unless-stopped
    ports:
      - "9090:9090"
    volumes:
      - ./config.yaml:/app/config.yaml
      - ./cookies.txt:/app/cookies.txt:ro
```

启动：

```bash
docker compose up -d
```

查看日志：

```bash
docker compose logs -f
```

说明：

- Web 控制台默认监听 `0.0.0.0:9090`
- 程序会直接读写 `/app/config.yaml`，因此 `config.yaml` 不要挂成只读
- `cookies.txt` 可选，仅在需要登录态拉流时挂载

## 二进制运行

Linux 示例：

```bash
./bilistream
```

如未安装依赖，可先安装：

```bash
apt update
apt install -y ffmpeg python3 python3-pip
pip3 install -U yt-dlp
```

## 配置方式

程序使用统一的 `config.yaml`。

如果启动时未检测到 `config.yaml`：

- 程序会自动初始化一份默认配置
- 如果 `WebConsole.Addr` 是公网监听地址且密码为空，会自动生成一个随机密码写回配置
- 启动日志会提示已自动生成密码，但不会把密码明文打到日志里

示例：

```yaml
Interval: 60
Platform: Twitch

BiliLive:
  SESSDATA: ""
  bili_jct: ""
  DedeUserID: ""
  DedeUserID__ckMd5: ""
  Room: 0
  BiliRtmpUrl: "rtmp://live-push.bilivideo.com/live-bvc/"
  BiliRtmpKey: ""

Twitch:
  Room: ""

Youtube:
  Room: ""
  AccessToken: ""

YoutubePreviewLive:
  ChannelId: ""

FfmpegProxy: ""
Cookies: ""

Notification:
  Enabled: true
  Channel: gotify
  GotifyEnabled: true
  NtfyEnabled: false

Gotify:
  Url: ""
  Token: ""

Ntfy:
  Url: "https://ntfy.sh"
  Topic: ""
  Token: ""
  Username: ""
  Password: ""
  Priority: ""
  Tags: ""

WebConsole:
  Addr: "0.0.0.0:9090"
  Username: "admin"
  Password: ""
```

## Web 控制台

Web 控制台支持：

- 查看源站状态、B 站状态、FFmpeg 状态、通知状态
- 远程启动监控、停止监控、重载配置
- 在线编辑 `config.yaml`
- 查看最近运行日志
- 展示当前版本号与 GitHub 仓库入口

更多说明见 [WEB_CONSOLE.md](./WEB_CONSOLE.md)。

## 通知策略

`Notification.Channel` 支持：

- `gotify`
- `ntfy`
- `both`

同时还支持：

- `Notification.Enabled`：通知总开关
- `Notification.GotifyEnabled`：Gotify 单独开关
- `Notification.NtfyEnabled`：ntfy 单独开关

也就是说，主策略和单通道开关可以同时生效。

## 发布说明

- GitHub Release 工作流仍然会继续构建 Rust 二进制包
- Dockerfile 已经升级为先构建前端，再把 `web/out` 一起打进镜像
- 如果只是发布 Docker 镜像或本地部署，当前 Dockerfile 已经匹配 `v0.2.0` 的前端方案
- 如果后续希望 GitHub Actions 额外做前端单独校验，可以再补 CI，但不是这次上线的阻塞项

## 仓库地址

GitHub: <https://github.com/limitcool/bilistream>
