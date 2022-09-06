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
            EventExtra::GroupMessage(_) => {
                self.handle_group_message_event(khl, event.down_case().unwrap())
                    .await
            }
            EventExtra::PersonMessage(_) => {
                self.handle_person_message_event(khl, event.down_case().unwrap())
                    .await
            }
        }
    }

    async fn handle(&self, _khl: &Kook, _event: Event<EventExtra>) {}
    async fn handle_system_event(&self, _khl: &Kook, _event: Event<SystemExtra>) {}
    async fn handle_group_message_event(&self, _khl: &Kook, _event: Event<GroupMessageExtra>) {}
    async fn handle_person_message_event(&self, _khl: &Kook, _event: Event<PersonMessageExtra>) {}
}
