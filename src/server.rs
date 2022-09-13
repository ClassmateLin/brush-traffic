use std::{sync::Arc, time::Duration};
use futures::future::join_all;
use async_channel::Sender;
use crate::{proxy::Proxy, ProxyBuilder};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProxyServer {
    sender: Arc<Sender<Proxy>>,
}


impl ProxyServer {
    pub fn new(sender: Arc<Sender<Proxy>>) -> Self {
        Self { sender }
    }

    pub async fn run(&self) -> Result<()> {
        let fetcher_list = ProxyBuilder::build();
        let mut handler_list = Vec::new();
        for fetcher in fetcher_list {
            let sender = self.sender.clone();
            let handler = tokio::spawn(async move {
                let mut num = 1;
                while num <= 5 {
                    if let Ok(result) = fetcher.fetch(Some(num)).await {
                        for item in result.into_iter() {
                            let _ = sender.send(item).await;
                            println!("add proxy:{}.", item.into_string());
                        }
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                    num += 1;
                }
            });
            handler_list.push(handler);
        }
        join_all(handler_list).await;
        self.sender.close();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_channel::bounded;

    use crate::Proxy;

    use super::ProxyServer;

    #[tokio::test]
    async fn test_proxy_server() {
        let (sender, receiver) = bounded::<Proxy>(1024);
        let sender = Arc::new(sender);
        let server = ProxyServer::new(sender);
        let _ = server.run().await;
        println!("{:?}", receiver.recv().await);
    }
}