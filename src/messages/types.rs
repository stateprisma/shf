use std::vec;

use axum::extract::ws::WebSocket;
use futures_util::stream::SplitSink;
use rmp_serde::decode;
use serde::{Deserialize, Serialize};

use crate::messages::handlers::handle_query;

pub type ConcurrentWebSocket = SplitSink<WebSocket, axum::extract::ws::Message>;

#[derive(Deserialize, Debug)]
pub enum Types {
    Error = 0,
    Query = 1,
}

#[derive(Deserialize, Debug)]
pub struct CommonMsg {
    /* Type */
    t: Types,
    /* Arguments */
    a: Vec<u8>,
}

impl CommonMsg {
    pub fn new(data: &[u8]) -> CommonMsg {
        match decode::from_slice::<Self>(data) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[err ] Couldn't parse incoming websocket message: {:?}", e);
                Self {
                    t: Types::Error,
                    a: vec![],
                }
            }
        }
    }

    pub async fn process(&self, websocket_rx: &ConcurrentWebSocket) {
        println!("[type] {:?}", self.t);
        match self.t {
            Types::Error => (),
            Types::Query => {
                handle_query(&decode::from_slice::<QueryMsg>(&self.a), &websocket_rx).await
            }
        }
    }
}

#[derive(Deserialize)]
pub struct QueryMsg {
    path: String,
}

#[derive(Serialize)]
pub struct QueryResp {}
