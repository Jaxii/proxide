#![feature(iterator_try_collect)]

use anyhow::Context;
use std::error::Error;
use std::fmt::Formatter;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::{fmt, fs};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use nanorand::{Rng, WyRand};
use tokio::runtime;
use tokio::runtime::Builder;

type Err = anyhow::Error;

#[derive(Debug, Clone)]
pub struct Proxy {
    pub proxy_type: ProxyType,
    pub ip: Ipv4Addr,
    pub port: u16,
}


#[derive(Debug, Clone)]
pub enum ProxyType {
    Socks5,
    Socks4,
    Https,
    Http,
    None,
}

impl FromStr for Proxy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let schema = match split.nth(0) {
            Some("http") => ProxyType::Http,
            Some("https") => ProxyType::Https,
            Some("socks4") => ProxyType::Socks4,
            Some("socks5") => ProxyType::Socks5,
            _ => ProxyType::None,
        };
        let ip: Ipv4Addr = split.next().context("invalid format")?.parse()?;
        let port: u16 = split.next().context("invalid format")?.parse()?;
        Ok(Proxy {
            proxy_type: schema,
            ip,
            port,
        })
    }
}

impl fmt::Display for ProxyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Socks5 => write!(f, "socks5"),
            Self::Socks4 => write!(f, "socks4"),
            Self::Http => write!(f, "http"),
            _ => write!(f, "https"),
        }
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}:{}", self.proxy_type, self.ip, self.port)
    }
}

#[tokio::main]
async fn main() {

    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    //do other work

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {

            let ip = generate_random_ip();
            handles.push(runtime.spawn( check_proxy(ip.clone(), 5, "https://google.com")));
    }

    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
runtime.spawn(handle).await.expect("test").expect("TODO: panic message").expect("TODO: panic message");
    }


}
async fn check_proxy(p: Ipv4Addr, timeout: u8, target: &str) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("https://".to_owned()+ &*p.to_string())?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .proxy(proxy)
        .build()?;
    client
        .get(target)
        .header("Accept", "text/plain")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:35.0) Gecko/20100101 Firefox/35.0",
        )
        .timeout(Duration::from_secs(timeout as u64))
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

async fn load_list(path: &str) -> Vec<String> {
    let mut f = fs::File::open(path).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read file");

    contents
        .as_str()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn generate_random_ip() -> Ipv4Addr {

    return Ipv4Addr::new(randu8(), randu8(), randu8(), randu8());

}

fn randu8() -> u8 {
    let mut rng = WyRand::new();
    rng.generate::<u8>()

}