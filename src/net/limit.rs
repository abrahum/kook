use hyper::header::HeaderValue;
use hyper::HeaderMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Default)]
pub(crate) struct RateLimit {
    pub(crate) remaining: u64,
    pub(crate) reset: Option<broadcast::Sender<()>>,
}

#[derive(Debug, Default)]
pub(crate) struct GlobalRateLimit {
    pub(crate) limit: AtomicBool,
    pub(crate) reset: AtomicU64,
    pub(crate) limits: RwLock<HashMap<String, RateLimit>>,
}

impl GlobalRateLimit {
    pub async fn check_limit(&self, bucket: &str) {
        if !self.limit.load(Ordering::SeqCst) {
            if let Some(Some(mut rx)) = self
                .limits
                .read()
                .await
                .get(bucket)
                .map(|ratelminit| ratelminit.reset.as_ref().map(|reset| reset.subscribe()))
            {
                rx.recv().await.unwrap();
            }
        } else {
            tokio::time::sleep(std::time::Duration::from_secs(
                self.reset.load(Ordering::SeqCst),
            ))
            .await;
        }
    }

    pub async fn update_limit(&self, bucket: &str, limit: u64) {
        self.limits.write().await.insert(
            bucket.to_string(),
            RateLimit {
                remaining: limit,
                reset: None,
            },
        );
    }

    pub async fn update_reset(&self, bucket: &str, reset: u64) {
        let (tx, _) = broadcast::channel(1);
        self.limits.write().await.insert(
            bucket.to_string(),
            RateLimit {
                remaining: 0,
                reset: Some(tx.clone()),
            },
        );
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(reset)).await;
            tx.send(())
        });
    }

    #[allow(dead_code)]
    pub fn update_global_limit(&self, limit: bool) {
        self.limit.store(limit, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub fn update_global_reset(&self, reset: u64) {
        self.reset.store(reset, Ordering::SeqCst);
    }

    pub async fn update_from_header(&self, header: &HeaderMap<HeaderValue>) {
        let bucket = header.get("X-Rate-Limit-Bucket").unwrap().to_str().unwrap();
        let limit = header
            .get("X-Rate-Limit-Remaining")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        if limit > 0 {
            self.update_limit(bucket, limit).await;
        } else {
            let reset = header
                .get("X-Rate-Limit-Reset")
                .unwrap()
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
            self.update_reset(bucket, reset).await;
        }
    }
}
