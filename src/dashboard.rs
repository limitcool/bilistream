use std::collections::VecDeque;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{header, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine;
use reqwest::{cookie::Jar, Url};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::Serialize;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::sync::{Mutex, Notify};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::{ensure_config_exists, load_config, save_config, Config, WebConsoleConfig};
use crate::error::AppResult;
use crate::plugins::{select_live, Live};
use crate::push::send_live_notification;

const LOG_LIMIT: usize = 120;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_URL: &str = "https://github.com/limitcool/bilistream";
const WEB_DIST_DIR: &str = "./web/out";

#[derive(Clone)]
struct RouterState {
    app: Arc<AppState>,
    auth: Arc<AuthConfig>,
}

#[derive(Clone)]
struct AuthConfig {
    username: String,
    password: Option<String>,
}

impl AuthConfig {
    fn from_config(config: Option<&WebConsoleConfig>) -> Self {
        let config = config.cloned().unwrap_or_default();
        let password = config.password.trim().to_string();
        Self {
            username: config.username,
            password: if password.is_empty() {
                None
            } else {
                Some(password)
            },
        }
    }

    fn enabled(&self) -> bool {
        self.password.is_some()
    }
}

#[derive(Clone, Serialize)]
struct LogEntry {
    timestamp: u64,
    level: String,
    message: String,
}

#[derive(Clone, Serialize)]
struct ConfigSummary {
    interval: u64,
    platform: String,
    bilibili_room: i32,
    twitch_room: String,
    youtube_room: String,
    youtube_preview_channel: String,
    notification_enabled: bool,
    notification_channel: String,
    gotify_enabled: bool,
    ntfy_enabled: bool,
    has_proxy: bool,
    has_cookies: bool,
}

#[derive(Clone, Serialize)]
struct StatusSnapshot {
    app_version: String,
    github_url: String,
    auth_enabled: bool,
    web_addr: String,
    config_path: String,
    config_loaded: bool,
    config_error: Option<String>,
    enabled: bool,
    worker_state: String,
    source_platform: String,
    source_room: String,
    source_live: Option<bool>,
    bilibili_live: Option<bool>,
    ffmpeg_running: bool,
    last_checked_at: Option<u64>,
    last_event: Option<String>,
    last_error: Option<String>,
    config_summary: ConfigSummary,
    logs: Vec<LogEntry>,
}

struct RuntimeState {
    config: Config,
    config_loaded: bool,
    config_error: Option<String>,
    enabled: bool,
    worker_state: String,
    source_platform: String,
    source_room: String,
    source_live: Option<bool>,
    bilibili_live: Option<bool>,
    ffmpeg_running: bool,
    last_checked_at: Option<u64>,
    last_event: Option<String>,
    last_error: Option<String>,
    last_notified_live: bool,
    logs: VecDeque<LogEntry>,
}

struct AppState {
    inner: Mutex<RuntimeState>,
    notify: Notify,
    config_path: PathBuf,
    web_addr: String,
    auth_enabled: bool,
}

impl AppState {
    fn new(config_path: PathBuf, web_addr: String, auth_enabled: bool) -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(RuntimeState {
                config: Config::default(),
                config_loaded: false,
                config_error: None,
                enabled: false,
                worker_state: "等待启动".to_string(),
                source_platform: "-".to_string(),
                source_room: "-".to_string(),
                source_live: None,
                bilibili_live: None,
                ffmpeg_running: false,
                last_checked_at: None,
                last_event: None,
                last_error: None,
                last_notified_live: false,
                logs: VecDeque::new(),
            }),
            notify: Notify::new(),
            config_path,
            web_addr,
            auth_enabled,
        })
    }

    async fn load_initial_config(self: &Arc<Self>) {
        match load_config(&self.config_path) {
            Ok(config) => {
                let mut state = self.inner.lock().await;
                state.source_platform = config.platform.clone();
                state.source_room = source_room_for_config(&config);
                state.config = config;
                state.config_loaded = true;
                state.config_error = None;
                state.enabled = true;
                state.worker_state = "已加载配置，等待下一轮检查".to_string();
                drop(state);
                self.log("info", "已加载 config.yaml，监控已启用").await;
            }
            Err(error) => {
                let message = format!("读取配置失败: {}", error);
                let mut state = self.inner.lock().await;
                state.config_error = Some(message.clone());
                state.enabled = false;
                state.worker_state = "配置缺失或无效".to_string();
                drop(state);
                self.log("error", message).await;
            }
        }
    }

    async fn snapshot(&self) -> StatusSnapshot {
        let state = self.inner.lock().await;
        StatusSnapshot {
            app_version: APP_VERSION.to_string(),
            github_url: GITHUB_URL.to_string(),
            auth_enabled: self.auth_enabled,
            web_addr: self.web_addr.clone(),
            config_path: self.config_path.display().to_string(),
            config_loaded: state.config_loaded,
            config_error: state.config_error.clone(),
            enabled: state.enabled,
            worker_state: state.worker_state.clone(),
            source_platform: state.source_platform.clone(),
            source_room: state.source_room.clone(),
            source_live: state.source_live,
            bilibili_live: state.bilibili_live,
            ffmpeg_running: state.ffmpeg_running,
            last_checked_at: state.last_checked_at,
            last_event: state.last_event.clone(),
            last_error: state.last_error.clone(),
            config_summary: ConfigSummary {
                interval: state.config.interval,
                platform: state.config.platform.clone(),
                bilibili_room: state.config.bililive.room,
                twitch_room: state.config.twitch.room.clone(),
                youtube_room: state.config.youtube.room.clone(),
                youtube_preview_channel: state.config.youtube_preview_live.channel_id.clone(),
                notification_enabled: state.config.notification.enabled,
                notification_channel: state.config.notification.channel.clone(),
                gotify_enabled: state.config.notification.gotify_enabled
                    && state
                        .config
                        .gotify
                        .as_ref()
                        .is_some_and(|value| {
                            !value.url.trim().is_empty() || !value.token.trim().is_empty()
                        }),
                ntfy_enabled: state.config.notification.ntfy_enabled
                    && state
                        .config
                        .ntfy
                        .as_ref()
                        .is_some_and(|value| !value.topic.trim().is_empty()),
                has_proxy: state
                    .config
                    .ffmpeg_proxy
                    .as_ref()
                    .is_some_and(|value| !value.trim().is_empty()),
                has_cookies: state
                    .config
                    .cookies
                    .as_ref()
                    .is_some_and(|value| !value.trim().is_empty()),
            },
            logs: state.logs.iter().cloned().collect(),
        }
    }

    async fn current_config(&self) -> Config {
        self.inner.lock().await.config.clone()
    }

    async fn save_config(&self, config: Config) -> AppResult<()> {
        save_config(&self.config_path, &config)?;
        let source_room = source_room_for_config(&config);
        let mut state = self.inner.lock().await;
        state.source_platform = config.platform.clone();
        state.source_room = source_room;
        state.config = config;
        state.config_loaded = true;
        state.config_error = None;
        state.last_error = None;
        state.worker_state = "配置已保存，等待下一轮检查".to_string();
        drop(state);
        self.log("info", "配置已保存到 config.yaml").await;
        self.notify.notify_waiters();
        Ok(())
    }

    async fn reload_config(&self) -> AppResult<()> {
        let config = load_config(&self.config_path)?;
        let source_room = source_room_for_config(&config);
        let mut state = self.inner.lock().await;
        state.source_platform = config.platform.clone();
        state.source_room = source_room;
        state.config = config;
        state.config_loaded = true;
        state.config_error = None;
        state.last_error = None;
        state.worker_state = "已从磁盘重载配置".to_string();
        drop(state);
        self.log("info", "已从磁盘重载 config.yaml").await;
        self.notify.notify_waiters();
        Ok(())
    }

    async fn set_enabled(&self, enabled: bool) {
        let mut state = self.inner.lock().await;
        state.enabled = enabled;
        state.worker_state = if enabled {
            "监控已启用".to_string()
        } else {
            "监控已停止".to_string()
        };
        drop(state);
        self.log(
            "info",
            if enabled {
                "已启用监控循环"
            } else {
                "已停止监控循环"
            },
        )
        .await;
        self.notify.notify_waiters();
    }

    async fn is_enabled(&self) -> bool {
        self.inner.lock().await.enabled
    }

    async fn config_for_worker(&self) -> Option<Config> {
        let state = self.inner.lock().await;
        if state.config_loaded {
            Some(state.config.clone())
        } else {
            None
        }
    }

    async fn current_interval(&self) -> u64 {
        self.inner.lock().await.config.interval.max(5)
    }

    async fn set_worker_state(&self, worker_state: impl Into<String>) {
        self.inner.lock().await.worker_state = worker_state.into();
    }

    async fn set_checked_now(&self) {
        self.inner.lock().await.last_checked_at = Some(now_ts());
    }

    async fn set_source_identity(&self, platform: &str, room: &str) {
        let mut state = self.inner.lock().await;
        state.source_platform = platform.to_string();
        state.source_room = room.to_string();
    }

    async fn set_source_live(&self, value: Option<bool>) {
        self.inner.lock().await.source_live = value;
    }

    async fn set_bilibili_live(&self, value: Option<bool>) {
        self.inner.lock().await.bilibili_live = value;
    }

    async fn set_ffmpeg_running(&self, value: bool) {
        self.inner.lock().await.ffmpeg_running = value;
    }

    async fn should_send_live_notification(&self, source_live: bool) -> bool {
        let mut state = self.inner.lock().await;
        let should_send = source_live && !state.last_notified_live;
        state.last_notified_live = source_live;
        should_send
    }

    async fn set_error(&self, error: impl Into<String>) {
        let message = error.into();
        let mut state = self.inner.lock().await;
        state.last_error = Some(message.clone());
        state.last_event = Some(message.clone());
        drop(state);
        self.log("error", message).await;
    }

    async fn log(&self, level: &str, message: impl Into<String>) {
        let message = message.into();
        match level {
            "error" => tracing::error!("{}", message),
            "warn" => tracing::warn!("{}", message),
            _ => tracing::info!("{}", message),
        }

        let mut state = self.inner.lock().await;
        state.last_event = Some(message.clone());
        if level != "error" {
            state.last_error = None;
        }
        state.logs.push_front(LogEntry {
            timestamp: now_ts(),
            level: level.to_string(),
            message,
        });
        while state.logs.len() > LOG_LIMIT {
            state.logs.pop_back();
        }
    }

    async fn wait_or_signal(&self, duration: Duration) {
        tokio::select! {
            _ = tokio::time::sleep(duration) => {}
            _ = self.notify.notified() => {}
        }
    }
}

