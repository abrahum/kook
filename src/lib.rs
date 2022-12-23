mod api;
pub mod card;
mod config;
mod error;
mod event;
mod handler;
mod net;
mod objects;
mod signal;
mod structs;
#[cfg(test)]
mod test;

pub const KOOK: &str = "KOOK";

pub mod prelude {
    pub use crate::config::Config;
    pub use crate::error::*;
    pub use crate::event::*;
    pub use crate::handler::*;
    pub use crate::objects::*;
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
    pub bot_block: bool,

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
            bot_block: config.bot_block,

            session_id: RwLock::default(),
            sn: AtomicI32::default(),
            pong: AtomicBool::from(true),
            limit: net::limit::GlobalRateLimit::default(),

            http_client: Self::new_https_client(),
            handler: Arc::new(hanlder),
        }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Text,
    Image,
    Video,
    File,
    Audio,
    KMarkdown,
    Card,
    System,
}

impl From<MessageType> for u8 {
    fn from(ty: MessageType) -> Self {
        match ty {
            MessageType::Text => 1,
            MessageType::Image => 2,
            MessageType::Video => 3,
            MessageType::File => 4,
            MessageType::Audio => 8,
            MessageType::KMarkdown => 9,
            MessageType::Card => 10,
            MessageType::System => 255,
        }
    }
}

#[cfg(test)]
pub fn init() -> Arc<Kook> {
    use config::Config;
    use tracing::metadata::LevelFilter;
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::filter::targets::Targets::new()
                    .with_default(LevelFilter::INFO)
                    .with_targets([(KOOK, LevelFilter::TRACE)]),
            ),
        )
        .init();
    let config = Config::load_from_file();
    Kook::new_from_config(config, ()).arc()
}
