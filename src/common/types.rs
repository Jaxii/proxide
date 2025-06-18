use std::net::{IpAddr, SocketAddr};
use std::time::Duration;


/// Supported proxy types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyType {
    Http,
    Https,
    Socks4,
    Socks5,
    Transparent,
    Ssl,
    Imap,
    Smtp,
    Pop3,
}

/// Proxy connection details
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub address: SocketAddr,
    pub proxy_type: ProxyType,
    pub username: Option<String>,
    pub password: Option<String>,
    pub timeout: Duration,
}

impl ProxyConfig {
    pub fn new(address: SocketAddr, proxy_type: ProxyType) -> Self {
        Self {
            address,
            proxy_type,
            username: None,
            password: None,
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_auth(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