pub async fn run() -> AppResult<()> {
    let config_path = PathBuf::from("./config.yaml");
    let ensure_result = ensure_config_exists(&config_path)?;
    let startup_config = load_config(&config_path).ok();
    let web_console = startup_config
        .as_ref()
        .and_then(|config| config.web_console.as_ref())
        .cloned()
        .unwrap_or_default();
    let web_addr = web_console.addr.clone();
    let socket_addr: SocketAddr = web_addr.parse()?;
    let auth = Arc::new(AuthConfig::from_config(Some(&web_console)));
    let app_state = AppState::new(config_path, web_addr.clone(), auth.enabled());
    app_state.load_initial_config().await;

    let worker_app = app_state.clone();
    tokio::spawn(async move {
        worker_loop(worker_app).await;
    });

    let router_state = RouterState {
        app: app_state.clone(),
        auth,
    };

    let static_service = ServeDir::new(WEB_DIST_DIR)
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new(frontend_index_path()));

    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/config", get(get_config).post(save_config_handler))
        .route("/api/control/start", post(start_monitor))
        .route("/api/control/stop", post(stop_monitor))
        .route("/api/control/reload", post(reload_config_handler))
        .route("/api/health", get(health))
        .fallback_service(static_service)
        .layer(middleware::from_fn_with_state(
            router_state.clone(),
            require_auth,
        ))
        .with_state(router_state);

    let listener = TcpListener::bind(socket_addr).await?;
    if ensure_result.created {
        app_state
            .log("warn", "未检测到 config.yaml，已自动初始化默认配置模板")
            .await;
    }
    if ensure_result.generated_web_password {
        app_state
            .log(
                "warn",
                "检测到 WebConsole 监听公网地址，已自动生成随机密码并写入 config.yaml",
            )
            .await;
    }
    app_state
        .log(
            "info",
            if app_state.auth_enabled {
                format!("控制台已启动，监听 {}，已启用 Basic Auth", web_addr)
            } else {
                format!("控制台已启动，监听 {}，当前未启用鉴权", web_addr)
            },
        )
        .await;
    if !Path::new(WEB_DIST_DIR).exists() {
        app_state
            .log(
                "warn",
                "æœªæ£€æµ‹åˆ° web/out å‰ç«¯æž„å»ºäº§ç‰©ï¼Œè¯·å…ˆåœ¨ web ç›®å½•æ‰§è¡Œ pnpm install å’Œ pnpm run build",
            )
            .await;
    }

    axum::serve(listener, app).await?;
    Ok(())
}

