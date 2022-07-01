use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Sync + Send {
    async fn _handle(&self, khl: &Kook, event: Event<EventExtra>) {
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

    async fn handle(&self, _khl: &Kook, _event: Event<EventExtra>) {}
    async fn handle_system_event(&self, _khl: &Kook, _event: Event<SystemExtra>) {}
    async fn handle_message_event(&self, _khl: &Kook, _event: Event<MessageExtra>) {}
}
