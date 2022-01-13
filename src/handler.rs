use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Sync + Send {
    async fn _handle(&self, khl: &KHL<3>, event: Event<EventExtra>) {
        self.handle(khl, event.clone()).await;
        match &event.extra {
            EventExtra::System(_) => {
                self.handle_system_event(khl, event.down_case().unwrap())
                    .await
            }
            EventExtra::Message(_) => {
                self.handle_message_event(khl, event.down_case().unwrap())
                    .await
            }
        }
    }

    async fn handle(&self, _khl: &KHL<3>, _event: Event<EventExtra>) {}
    async fn handle_system_event(&self, _khl: &KHL<3>, _event: Event<SystemExtra>) {}
    async fn handle_message_event(&self, _khl: &KHL<3>, _event: Event<MessageExtra>) {}
}

pub struct EchoHandler;

#[async_trait]
impl EventHandler for EchoHandler {
    async fn handle_message_event(&self, khl: &KHL<3>, event: Event<MessageExtra>) {
        let msg = event.content.clone();
        if msg.starts_with("echo") {
            khl.send_direct_message(None,  Some(&event.author_id),None, msg, None, None)
                .await
                .unwrap();
        }
    }
}
