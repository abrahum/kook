mod config;
mod error;
mod event;
mod net;
mod signal;
mod structs;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::event::*;
    pub use crate::structs::*;
    pub use crate::config::Config;
}

use std::sync::{
    atomic::{AtomicBool, AtomicI32},
    Arc,
};
use tokio::sync::RwLock;

pub struct KHL<const V: u8> {
    pub author: String,

    http_client: net::http::HttpClient,
    session_id: RwLock<String>,
    sn: AtomicI32,
    pong: AtomicBool,
}

impl<const V: u8> KHL<V> {
    pub fn new_from_config(config: config::Config) -> Self {
        use reqwest::header::HeaderMap;
        let author = format!("Bot {}", config.bot_token);
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&author).unwrap(),
        );
        let http_client = net::http::HttpClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            author,
            http_client,
            session_id: RwLock::default(),
            sn: AtomicI32::default(),
            pong: AtomicBool::default(),
        }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[tokio::test]
async fn test() {
    let config = config::Config::load_from_file();
    let khl = KHL::new_from_config(config).arc();
    khl.ws_connect().await.unwrap();
}
