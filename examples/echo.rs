use async_trait::async_trait;
use kook::prelude::*;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::filter::targets::Targets::new()
                    .with_targets([(KOOK, LevelFilter::TRACE)]),
            ),
        )
        .init();
    let config = Config::load_from_file();
    let kook = Kook::new_from_config(config, EchoHandler).arc();
    kook.ws_loop().await.unwrap();
}
pub struct EchoHandler;

#[async_trait]
impl EventHandler for EchoHandler {
    async fn handle_message_event(&self, khl: &Kook, event: Event<MessageExtra>) {
        let msg = event.content.clone();
        if msg.starts_with("echo") {
            khl.send_direct_message(None, Some(&event.author_id), &msg, None, None, None)
                .await
                .unwrap();
        }
    }
}
