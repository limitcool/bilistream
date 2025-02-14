use async_trait::async_trait;
use regex::Regex;
use reqwest_middleware::ClientWithMiddleware;
use std::error::Error;
use std::process::Command;

use super::{get_youtube_live_status, Live};
pub struct Youtube {
    pub room: String,
    pub access_token: String,
    pub client: ClientWithMiddleware,
    pub config: crate::config::Config,
}
#[async_trait]
impl Live for Youtube {
    fn room(&self) -> &str {
        &self.room
    }
    async fn get_status(&self) -> Result<bool, Box<dyn Error>> {
        // let res = self.client
        // .get(format!("https://www.googleapis.com/youtube/v3/videos?part=liveStreamingDetails&id={}&key={}", self.room.replace("\"", ""), self.access_token))
        // .send()
        // .await?;
        // let res = res.json::<serde_json::Value>().await?;
        // // println!("{:?}", self.room);
        // // println!("re{}", res);
        // // println!(
        // //     "AT{}",
        // //     res["items"][0]["liveStreamingDetails"]["actualStartTime"]
        // // );
        // if res["items"][0]["liveStreamingDetails"]["actualStartTime"].is_null() {
        //     Ok(false)
        // } else {
        //     Ok(true)
        // }
        get_youtube_live_status(&self.room).await
    }
    async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>> {
        return self.ytdlp();
    }
    fn set_room(&mut self, room: &str) {
        self.room = room.to_string().replace("\"", "");
    }
}

impl Youtube {
    pub fn new(
        room: &str,
        access_token: String,
        client: ClientWithMiddleware,
        config: crate::config::Config,
    ) -> impl Live {
        Youtube {
            room: room.to_string(),
            access_token,
            client,
            config,
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

        command.arg(format!(
            "https://www.youtube.com/channel/{}/live",
            self.room.as_str().replace("\"", "")
        ));

        match command.status().unwrap().code() {
            Some(code) => {
                if code == 0 {
                    let res = command.output().unwrap();
                    let res = String::from_utf8(res.stdout).unwrap();
                    Ok(self.replace_url(res.as_str()))
                } else {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "yt-dlp error",
                    )))
                }
            }
            None => Err("yt-dlp error".into()),
        }
    }

    fn replace_url(&self, content: &str) -> String {
        let re = Regex::new(r"^WARNING.*").unwrap();
        let res = re.replace_all(content, "");
        return res.to_string();
    }
}
