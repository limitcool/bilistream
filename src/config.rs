use crate::error::AppResult;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
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
    #[serde(rename = "YoutubePreviewLive")]
    pub youtube_preview_live: YoutubePreviewLive,
    #[serde(rename = "FfmpegProxy")]
    pub ffmpeg_proxy: Option<String>,
    #[serde(rename = "Gotify")]
    pub gotify: Option<GotifyConfig>,
    #[serde(rename = "Ntfy")]
    pub ntfy: Option<NtfyConfig>,
    #[serde(rename = "Notification", default)]
    pub notification: NotificationConfig,
    #[serde(rename = "Cookies")]
    pub cookies: Option<String>,
    #[serde(rename = "WebConsole")]
    pub web_console: Option<WebConsoleConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bililive: BiliLive::default(),
            twitch: TwitchC::default(),
            interval: 60,
            youtube: YoutubeC::default(),
            platform: "Twitch".to_string(),
            youtube_preview_live: YoutubePreviewLive::default(),
            ffmpeg_proxy: None,
            gotify: None,
            ntfy: None,
            notification: NotificationConfig::default(),
            cookies: None,
            web_console: Some(WebConsoleConfig::default()),
        }
    }
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

impl Default for BiliLive {
    fn default() -> Self {
        Self {
            sessdata: String::new(),
            bili_jct: String::new(),
            dede_user_id: String::new(),
            dede_user_id_ckmd5: String::new(),
            room: 0,
            bili_rtmp_url: String::new(),
            bili_rtmp_key: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchC {
    #[serde(rename = "Room")]
    pub room: String,
}

impl Default for TwitchC {
    fn default() -> Self {
        Self {
            room: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoutubeC {
    #[serde(rename = "Room")]
    pub room: String,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

impl Default for YoutubeC {
    fn default() -> Self {
        Self {
            room: String::new(),
            access_token: String::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubePreviewLive {
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GotifyConfig {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Token")]
    pub token: String,
}

impl Default for GotifyConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            token: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NtfyConfig {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Topic")]
    pub topic: String,
    #[serde(rename = "Token")]
    pub token: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "Priority")]
    pub priority: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<String>,
}

impl Default for NtfyConfig {
    fn default() -> Self {
        Self {
            url: "https://ntfy.sh".to_string(),
            topic: String::new(),
            token: None,
            username: None,
            password: None,
            priority: None,
            tags: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Channel")]
    pub channel: String,
    #[serde(rename = "GotifyEnabled")]
    pub gotify_enabled: bool,
    #[serde(rename = "NtfyEnabled")]
    pub ntfy_enabled: bool,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channel: "gotify".to_string(),
            gotify_enabled: true,
            ntfy_enabled: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConsoleConfig {
    #[serde(rename = "Addr")]
    pub addr: String,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,
}

impl Default for WebConsoleConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:9090".to_string(),
            username: "admin".to_string(),
            password: String::new(),
        }
    }
}

pub fn load_config(config: &Path) -> AppResult<Config> {
    let file = std::fs::File::open(config)?;
    let config: Config = serde_yaml::from_reader(file)?;
    Ok(config)
}

pub fn save_config(config_path: &Path, config: &Config) -> AppResult<()> {
    let file = std::fs::File::create(config_path)?;
    serde_yaml::to_writer(file, config)?;
    Ok(())
}

pub struct EnsureConfigResult {
    pub created: bool,
    pub generated_web_password: bool,
}

pub fn ensure_config_exists(config_path: &Path) -> AppResult<EnsureConfigResult> {
    if config_path.exists() {
        return Ok(EnsureConfigResult {
            created: false,
            generated_web_password: false,
        });
    }

    let mut config = Config::default();
    let mut generated_web_password = false;

    if let Some(web_console) = config.web_console.as_mut() {
        let looks_public = !matches!(
            web_console.addr.as_str(),
            addr if addr.starts_with("127.0.0.1:")
                || addr.starts_with("localhost:")
                || addr.starts_with("[::1]:")
        );

        if looks_public && web_console.password.trim().is_empty() {
            web_console.password = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);
            generated_web_password = true;
        }
    }

    save_config(config_path, &config)?;
    Ok(EnsureConfigResult {
        created: true,
        generated_web_password,
    })
}
