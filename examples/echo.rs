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
                    .with_default(LevelFilter::INFO)
                    .with_targets([(KOOK, LevelFilter::TRACE)]),
            ),
        )
        .init();
    let config = Config::load_from_file();
    let kook = Kook::new_from_config(config, EchoHandler).arc();
    kook.start_ws().await.unwrap();
}
pub struct EchoHandler;

#[async_trait]
impl EventHandler for EchoHandler {
    async fn handle_group_message_event(&self, khl: &Kook, event: Event<GroupMessageExtra>) {
        let msg = event.content.clone();
        if msg.starts_with("echo") {
            khl.create_message(None, &event.target_id, "echo", None, None, None)
                .await
                .unwrap();
        }
    }
}
