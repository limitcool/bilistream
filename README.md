

# bilistream

B站直播转播工具,,一键转播Twitch,Youtube,Mildom等直播。(目前支持Twitch,Youtube)

## 使用指南

```bash
# windows
.\bilistream.exe
# linux-arm
xz -d bilistream-v0.1.0-aarch64-linux.tar.xz
tar -xvf bilistream-v0.1.0-aarch64-linux.tar
cd bilistream-v0.1.0-aarch64-linux
./bilistream
# 在解压目录新建config.yaml
touch config.yaml
```

将以下内容填写至`config.yaml`文件内

``` yaml
# 检测直播间隔
Interval: 60
# 需要转播的平台
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
# youtube 需要使用AK或者使用Yt-dlp
Youtube:
  Room: 
  AccessToken: 
# QQ群推送功能,需在本地部署QQ机器人
Push:
  # 本地QQ机器人的Api地址: 127.0.0.1:8080
  Host: 127.0.0.1:8080
  # 推送到的qq群号
  Target:
```