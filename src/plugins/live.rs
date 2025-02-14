use crate::config::Config;
use async_trait::async_trait;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

use super::{Twitch, Youtube};

#[allow(dead_code)]
/// Status of the live stream
pub enum Status {
    /// Stream is online.
    Online,
    /// Stream is offline.
    Offline,
    /// The status of the stream could not be determined.
    Unknown,
}

#[async_trait]
pub trait Live {
    async fn get_status(&self) -> Result<bool, Box<dyn Error>>;
    fn room(&self) -> &str;
    async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>>;
    fn set_room(&mut self, room: &str);
}
pub async fn select_live(cfg: Config) -> Result<Box<dyn Live>, Box<dyn Error>> {
    // 设置最大重试次数为4294967295次
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();
    let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    match cfg.platform.as_str() {
        "Youtube" => Ok(Box::new(Youtube::new(
            cfg.youtube.room.as_str(),
            cfg.youtube.access_token.clone(),
            client.clone(),
            cfg.clone(),
        ))),
        "Twitch" => Ok(Box::new(Twitch::new(
            cfg.twitch.room.as_str(),
            client.clone(),
            cfg.clone(),
        ))),
        "YoutubePreviewLive" => {
            let room_id = get_live_id_by_jump(cfg.youtube_preview_live.channel_id.as_str())
                .await
                .unwrap();
            Ok(Box::new(Youtube::new(
                room_id.as_str(),
                cfg.youtube.access_token.clone(),
                client.clone(),
                cfg.clone(),
            )))
        }
        _ => Err("unknown platform".into()),
    }
}

// https://www.youtube.com/channel/UC1zFJrfEKvCixhsjNSb1toQ
// 通过channel_name获取channel_id
#[allow(dead_code)]
async fn get_channel_id(channel_name: &str) -> Result<String, Box<dyn Error>> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();
    let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/c/{}", channel_name);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let room_id = body
        .split("\"channelId\":\"")
        .nth(1)
        .unwrap()
        .split("\"")
        .nth(0)
        .unwrap();
    Ok(room_id.to_string())
}

#[allow(dead_code)]
// 通过channel_id获取live_id
pub async fn get_live_id(channel_name: &str) -> Result<String, Box<dyn Error>> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();
    let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}", channel_name);
    tracing::debug!("{}", url);
    // println!("channel地址为:{}", url);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    // 保存body为文件,后缀为html
    let html = prettyish_html::prettify(body.as_str());
    // let mut file = std::fs::File::create("body.html").unwrap();
    // std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    let re = regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)
        .unwrap();
    // if re.is_match(html.as_str()) {
    //     let live_id = re.captures(html.as_str()).unwrap().get(1).unwrap().as_str();
    //     let live_id = live_id.split("\"").nth(1).unwrap();
    //     println!("{}", live_id);
    // } else {
    //     println!("no match");
    // }
    for cap in re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        // let json = json.split(";").nth(0).unwrap();
        // let json = json.split("=").nth(1).unwrap();
        // let json = json.split(";").nth(0).unwrap();
        // let json = json.split("}").nth(0).unwrap();
        // let json = json.split("{").nth(1).unwrap();
        // let json = json.split("\"").nth(1).unwrap();
        // let json = json.split("\"").nth(0).unwrap();
        let j: Value = serde_json::from_str(json).unwrap();
        let mut video_id = j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]
            ["tabRenderer"]["content"]["sectionListRenderer"]["contents"][1]["itemSectionRenderer"]
            ["contents"][0]["shelfRenderer"]["content"]["horizontalListRenderer"]["items"][0]
            ["gridVideoRenderer"]["videoId"]
            .to_string();
        if video_id == "null" {
            video_id = j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]
                ["content"]["sectionListRenderer"]["contents"][2]["itemSectionRenderer"]
                ["contents"][0]["shelfRenderer"]["content"]["horizontalListRenderer"]["items"][0]
                ["gridVideoRenderer"]["videoId"]
                .to_string();
        }
        // println!("获取到的videoId为:{}", video_id);
        tracing::debug!(
            "{}",
            j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]
                ["sectionListRenderer"]["contents"][1]["itemSectionRenderer"]["contents"][0]
                ["shelfRenderer"]["content"]["horizontalListRenderer"]["items"][0]
                ["gridVideoRenderer"]["videoId"]
        );
        // 将结果保存为一个json文件
        let mut file = std::fs::File::create("live_id.json").unwrap();
        std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
        if video_id == "null" {
        } else {
            return Ok(video_id);
        }
    }

    Err("获取video_id失败".into())
}


#[allow(dead_code)]
// 传入materials.canvases.0.image_id,返回 ["materials"]["canvases"][0]["image_id"]
fn json_path_to_map_string(path: &str) -> String {
    let r = path.split(".");
    let mut s = String::new();
    r.for_each(|x| {
        s.push_str(&format!("[\"{}\"]", x));
    });
    return s;
}

