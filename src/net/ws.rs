use crate::prelude::*;
use crate::signal::Signal;
use futures_util::{SinkExt, StreamExt};
use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMsg, MaybeTlsStream, WebSocketStream,
};
use tracing::{trace, warn, debug};

impl crate::Kook {
    pub async fn start_ws(self: &Arc<Self>) -> KHLResult<()> {
        let gateway = self.get_gateway(false).await?;
        if let Ok((mut ws_stream, _)) = connect_async(gateway.url).await {
            let mut count = 0;
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(6)) => {
                        if self.ping(&mut count, &mut ws_stream).await {
                            break;
                        }
                    }
                    Some(Ok(msg)) = ws_stream.next() => {
                        if self.msg_handle(msg).await {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn ping(
        &self,
        count: &mut u8,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> bool {
        if *count == 0 && !self.pong.load(Ordering::SeqCst) {
            // todo
            true
        } else if *count == 3 {
            self.pong.store(false, Ordering::SeqCst);
            trace!(target: KOOK, "sending ping.");
            ws_stream
                .send(serde_json::to_string(&self.new_ping()).unwrap().into())
                .await
                .unwrap();
            *count = 0;
            false
        } else {
            *count += 1;
            false
        }
    }

    async fn msg_handle(self: &Arc<Self>, msg: WsMsg) -> bool {
        match msg {
            WsMsg::Text(t) => {
                trace!(target: KOOK, "received WsText: {}", t);
                match serde_json::from_str::<Signal>(&t) {
                    Ok(sig) => {
                        debug!(target: KOOK, "received signal: {:?}", sig);
                        match sig {
                            Signal::Event(event, sn) => {
                                self.sn.store(sn, Ordering::SeqCst);
                                let khl = self.clone();
                                if self.bot_block {
                                    if event.author().map_or(false, |user| user.bot) {
                                        debug!(target: KOOK, "blocked a bot event");
                                        return false;
                                    }
                                }
                                tokio::spawn(async move {
                                    khl.handler._handle(&khl, event).await;
                                });
                                false
                            }
                            Signal::Hello(hello) => {
                                *self.session_id.write().await = hello.session_id;
                                false
                            }
                            Signal::Ping(_) => unreachable!(),
                            Signal::Pong => {
                                self.pong.store(true, Ordering::SeqCst);
                                false
                            }
                            Signal::Resume(_) => unreachable!(),
                            Signal::Reconnect(_content) => todo!(),
                            Signal::ResumeAck(content) => {
                                *self.session_id.write().await = content.session_id;
                                false
                            }
                        }
                    }
                    Err(e) => {
                        warn!(target: KOOK, "parse signal failed {:?}", t);
                        warn!(target: KOOK, "parse error: {}", e);
                        false
                    }
                }
            }
            WsMsg::Binary(_b) => {
                unimplemented!()
            }
            WsMsg::Close(_) => true,
            _ => unreachable!(),
        }
    }
}