async fn worker_loop(app: Arc<AppState>) {
    app.log("info", "后台监控 worker 已启动").await;
    loop {
        if !app.is_enabled().await {
            app.set_worker_state("监控已暂停，等待手动启动").await;
            if let Some(config) = app.config_for_worker().await {
                maybe_stop_bilibili_when_disabled(app.clone(), &config).await;
            }
            app.wait_or_signal(Duration::from_secs(1)).await;
            continue;
        }

        let Some(config) = app.config_for_worker().await else {
            app.set_worker_state("尚未加载有效配置").await;
            app.wait_or_signal(Duration::from_secs(2)).await;
            continue;
        };

        if let Err(error) = monitor_cycle(app.clone(), config).await {
            app.set_error(error.to_string()).await;
        }

        let interval = app.current_interval().await;
        app.wait_or_signal(Duration::from_secs(interval)).await;
    }
}

async fn monitor_cycle(app: Arc<AppState>, config: Config) -> AppResult<()> {
    app.set_checked_now().await;
    app.set_source_identity(&config.platform, &source_room_for_config(&config))
        .await;
    app.set_worker_state("正在检查源站状态").await;

    let mut live = select_live(config.clone()).await?;
    let room = live.room().to_string().replace('"', "");
    app.set_source_identity(&config.platform, &room).await;

    let source_live = live.get_status().await.unwrap_or(false);
    app.set_source_live(Some(source_live)).await;

    if source_live {
        app.log("info", format!("{} 正在直播", room)).await;

        if app.should_send_live_notification(true).await {
            send_live_notification(&config, &format!("{} 开始直播", room), "bilistream").await;
        }

        let bili_live = get_bili_live_state(config.bililive.room).await.unwrap_or(false);
        app.set_bilibili_live(Some(bili_live)).await;

        if !bili_live {
            app.log("info", "B站未开播，准备自动开播").await;
            bili_start_live(&config).await?;
            app.set_bilibili_live(Some(true)).await;
            app.log("info", "B站已自动开播").await;
        }

        run_ffmpeg_session(app.clone(), &mut live, &config).await?;
    } else {
        app.should_send_live_notification(false).await;
        app.log("info", format!("{} 当前未直播", room)).await;

        let bili_live = get_bili_live_state(config.bililive.room).await.unwrap_or(false);
        app.set_bilibili_live(Some(bili_live)).await;
        if bili_live {
            app.log("info", "源站离线，准备关闭 B站直播").await;
            bili_stop_live(&config).await?;
            app.set_bilibili_live(Some(false)).await;
            app.log("info", "B站已关播").await;
        }
    }

    Ok(())
}

