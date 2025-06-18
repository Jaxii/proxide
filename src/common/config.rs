use crate::common::types::ProxyType;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub concurrency: usize,
    pub timeout: Duration,
    pub retry_count: usize,
    pub check_all_types: bool,
    pub preferred_types: Vec<ProxyType>,
    pub verify_url: Option<String>,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            concurrency: 100,
            timeout: Duration::from_secs(10),
            retry_count: 2,
            check_all_types: false,
            preferred_types: vec![ProxyType::Http, ProxyType::Socks5],
            verify_url: Some(String::from("http://www.google.com")),
        }
    }
}
