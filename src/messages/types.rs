use std::vec;

use axum::extract::ws::WebSocket;
use futures_util::stream::SplitSink;
use rmp_serde::decode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Types {
    Error = 0,
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

    pub async fn process(&self, _: &SplitSink<WebSocket, axum::extract::ws::Message>) {
        println!("[type] {:?}", self.t);
        match self.t {
            Types::Error => (),
        }
    }
}