// 通过channel_id获取live_id
pub async fn get_live_id_by_jump(channel_name: &str) -> Result<String, Box<dyn Error>> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();
    let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}/live", channel_name);
    tracing::debug!("{}", url);
    // println!("channel地址为:{}", url);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    // 保存body为文件,后缀为html
    let html = prettyish_html::prettify(body.as_str());
    // let mut file = std::fs::File::create("jump.html").unwrap();
    // std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    let re = regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)
        .unwrap();
    // if re.is_match(html.as_str()) {
    //     let live_id = re.captures(html.as_str()).unwrap().get(1).unwrap().as_str();
    //     let live_id = live_id.split("\"").nth(1).unwrap();
    //     println!("{}", live_id);
    // } else {
    //     println!("no match");
    // }
    for cap in re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        let j: Value = serde_json::from_str(json).unwrap();
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
        // println!("获取到的videoId为:{}", video_id);
        tracing::debug!(
            "{}",
            j["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]
                ["sectionListRenderer"]["contents"][1]["itemSectionRenderer"]["contents"][0]
                ["shelfRenderer"]["content"]["horizontalListRenderer"]["items"][0]
                ["gridVideoRenderer"]["videoId"]
        );
        // 将结果保存为一个json文件
        let mut file = std::fs::File::create("jump_live_id.json").unwrap();
        std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
        if video_id == "null" {
        } else {
            return Ok(video_id);
        }
    }

    Err("获取video_id失败".into())
}

pub async fn get_youtube_live_status(channel_name: &str) -> Result<bool, Box<dyn Error>> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();
    let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let url = format!("https://www.youtube.com/channel/{}/live", channel_name);
    tracing::debug!("{}", url);
    // println!("channel地址为:{}", url);
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    // 保存body为文件,后缀为html
    let html = prettyish_html::prettify(body.as_str());
    // let mut file = std::fs::File::create("jump.html").unwrap();
    // std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    let re = regex::Regex::new(r#"\s*<script nonce=".*">var ytInitialData = (.*);\s*?</script>"#)
        .unwrap();
    // if re.is_match(html.as_str()) {
    //     let live_id = re.captures(html.as_str()).unwrap().get(1).unwrap().as_str();
    //     let live_id = live_id.split("\"").nth(1).unwrap();
    //     println!("{}", live_id);
    // } else {
    //     println!("no match");
    // }
    for cap in re.captures(html.as_str()) {
        let json = cap.get(1).unwrap().as_str();
        let j: Value = serde_json::from_str(json).unwrap();
        let live_status = j["contents"]["twoColumnWatchNextResults"]["results"]["results"]
            ["contents"][0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]
            ["isLive"]
            .to_string();
        // let mut file = std::fs::File::create("jump_live_id.json").unwrap();
        // std::io::Write::write_all(&mut file, json.as_bytes()).unwrap();
        // println!("live status{}", live_status);
        if live_status != "true" {
            return Ok(false);
        } else {
            return Ok(true);
        }
    }

    // Err("获取video_id失败".into())
    return Ok(false);
}

// 测试get_room_id 传入UC1zFJrfEKvCixhsjNSb1toQ
// #[cfg(test)]
// mod tests {
//     use super::*;

//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }
//     #[test]
//     fn test_get_room_id() {
//         let channel_id = "GameSpun";
//         let r = aw!(get_channel_id(channel_id)).unwrap();
//         println!("id:{}", r);
//     }
//     #[test]
//     fn test_get_live_id() {
//         let channel_id = "UC1zFJrfEKvCixhsjNSb1toQ";
//         let r = aw!(get_live_id(channel_id)).unwrap();
//         println!("id:{}", r);
//     }
//     #[test]
//     fn test_json_path_to_string() {
//         let re = json_path_to_map_string("x.contents.twoColumnWatchNextResults.results.results.contents[0].videoPrimaryInfoRenderer.videoActions.menuRenderer.topLevelButtons[0].toggleButtonRenderer.defaultNavigationEndpoint.modalEndpoint.modal.modalWithTitleAndButtonRenderer.button.buttonRenderer.navigationEndpoint.signInEndpoint.nextEndpoint.watchEndpoint.videoId");
//         println!("re:{}", re);
//     }
//     #[test]
//     fn test_get_jump_url() {
//         // lofi girl
//         let channel_id = "UCSJ4gkVC6NrvII8umztf0Ow";
//         let r: String = aw!(get_live_id_by_jump(channel_id)).unwrap();
//         println!("url:{}", r);
//     }
//     #[test]
//     fn tes_get_youtube_status() {
//         let channel_id = "UCcHWhgSsMBemnyLhg6GL1vA";
//         let r = aw!(get_youtube_live_status(channel_id)).unwrap();
//         println!("直播状态为:{}", r);
//     }
// }
