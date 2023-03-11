```
 _     _ _ _     _
| |__ (_) (_)___| |_ _ __ ___  __ _ _ __ ___
| '_ \| | | / __| __| '__/ _ \/ _` | '_ ` _ \
| |_) | | | \__ \ |_| | |  __/ (_| | | | | | |
|_.__/|_|_|_|___/\__|_|  \___|\__,_|_| |_| |_|
```



# bilistream

bilistream是一个支持无人值守自动转播Twitch和Youtube（包括预告类型直播）的B站直播自动转播工具。

### QQ群: 715748617

```bash
# Debian
apt update
# 安装ffmpeg。
apt install ffmpeg -y
apt install python3-pip -y # 部分机器可能还需要此操作。
# CentOS
# 安装ffmpeg。
yum install ffmpeg -y
# 如需转播Youtube，需单独安装Yt-dlp。
pip3 install yt-dlp
# 更新yt-dlp至最新版。
pip3 install -U yt-dlp
```

## 使用指南

### Windows

```
.\bilistream.exe
```

### Linux-arm

```
xz -d bilistream-v0.1.0-aarch64-linux.tar.xz
tar -xvf bilistream-v0.1.0-aarch64-linux.tar
cd bilistream-v0.1.0-aarch64-linux
./bilistream
```

若使用的Linux版本glibc库较旧，可尝试使用Linux-musl版。Linux的编译环境版本为Ubuntu 20.04，低于此版本可尝试使用Linux-musl。

在解压目录新建 `config.yaml` 文件：

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
  Room: 
  AccessToken: 
# youtube 预告类型直播转播请填写以下内容
YoutubePreviewLive:
  ChannelId: UC1zFJrfEKvCixhsjNSb1toQ
# QQ群推送功能,需在本地部署QQ机器人
Push:
  # 本地QQ机器人的Api地址: 127.0.0.1:8080
  Host: 127.0.0.1:8080
  # 推送到的qq群号
  Target:
FfmpegProxy: http://127.0.0.1:7890
# Ffmpeg代理地址,无需代理可以不填此项或者留空
```

## Youtube API申请地址

https://developers.google.com/youtube/v3

## 常见问题FAQ

- Q: 转播时出现 Input/output error
  - A: 可能是BiliRtmpUrl及BiliRtmpKey填写错误或使用海外机器进行推流。B站不支持海外机器推流，建议使用国内服务器+代理推流。
- Q: 转播Youtube时出现Connection to tcp://manifest.googlevideo.com:443 failed: Error number -138 occurred
  - A: 可能是Ffmpeg拉流未通过代理，请在配置项填写 FfmpegProxy: [http://127.0.0.1:7890。](http://127.0.0.1:7890。/)
