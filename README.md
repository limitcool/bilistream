```   _
| |__ (_) (_)___| |_ _ __ ___  __ _ _ __ ___
| '_ \| | | / __| __| '__/ _ \/ _` | '_ ` _ \
| |_) | | | \__ \ |_| | |  __/ (_| | | | | | |
|_.__/|_|_|_|___/\__|_|  \___|\__,_|_| |_| |_|
```


# bilistream

bilistream是一个支持无人值守自动转播Twitch和Youtube（包括预告类型直播）的B站直播自动转播工具。

### QQ群: 715748617

## 使用指南

### Docker 部署（推荐）

推荐使用 Docker 来部署 bilistream，这是最简单和稳定的方式：

1. 安装 Docker 和 Docker Compose：
   - Windows/Mac: 安装 [Docker Desktop](https://www.docker.com/products/docker-desktop)
   - Linux: 参考 [Docker 安装指南](https://docs.docker.com/engine/install/)

2. 创建 docker-compose.yaml 文件：
```yaml
services:
  bilistream:
    image: ghcr.io/limitcool/bilistream:v0.1.12
    container_name: bilistream
    volumes:
      - ./config.yaml:/app/config.yaml:ro  # 挂载配置文件（只读）
      - ./cookies.txt:/app/cookies.txt:ro   # 可选：挂载 cookies 文件（只读）
    restart: unless-stopped
    environment:
      - TZ=Asia/Shanghai    # 设置时区
```

3. 创建配置文件：
```bash
# 创建配置文件
touch config.yaml
```

4. 编辑 config.yaml 文件（配置示例见下文）

5. 启动服务：
```bash
docker-compose up -d
```

6. 查看运行状态：
```bash
# 查看日志
docker-compose logs -f

# 查看容器状态
docker-compose ps
```

7. 停止服务：
```bash
docker-compose down
```

注意事项：
- 确保配置文件中的路径使用容器内的路径（如 cookies 文件路径应该是 `/app/cookies.txt`）
- 如果需要使用代理，在 docker-compose.yaml 中添加：
```yaml
    network_mode: "host"  # 如果使用本地代理
    environment:
      - http_proxy=http://host.docker.internal:7890
      - https_proxy=http://host.docker.internal:7890
```

### 二进制部署

如果您不想使用 Docker，也可以直接下载二进制文件运行。首先需要安装以下依赖：

```bash
# Debian/Ubuntu
apt update
apt install ffmpeg python3-pip -y

# CentOS
yum install ffmpeg -y

# 安装 yt-dlp
pip3 install -U yt-dlp
```

## 使用指南

### 下载和安装

根据您的系统选择对应的版本：

#### Windows
- x64系统: 下载 `bilistream-v{版本号}-x86_64-pc-windows-msvc.zip`
- ARM64系统: 下载 `bilistream-v{版本号}-aarch64-pc-windows-msvc.zip`

下载后解压，直接运行 `bilistream.exe` 即可。

#### Linux
- x64系统: 下载 `bilistream-v{版本号}-x86_64-unknown-linux-musl.tar.gz`
- ARM64系统: 下载 `bilistream-v{版本号}-aarch64-unknown-linux-gnu.tar.gz`

```bash
# 解压
tar -xzf bilistream-v{版本号}-{架构}.tar.gz
cd bilistream-v{版本号}-{架构}
# 运行
./bilistream
```

#### macOS
- Intel芯片: 下载 `bilistream-v{版本号}-x86_64-apple-darwin.tar.gz`
- M系列芯片: 下载 `bilistream-v{版本号}-aarch64-apple-darwin.tar.gz`

```bash
# 解压
tar -xzf bilistream-v{版本号}-{架构}.tar.gz
cd bilistream-v{版本号}-{架构}
# 运行
./bilistream
```

### 验证下载

每个发布包都附带了 SHA256 校验和文件（.sha256 后缀），您可以使用它来验证下载的完整性：

```bash
# Windows (PowerShell)
Get-FileHash bilistream-v{版本号}-{架构}.zip -Algorithm SHA256

# Linux/macOS
shasum -a 256 bilistream-v{版本号}-{架构}.tar.gz
```

将输出的哈希值与对应的 .sha256 文件中的内容进行比对。

### 配置

在程序所在目录新建 `config.yaml` 文件：

```
touch config.yaml
```

将以下内容填写至 `config.yaml` 文件内：


``` yaml
# 检测直播间隔
Interval: 60
# 需要转播的平台 Twitch || Youtube || YoutubePreviewLive
Platform: Twitch
# B站推流账号Cookie
BiliLive:
  SESSDATA:
  bili_jct:
  DedeUserID: 2235894
  DedeUserID__ckMd5:
  Room: 660428
  BiliRtmpUrl: rtmp://live-push.bilivideo.com/live-bvc/
  # BiliRtmpUrl: B站开播设置页面的服务器地址
  BiliRtmpKey: "?streamname=live_0000000_0000000&key=xxxxxxxxxxb8289c6acc97xxxxxxxxx&schedule=rtmp&pflag=1"
  # BiliRtmpKey: B站开播设置页面的串流密钥,需注意,由于是?号开头的,本行需要对内容加双引号
# Twitch 直播间Id
Twitch:
  # Room: maximilian_dood
  Room:
# youtube 需要使用Youtube API AK以及Yt-dlp
Youtube:
  Room: UC1zFJrfEKvCixhsjNSb1toQ
  AccessToken:
# youtube 预告类型直播转播请填写以下内容
YoutubePreviewLive:
  ChannelId: UC1zFJrfEKvCixhsjNSb1toQ
FfmpegProxy: http://127.0.0.1:7890
# Ffmpeg代理地址,无需代理可以不填此项或者留空

### Gotify推送配置 (可选)

# 如果您想使用Gotify进行推送通知,请在`config.yaml`中添加以下配置:

Gotify:
  url: "https://example.com/gotify"
  token: "your_gotify_token_here"

### Cookies配置 (可选)
# 如果需要转播会员限定或需要登录的内容，可以配置cookies
Cookies: "/path/to/cookies.txt"  # cookies文件路径，支持YouTube和Twitch
```

## Youtube API申请地址

https://developers.google.com/youtube/v3

## 常见问题FAQ

- Q: 转播时出现 Input/output error
  - A: 可能是BiliRtmpUrl及BiliRtmpKey填写错误或使用海外机器进行推流。B站不支持海外机器推流，建议使用国内服务器+代理推流。
- Q: 转播Youtube时出现Connection to tcp://manifest.googlevideo.com:443 failed: Error number -138 occurred
  - A: 可能是Ffmpeg拉流未通过代理，请在配置项填写 FfmpegProxy: [http://127.0.0.1:7890。](http://127.0.0.1:7890。/)
- Q: 如何获取cookies文件？
  - A: 可以使用浏览器扩展（如Get cookies.txt）导出Netscape格式的cookies文件。对于YouTube和Twitch，需要先在浏览器中登录账号，然后导出cookies。
