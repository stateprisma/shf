use axum::extract::ws::WebSocket;

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            println!("[info] Client disconnected");
            return;
        };

        if socket.send(msg).await.is_err() {
            println!("[info] Client disconnected");
            return;
        }
    }
}
