use crate::config::Config;
use crate::error::AppResult;
use async_trait::async_trait;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde_json::Value;
use std::time::Duration;

use super::{Twitch, Youtube};

#[allow(dead_code)]
pub enum Status {
    Online,
    Offline,
    Unknown,
}

#[async_trait]
pub trait Live: Send {
    async fn get_status(&self) -> AppResult<bool>;
    fn room(&self) -> &str;
    async fn get_real_m3u8_url(&self) -> AppResult<String>;
}

pub async fn select_live(cfg: Config) -> AppResult<Box<dyn Live + Send>> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(30, 0))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    match cfg.platform.as_str() {
        "Youtube" => Ok(Box::new(Youtube::new(
            cfg.youtube.room.as_str(),
            cfg.clone(),
        ))),
        "Twitch" => Ok(Box::new(Twitch::new(
            cfg.twitch.room.as_str(),
            client.clone(),
            cfg.clone(),
        ))),
        "YoutubePreviewLive" => {
            let room_id = get_live_id_by_jump(cfg.youtube_preview_live.channel_id.as_str()).await?;
            Ok(Box::new(Youtube::new(room_id.as_str(), cfg.clone())))
        }
        _ => Err("unknown platform".into()),
    }
}

#[allow(dead_code)]
async fn get_channel_id(channel_name: &str) -> AppResult<String> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(30, 0))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/c/{}", channel_name);
    let body = client.get(&url).send().await?.text().await?;
    let room_id = body
        .split("\"channelId\":\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    Ok(room_id.to_string())
}

#[allow(dead_code)]
pub async fn get_live_id(channel_name: &str) -> AppResult<String> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(30, 0))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}", channel_name);
    tracing::debug!("{}", url);
    let body = client.get(&url).send().await?.text().await?;
    let html = prettyish_html::prettify(body.as_str());

    let re =
        regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)?;
    if let Some(cap) = re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        let j: Value = serde_json::from_str(json)?;
        let mut video_id = j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]
            ["tabRenderer"]["content"]["sectionListRenderer"]["contents"][1]["itemSectionRenderer"]
            ["contents"][0]["shelfRenderer"]["content"]["horizontalListRenderer"]["items"][0]
            ["gridVideoRenderer"]["videoId"]
            .to_string();
        if video_id == "null" {
            video_id = j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]
                ["tabRenderer"]["content"]["sectionListRenderer"]["contents"][2]
                ["itemSectionRenderer"]["contents"][0]["shelfRenderer"]["content"]
                ["horizontalListRenderer"]["items"][0]["gridVideoRenderer"]["videoId"]
                .to_string();
        }
        if video_id != "null" {
            let mut file = std::fs::File::create("live_id.json")?;
            std::io::Write::write_all(&mut file, json.as_bytes())?;
            return Ok(video_id);
        }
    }

    Err("获取 video_id 失败".into())
}

#[allow(dead_code)]
fn json_path_to_map_string(path: &str) -> String {
    let mut result = String::new();
    for part in path.split('.') {
        result.push_str(&format!("[\"{}\"]", part));
    }
    result
}

pub async fn get_live_id_by_jump(channel_name: &str) -> AppResult<String> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(30, 0))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}/live", channel_name);
    tracing::debug!("{}", url);
    let body = client.get(&url).send().await?.text().await?;
    let html = prettyish_html::prettify(body.as_str());

    let re =
        regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)?;
    if let Some(cap) = re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        let j: Value = serde_json::from_str(json)?;
        let mut video_id = j["contents"]["twoColumnWatchNextResults"]["results"]["results"]
            ["contents"][0]["videoPrimaryInfoRenderer"]["videoActions"]["menuRenderer"]
            ["topLevelButtons"][0]["toggleButtonRenderer"]["defaultNavigationEndpoint"]
            ["modalEndpoint"]["modal"]["modalWithTitleAndButtonRenderer"]["button"]
            ["buttonRenderer"]["navigationEndpoint"]["signInEndpoint"]["nextEndpoint"]
            ["watchEndpoint"]["videoId"]
            .to_string();
        if video_id == "null" {
            video_id = j["currentVideoEndpoint"]["watchEndpoint"]["videoId"].to_string();
        }
        if video_id != "null" {
            let mut file = std::fs::File::create("jump_live_id.json")?;
            std::io::Write::write_all(&mut file, json.as_bytes())?;
            return Ok(video_id);
        }
    }

    Err("获取 video_id 失败".into())
}

pub async fn get_youtube_live_status(channel_name: &str) -> AppResult<bool> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(30, 0))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}/live", channel_name);
    tracing::debug!("{}", url);
    let body = client.get(&url).send().await?.text().await?;
    let html = prettyish_html::prettify(body.as_str());

    let re =
        regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)?;
    if let Some(cap) = re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        let j: Value = serde_json::from_str(json)?;
        let live_status = j["contents"]["twoColumnWatchNextResults"]["results"]["results"]
            ["contents"][0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]
            ["isLive"]
            .to_string();
        return Ok(live_status == "true");
    }

    Ok(false)
}
