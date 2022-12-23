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

impl EventHandler for tokio::sync::broadcast::Sender<Event<EventExtra>> {
    fn _handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _: &'life1 Kook,
        event: Event<EventExtra>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.send(event).ok();
        })
    }
}

impl EventHandler for tokio::sync::mpsc::Sender<Event<EventExtra>> {
    fn _handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _: &'life1 Kook,
        event: Event<EventExtra>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.send(event).await.ok();
        })
    }
}

impl EventHandler for tokio::sync::mpsc::UnboundedSender<Event<EventExtra>> {
    fn _handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _: &'life1 Kook,
        event: Event<EventExtra>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.send(event).ok();
        })
    }
}

impl EventHandler for tokio::sync::watch::Sender<Event<EventExtra>> {
    fn _handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _: &'life1 Kook,
        event: Event<EventExtra>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.send(event).ok();
        })
    }
}

#[cfg(test)]
impl EventHandler for () {
    fn _handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _khl: &'life1 Kook,
        event: Event<EventExtra>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        tracing::info!(target: crate::KOOK, "{:?}", event);
        Box::pin(async move {})
    }
}
