use std::time::Duration;

use regex::Regex;
use reqwest::{Client};
use async_trait::async_trait;
use anyhow::{Result};
use crate::Proxy;
use lazy_static::lazy_static;
use fake_useragent::UserAgents;

lazy_static! {
    static ref RE_OF_66: Regex = Regex::new(r"<td>(?P<ipv4addr>\d+\.\d+\.\d+\.\d+)</td><td>(?P<port>\d+)</td>").unwrap();
    static ref RE_OF_HAPPY: Regex = Regex::new(r"<td>(?P<ipv4addr>\d+\.\d+\.\d+\.\d+)</td>\s+<td>(?P<port>\d+)</td>\s+<td>\w+</td>\s+<td>(?P<protocol>\w+)</td>").unwrap();
    static ref RE_OF_QUICK: Regex = Regex::new(r"<td[\s\S]+>(?P<ipv4addr>\d+\.\d+\.\d+\.\d+)</td>\s+<td[\s\S]+>(?P<port>\d+)</td>\s+<td[\s\S]+>\w+</td>\s+<td[\s\S]+>(?P<protocol>\w+)</td>").unwrap();
}

#[async_trait]
pub trait Fetch: Send + Sync {
    async fn fetch(&self, page: Option<usize>) -> Result<Vec<Proxy>>;
}

// 66代理
pub struct FetcherOf66;
impl FetcherOf66 {
    pub fn new() -> Self {
       Self
    }
}

#[async_trait]
impl Fetch for FetcherOf66 {

    async fn fetch(&self, page: Option<usize>) -> Result<Vec<Proxy>> {
        let user_agent = UserAgents::new();
        let url = format!("http://www.66ip.cn/{:?}.html", page.unwrap_or(1));
        let client = Client::builder()
            .user_agent(user_agent.random())
            .build()?;
        let body = client
            .get(url)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Accept-Language","zh-CN,zh;q=0.8")
            .send()
            .await?
            .text()
            .await?;
        let result: Vec<Proxy> = RE_OF_66.captures_iter(&body).filter_map(|cap|{
            let groups = (cap.name("ipv4addr"), cap.name("port"));
            match groups {
                (Some(ipv4addr), Some(port)) =>
                         Some(Proxy::new("http".to_string(), 
                            ipv4addr.as_str().to_string(), 
                            port.as_str().parse::<u16>().unwrap())),
                _ => None
            }
        }).collect();
        Ok(result)
    }
}

pub struct FetcherOfHappy;

impl FetcherOfHappy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Fetch for FetcherOfHappy {
    async fn fetch(&self, page: Option<usize>) -> Result<Vec<Proxy>> {
        let user_agent = UserAgents::new();
        let url_list = [
            format!("http://www.kxdaili.com/dailiip/1/{}.html", page.unwrap_or(1)),
            format!("http://www.kxdaili.com/dailiip/2/{}.html", page.unwrap_or(1)),
        ];
        let mut res = Vec::new();
        for url in url_list {
            let client = Client::builder()
            .user_agent(user_agent.random())
            .build()?;
            let body = client
                .get(url)
                .header("Content-Type", "text/html; charset=utf-8")
                .header("Accept-Language","zh-CN,zh;q=0.8")
                .send()
                .await?
                .text()
                .await?;
            let result: Vec<Proxy> = RE_OF_HAPPY.captures_iter(&body).filter_map(|cap|{
                let groups = (cap.name("ipv4addr"), cap.name("port"), cap.name("protocol"));
                match groups {
                    (Some(ipv4addr), Some(port), Some(protocol)) =>
                        Some(Proxy::new(protocol.as_str().to_string(), 
                                ipv4addr.as_str().to_string(), 
                                port.as_str().parse::<u16>().unwrap())),
                        _ => None
                }
            }).collect();
            res.extend(result);
        }
        Ok(res)
    }
}

pub struct FetcherOfQuick;

impl FetcherOfQuick {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Fetch for FetcherOfQuick {
    async fn fetch(&self, page: Option<usize>) -> Result<Vec<Proxy>> {
        let user_agent = UserAgents::new();
        let url_list = [
            format!("https://free.kuaidaili.com/free/inha/{}/", page.unwrap_or(1)),
            format!("https://free.kuaidaili.com/free/intr/{}/", page.unwrap_or(1)),
        ];
        let mut res = Vec::new();
        for url in url_list {
            let client = Client::builder()
            .user_agent(user_agent.random())
            .build()?;
            let body = client
                .get(url)
                .header("Content-Type", "text/html; charset=utf-8")
                .send()
                .await?
                .text()
                .await?;
            println!("{:?}", body);
            let result: Vec<Proxy> = RE_OF_HAPPY.captures_iter(&body).filter_map(|cap|{
                let groups = (cap.name("ipv4addr"), cap.name("port"), cap.name("protocol"));
                match groups {
                    (Some(ipv4addr), Some(port), Some(protocol)) =>
                        Some(Proxy::new(protocol.as_str().to_string(), 
                                ipv4addr.as_str().to_string(), 
                                port.as_str().parse::<u16>().unwrap())),
                        _ => None
                }
            }).collect();
            res.extend(result);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(res)
    }
 }


pub struct ProxyBuilder;
impl ProxyBuilder {
    pub fn build() -> Vec<Box<dyn Fetch>> {
        vec![
            Box::new(FetcherOf66::new()),
            Box::new(FetcherOfHappy::new()),
        ]
    }
}


#[cfg(test)]
mod tests {
    use crate::{FetcherOf66, Fetch, FetcherOfHappy, FetcherOfQuick};

    #[tokio::test]
    async fn test_fetch_66() {
        let fetcher = FetcherOf66::new();
        let res = fetcher.fetch(Some(1)).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_fetch_happy() {
        let fetcher = FetcherOfHappy::new();
        let res = fetcher.fetch(Some(2)).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_fetch_quick() {
        let fetcher = FetcherOfQuick::new();
        let res = fetcher.fetch(Some(3)).await;
        println!("{:?}", res);
    }

}
