use axum::extract::ws::{Message, WebSocket};

use crate::messages::types::CommonMsg;

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Binary(data)) = msg {
            CommonMsg::new(&data).process().await;
        } else {
            println!("[info] Client disconnected");
            return;
        }
    }
}
