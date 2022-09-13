use std::{sync::Arc, time::Duration};
use anyhow::{Result};
use async_channel::{bounded};
use brush_traffic::{Proxy, ProxyServer};
use brush_traffic::visit;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    #[clap(short, long, default_value_t=String::from("https://www.baidu.com"))]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse();
    let url = Arc::new(args.url);
    let (sender, receiver) = bounded::<Proxy>(1024);
    let sender = Arc::new(sender);
    let server = ProxyServer::new(sender);
    tokio::spawn(async move {
        let _ = server.run().await;
    });

    loop {
        tokio::select! {
            res = receiver.recv() => {
                if let Ok(proxy) = res {
                    let url = url.to_string();
                    let _ = visit(url, proxy).await;
                }else {
                    println!("No proxy ip found, exit.");
                    break;
                }
            }
            _   = tokio::time::sleep(Duration::from_secs(5)) => {}
        }
    }
    Ok(())
}
