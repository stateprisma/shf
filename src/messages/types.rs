use std::vec;

use axum::extract::ws::WebSocket;
use futures_util::stream::SplitSink;
use rmp_serde::decode;
use serde::{Deserialize, Serialize};

pub type ConcurrentWebSocket = SplitSink<WebSocket, axum::extract::ws::Message>;

#[derive(Deserialize, Debug)]
pub enum Types {
    Error = 0,
}

pub trait FromMsgPack<T: Default + for<'a> Deserialize<'a>> {
    fn from_msgpack(data: &[u8]) -> T {
        match decode::from_slice::<T>(data) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[err ] Couldn't parse incoming websocket message: {:?}", e);
                T::default()
            }
        }
    }
}

pub trait ProcessMsg<T: Default + for<'a> Deserialize<'a>> {
    /// Process incoming message
    async fn process(&self, websocket: &ConcurrentWebSocket);
}

#[derive(Deserialize, Debug)]
pub struct CommonMsg {
    /* Type */
    t: Types,
    /* Arguments */
    a: Vec<u8>,
}

impl Default for CommonMsg {
    fn default() -> Self {
        Self {
            t: Types::Error,
            a: vec![],
        }
    }
}

impl FromMsgPack<CommonMsg> for CommonMsg {}

impl ProcessMsg<CommonMsg> for CommonMsg {
    async fn process(&self, _: &ConcurrentWebSocket) {
        println!("[type] {:?}", self.t);
        match self.t {
            Types::Error => (),
        }
    }
}

#[derive(Serialize)]
pub struct QueryResp {}
