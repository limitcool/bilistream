use async_trait::async_trait;
use m3u8_rs::Playlist;
use rand::prelude::*;
use reqwest_middleware::ClientWithMiddleware;
use serde_json::json;
use std::error::Error;
use urlencoding::encode;
use std::process::Command;

use super::Live;

pub struct Twitch {
    pub room: String,
    pub client: ClientWithMiddleware,
    pub config: crate::config::Config,
}

struct Token {
    pub token: String,
    #[allow(dead_code)]
    pub expire: String,
    pub sig: String,
}
#[async_trait]
impl Live for Twitch {
    async fn get_status(&self) -> Result<bool, Box<dyn Error>> {
        let j = json!(
            {
                "operationName":"StreamMetadata",
                "variables":{
                    "channelLogin":&self.room,
                },
                "extensions":{
                    "persistedQuery":{
                        "version":1,
                        "sha256Hash":"1c719a40e481453e5c48d9bb585d971b8b372f8ebb105b17076722264dfa5b3e"
                    }
                }
            }
        );
        let res: serde_json::Value = self
            .client
            .post("https://gql.twitch.tv/gql")
            .header("Client-ID", "kimne78kx3ncx6brgo4mv6wki5h1ko")
            .json(&j)
            .send()
            .await?
            .json()
            .await?;
        if res["data"]["user"]["stream"]["type"] == "live" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    // async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>> {
    //     let res: serde_json::Value = self
    //         .client
    //         .get(
    //             format!(
    //                 "https://api.twitch.tv/api/channels/{}/access_token",
    //                 self.room
    //             )
    //             .as_str(),
    //         )
    //         .header("Client-ID", "jzkbprff40iqj646a697cyrvl0zt2m6")
    //         .send()
    //         .await?
    //         .json()
    //         .await?;
    //     // println!("{:#?}", res);
    //     let v = self
    //         .get_m3u8_url(Token {
    //             token: res["token"].as_str().unwrap().to_string(),
    //             expire: res["expires_in"].to_string(),
    //             sig: res["sig"].as_str().unwrap().to_string(),
    //         })
    //         .await?;
    //     // println!("{:#?}", res);
    //     return Ok(v);
    // }
    async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>> {
        return self.ytdlp();
    }
    fn room(&self) -> &str {
        &self.room
    }
    fn set_room(&mut self, room: &str) {
        self.room = room.to_string();
    }
}

impl Twitch {
    pub fn new(room: &str, client: ClientWithMiddleware, config: crate::config::Config) -> impl Live {
        return Twitch {
            room: room.to_string(),
            client: client,
            config: config,
        };
    }
    async fn get_m3u8_url(&self, token: Token) -> Result<String, Box<dyn Error>> {
        let num = rand::thread_rng().gen_range(1..9000000);
        let url=  format!("https://usher.ttvnw.net/api/channel/hls/{}.m3u8?allow_audio_only=true&allow_source=true&allow_spectre=true&p={}&player=twitchweb&segment_preference=4&sig={}&token={}", self.room,num+1000000, token.sig,encode(token.token.as_str()).to_string());
        let res = self.client.get(url).send().await?.text().await?;
        // print!("{}", res);
        let parsed = m3u8_rs::parse_playlist_res(res.as_bytes());
        match parsed {
            Ok(Playlist::MasterPlaylist(pl)) => Ok(pl.variants[0].uri.to_string()),
            Ok(Playlist::MediaPlaylist(_pl)) => Ok("".to_string()),
            Err(_e) => Err("".into()),
        }
    }

    pub fn ytdlp(&self) -> Result<String, Box<dyn Error>> {
        let mut command = Command::new("yt-dlp");
        command.arg("-g");

        // 如果配置了cookies，添加cookies参数
        if let Some(cookies) = &self.config.cookies {
            command.arg("--cookies");
            command.arg(cookies);
        }

        command.arg(format!("https://www.twitch.tv/{}", self.room));

        let output = command.output()?;
        let stdout = String::from_utf8(output.stdout)?;
        let urls: Vec<&str> = stdout.trim().split('\n').collect();

        if urls.is_empty() {
            return Err("No URL found".into());
        }

        Ok(urls[0].to_string())
    }

    fn replace_url(&self, content: &str) -> String {
        let re = regex::Regex::new(r"^WARNING.*").unwrap();
        let res = re.replace_all(content, "");
        return res.to_string();
    }
}

#[allow(dead_code)]
pub fn name(item: &impl Live) -> String {
    return "Twitch:".to_string() + &item.room().to_string();
}
