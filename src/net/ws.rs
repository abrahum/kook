use crate::prelude::*;
use crate::signal::Signal;
use futures_util::{SinkExt, StreamExt};
use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio_tungstenite::connect_async;

impl crate::KHL<3> {
    pub async fn ws_connect(self: &Arc<Self>) -> KHLResult<()> {
        let gateway = self.get_gateway(false).await?;
        if let Ok((ws_stream, _)) = connect_async(gateway.url).await {
            let (mut stx, mut srx) = ws_stream.split();
            let khl = self.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(6)).await;
                loop {
                    tokio::time::sleep(Duration::from_secs(24)).await;
                    khl.pong.store(false, Ordering::SeqCst);
                    let ping = khl.new_ping();
                    println!("ping: {:?}", ping);
                    stx.send(serde_json::to_string(&khl.new_ping()).unwrap().into())
                        .await
                        .unwrap();
                    tokio::time::sleep(Duration::from_secs(6)).await;
                    if !khl.pong.load(Ordering::SeqCst) {
                        panic!("pong not received"); //todo
                    }
                }
            });
            while let Some(Ok(msg)) = srx.next().await {
                if let Ok(sig) = serde_json::from_str::<Signal>(msg.to_string().as_str()) {
                    println!("{:?}", sig);
                    match sig {
                        Signal::Event(_, sn) => {
                            self.sn.store(sn, Ordering::SeqCst);
                        }
                        Signal::Hello(hello) => {
                            *self.session_id.write().await = hello.session_id;
                        }
                        Signal::Ping(_) => unreachable!(),
                        Signal::Pong => self.pong.store(true, Ordering::SeqCst),
                        Signal::Resume(_) => unreachable!(),
                        Signal::Reconnect(_content) => todo!(),
                        Signal::ResumeAck(content) => {
                            *self.session_id.write().await = content.session_id;
                        }
                    }
                } else {
                    println!("parse failure msg:{}", msg);
                }
            }
        }
        Ok(())
    }
}
