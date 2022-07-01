mod api;
mod card;
mod config;
mod error;
mod event;
mod handler;
mod net;
mod signal;
mod structs;

pub const KOOK: &str = "KOOK";

pub mod prelude {
    pub use crate::config::Config;
    pub use crate::error::*;
    pub use crate::event::*;
    pub use crate::handler::*;
    pub use crate::structs::*;
    pub use crate::Kook;
    pub use crate::KOOK;
}

use handler::EventHandler;
use std::sync::{
    atomic::{AtomicBool, AtomicI32},
    Arc,
};
use tokio::sync::RwLock;

pub struct Kook {
    pub author: String,

    session_id: RwLock<String>,
    sn: AtomicI32,
    pong: AtomicBool,
    limit: net::limit::GlobalRateLimit,

    http_client: net::http::HttpsClient,
    handler: Arc<dyn EventHandler>,
}

impl Kook {
    pub fn new_from_config<T>(config: config::Config, hanlder: T) -> Self
    where
        T: EventHandler + 'static,
    {
        let author = format!("Bot {}", config.bot_token);
        Self {
            author,

            session_id: RwLock::default(),
            sn: AtomicI32::default(),
            pong: AtomicBool::default(),
            limit: net::limit::GlobalRateLimit::default(),

            http_client: Self::new_https_clent(),
            handler: Arc::new(hanlder),
        }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
