use async_trait::async_trait;
use std::error::Error;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_middleware::{ClientBuilder};
use std::{time::{Duration,}};
use crate::{config::Config, twitch};

#[async_trait]
pub trait Live {
    async fn get_status(&self) -> Result<bool, Box<dyn Error>>;
    fn room(&self) -> &str;
    async fn get_real_m3u8_url(&self) -> Result<String, Box<dyn Error>>;
}



pub fn select_live(cfg:Config) -> Result<Box<dyn Live>,Box<dyn Error>> {
    // 设置最大重试次数为3次
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let raw_client = reqwest::Client::builder()
    .cookie_store(true)
    // 设置超时时间为300秒
    .timeout(Duration::new(10, 0))
    .build().unwrap();
    let client = ClientBuilder::new(raw_client.clone())
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();

    match cfg.platform.as_str() {
        "Youtube" => Ok(Box::new(crate::youtube::new(cfg.youtube.room.as_str(), cfg.youtube.access_token, client.clone()))),
        "Twitch" => Ok(Box::new(twitch::new(cfg.twitch.room.as_str(), client.clone()))),
        _ => Err("unknown platform".into()),
    }
}