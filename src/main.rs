// src/main.rs
use clap::{Arg, ArgMatches, Command};
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use proxide::{
    common::{config::ScannerConfig, types::{ProxyConfig, ProxyType}},
    scanner::Scanner,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Proxide")
        .version("1.0")
        .author("github.com/Jaxii")
        .about("Scans and validates proxies")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file with proxy list")
                .num_args(1),
        )
        .arg(
            Arg::new("concurrency")
                .short('c')
                .long("concurrency")
                .value_name("NUM")
                .help("Number of concurrent checks")
                .default_value("100")
                .num_args(1),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout for each check")
                .default_value("10")
                .num_args(1),
        )
        .arg(
            Arg::new("type")
                .short('p')
                .long("type")
                .value_name("TYPE")
                .help("Proxy type to check (http, socks5, etc.)")
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for results (json format)")
                .num_args(1),
        )
        .get_matches();

    // Configure scanner
    let scanner = build_scanner_from_args(&matches)?;

    // Load proxy list
    let proxies = load_proxies_from_args(&matches)?;

    // Scan the proxies
    let results = scanner.scan_proxies(proxies).await;

    // Print results
    report_results(&matches, &results)?;

    Ok(())
}

fn build_scanner_from_args(matches: &ArgMatches) -> Result<Scanner, Box<dyn std::error::Error>> {
    let concurrency = matches.get_one::<String>("concurrency")
        .unwrap_or(&"100".to_string())
        .parse::<usize>()?;

    let timeout = matches.get_one::<String>("timeout")
        .unwrap_or(&"10".to_string())
        .parse::<u64>()?;

    let mut builder = Scanner::builder()
        .with_concurrency(concurrency)
        .with_timeout(Duration::from_secs(timeout));

    // Configure proxy types if specified
    if let Some(types) = matches.get_many::<String>("type") {
        let proxy_types: Vec<ProxyType> = types
            .map(|t| match t.to_lowercase().as_str() {
                "http" => ProxyType::Http,
                "https" => ProxyType::Https,
                "socks4" => ProxyType::Socks4,
                "socks5" => ProxyType::Socks5,
                "transparent" => ProxyType::Transparent,
                "ssl" => ProxyType::Ssl,
                "imap" => ProxyType::Imap,
                "smtp" => ProxyType::Smtp,
                "pop3" => ProxyType::Pop3,
                _ => panic!("Unsupported proxy type: {}", t),
            })
            .collect();

        builder = builder.with_preferred_types(proxy_types);
    }

    Ok(builder.build())
}

fn load_proxies_from_args(matches: &ArgMatches) -> Result<Vec<ProxyConfig>, Box<dyn std::error::Error>> {
    let mut proxies = Vec::new();

    if let Some(input_file) = matches.get_one::<String>("input") {
        let content = std::fs::read_to_string(input_file)?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            // Parse proxy line (format: ip:port:type:username:password)
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 2 {
                eprintln!("Invalid proxy format: {}", line);
                continue;
            }

            let host = parts[0];
            let port = parts[1].parse::<u16>()?;
            let address = format!("{}:{}", host, port).parse::<SocketAddr>()?;

            let proxy_type = if parts.len() > 2 {
                match parts[2].to_lowercase().as_str() {
                    "http" => ProxyType::Http,
                    "https" => ProxyType::Https,
                    "socks4" => ProxyType::Socks4,
                    "socks5" => ProxyType::Socks5,
                    "transparent" => ProxyType::Transparent,
                    "ssl" => ProxyType::Ssl,
                    "imap" => ProxyType::Imap,
                    "smtp" => ProxyType::Smtp,
                    "pop3" => ProxyType::Pop3,
                    _ => {
                        eprintln!("Unknown proxy type: {}, defaulting to HTTP", parts[2]);
                        ProxyType::Http
                    }
                }
            } else {
                ProxyType::Http
            };

            let mut config = ProxyConfig::new(address, proxy_type);

            if parts.len() > 4 {
                config = config.with_auth(parts[3].to_string(), parts[4].to_string());
            }

            proxies.push(config);
        }
    } else {
        eprintln!("No input file provided. Using example proxies for demonstration.");
    }

    Ok(proxies)
}

fn report_results(
    matches: &ArgMatches,
    results: &[proxide::scanner::ScanResult],
) -> Result<(), Box<dyn std::error::Error>> {
    for result in results {
        println!("IP: {}:{}, Latency: {:#?}, Working: {}", result.address.ip(), result.address.port(), result.latency, result.is_working);
    }

    Ok(())
}