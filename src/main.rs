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
            ProxyType::Socks5 => write!(f, "socks5"),
            ProxyType::Socks4 => write!(f, "socks4"),
            ProxyType::Http => write!(f, "http"),
            ProxyType::Https => write!(f, "https"),
            ProxyType::None => write!(f, "https"),
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

    // load_list("input.txt");
}
async fn check_proxy(p: Proxy, timeout: u8, target: &String) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all(p.proxy_type.to_string())?;
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

    Ok::<Vec<String>, Box<dyn Error>>(
        contents
            .as_str()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    )
    .unwrap()
}
