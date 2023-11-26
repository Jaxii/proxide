#![feature(iterator_try_collect)]

use anyhow::Context;
use nanorand::{Rng, WyRand};
use std::error::Error;
use std::fmt::Formatter;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::{fmt, fs};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::runtime;

type Err = anyhow::Error;

mod proxy;

#[tokio::main]
async fn main() {
    let ip = generate_random_ip();
    check_proxy(&ip, 5, "https://google.com").await;
}

async fn check_proxy(p: &Ipv4Addr, timeout: u8, target: &str) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("https://".to_owned() + &*p.to_string())?;
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
    loop {
        let ip = Ipv4Addr::new(randu8(), randu8(), randu8(), randu8());

        if !ip.is_loopback() {
            return ip;
        }
    }
}

fn randu8() -> u8 {
    let mut rng = WyRand::new();
    rng.generate::<u8>()
}
