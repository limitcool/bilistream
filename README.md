```
 _     _ _ _     _
| |__ (_) (_)___| |_ _ __ ___  __ _ _ __ ___
| '_ \| | | / __| __| '__/ _ \/ _` | '_ ` _ \
| |_) | | | \__ \ |_| | |  __/ (_| | | | | | |
|_.__/|_|_|_|___/\__|_|  \___|\__,_|_| |_| |_|
```



# bilistream

B站直播自动转播工具,无人值守自动转播Twitch,Youtube(支持youtube预告类型直播自动转播)

### QQ群: 715748617

```bash
# 安装必须依赖ffmpeg,Twitch及youtube都需要安装ffmpeg
# debian
apt install ffmpeg -y
# centos
yum install ffmpeg -y
# 如需转播Youtube需单独安装Yt-dlp
pip3 install yt-dlp
# 更新yt-dlp至最新版
pip3 install -U yt-dlp
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
```

### Youtube API申请地址

```bash
https://developers.google.com/youtube/v3
```


## 常见问题FAQ

Q: 转播时出现  `Input/output error`

A:BiliRtmpUrl及 BiliRtmpKey 填写错误或使用海外机器进行推流,B站不支持海外机器推流,建议使用国内服务器+代理推流。

Q:转播Youtube时出现`Connection to tcp://manifest.googlevideo.com:443 failed: Error number -138 occurred`

A: YT-DLP获取到的M3U8链接无法正常播放,需要等待Yt-dlp修复或更换其他方式获取M3U8链接。
