use async_trait::async_trait;
use m3u8_rs::Playlist;
use rand::prelude::*;
use reqwest_middleware::ClientWithMiddleware;
use serde_json::json;
use std::error::Error;
use urlencoding::encode;

use super::Live;

pub struct Twitch {
    pub room: String,
    pub client: ClientWithMiddleware,
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
    async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>> {
        let res: serde_json::Value = self
            .client
            .get(
                format!(
                    "https://api.twitch.tv/api/channels/{}/access_token",
                    self.room
                )
                .as_str(),
            )
            .header("Client-ID", "jzkbprff40iqj646a697cyrvl0zt2m6")
            .send()
            .await?
            .json()
            .await?;
        // println!("{:#?}", res);
        let v = self
            .get_m3u8_url(Token {
                token: res["token"].as_str().unwrap().to_string(),
                expire: res["expires_in"].to_string(),
                sig: res["sig"].as_str().unwrap().to_string(),
            })
            .await?;
        // println!("{:#?}", res);
        return Ok(v);
    }
    fn room(&self) -> &str {
        &self.room
    }
    fn set_room(&mut self, room: &str) {
        self.room = room.to_string();
    }
}

impl Twitch {
    pub fn new(room: &str, client: ClientWithMiddleware) -> impl Live {
        return Twitch {
            room: room.to_string(),
            client: client,
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
}

#[allow(dead_code)]
pub fn name(item: &impl Live) -> String {
    return "Twitch:".to_string() + &item.room().to_string();
}
