mod config;
mod dashboard;
mod error;
mod plugins;
mod push;

use dashboard::run;
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    if let Err(error) = run().await {
        tracing::error!("bilistream 启动失败: {}", error);
        std::process::exit(1);
    }
}
