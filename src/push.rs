use crate::config::{Config, GotifyConfig, NtfyConfig};
use crate::error::AppResult;
use gotify::Client as GotifyClient;

pub async fn send_live_notification(config: &Config, message: &str, title: &str) {
    if !config.notification.enabled {
        tracing::info!("通知已关闭，跳过发送");
        return;
    }

    match config.notification.channel.as_str() {
        "gotify" => {
            if config.notification.gotify_enabled {
                if let Some(gotify) = &config.gotify {
                    send_gotify_notification(gotify, message, title).await;
                } else {
                    tracing::warn!("已选择 Gotify，但未配置 Gotify");
                }
            } else {
                tracing::info!("Gotify 通道已关闭，跳过发送");
            }
        }
        "ntfy" => {
            if config.notification.ntfy_enabled {
                if let Some(ntfy) = &config.ntfy {
                    if let Err(error) = send_ntfy_notification(ntfy, message, title).await {
                        tracing::error!("ntfy 通知发送失败: {}", error);
                    }
                } else {
                    tracing::warn!("已选择 ntfy，但未配置 ntfy");
                }
            } else {
                tracing::info!("ntfy 通道已关闭，跳过发送");
            }
        }
        "both" => {
            if config.notification.gotify_enabled {
                if let Some(gotify) = &config.gotify {
                    send_gotify_notification(gotify, message, title).await;
                }
            }
            if config.notification.ntfy_enabled {
                if let Some(ntfy) = &config.ntfy {
                    if let Err(error) = send_ntfy_notification(ntfy, message, title).await {
                        tracing::error!("ntfy 通知发送失败: {}", error);
                    }
                }
            }
        }
        other => {
            tracing::warn!("未知通知通道 `{}`，跳过发送", other);
        }
    }
}

pub async fn send_gotify_notification(config: &GotifyConfig, message: &str, title: &str) {
    match GotifyClient::new(config.url.as_str(), &config.token) {
        Ok(client) => match client.create_message(message).with_title(title).await {
            Ok(_) => tracing::info!("Gotify 通知发送成功"),
            Err(error) => tracing::error!("Gotify 通知发送失败: {}", error),
        },
        Err(error) => tracing::error!("Gotify 客户端初始化失败: {}", error),
    }
}

pub async fn send_ntfy_notification(
    config: &NtfyConfig,
    message: &str,
    title: &str,
) -> AppResult<()> {
    if config.topic.trim().is_empty() {
        tracing::warn!("ntfy Topic 为空，已跳过通知发送");
        return Ok(());
    }

    let base_url = config.url.trim_end_matches('/');
    let url = format!("{}/{}", base_url, config.topic.trim());
    let client = reqwest::Client::builder().build()?;
    let mut request = client
        .post(url)
        .header("Title", title)
        .body(message.to_string());

    if let Some(priority) = config.priority.as_ref().filter(|v| !v.trim().is_empty()) {
        request = request.header("Priority", priority);
    }

    if let Some(tags) = config.tags.as_ref().filter(|v| !v.trim().is_empty()) {
        request = request.header("Tags", tags);
    }

    if let Some(token) = config.token.as_ref().filter(|v| !v.trim().is_empty()) {
        request = request.bearer_auth(token);
    } else if let Some(username) = config
        .username
        .as_ref()
        .filter(|v| !v.trim().is_empty())
    {
        request = request.basic_auth(username, config.password.as_deref());
    }

    let response = request.send().await?;
    response.error_for_status()?;
    tracing::info!("ntfy 通知发送成功");
    Ok(())
}
