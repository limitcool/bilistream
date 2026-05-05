use async_trait::async_trait;
use regex::Regex;
use std::process::Command;

use crate::error::AppResult;
use super::{get_youtube_live_status, Live};

pub struct Youtube {
    pub room: String,
    pub config: crate::config::Config,
}

#[async_trait]
impl Live for Youtube {
    fn room(&self) -> &str {
        &self.room
    }

    async fn get_status(&self) -> AppResult<bool> {
        get_youtube_live_status(&self.room).await
    }

    async fn get_real_m3u8_url(&self) -> AppResult<String> {
        self.ytdlp()
    }
}

impl Youtube {
    pub fn new(room: &str, config: crate::config::Config) -> Self {
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

        command.arg(format!(
            "https://www.youtube.com/channel/{}/live",
            self.room.replace('"', "")
        ));

        let output = command.output()?;
        if !output.status.success() {
            return Err("yt-dlp error".into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        Ok(self.replace_url(stdout.as_str()))
    }

    fn replace_url(&self, content: &str) -> String {
        let re = Regex::new(r"^WARNING.*").unwrap();
        re.replace_all(content, "").trim().to_string()
    }
}
