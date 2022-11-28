mod config;
mod plugins;
mod push;
// use tracing::info;
use std::{time::{Duration,}};
use plugins::{select_live, get_live_id_by_jump};
use push::Mirai;
use tokio;
use std::path::Path;
use reqwest::{cookie::Jar, Url};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_middleware::{ClientBuilder};
use tracing_subscriber::{filter::EnvFilter,fmt::{self, format}, layer::SubscriberExt, util::SubscriberInitExt};
use std::process::Command;
use config::{load_config,Config};


#[tokio::main]
async fn main() {
    // let p = Mirai::new(host, target);
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry()
    .with(env_filter)
    .with(fmt::layer())
    .init();
    let cfg = load_config(Path::new("./config.yaml")).unwrap();
    let mut r = select_live(cfg.clone()).await.unwrap();
    // 设置tracing日志等级为Info
    
    loop {
        if r.get_status().await.unwrap_or(false) {
            tracing::info!("{}", format!("{}直播中",r.room()));

            match cfg.push {
                Some(ref push) => {

                    if push.host.clone()=="~"|| push.target.clone()=="~" {
                        tracing::info!("如需使用推送请在config.yaml中配置push.host");
                    }else {
                        let p = Mirai::new(push.host.clone(), push.target.clone()); 
                        p.send_message(r.room().to_string(),cfg.bililive.room.to_string()).await;
                    }

                }
                None => {
                    tracing::info!("如需使用推送请在config.yaml中配置push.host");
                }
            };

            if get_bili_live_state(cfg.bililive.room.clone()).await {
                tracing::info!("B站直播中");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), r.get_real_m3u8_url().await.unwrap());
            }else{
                tracing::info!("B站未直播");
                bili_start_live(&cfg).await;
                tracing::info!("B站已开播");
                ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), r.get_real_m3u8_url().await.unwrap());
                loop {
                    if r.get_status().await.unwrap() {
                        ffmpeg(cfg.bililive.bili_rtmp_url.clone(), cfg.bililive.bili_rtmp_key.clone(), r.get_real_m3u8_url().await.unwrap());
                    }else{
                        break;
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        } else {
            tracing::info!("{}", format!("{}未直播",r.room()));
            if get_bili_live_state(cfg.bililive.room.clone()).await {
                tracing::info!("B站直播中");
                bili_stop_live(&cfg).await;
                tracing::info!("B站已关播");
            }
        }
        // 判断是否预告类型
        if cfg.platform == "YoutubePreviewLive" {
            tracing::info!("检测到预告类型,正在重新获取直播间");
            r.set_room(get_live_id_by_jump(cfg.youtube_preview_live.channel_id.as_str())
            .await
            .unwrap().as_str())
        }
        // 每60秒检测一下直播状态
        tokio::time::sleep(Duration::from_secs(cfg.interval)).await;
    }
    

}


// 获取B站直播状态
async fn get_bili_live_state(room:i32, ) -> bool {
    // 设置最大重试次数为4294967295次
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
    .cookie_store(true)
    // 设置超时时间为30秒
    .timeout(Duration::new(30, 0))
    .build().unwrap();
    let client = ClientBuilder::new(raw_client.clone())
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();
    let res:serde_json::Value = client
    .get(format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&platform=web",room))
    
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();
    // println!("{:#?}",res["data"]["live_status"]);
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
    // 设置最大重试次数为4294967295次
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
    let raw_client = reqwest::Client::builder()
    .cookie_store(true)
    .cookie_provider(jar.into())
    // 设置超时时间为30秒
    .timeout(Duration::new(30, 0))
    .build().unwrap();
    let client = ClientBuilder::new(raw_client.clone())
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();
    let _res:serde_json::Value = client.post("https://api.live.bilibili.com/room/v1/Room/startLive")
    .header("Accept", "application/json, text/plain, */*")
    .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
    .body(format!("room_id={}&platform=pc&area_v2=433&csrf_token={}&csrf={}",cfg.bililive.room,cfg.bililive.bili_jct,cfg.bililive.bili_jct)).send().await.unwrap().json().await.unwrap();
    // println!("{:#?}",res);
}

// bilibili关播
async fn bili_stop_live(cfg:&Config){
    let cookie = format!("SESSDATA={};bili_jct={};DedeUserID={};DedeUserID__ckMd5={}",cfg.bililive.sessdata,cfg.bililive.bili_jct,cfg.bililive.dede_user_id,cfg.bililive.dede_user_id_ckmd5);
    let url= "https://api.live.bilibili.com/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);
        // 设置最大重试次数为4294967295次
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(4294967295);
        let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        // 设置超时时间为30秒
        .timeout(Duration::new(30, 0))
        .build().unwrap();
        let client = ClientBuilder::new(raw_client.clone())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let _res:serde_json::Value = client.post("https://api.live.bilibili.com/room/v1/Room/stopLive")
    .header("Accept", "application/json, text/plain, */*")
    .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
    .body(format!("room_id={}&platform=pc&csrf_token={}&csrf={}",cfg.bililive.room,cfg.bililive.bili_jct,cfg.bililive.bili_jct)).send().await.unwrap().json().await.unwrap();
    // println!("{:#?}",res);
}



pub fn ffmpeg(rtmp_url:String,rtmp_key:String,m3u8_url:String){
    // let cmd = format!("{}&key={}",rtmp_url,rtmp_key);
    let cmd = format!("{}{}",rtmp_url,rtmp_key);
    let mut command =Command::new("ffmpeg");
    command.arg("-re");
    command.arg("-i");
    command.arg(m3u8_url.clone());
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
        if code == 0 {
            println!("Command executed successfully");
        } else {
            ffmpeg(rtmp_url, rtmp_key, m3u8_url)
        }
    }
    None => {
        println!("Process terminated.");
    }
}
}