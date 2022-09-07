use std::sync::Arc;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;

use crate::prelude::*;

fn init() -> Arc<Kook> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::filter::targets::Targets::new()
                    .with_default(LevelFilter::INFO)
                    .with_targets([(crate::KOOK, LevelFilter::TRACE)]),
            ),
        )
        .init();
    let config = Config::load_from_file();
    println!("{:?}", config);
    Kook::new_from_config(config, EchoHandler).arc()
}

pub struct EchoHandler;

#[async_trait::async_trait]
impl EventHandler for EchoHandler {
    async fn handle_group_message_event(&self, khl: &Kook, event: Event<GroupMessageExtra>) {
        let msg = event.content.clone();
        if msg.starts_with("echo") {
            khl.create_message(None, &event.target_id, &msg, None, None, None)
                .await
                .unwrap();
        }
    }
    async fn handle_person_message_event(&self, khl: &Kook, event: Event<PersonMessageExtra>) {
        if event.content.starts_with("echo") {
            khl.create_direct_message(
                Some(&event.author_id),
                None,
                &event.content,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        }
    }
}

#[tokio::test]
async fn api_test() {
    let kook = init();
    kook.start_ws().await.unwrap();
}

#[test]
fn de_test() {
    let data = std::fs::read("test_data.json").unwrap();
    let s = String::from_utf8(data).unwrap();
    println!("{:?}", serde_json::from_str::<Event<SystemExtra>>(&s));
}
