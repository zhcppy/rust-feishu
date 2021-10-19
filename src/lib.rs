#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

use eyre::Result;
use hmac::{Hmac, Mac, NewMac};
use reqwest::header::CONTENT_TYPE;
use sha2::Sha256;

use crate::msg_type::{Card, Text};

pub mod msg_type;

fn get_feishu_key() -> &'static str {
    std::env::var("FEISHU_KEY").unwrap().as_str()
}

fn get_feishu_url() -> &'static str {
    std::env::var("FEISHU_URL").unwrap().as_str()
}

pub async fn send_text(msg: String) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1;

    let input = WebHookInput {
        timestamp: timestamp.to_string(),
        sign: gen_sign(get_feishu_key(), timestamp),
        msg_type: "text".to_string(),
        content: Some(Text::new(msg)),
        card: None,
    };
    match send(&input).await {
        Err(err) => {
            error!("send to feishu failed: {:?}", err)
        }
        _ => {}
    };
}

pub async fn send_card(card: Card) {
    let disable_feishu = std::env::var("DISABLED_NOTIFIER");
    if disable_feishu.is_ok() && disable_feishu.unwrap() == "true" {
        return;
    }
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1;
    let input = WebHookInput {
        timestamp: timestamp.to_string(),
        sign: gen_sign(get_feishu_key(), timestamp),
        msg_type: "interactive".to_string(),
        content: None,
        card: Some(card),
    };
    match send(&input).await {
        Err(err) => {
            error!("send to feishu failed: {:?}", err)
        }
        _ => {}
    };
}

pub async fn send(input: &WebHookInput) -> Result<WebHookOutput> {
    debug!("send to feishu talk input: {:?}", input);
    let response = reqwest::Client::new().post(get_feishu_url()).header(CONTENT_TYPE, "application/json").json(&input).send().await?;
    let http_code = response.status();
    let body = response.text().await?;
    debug!("send to feishu http status code: {:?}, body: {:?}", http_code, body);
    let out_put: WebHookOutput = serde_json::from_str(body.as_str())?;
    if out_put.status_code != 0 {
        return Err(eyre::Error::msg(out_put.status_message));
    }
    Ok(out_put)
}

fn gen_sign(secret: &str, timestamp: u64) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(format!("{}\n{}", timestamp, secret).as_bytes()).expect("HMAC can take key of any size");
    mac.update(b"");
    let result = mac.finalize();
    base64::encode(result.into_bytes())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebHookInput {
    timestamp: String,
    sign: String,
    msg_type: String,
    content: Option<Text>,
    card: Option<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebHookOutput {
    #[serde(rename = "StatusCode")]
    status_code: i64,

    #[serde(rename = "StatusMessage")]
    status_message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_text() {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        send_text("hello word".to_string()).await;
    }

    #[tokio::test]
    async fn test_send_card() {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        send_card(Card::new(
            "test".to_string(),
            vec!["**key1**: value1".to_string(), "**key1**: value1".to_string()],
            Some("https://www.google.com".to_string()),
        ))
        .await;
    }
}
