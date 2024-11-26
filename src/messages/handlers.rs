use super::types::{ConcurrentWebSocket, QueryMsg};

pub async fn handle_query(
    msg: &Result<QueryMsg, rmp_serde::decode::Error>,
    _: &ConcurrentWebSocket,
) {
    if let Ok(_msg) = msg {
        println!("[info] List directory");
    } else {
        eprintln!("[err ] Couldn't parse \"Query\" type message");
    }
}
