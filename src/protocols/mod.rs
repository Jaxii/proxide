use async_trait::async_trait;
use crate::common::{errors::ProxyError, types::{ProxyConfig, ProxyType}};

mod http;
// mod socks;
// mod ssl;
// mod imap;
// mod smtp;
// mod pop3;
// mod transparent;

pub use http::HttpProxyChecker;
// pub use socks::{Socks4ProxyChecker, Socks5ProxyChecker};
// pub use ssl::SslProxyChecker;
// pub use imap::ImapProxyChecker;
// pub use smtp::SmtpProxyChecker;
// pub use pop3::Pop3ProxyChecker;
// pub use transparent::TransparentProxyChecker;

#[async_trait]
pub trait ProxyChecker {
    fn supports_type(&self, proxy_type: ProxyType) -> bool;
    async fn check(&self, proxy: &ProxyConfig) -> Result<(), ProxyError>;
}