async fn run_ffmpeg_session(
    app: Arc<AppState>,
    live: &mut Box<dyn Live + Send>,
    config: &Config,
) -> AppResult<()> {
    loop {
        if !app.is_enabled().await {
            app.set_ffmpeg_running(false).await;
            return Ok(());
        }

        let source_live = live.get_status().await.unwrap_or(false);
        app.set_source_live(Some(source_live)).await;
        if !source_live {
            app.log("warn", "检测到源站已离线，结束推流").await;
            app.set_ffmpeg_running(false).await;
            return Ok(());
        }

        let m3u8_url = live.get_real_m3u8_url().await?;
        let mut command = build_ffmpeg_command(config, &m3u8_url);
        let room = live.room().to_string().replace('"', "");
        app.log("info", format!("启动 FFmpeg 推流: {}", room)).await;

        let mut child = command.spawn()?;
        app.set_ffmpeg_running(true).await;

        let mut should_stop = false;
        loop {
            tokio::select! {
                status = child.wait() => {
                    let status = status?;
                    app.set_ffmpeg_running(false).await;
                    if status.success() {
                        app.log("info", "FFmpeg 正常退出").await;
                    } else {
                        app.log("warn", format!("FFmpeg 异常退出: {:?}", status.code())).await;
                    }
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    if !app.is_enabled().await {
                        should_stop = true;
                        break;
                    }

                    let still_live = live.get_status().await.unwrap_or(false);
                    app.set_source_live(Some(still_live)).await;
                    if !still_live {
                        break;
                    }
                }
                _ = app.notify.notified() => {
                    if !app.is_enabled().await {
                        should_stop = true;
                        break;
                    }
                }
            }
        }

        if should_stop {
            let _ = child.start_kill();
            let _ = child.wait().await;
            app.set_ffmpeg_running(false).await;
            app.log("info", "收到停止指令，FFmpeg 已中断").await;
            return Ok(());
        }

        let source_live = live.get_status().await.unwrap_or(false);
        app.set_source_live(Some(source_live)).await;
        if !source_live {
            let _ = child.start_kill();
            let _ = child.wait().await;
            app.set_ffmpeg_running(false).await;
            app.log("warn", "源站已离线，FFmpeg 已停止").await;
            return Ok(());
        }

        app.log("warn", "准备重新拉起 FFmpeg").await;
        app.wait_or_signal(Duration::from_secs(2)).await;
    }
}

