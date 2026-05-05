use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;
use std::process::Command;

use crate::error::AppResult;
use super::Live;

pub struct Twitch {
    pub room: String,
    pub config: crate::config::Config,
}

#[async_trait]
impl Live for Twitch {
    async fn get_status(&self) -> AppResult<bool> {
        let client = reqwest::Client::builder().build()?;
        let payload = serde_json::json!({
            "operationName": "StreamMetadata",
            "variables": {
                "channelLogin": &self.room,
                "includeIsDJ": true
            },
            "extensions": {
                "persistedQuery": {
                    "version": 1,
                    "sha256Hash": "b57f9b910f8cd1a4659d894fe7550ccc81ec9052c01e438b290fd66a040b9b93"
                }
            }
        });
        let response: serde_json::Value = client
            .post("https://gql.twitch.tv/gql")
            .header("Client-ID", "kimne78kx3ncx6brgo4mv6wki5h1ko")
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;
        Ok(response["data"]["user"]["stream"]["type"] == "live")
    }

    async fn get_real_m3u8_url(&self) -> AppResult<String> {
        self.ytdlp()
    }

    fn room(&self) -> &str {
        &self.room
    }
}

impl Twitch {
    pub fn new(room: &str, _client: ClientWithMiddleware, config: crate::config::Config) -> Self {
        Self {
            room: room.to_string(),
            config,
        }
    }

    pub fn ytdlp(&self) -> AppResult<String> {
        let mut command = Command::new("yt-dlp");
        command.arg("-g");

        if let Some(cookies) = &self.config.cookies {
            command.arg("--cookies");
            command.arg(cookies);
        }

        command.arg(format!("https://www.twitch.tv/{}", self.room));

        let output = command.output()?;
        let stdout = String::from_utf8(output.stdout)?;
        let url = stdout
            .lines()
            .find(|line| !line.trim().is_empty() && !line.starts_with("WARNING"))
            .ok_or("No URL found")?;

        Ok(url.to_string())
    }
}
