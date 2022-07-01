mod twitch;
mod live;
mod config;
mod youtube;
// use tracing::info;
use std::{time::{Duration,}};
use tokio;
use std::path::Path;
use reqwest::{cookie::Jar, Url};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::process::Command;
use tracing::info;
use config::{load_config,Config};

// }
#[tokio::main]
async fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
    let cfg = load_config(Path::new("./config.yaml")).unwrap();
    let r = live::select_live(cfg.clone()).unwrap();
    // 设置tracing日志等级为Info
    loop {
        if r.get_status().await.unwrap() {
            info!("{}", format!("{}直播中",r.room()));
            if get_bili_live_state(cfg.bililive.room.clone()).await {
                info!("B站直播中");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), r.get_real_m3u8_url().await.unwrap());
            }else{
                info!("B站未直播");
                bili_start_live(&cfg).await;
                info!("B站已开播");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), r.get_real_m3u8_url().await.unwrap());
            }
        } else {
            info!("{}", format!("{}未直播",r.room()));
            if get_bili_live_state(cfg.bililive.room.clone()).await {
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
async fn get_bili_live_state(room:i32) -> bool {
    let client = reqwest::Client::new();
    let res:serde_json::Value = client
    .get(format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&platform=web",room))
    
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