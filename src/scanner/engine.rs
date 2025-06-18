
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

use crate::common::types::ProxyConfig;
use crate::scanner::scanner::{ScanResult, Scanner};

pub struct ScanEngine {
    concurrency_limit: Arc<Semaphore>,
}

impl ScanEngine {
    pub fn new(concurrency: usize) -> Self {
        Self {
            concurrency_limit: Arc::new(Semaphore::new(concurrency)),
        }
    }

    pub async fn scan_all(
        &self,
        scanner: &Scanner,
        proxies: Vec<ProxyConfig>,
    ) -> Vec<ScanResult> {
        stream::iter(proxies)
            .map(|proxy| {
                let scanner = scanner.clone();
                let semaphore = Arc::clone(&self.concurrency_limit);

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    let result = scanner.scan_proxy(proxy).await;
                    result
                }
            })
            .buffer_unordered(self.concurrency_limit.available_permits())
            .collect::<Vec<_>>()
            .await
    }
}