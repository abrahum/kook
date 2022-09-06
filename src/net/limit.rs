use dashmap::DashMap;
use hyper::header::HeaderValue;
use hyper::HeaderMap;
use tokio::sync::broadcast;
use tracing::trace;

use crate::KOOK;

#[derive(Debug, Default)]
pub(crate) struct GlobalRateLimit {
    // pub(crate) reset: Option<broadcast::Sender<()>>,
    pub(crate) limits: DashMap<String, Option<broadcast::Sender<()>>>,
}

impl GlobalRateLimit {
    pub async fn check_limit(&self, bucket: &str) {
        let rx = self
            .limits
            .get(bucket)
            .and_then(|i| i.as_ref().map(|tx| tx.subscribe()));
        if let Some(mut rx) = rx {
            rx.recv().await.ok();
            if let Some(mut v) = self.limits.get_mut(bucket) {
                trace!(target: KOOK, "api {} limited, waitting", bucket);
                *v = None;
            }
        }
    }

    pub async fn update_from_header(&self, header: &HeaderMap<HeaderValue>, bucket: &str) {
        let remaining: i32 = header
            .get("X-Rate-Limit-Remaining")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        if remaining == 0 {
            let bucket = header
                .get("X-Rate-Limit-Bucket")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let (tx, _) = broadcast::channel(1);
            let mut entry = self.limits.entry(bucket).or_default();
            *entry = Some(tx.clone());
            let reset_time: i32 = header
                .get("X-Rate-Limit-Reset")
                .unwrap()
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(reset_time as u64)).await;
                tx.send(()).ok();
            });
        } else {
            trace!(
                target: KOOK,
                "updated {} limit reaining: {}",
                bucket,
                remaining
            );
        }
    }
}
