use std::time::Duration;

use anyhow::Result;

use crate::Proxy;
use reqwest::{Client};

use fake_useragent::UserAgents;

pub async fn visit(url: String, proxy: Proxy) -> Result<()> {
    let user_agent = UserAgents::new();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(user_agent.random())
        .proxy(reqwest::Proxy::https(proxy.into_string())?)
        .build()?;
    let status = client.get(&url)
        .send().await?
        .status();
    println!("request: {url}, status: {status}");
    Ok(())
}
