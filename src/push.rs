use crate::config::GotifyConfig;
use gotify::Client as GotifyClient;
use tracing;

pub async fn send_gotify_notification(config: &GotifyConfig, message: &str, title: &str) {
    match GotifyClient::new(config.url.as_str(), &config.token) {
        Ok(client) => match client.create_message(message).with_title(title).await {
            Ok(_) => tracing::info!("Gotify通知发送成功"),
            Err(e) => tracing::error!("Gotify通知发送失败: {}", e),
        },
        Err(e) => tracing::error!("Gotify客户端创建失败: {}", e),
    }
}