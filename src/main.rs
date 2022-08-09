use std::error::Error;
use std::{fmt, fs};
use std::fmt::Formatter;
use std::io::Read;
use std::net::Ipv4Addr;
use std::ops::Deref;
use std::str::{FromStr, Split};
use std::time::Duration;
use anyhow::Context;

type Err = anyhow::Error;

#[derive(Debug, Clone)]
pub struct Proxy {
    pub proxy_type: ProxyType,
    pub ip: Ipv4Addr,
    pub port: u16,
}


#[derive(Debug, Clone)]
pub enum ProxyType {
    socks5,
    socks4,
    https,
    http,
    none
}

impl FromStr for Proxy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let schema = match split.nth(0) {
            Some("http") => ProxyType::http,
            Some("https") => ProxyType::https,
            Some("socks4") => ProxyType::socks4,
            Some("socks5") => ProxyType::socks5,
            _ => ProxyType::none
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
            ProxyType::socks5 => write!(f, "socks5"),
            ProxyType::socks4 => write!(f, "socks4"),
            ProxyType::http => write!(f, "http"),
            ProxyType::https => write!(f, "https"),
            ProxyType::none => write!(f, "https")
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

}

//fn parse_proxy()

//Returns timeout
fn check_proxy(proxy: &Proxy, timeout: u32) -> u32 {



    0
}