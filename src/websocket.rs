use std::sync::Arc;

use crate::messages::types::{FromMsgPack, Header, ProcessMsg};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use tokio::sync::Mutex;

pub async fn handle_socket(socket: WebSocket) {
    let (sender, mut receiver) = socket.split();
    let conc_sender = Arc::new(Mutex::new(sender));
    while let Some(msg) = { receiver.next().await } {
        match msg {
            Ok(Message::Binary(data)) => {
                let clone_sender = conc_sender.clone();
                tokio::spawn(async move {
                    Header::from_msgpack(&data)
                        .process(&mut *clone_sender.lock().await)
                        .await;
                });
            }
            Ok(Message::Close(_)) => {
                println!("[info] Client disconnected gracefully");
                return;
            }
            Ok(other) => {
                eprintln!("[warn] Unsupported message type: {:?}", other);
            }
            Err(e) => {
                eprintln!("[err ] WebSocket error: {:?}", e);
                return;
            }
        }
    }
}
