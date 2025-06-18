use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

use crate::common::{
    config::ScannerConfig,
    errors::ProxyError,
    types::{ProxyConfig, ProxyType},
};
use crate::protocols::{HttpProxyChecker, ProxyChecker};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub address: SocketAddr,
    pub proxy_type: ProxyType,
    pub is_working: bool,
    pub latency: Option<Duration>,
    pub error: Option<String>,
    pub anonymity_level: Option<AnonymityLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnonymityLevel {
    Transparent,
    Anonymous,
    Elite,
}

pub struct Scanner {
    config: ScannerConfig,
    checkers: Vec<Box<dyn ProxyChecker + Send + Sync>>,
}

pub struct ScannerBuilder {
    config: ScannerConfig,
}

impl ScannerBuilder {
    pub fn new() -> Self {
        Self {
            config: ScannerConfig::default(),
        }
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn with_retry_count(mut self, retry_count: usize) -> Self {
        self.config.retry_count = retry_count;
        self
    }

    pub fn check_all_types(mut self, check_all: bool) -> Self {
        self.config.check_all_types = check_all;
        self
    }

    pub fn with_preferred_types(mut self, types: Vec<ProxyType>) -> Self {
        self.config.preferred_types = types;
        self
    }

    pub fn with_verify_url(mut self, url: Option<String>) -> Self {
        self.config.verify_url = url;
        self
    }

    pub fn build(self) -> Scanner {
        Scanner::new(self.config)
    }
}

impl Scanner {
    pub fn new(config: ScannerConfig) -> Self {
        // Initialize all the proxy checkers
        let mut checkers: Vec<Box<dyn ProxyChecker + Send + Sync>> = Vec::new();

        let verify_url = config
            .verify_url
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "http://google.com".to_string());

        // Always add HTTP checker if we're checking HTTP/HTTPS types
        if config.check_all_types
            || config.preferred_types.contains(&ProxyType::Http)
            || config.preferred_types.contains(&ProxyType::Https)
        //    ||
        //    config.preferred_types.is_empty() // Default to checking all if none specified`
        {
            let http_checker = HttpProxyChecker::new(verify_url.clone());
            checkers.push(Box::new(http_checker));
        }

        Self { config, checkers }
    }

    pub fn builder() -> ScannerBuilder {
        ScannerBuilder::new()
    }

    pub async fn scan_proxy(&self, proxy: ProxyConfig) -> ScanResult {
        // Find the appropriate checker for this proxy type
        let start = Instant::now();
        let result = self.check_proxy(&proxy).await;
        let latency = start.elapsed();

        match result {
            Ok(_) => ScanResult {
                address: proxy.address,
                proxy_type: proxy.proxy_type,
                is_working: true,
                latency: Some(latency),
                error: None,
                anonymity_level: Some(AnonymityLevel::Anonymous), // Would be determined by the actual check
            },
            Err(err) => ScanResult {
                address: proxy.address,
                proxy_type: proxy.proxy_type,
                is_working: false,
                latency: None,
                error: Some(err.to_string()),
                anonymity_level: None,
            },
        }
    }

    async fn check_proxy(&self, proxy: &ProxyConfig) -> Result<(), ProxyError> {
        // Find the appropriate checker and run the check

        println!("running Scanner.check_proxy");

        for checker in &self.checkers {
            if checker.supports_type(proxy.proxy_type) {
                return checker.check(proxy).await;
            }
        }

        Err(ProxyError::UnsupportedProxyType)
    }

    pub async fn scan_proxies(&self, proxies: Vec<ProxyConfig>) -> Vec<ScanResult> {
        use crate::scanner::engine::ScanEngine;

        let engine = ScanEngine::new(self.config.concurrency);
        engine.scan_all(self, proxies).await
    }
}
