use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "BiliLive")]
    pub bililive: BiliLive,
    #[serde(rename = "Twitch")]
    pub twitch: TwitchC,
    #[serde(rename = "Interval")]
    pub interval: u64,
    #[serde(rename = "Youtube")]
    pub youtube: YoutubeC,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "Email")]
    pub email: Option<EmailConfig>,
    #[serde(rename = "YoutubePreviewLive")]
    pub youtube_preview_live: YoutubePreviewLive,
    #[serde(rename = "FfmpegProxy")]
    pub ffmpeg_proxy: Option<String>,
    #[serde(rename = "Gotify")]
    pub gotify: Option<GotifyConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiliLive {
    #[serde(rename = "SESSDATA")]
    pub sessdata: String,
    pub bili_jct: String,
    #[serde(rename = "DedeUserID")]
    pub dede_user_id: String,
    #[serde(rename = "DedeUserID__ckMd5")]
    pub dede_user_id_ckmd5: String,
    #[serde(rename = "Room")]
    pub room: i32,
    #[serde(rename = "BiliRtmpUrl")]
    pub bili_rtmp_url: String,
    #[serde(rename = "BiliRtmpKey")]
    pub bili_rtmp_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchC {
    #[serde(rename = "Room")]
    pub room: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoutubeC {
    #[serde(rename = "Room")]
    pub room: String,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
    #[serde(rename = "To")]
    pub to: String,

    #[serde(rename = "Subject")]
    pub subject: String,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "Host")]
    pub host: String,

    #[serde(rename = "Sender")]
    pub sender: String,

    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubePreviewLive {
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
}

// 读取配置文件
pub fn load_config(config: &Path) -> Result<Config, Box<dyn Error>> {
    let file = std::fs::File::open(config)?;
    let config: Config = serde_yaml::from_reader(file)?;
    // println!("body = {:?}", client);
    Ok(config)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GotifyConfig {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Token")]
    pub token: String,
}
