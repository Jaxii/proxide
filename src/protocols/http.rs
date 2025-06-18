use async_trait::async_trait;
use reqwest::{Client, Proxy};
use std::time::Duration;

use crate::common::{
    errors::ProxyError,
    types::{ProxyConfig, ProxyType},
};
use crate::protocols::ProxyChecker;

pub struct HttpProxyChecker {
    verify_url: String,
}

impl HttpProxyChecker {
    pub fn new(verify_url: String) -> Self {
        Self { verify_url }
    }
}

#[async_trait]
impl ProxyChecker for HttpProxyChecker {
    fn supports_type(&self, proxy_type: ProxyType) -> bool {
        matches!(proxy_type, ProxyType::Http | ProxyType::Https)
    }

    async fn check(&self, proxy: &ProxyConfig) -> Result<(), ProxyError> {

        println!("Calling check");

        let proxy_url = match proxy.proxy_type {
            ProxyType::Http => format!("http://{}", proxy.address),
            ProxyType::Https => format!("https://{}", proxy.address),
            _ => return Err(ProxyError::UnsupportedProxyType),
        };

        let mut client_builder = Client::builder()
            .timeout(proxy.timeout)
            .proxy(Proxy::all(&proxy_url).map_err(|e| ProxyError::ConnectionFailed(e))?);

        // Add authentication if provided
        if let (Some(username), Some(password)) = (&proxy.username, &proxy.password) {
            client_builder = client_builder.proxy(
                Proxy::all(&proxy_url)
                    .map_err(|e| ProxyError::ConnectionFailed(e))?
                    .basic_auth(username, password),
            );
        }

        let client = client_builder
            .build()
            .map_err(|e| ProxyError::ConnectionFailed(e.into()))?;

        println!("Verify URL: {}", self.verify_url);

        // Try to make a request through the proxy
        let response = client
            .get(&self.verify_url)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ProxyError::Timeout
                } else if e.is_connect() {
                    ProxyError::ConnectionFailed(e)
                } else {
                    ProxyError::ProtocolError(e.to_string())
                }
            })?;


            println!("Did response end?");

            println!("Response: {}", response.status());

        if response.status().is_success() {

            println!("Response successful with: {}", proxy.address);

            Ok(())

        } else if response.status().as_u16() == 407 {
            Err(ProxyError::AuthenticationFailed)
        } else {


            println!("Response failed with IP: {}", proxy.address);
            Err(ProxyError::InvalidResponse(format!(
                "Unexpected status code: {}",
                response.status()
            )))
        }
    }
}