async fn maybe_stop_bilibili_when_disabled(app: Arc<AppState>, config: &Config) {
    if let Some(true) = app.snapshot().await.bilibili_live {
        if let Err(error) = bili_stop_live(config).await {
            app.set_error(format!("停止 B站直播失败: {}", error)).await;
        } else {
            app.set_bilibili_live(Some(false)).await;
            app.log("info", "监控停用后，已关闭 B站直播").await;
        }
    }
}

fn build_ffmpeg_command(config: &Config, m3u8_url: &str) -> Command {
    let output = format!(
        "{}{}",
        config.bililive.bili_rtmp_url, config.bililive.bili_rtmp_key
    );
    let mut command = Command::new("ffmpeg");
    command.kill_on_drop(true);
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    if let Some(proxy) = &config.ffmpeg_proxy {
        if !proxy.is_empty() {
            command.arg("-http_proxy").arg(proxy);
        }
    }
    command
        .arg("-re")
        .arg("-i")
        .arg(m3u8_url)
        .arg("-vcodec")
        .arg("copy")
        .arg("-acodec")
        .arg("aac")
        .arg("-f")
        .arg("flv")
        .arg(output);
    command
}

async fn get_bili_live_state(room: i32) -> AppResult<bool> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(30))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let response: serde_json::Value = client
        .get(format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&platform=web", room))
        .send()
        .await?
        .json()
        .await?;
    Ok(response["data"]["live_status"] != 0)
}

