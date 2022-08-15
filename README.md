

# bilistream

B站直播转播工具,,一键转播Twitch,Youtube(支持youtube预告类型直播自动转播)

```bash
# 安装必须依赖ffmpeg,Twitch及youtube都需要安装ffmpeg
# debian
apt install ffmpeg -y
# centos
yum install ffmpeg -y
# 如需转播Youtube需单独安装Yt-dlp
pip3 install yt-dlp
```



## 使用指南

```bash
# windows
.\bilistream.exe
# linux-arm
xz -d bilistream-v0.1.0-aarch64-linux.tar.xz
tar -xvf bilistream-v0.1.0-aarch64-linux.tar
cd bilistream-v0.1.0-aarch64-linux
# 若使用的linux版本glibc库较旧,可尝试使用linux-musl版。
# linux的编译环境版本为ubuntu-20.04,低于此版本可尝试使用linux-musl
./bilistream
# 在解压目录新建config.yaml
touch config.yaml
```

将以下内容填写至`config.yaml`文件内

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
  BiliRtmpUrl: 
  BiliRtmpKey: 
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
```

### Youtube API申请地址

```bash
https://developers.google.com/youtube/v3
```

