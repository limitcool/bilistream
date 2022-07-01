mod twitch;
use serde::{Deserialize, Serialize};
// use tracing::info;
use std::{time::{Duration,}, error::Error,};
use tokio;
use reqwest::{cookie::Jar, Url};
use std::path::Path;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_middleware::{ClientBuilder};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::process::Command;
use twitch::{Live, Twitch};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "BiliLive")]
    bililive: BiliLive,
    #[serde(rename = "Twitch")]
    twitch:TwitchC,
    #[serde(rename = "Interval")]
    interval:u64,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct BiliLive{
    #[serde(rename = "SESSDATA")]
    sessdata: String,
    bili_jct: String,
    #[serde(rename = "DedeUserID")]
    dede_user_id: String,
    #[serde(rename= "DedeUserID__ckMd5")]
    dede_user_id_ckmd5: String,
    #[serde(rename= "Room")]
    room:i32,
    #[serde(rename= "BiliRtmpUrl")]
    bili_rtmp_url: String,
    #[serde(rename= "BiliRtmpKey")]
    bili_rtmp_key: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TwitchC{
    #[serde(rename= "Room")]
    room:String,
}

// }
#[tokio::main]
async fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
    let cfg = load_config(Path::new("./config.yaml")).unwrap();
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

    let  t = Twitch{
        room : cfg.twitch.room.clone(),
        client: client.clone()
    };

    loop {
        if t.get_status().await.unwrap() {
            info!("Twitch直播中");
            if get_bili_live_state().await {
                info!("B站直播中");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), t.get_token().await.unwrap());
            }else{
                info!("B站未直播");
                bili_start_live(&cfg).await;
                info!("B站已开播");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), t.get_token().await.unwrap());
            }
        } else {
            info!("Twitch未直播");
            if get_bili_live_state().await {
                info!("B站直播中");
                bili_stop_live(&cfg).await;
                info!("B站已关播");
            }
        }
        // 每60秒检测一下直播状态
        tokio::time::sleep(Duration::from_secs(cfg.interval)).await;
    }

}


// 获取B站直播状态
async fn get_bili_live_state() -> bool {
    let client = reqwest::Client::new();
    let res:serde_json::Value = client
    .get("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id=660428&platform=web")
    
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();
    println!("{:#?}",res["data"]["live_status"]);
    if res["data"]["live_status"] == 0{
        return false;
    }else{
        return true;
    }
}



// bilibili开播
async fn bili_start_live(cfg:&Config){
    let cookie = format!("SESSDATA={};bili_jct={};DedeUserID={};DedeUserID__ckMd5={}",cfg.bililive.sessdata,cfg.bililive.bili_jct,cfg.bililive.dede_user_id,cfg.bililive.dede_user_id_ckmd5);
    let url= "https://api.live.bilibili.com/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);
    let client = reqwest::Client::builder().cookie_provider(jar.into()).build().unwrap();
    let res:serde_json::Value = client.post("https://api.live.bilibili.com/room/v1/Room/startLive")
    .header("Accept", "application/json, text/plain, */*")
    .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
    .body(format!("room_id={}&platform=pc&area_v2=433&csrf_token={}&csrf={}",cfg.bililive.room,cfg.bililive.bili_jct,cfg.bililive.bili_jct)).send().await.unwrap().json().await.unwrap();
    println!("{:#?}",res);
}

// bilibili关播
async fn bili_stop_live(cfg:&Config){
    let cookie = format!("SESSDATA={};bili_jct={};DedeUserID={};DedeUserID__ckMd5={}",cfg.bililive.sessdata,cfg.bililive.bili_jct,cfg.bililive.dede_user_id,cfg.bililive.dede_user_id_ckmd5);
    let url= "https://api.live.bilibili.com/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);
    let client = reqwest::Client::builder().cookie_provider(jar.into()).build().unwrap();
    let res:serde_json::Value = client.post("https://api.live.bilibili.com/room/v1/Room/stopLive")
    .header("Accept", "application/json, text/plain, */*")
    .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
    .body(format!("room_id={}&platform=pc&csrf_token={}&csrf={}",cfg.bililive.room,cfg.bililive.bili_jct,cfg.bililive.bili_jct)).send().await.unwrap().json().await.unwrap();
    println!("{:#?}",res);
}


// 读取配置文件
pub fn load_config(config: &Path) -> Result<Config,Box<dyn Error>>{
    let file = std::fs::File::open(config)?;
    let config: Config = serde_yaml::from_reader(file)?;
    // println!("body = {:?}", client);
    Ok(config)
}

pub fn ffmpeg(rtmp_url:String,rtmp_key:String,m3u8_url:String){
    let cmd = format!("{}&key={}",rtmp_url,rtmp_key);
    let mut command =Command::new("ffmpeg");
    command.arg("-re");
    command.arg("-i");
    command.arg(m3u8_url);
    command.arg("-vcodec");
    command.arg("copy");
    command.arg("-acodec");
    command.arg("aac");
    command.arg("-f");
    command.arg("flv");
    command.arg(cmd);
    match command.status().unwrap().code() {
    Some(code) => {
        println!("Exit Status: {}", code);
    }
    None => {
        println!("Process terminated.");
    }
}
}