async fn bili_start_live(config: &Config) -> AppResult<()> {
    bilibili_live_action(config, "startLive").await
}

async fn bili_stop_live(config: &Config) -> AppResult<()> {
    bilibili_live_action(config, "stopLive").await
}

async fn bilibili_live_action(config: &Config, action: &str) -> AppResult<()> {
    let cookie = format!(
        "SESSDATA={};bili_jct={};DedeUserID={};DedeUserID__ckMd5={}",
        config.bililive.sessdata,
        config.bililive.bili_jct,
        config.bililive.dede_user_id,
        config.bililive.dede_user_id_ckmd5
    );
    let url = "https://api.live.bilibili.com/".parse::<Url>()?;
    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let raw_client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .timeout(Duration::from_secs(30))
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let endpoint = format!("https://api.live.bilibili.com/room/v1/Room/{}", action);
    client
        .post(endpoint)
        .header("Accept", "application/json, text/plain, */*")
        .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
        .body(format!(
            "room_id={}&platform=pc&area_v2=433&csrf_token={}&csrf={}",
            config.bililive.room, config.bililive.bili_jct, config.bililive.bili_jct
        ))
        .send()
        .await?;
    Ok(())
}

async fn require_auth(
    State(state): State<RouterState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if !state.auth.enabled() {
        return next.run(request).await;
    }

    if let Some(authorization) = request.headers().get(header::AUTHORIZATION) {
        if let Ok(authorization) = authorization.to_str() {
            if let Some(encoded) = authorization.strip_prefix("Basic ") {
                let decoded = base64::engine::general_purpose::STANDARD.decode(encoded);
                if let Ok(decoded) = decoded {
                    if let Ok(decoded) = String::from_utf8(decoded) {
                        if let Some((username, password)) = decoded.split_once(':') {
                            if username == state.auth.username
                                && state.auth.password.as_deref() == Some(password)
                            {
                                return next.run(request).await;
                            }
                        }
                    }
                }
            }
        }
    }

    (
        StatusCode::UNAUTHORIZED,
        [
            (header::WWW_AUTHENTICATE, "Basic realm=\"bilistream\""),
            (header::CACHE_CONTROL, "no-store"),
        ],
        "Unauthorized",
    )
        .into_response()
}

async fn get_status(State(state): State<RouterState>) -> Json<StatusSnapshot> {
    Json(state.app.snapshot().await)
}

async fn get_config(State(state): State<RouterState>) -> Json<Config> {
    Json(state.app.current_config().await)
}

async fn save_config_handler(
    State(state): State<RouterState>,
    Json(config): Json<Config>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state
        .app
        .save_config(config)
        .await
        .map_err(internal_error)?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn start_monitor(
    State(state): State<RouterState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state.app.set_enabled(true).await;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn stop_monitor(
    State(state): State<RouterState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state.app.set_enabled(false).await;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn reload_config_handler(
    State(state): State<RouterState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state.app.reload_config().await.map_err(internal_error)?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}

fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

fn frontend_index_path() -> PathBuf {
    Path::new(WEB_DIST_DIR).join("index.html")
}

fn source_room_for_config(config: &Config) -> String {
    match config.platform.as_str() {
        "Youtube" => config.youtube.room.clone(),
        "YoutubePreviewLive" => config.youtube_preview_live.channel_id.clone(),
        _ => config.twitch.room.clone(),
    }
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
