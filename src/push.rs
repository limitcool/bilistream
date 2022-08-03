use std::time::Duration;

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct Mirai {
    host: String,
    target: String,
}

impl Mirai {
    pub fn new(host: String, target: String) -> Self {
        Mirai { host, target }
    }

    pub async fn send_message(&self, author: String, bili_room: String) {
        let message = format!("开始直播了!🔔\n➡️: https://live.bilibili.com/{}", bili_room);
        let j = json!(
            {
                "target":self.target,
                "messageChain":[
                  { "type":"Plain", "text":format!("{}:{}", author, message) }
                ]
              }
        );
        let url = format!("https://{}/sendGroupMessage", self.host);
        // 设置最大重试次数为4294967295次
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
        let raw_client = reqwest::Client::builder()
            .cookie_store(true)
            // 设置超时时间为30秒
            .timeout(Duration::new(5, 0))
            .build()
            .unwrap();
        let client = ClientBuilder::new(raw_client.clone())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        let text = client
            .post(url)
            .json(&j)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{}", text);
    }
}

// pub async fn mirai(target: i32, author: String) {
//     let j = json!(
//         {
//             "target":target,
//             "messageChain":[
//               { "type":"Plain", "text":format!("{}", author + "开始直播了!🔔\n➡️: https://live.bilibili.com/room") }
//             ]
//           }
//     );
//     // 设置最大重试次数为4294967295次
//     let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
//     let raw_client = reqwest::Client::builder()
//         .cookie_store(true)
//         // 设置超时时间为30秒
//         .timeout(Duration::new(5, 0))
//         .build()
//         .unwrap();
//     let client = ClientBuilder::new(raw_client.clone())
//         .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//         .build();
//     let text = client
//         .post("http://127.0.0.1:8080/sendGroupMessage")
//         .json(&j)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     println!("{}", text);
// }
// // test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }
//     #[test]
//     fn push() {
//         aw!(mirai(group, "test".to_string()));
//     }
// }
